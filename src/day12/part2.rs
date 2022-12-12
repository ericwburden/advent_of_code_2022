use crate::day12::{Hill, HillMap, Input, Output};
use core::cmp::Reverse;
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};

/// Solve Day 12, Part 2
pub fn solve(input: &Input) -> Output {
    // Turns out we need to 'invert' our `HillMap` in order to efficiently find the
    // shortest path to _any_ hill with a height of 0.
    let descent_map = DescentMap::from(input);

    // Get the number of steps from the summit to _every_ single hill in
    // the jungle!
    let steps_to_short_hills = descent_map.shortest_paths_from_summit();

    // Check each hill to see if it's a short hill, and if it is, check to see if
    // it's the shortest path to a short hill found so far. If so, record it!
    let mut shortest_path = u32::MAX;
    for (pos, steps_to_pos) in steps_to_short_hills.iter() {
        let (row, col) = *pos;
        let Hill::Hill(0) = descent_map.hills[row][col] else { continue; };
        shortest_path = min(shortest_path, *steps_to_pos);
    }

    // Return the shortest path to a short hill
    shortest_path.into()
}

// Type alias we'll use here to refer to the hills that can reach the current hill
type Neighbors = [Option<(usize, usize)>; 4];

// Very much like the `HillMap`. The biggest difference is that now `graph` represents
// relationships between hills that can be moved _to_ and the hills that can reach
// them (the reverse of the relationship for `HillMap`).
struct DescentMap {
    hills: Vec<Vec<Hill>>,
    graph: HashMap<(usize, usize), Neighbors>,
    summit: (usize, usize),
}

// Produce a `DescentMap` from a reference to a `HillMap`.
impl From<&HillMap> for DescentMap {
    fn from(hill_map: &HillMap) -> Self {
        // We need to invert the graph, so that we can essentially walk backwards
        // starting from the summit (our previous end point) down to all those
        // short hills.
        let mut graph: HashMap<(usize, usize), Neighbors> = HashMap::new();

        // For each entry in the `HillMap`s graph...
        for (pos, neighbors) in hill_map.graph.iter() {
            // For each neighbor in the entry's list of neighbors (skipping the empty
            // spaces in the neighbor array)
            for neighbor in neighbors.iter().flatten() {
                // Yeah, this is a bit of a gnarly iterator chain. Here's what's going
                // on: We're checking the entry in our inverted `graph` where the
                // neighbor is the key, creating an entry with an empty set of
                // neighbors if the neighbor doesn't have an entry yet. Then, for each
                // slot in the value array for `neighbor`, find the first index that
                // doesn't have a value yet and put `pos` there. This 'inverts' the
                // relationships by making `neighbor` the key and adding `pos` as one
                // of the positions from which `neighbor` can be reached.
                graph
                    .entry(*neighbor)
                    .or_default()
                    .iter_mut()
                    .filter(|slot| slot.is_none())
                    .take(1)
                    .for_each(|slot| *slot = Some(*pos));
            }
        }

        // Copy the `hills` and `end_at` fields from the `HillMap`
        let hills = hill_map.hills.to_vec();
        let summit = hill_map.end_at;

        // Return the new `DescentMap` with the inverted graph.
        DescentMap {
            hills,
            graph,
            summit,
        }
    }
}

impl DescentMap {
    /// Identify and return the minimum number of steps every other hill is from
    /// the summit as a HashMap where the keys are hill positions and the values
    /// are the number of steps from the summit.
    pub fn shortest_paths_from_summit(&self) -> HashMap<(usize, usize), u32> {
        // The procedure here is the same Dijkstra's algorithm from part one, just
        // walking down from the summit instead of up from the start space.
        let start_at = self.summit;
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);
        let mut steps = HashMap::from([(start_at, 0)]);

        // While there are still hills to explore...
        while let Some((_, pos)) = open.pop() {
            // No need for an early return here, we want to find a path to _all_ the
            // other hills.

            // As before, we check all the neighbors and any time we're able to
            // reach that neighbor by the shortest path found so far, we add that
            // neighbor to the open set.
            let Some(neighbors) = self.graph.get(&pos) else { continue; };
            for maybe_neighbor in neighbors {
                let Some(neighbor) = maybe_neighbor else { continue; };
                let next_steps: u32 = steps.get(&pos).unwrap() + 1;
                let curr_steps: u32 = *steps.get(neighbor).unwrap_or(&u32::MAX);
                if next_steps >= curr_steps {
                    continue;
                }
                open.push((Reverse(next_steps), *neighbor));
                steps.insert(*neighbor, next_steps);
            }
        }

        // Returns a mapping of the fewest steps to every hill from the summit
        steps
    }
}
