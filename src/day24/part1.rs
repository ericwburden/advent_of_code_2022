use super::input::{Space, Valley};
use crate::day24::{Input, Output};
use core::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Solve Day 24, Part 1
pub fn solve(input: &Input) -> Output {
    let first_state = input.get(0).expect("Valley should have an initial state!");

    // All examples and input have the start Space at index (0, 1) and the end
    // Space on the bottom row, next to the last column.
    let start_at = Expedition(0, 1);
    let end_at = Expedition(first_state.rows - 1, first_state.cols - 2);

    // Calculate the length of the shortest path through the Valley.
    if let Some(minutes) = start_at.shortest_path(end_at, 0, input) {
        return (minutes as u32).into();
    }

    // Unless we can't find a path. Then, freak out! This doesn't happen,
    // though. Not anymore...
    panic!("Could not find a way through the valley. Died of frostbite!");
}

/// Represents the location of the elven Expedition through the Valley.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Expedition(pub usize, pub usize);

impl Expedition {
    /// Given the current location of the elves and a state of the Valley,
    /// determine which next steps are possible, in order to avoid landing
    /// on a Space with an active Blizzard. There are a maximum of five
    /// possible moves, including waiting.
    fn possible_next_steps(&self, valley: &Valley) -> [Option<Expedition>; 5] {
        let Expedition(row, col) = *self;
        let mut possible_steps = [None; 5];

        // Attempt to move to the left
        if let Space::Empty = valley.spaces[row][col - 1] {
            possible_steps[0] = Some(Expedition(row, col - 1));
        }

        // Attempt to move to the right
        if let Space::Empty = valley.spaces[row][col + 1] {
            possible_steps[1] = Some(Expedition(row, col + 1));
        }

        // The upward move needs to account for the fact that the
        // final space is in the top row, which means that moving into
        // the top row is possible.
        if row > 0 {
            if let Space::Empty = valley.spaces[row - 1][col] {
                possible_steps[2] = Some(Expedition(row - 1, col));
            }
        }

        // The downard move needs to account for the beginning space
        // being on the last row, which means that moving into the last row
        // is possible.
        if row < (valley.spaces.len() - 1) {
            if let Space::Empty = valley.spaces[row + 1][col] {
                possible_steps[3] = Some(Expedition(row + 1, col));
            }
        }

        // Waiting is a necessary option if there's nothing in our current space
        if let Space::Empty = valley.spaces[row][col] {
            possible_steps[4] = Some(Expedition(row, col));
        }

        possible_steps
    }

    /// Find the shortest path from this Expedition's location to the `target`,
    /// assuming we start the journey at minute `start_time`. Pass in a reference
    /// to the time states of the Valley so we can know which Spaces are Empty
    /// for each minute. It's a Dijkstra's implementation.
    pub fn shortest_path(
        &self,
        target: Expedition,
        start_time: usize,
        valley_states: &[Valley],
    ) -> Option<usize> {
        // Sort locations in the Heap by the least number of minutes spent. Uniquely
        // identify states along the path by the location of the Expedition for a
        // given state of the Valley.
        let mut open = BinaryHeap::from([(Reverse(start_time), *self)]);
        let mut minutes = HashMap::from([((*self, start_time % valley_states.len()), start_time)]);

        // So long as there are states to explore, take the one with the fewest
        // number of minutes passed.
        while let Some((Reverse(minutes_passed), expedition)) = open.pop() {
            // If we've found the end, return the number of minutes passed
            if expedition == target {
                return Some(minutes_passed);
            }

            // Get the state of the Valley in the next minute to identify
            // which Spaces are available to be moved to.
            let state_idx = (minutes_passed + 1) % valley_states.len();
            let state = &valley_states[state_idx];

            // Check each next step to see if this is the fastest we've gotten
            // to that state. If so, keep it and add it to the heap. Otherwise,
            // keep moving on.
            for step in expedition.possible_next_steps(state).into_iter().flatten() {
                let next_minutes = minutes_passed + 1;
                let curr_minutes = *minutes.get(&(step, state_idx)).unwrap_or(&usize::MAX);
                if next_minutes >= curr_minutes {
                    continue;
                }
                open.push((Reverse(next_minutes), step));
                minutes.insert((step, state_idx), next_minutes);
            }
        }

        None // Something has gone terribly wrong...
    }
}
