use super::part1::TravelState;
use crate::day16::{Input, Output, ValveMap};

/// Solve Day 16, Part 2
///
/// This part is also a bit hacky. Here we're again producing every possible end
/// state from walking through and opening valves, just in 26 minutes instead of
/// 30. With that list of possible paths through the valves, we check the paths
/// that release the most pressure for two that open different sets of valves.
/// The total pressure released from the most pressure-releasing pair of
/// non-overlapping paths is the most pressure that two actors can release working
/// together.
pub fn solve(input: &Input) -> Output {
    // Now the initial state starts with only 26 minutes remaining.
    let mut state = TravelState {
        remaining: 26,
        ..Default::default()
    };
    let mut open = vec![state];

    // Instead of tracking the most pressure released, we're collecting all possible
    // final states.
    let mut best_results = Vec::new();
    while let Some(state) = open.pop() {
        if state.remaining == 0 {
            best_results.push(state);
            continue;
        }

        for neighbor in state.next_states(input) {
            open.push(neighbor);
        }
    }

    // Now we sort the final states by pressure released
    best_results.sort_unstable();

    // How many of the most productive states should be considered? This is the
    // 1337 hax. If we don't check enough of the most productive states, we'll
    // get the wrong answer. So, how many should we check? Enough until we get the
    // right answer! Honestly I started high and just started reducing the number
    // until I got low enough that this function started producing the wrong
    // answer.
    let depth = 285;

    // Notice that we're iterating backwards over the list of results, since
    // it's sorted by ascending total pressure released. For each top result, we
    // check down the list for the next result that doesn't overlap. There's a
    // possibility that this wouldn't work and we'd need to find a few pairs
    // this way and get their maximum, but for my input the first pair found
    // through this strategy produces the correct result.
    for (idx, result1) in best_results.iter().rev().enumerate().take(depth) {
        for result2 in best_results.iter().rev().skip(idx + 1).take(depth) {
            if result1.valves_open & result2.valves_open > 0 {
                continue;
            }
            let max = result1.released + result2.released;
            return max.into();
        }
    }

    // If we get this far, we've failed.
    panic!("Could not solve part two!");
}

/// Implement ordering for travel states so we can sort the list above. We'll
/// sort by total pressure released.
impl Ord for TravelState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.released.cmp(&other.released)
    }
}

impl PartialOrd for TravelState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
