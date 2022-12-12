use crate::day11::{Input, Monkey, Operation, Output};
use itertools::Itertools;

/// Solve Day 11, Part 2
pub fn solve(input: &Input) -> Output {
    // Similar to last time, but somehow worse...
    let mut monkey_game = WorseGame::from(input);

    // One of the ways it's worse is that it goes on for 500x longer
    (0..10_000).for_each(|_| monkey_game.play_rough());

    // Calculate and return the extreme level of monkey business
    monkey_game.max_monkey_business().into()
}

/// Represents the more intense version of the cruel, cruel game being played by
/// these dastardly monkeys. Includes the same fields as the original `CruelGame`,
/// plus the limit where your stress levels become overwhelming and you black out,
/// experiencing short-term memory loss.
pub struct WorseGame {
    items_in_flight: Vec<(u64, usize)>,
    monkeys: Vec<Monkey>,
    absolute_limit: u64,
}

impl WorseGame {
    // Start up a much worse version of the monkey game
    fn from(monkeys: &[Monkey]) -> Self {
        let monkeys = monkeys.to_vec();
        let items_in_flight = Vec::new();

        // In order to keep the computer from blowing its stack, too, we need to
        // identify the periodicity of the item worry values. This turns out to
        // be the least common multiple of all the monkey rule divisors by which
        // the monkeys check where to fling your things. Since all these divisors
        // are prime, this is the product of all the divisors.
        let absolute_limit = monkeys.iter().map(|m| m.rule.divisor).product();

        WorseGame {
            items_in_flight,
            monkeys,
            absolute_limit,
        }
    }

    // The monkeys have upped their level of maliciousness and now pretend to drop
    // and break your items on each turn. You're pretty sure they've broken a few
    // things already, but they're throwing them so fast you can't tell for sure.
    fn play_rough(&mut self) {
        // For each monkey...
        for id in 0..self.monkeys.len() {
            // Have the monkey handle each item it's currently holding with obnoxious
            // roughness and glee, adding items to the list of items in flight after
            // handling each.
            self.monkeys[id].handle_items_roughly(self.absolute_limit, &mut self.items_in_flight);

            // For each item in flight, have the monkey it was tossed to deftly
            // snatch it, along with your hopes and dreams, from the air.
            while let Some((item, target)) = self.items_in_flight.pop() {
                self.monkeys[target].catch(item);
            }
        }
    }

    /// Calculate and return the maximum monkey business. Identifies the number
    /// of items handled by each monkey and returns the product of the two
    /// largest totals.
    fn max_monkey_business(&self) -> u64 {
        let monkey_business: Vec<_> = self
            .monkeys
            .iter()
            .map(|m| m.inspected)
            .sorted_unstable()
            .rev()
            .take(2)
            .collect();
        (monkey_business[0] as u64 * monkey_business[1] as u64)
    }
}

impl Monkey {
    /// Have a monkey handle your precious items with exuberant malice, purposely
    /// stoking your alarm.
    fn handle_items_roughly(
        &mut self,
        absolute_limit: u64,
        items_in_flight: &mut Vec<(u64, usize)>,
    ) {
        while let Some(mut item) = self.items.pop() {
            // Increase your worry over that item according to the puzzle rules.
            item = self.operation.apply(item);

            // Black out for a moment from the stress caused by these monkeys
            // tossing your precious things about, experiencing an odd form of
            // amnesia and "resetting" your stress levels a bit.
            item %= absolute_limit;

            // Have the monkey decide on a target with a malicious glint in
            // its beady monkey eyes.
            let target = self.rule.check(item);

            // Toss the item to its intended target.
            items_in_flight.push((item, target));

            // Increment the number of items this monkey has inspected
            self.inspected += 1;
        }
    }
}
