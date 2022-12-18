use crate::day17::{Input, Output};
use super::part1::{Chamber, Rock};
use std::collections::HashMap;

/// Solve Day 17, Part 2
pub fn solve(input: &Input) -> Output {
    // We need an owned copy of the iterator so we can use it in both parts
    let mut gas_jets = input.to_owned();
    let total_rocks = 1_000_000_000_000; // That's a _lot_ of rocks...
    let mut rocks_added = 0; // Number of rocks added to the chamber

    // Keep up with the states we've seen of the chamber before. The state includes
    // the top 8 levels of the chamber, the shape of the last rock added to the 
    // chamber, and the current internal index of the `gas_jets`.
    let mut seen = HashMap::with_capacity(2048);

    // Why use 2048 as the capacity now? It's larger than the repeat length of the
    // chamber. What repeat length? Well, as it turns out, typically when Advent of
    // Code asks you what the state of some data structure will be one bazillion
    // iterations later, there's probably a cycle in the state of the data structure
    // somewhere that can be used to calculate the final state without needing to
    // simulate each step. In my case, the state of the top layers of the chamber
    // repeated every 1700 rocks dropped or so.
    let mut chamber = Chamber::with_capacity(2048);

    // How much height has been accumulated so far?
    let mut accumulated_height = 0;

    // An iterator to produce rocks of the appropriate shape in a cycle.
    let mut rock_types = Rock::all().into_iter().cycle();

    // Until we've added all gajillion rocks...
    while rocks_added < total_rocks {

        // Get the next rock, add it to the chamber, account for it, and if the 
        // chamber has fewer than 8 levels, do it again. We're checking the state
        // based on the top 8 levels, so no checking until we have at least 8 levels.
        let rock = rock_types.next().unwrap();
        chamber.add_rock(&mut gas_jets, rock);
        rocks_added += 1;
        if chamber.height() < 8 {
            continue;
        }

        // Check to see if we've seen this state of the top 8 levels of the chamber
        // before. If so, time to use that cycle!
        let state = (chamber.skyline(), rock, gas_jets.idx);
        if let Some((prev_rocks_added, prev_height)) = seen.get(&state) {
            // The number of rocks added in each repeating cycle.
            let repeat_len: usize = rocks_added - prev_rocks_added;

            // The number of repeats left before we add the final rock.
            let repeats: usize = (total_rocks - rocks_added) / repeat_len;

            // Add all the rocks in all the repeating cycles between here and the end.
            rocks_added += repeat_len * repeats;

            // Add the chamber height of the cycle to the accumulated height
            accumulated_height += repeats * (chamber.height() - prev_height);

            // Clear the map of seen states. We don't want to do everything inside
            // this `if` block again on the next iteration, after all.
            seen.clear();
            continue;
        }

        // If we haven't seen this state before, add it to the map and keep going.
        seen.insert(state, (rocks_added, chamber.height()));
    }

    // Report the current height of the chamber and the accumulated height from all
    // the cycles. The chamber will contain all the rocks dropped up to the start
    // of the second repetition of the cycle, then all the rocks that would be
    // dropped after the last full cycle ended.
    (chamber.height() as u64 + accumulated_height as u64).into()
}

impl Chamber {
    /// Get the 'skyline' of the top of the chamber. Really, it's just a u64 with 
    /// bits representing the top 8 levels of the chamber.
    fn skyline(&self) -> Option<u64> {

        // If the chamber is less than 8 levels tall, we can't take a skyline
        if self.height() < 8 {
            return None;
        }

        // Take the top 8 levels of the chamber and fold them into a 64-bit integer.
        let result = self
            .0
            .iter()
            .rev()
            .take(8)
            .fold(0u64, |acc, byte| (acc << 8) | *byte as u64);
        Some(result)
    }
}
