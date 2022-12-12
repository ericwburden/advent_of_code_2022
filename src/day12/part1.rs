use crate::day12::{Hill, HillMap, Input, Output};
use core::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Solve Day 12, Part 1
pub fn solve(input: &Input) -> Output {
    // Starting at the start hill, count the number of steps to the end hill
    // with an implementation of Dijkstra's algorithm.
    let start_at = input.start_at;
    input.shortest_path_to_summit(start_at).unwrap().into()
}

impl HillMap {
    /// Dijkstra's Algorithm!!!
    pub fn shortest_path_to_summit(&self, start_at: (usize, usize)) -> Option<u32> {
        // The 'open set': the hills we know how to travel to, but don't
        // know how to travel _from_ yet. You can think of this like the expanding
        // outer edge of our search space, if that helps. Because it's a binary
        // heap (we're using as a _min_ binary heap), the next hill to be fetched
        // will always be the one with the shortest travel time found so far.
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);

        // Maintain a listing of the shortest number of steps to each hill we've
        // traveled to. It's the shortest number of steps _so far_, it's possible
        // to update these if a shorter path is found.
        let mut steps = HashMap::from([(start_at, 0)]);

        // So long as there are hills left to climb...
        while let Some((_, pos)) = open.pop() {
            // Check the hill we're currently on. If it's the end, then just return
            // the number of steps it took us to get here.
            let (row, col) = pos;
            if pos == self.end_at {
                return steps.get(&pos).copied();
            }

            // Otherwise, see if this hill has any neighbors we can reach. If not,
            // skip it and move on to the next hill in our 'open set'.
            let Some(neighbors) = self.graph.get(&pos) else { continue; };

            // For each direction where there might be a neighbor...
            for maybe_neighbor in neighbors {
                // If there's no reachable neighbor, try the next direction.
                let Some(neighbor) = maybe_neighbor else { continue; };

                // Otherwise, calculate how many steps it will take to get to that
                // neighbor from the path you're currently on. That is, one more step
                // than it took to get to the current hill.
                let next_steps: u32 = steps.get(&pos).unwrap() + 1;

                // Check how many steps are in the current shortest path to that neighbor
                let curr_steps: u32 = *steps.get(neighbor).unwrap_or(&u32::MAX);

                // If we've already found a shorter way to get there, we can just
                // move on.
                if next_steps >= curr_steps {
                    continue;
                }

                // If we're on the shortest path, then add the neighbor to the open
                // set and record the number of steps
                open.push((Reverse(next_steps), *neighbor));
                steps.insert(*neighbor, next_steps);
            }
        }

        None
    }
}
