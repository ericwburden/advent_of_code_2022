use crate::day14::Input;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

/// Represents a point on the 2D grid where the rocks are
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub u32, pub u32);

/// An offset to a point, used to adjust `Point`s
#[derive(Debug, Default, Clone, Copy)]
pub struct Offset(pub i32, pub i32);

impl Point {
    /// Calculate the unit offset from one `Point` to another. This represents
    /// the incremental step that, if taken repeatedly, will take you from
    /// `other` to `self`.
    fn offset_from(&self, other: &Self) -> Offset {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;
        match (x1.cmp(x2), y1.cmp(y2)) {
            (Ordering::Less, Ordering::Less) => Offset(-1, -1),
            (Ordering::Less, Ordering::Equal) => Offset(-1, 0),
            (Ordering::Less, Ordering::Greater) => Offset(-1, 1),
            (Ordering::Equal, Ordering::Less) => Offset(0, -1),
            (Ordering::Equal, Ordering::Equal) => Offset(0, 0),
            (Ordering::Equal, Ordering::Greater) => Offset(0, 1),
            (Ordering::Greater, Ordering::Less) => Offset(1, -1),
            (Ordering::Greater, Ordering::Equal) => Offset(1, 0),
            (Ordering::Greater, Ordering::Greater) => Offset(1, 1),
        }
    }
}

/// Implementation to allow adding an `Offset` to a `Point` to
/// move the `Point` in 2D space.
impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, rhs: Offset) -> Self::Output {
        let Point(px, py) = self;
        let Offset(ox, oy) = rhs;
        let x = px.saturating_add_signed(ox);
        let y = py.saturating_add_signed(oy);
        Point(x, y)
    }
}

/// Module wrapping the input parser to parse lines from the input.
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag,
        character::complete::{newline, u32},
        multi::separated_list1,
        sequence::separated_pair,
        Finish, IResult,
    };

    /// Nom parser for "15,30" -> Point(15, 30)
    fn point(s: &str) -> IResult<&str, Point> {
        let (s, (first, second)) = separated_pair(u32, tag(","), u32)(s)?;
        Ok((s, Point(first, second)))
    }

    /// Nom parser to convert a list like
    /// "15,30 -> 15,45" -> [Point(15, 30), Point(15, 45)]
    fn point_list(s: &str) -> IResult<&str, Vec<Point>> {
        separated_list1(tag(" -> "), point)(s)
    }

    /// Nom parser to convert all lines to a list of lists of `Point`s
    fn point_lists(s: &str) -> IResult<&str, Vec<Vec<Point>>> {
        separated_list1(newline, point_list)(s)
    }

    /// Parse the input file into a list of lists of `Point`s
    pub fn parse(s: &str) -> Result<Vec<Vec<Point>>> {
        let (_, result) = point_lists(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

/// A struct to house everything we need to iterate one space on the 2D grid
/// at a time to follow a line of rocks, given by a start and end `Point`.
/// We'll use this iterator to produce all the `Point`s containing rocks
/// between `start` and `end`.
struct RockLineIter {
    start: Point,        // The point where the rock line starts
    end: Point,          // The point where the rock line ends
    offset: Offset,      // The incremental change from `start` to `end`
    next: Option<Point>, // The next item to return from this iterator
}

/// A trait for a pair of points to implement to produce a list of rocky points
/// in the line from one point to another.
trait RockLine {
    fn rock_line(self) -> RockLineIter;
}

/// Take a pair of points and return a `RockLineIter`.
impl RockLine for (Point, Point) {
    fn rock_line(self) -> RockLineIter {
        let (start, end) = self;
        let offset = end.offset_from(&start);
        RockLineIter {
            start,
            end,
            offset,
            next: Some(start), // The first point returned is the start
        }
    }
}

/// Let `RockLineIter` be used for iteration! Produces all the points containing
/// rocks from `start` to `end`.
impl Iterator for RockLineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None, // This is how we know when `RockLineIter` is empty
            Some(current) => {
                // If we're currently on the end point, then we've emptied the
                // iterator. Otherwise, the next point returned will be the
                // current point plus the offset.
                self.next = if current == self.end {
                    None
                } else {
                    Some(current + self.offset)
                };

                Some(current)
            }
        }
    }
}

const INPUT: &str = include_str!("../../input/14/input.txt");

/// Parse the input!
pub fn read() -> Input {
    // List of lists of `Point`s, basically the input file
    let point_lists = parser::parse(INPUT).unwrap();

    // The set of points that contain obstacles (rocks)
    let mut obstacles = HashSet::new();

    // For each list of `Point`s...
    for point_list in point_lists {
        // For each pair of points in that list...
        for point_pair in point_list.into_iter().tuple_windows::<(_, _)>() {
            // For each "rocky" point in the line of rocks...
            for rock_point in point_pair.rock_line() {
                // Add that rock point to the set of obstacles
                obstacles.insert(rock_point);
            }
        }
    }

    // Return our set of points that sand can't cross
    obstacles
}
