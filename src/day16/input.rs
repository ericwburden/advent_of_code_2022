use crate::day16::Input;
use std::collections::HashMap;
use itertools::Itertools;

/// Represents a line from the input, parsed to pull out the relevant values.
/// Using this intermediate data structure because the _actual_ input gets
/// a lot of massaging before it's ready for use.
#[derive(Debug)]
pub struct ParsedInput<'a> {
    label: &'a str,
    flow: u32,
    leads_to: Vec<&'a str>,
}

impl<'a> ParsedInput<'a> {
    fn new(label: &'a str, flow: u32, leads_to: Vec<&'a str>) -> Self {
        Self {
            label,
            flow,
            leads_to,
        }
    }
}

/// Conversion for a tuple containing the parsed input values to a `ParsedInput`.
/// Really, `ParsedInput` is just a tuple with appropriate names.
impl<'a> From<(&'a str, u32, Vec<&'a str>)> for ParsedInput<'a> {
    fn from(value: (&'a str, u32, Vec<&'a str>)) -> Self {
        let (label, flow, leads_to) = value;
        ParsedInput::new(label, flow, leads_to)
    }
}

/// Module wrapping the parsing functions for parsing the input.
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, newline, u32},
        combinator::{map, verify},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        Finish, IResult,
    };

    /// Parses a valve label, which is always a string of two capital letters.
    fn label(s: &str) -> IResult<&str, &str> {
        let ver_fn = |s: &str| s.len() == 2 && s.chars().all(|c| c.is_ascii_uppercase());
        verify(alpha1, ver_fn)(s)
    }

    /// Nom parser for "Valve AA" -> "AA"
    fn valve(s: &str) -> IResult<&str, &str> {
        preceded(tag("Valve "), label)(s)
    }

    /// Parses the flow rate of a valve
    fn flow(s: &str) -> IResult<&str, u32> {
        delimited(tag(" has flow rate="), u32, tag(";"))(s)
    }

    /// Parses the list of valves that are listed at the end of each input line.
    /// Note that the prefixes use proper subject-verb agreement.
    fn leads_to(s: &str) -> IResult<&str, Vec<&str>> {
        let prefix1 = tag(" tunnels lead to valves ");
        let prefix2 = tag(" tunnel leads to valve ");
        preceded(alt((prefix1, prefix2)), separated_list1(tag(", "), label))(s)
    }

    /// Parses a line of the input into a `ParsedInput`
    fn valve_map_entry(s: &str) -> IResult<&str, ParsedInput> {
        map(tuple((valve, flow, leads_to)), ParsedInput::from)(s)
    }

    /// Parses multiple lines from the input into a list of `ParsedInput`s
    fn valve_map_entries(s: &str) -> IResult<&str, Vec<ParsedInput>> {
        separated_list1(newline, valve_map_entry)(s)
    }

    /// Main parsing function, attemtps to read the input file as a string into
    /// a list of `ParsedInput`s.
    pub fn parse(s: &str) -> Result<Vec<ParsedInput>> {
        let (_, result) = valve_map_entries(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

/// Represents a valve. Includes fields for its ID and flow rate. The ID for
/// a valve is a u64 with a single bit set. This makes keeping up with which
/// valves have been opened as easy as bitwise -or- between multiple valves.
#[derive(Debug, Copy, Clone)]
pub struct Valve {
    pub id: u64,
    pub flow: u32,
}

impl Valve {
    fn new(id: u64, flow: u32) -> Self {
        Valve { id, flow }
    }
}

/// Represents a graph indicating the connections between valves. Edges are a list
/// of lists, where each list contains a pair of (edge index, distance to edge). For
/// example, if edges[0] == [(1, 2), (5, 3)], that indicates that from the valve
/// at index [0], the valve at index [1] can be reached in 2 steps and the valve at
/// index [5] can be reached in 3 steps.
/// The nodes are just the list of `Valve`s at an index that corresponds to an index
/// in `edges`. For example, if nodes[0] is valve "AA", then edges[0] indicates the
/// valves that can be reached from valve "AA".
#[derive(Debug)]
pub struct ValveMap {
    pub edges: Vec<Vec<(usize, u32)>>,
    pub nodes: Vec<Valve>,
}

impl ValveMap {
    fn new(edges: Vec<Vec<(usize, u32)>>, nodes: Vec<Valve>) -> Self {
        Self { edges, nodes }
    }
}

/// Convert a list of parsed input lines into a ValveMap
impl From<Vec<ParsedInput<'_>>> for ValveMap {
    fn from(value: Vec<ParsedInput>) -> Self {
        // The first part is the easy part, the nodes (valves). We set each ID
        // sequentially and extract the flow rate from the parsed input line.
        let mut nodes = Vec::new();
        for (idx, entry) in value.iter().enumerate() {
            let id = 2u64.pow(idx as u32);
            let flow = entry.flow;
            let valve = Valve::new(id, flow);
            nodes.push(valve);
        }

        // Given the fact that valves and their corresponding paths out are stored
        // and referenced by index in the ValveMap and they are given by label
        // (like "AA") by the input lines, it's handy to be able to get the appropriate
        // index for a valve by it's label.
        let label_idx_map = value
            .iter()
            .enumerate()
            .map(|(idx, entry)| (entry.label, idx))
            .collect::<HashMap<_, _>>();

        // The first step in preparing the `ValveMap` is to figure out the minimum
        // distance from each valve to each other valve. This is an implementation of
        // a thing I just learned about today called the 'Floyd-Warshall' algorithm.
        // The TLDR is that we're creating a 2D vector where each row represents a
        // valve to travel from, each column represents a valve to travel to, and the
        // value at that grid intersection is the cost. To begin, we set all the travel
        // distances to (essentially) infinity, then for each parsed input line telling
        // us which valves are directly connected, set those distances to 1, since we
        // know it takes 1 minute to travel each direct connection
        let mut dist_matrix = vec![vec![u32::MAX; value.len()]; value.len()];
        for (idx, entry) in value.iter().enumerate() {
            dist_matrix[idx][idx] = 0;
            for neighbor in entry.leads_to.iter() {
                let neighbor_idx = label_idx_map[neighbor];
                dist_matrix[idx][neighbor_idx] = 1;
            }
        }

        // With the direct connections in place, we check over every 3-wise
        // permutation of grid indices. Then we compare the distance from valve [i] to
        // valve [j] directly and the distance from valves [i] > [k] > [j]. If the path
        // with the detour is shorter than the direct path, update the distance
        // from [i] > [j] with the shorter distance.
        for permutation in (0..value.len()).permutations(3) {
            let (k, i, j) = (permutation[0], permutation[1], permutation[2]);
            let detour_dist = dist_matrix[i][k].saturating_add(dist_matrix[k][j]);
            if detour_dist < dist_matrix[i][j] {
                dist_matrix[i][j] = detour_dist;
            }
        }

        // Now that we know the shortest distance between all pairs of valves, we
        // build out our list of edges. We'll only include edges to valves that
        // actually release some amount of pressure (more than 0).
        let mut edges = Vec::new();
        for (start, valve) in nodes.iter().enumerate() {
            // For each possible destination valve, decide whether it should be
            // added to the list of edges from the [start] node.
            let mut edges_from = Vec::new();
            for (end, end_valve) in nodes.iter().enumerate() {
                // Don't include edges where the destination is the same
                // as the start or the valve's flow rate is zero.
                if start == end || end_valve.flow == 0 {
                    continue;
                }

                // Otherwise, add the edge as the destination index and the
                // minimum distance from [start] to [end].
                edges_from.push((end, dist_matrix[start][end]));
            }

            edges.push(edges_from);
        }

        ValveMap::new(edges, nodes)
    }
}

const INPUT: &str = include_str!("../../input/16/input.txt");

pub fn read() -> Input {
    let entries = parser::parse(INPUT).unwrap();
    ValveMap::from(entries)
}
