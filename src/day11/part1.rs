use crate::day11::{Input, Monkey, Output};
use itertools::Itertools;

/// Solve Day 11, Part 1
pub fn solve(input: &Input) -> Output {
    // Initiate a cruel, cruel game played by monkeys
    let mut monkey_game: CruelGame = CruelGame::from(input);

    // "Play" the game for 20 rounds
    (0..20).for_each(|_| monkey_game.play());

    // Return the maximum monkey business, AKA the product of the
    // number of items handled by the two most rambunctious monkeys.
    monkey_game.max_monkey_business().into()
}

/// Represents the cruel and insensitive game being played by the monkeys,
/// at your expense. Includes fields for the list of monkeys where their index
/// also indicates their ID and for the items currently flying through the
/// air from one monkey to another.
pub struct CruelGame {
    items_in_flight: Vec<(u64, usize)>,
    monkeys: Vec<Monkey>,
}

impl CruelGame {
    /// Produce a `CruelGame` from a list of monkeys
    fn from(monkeys: &[Monkey]) -> Self {
        let monkeys = monkeys.to_vec();
        let items_in_flight = Vec::new();
        CruelGame {
            items_in_flight,
            monkeys,
        }
    }

    /// "Play" one round of the "game".
    fn play(&mut self) {
        // For each monkey...
        for id in 0..self.monkeys.len() {
            // Have the monkey handle each item it's currently holding, adding
            // items to the list of items in flight after handling each.
            self.monkeys[id].handle_items(&mut self.items_in_flight);

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
        (monkey_business[0] * monkey_business[1]).into()
    }
}

impl Monkey {
    /// Have a monkey handle your precious items with callous disregard
    /// for your concerns.
    fn handle_items(&mut self, items_in_flight: &mut Vec<(u64, usize)>) {
        // For each item the monkey has...
        while let Some(mut item) = self.items.pop() {
            // Increase your worry over that item according to the puzzle rules.
            item = self.operation.apply(item);

            // Calm down a bit since the monkey didn't break it (this time).
            item /= 3;

            // Have the monkey decide on a target with a mischievous gleam in
            // its beady monkey eyes.
            let target = self.rule.check(item);

            // Toss the item to its intended target.
            items_in_flight.push((item, target));

            // Increment the number of items this monkey has inspected
            self.inspected += 1;
        }
    }

    /// Catch an item thrown from another monkey. Probably pretend to fumble it
    /// or something just to get that human even more riled up.
    pub fn catch(&mut self, item: u64) {
        self.items.push(item);
    }
}
