use crate::day11::{Input, Monkey, Operation, Output, Rule};
use itertools::Itertools;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &Input) -> Output {
    let mut monkeys: Vec<_> = input.to_vec();
    let worry_factor = input.iter().map(|m| m.rule.divisor).product();
    let mut items_in_flight = VecDeque::new();
    for round in 0..10_000 {
        for id in 0..monkeys.len() {
            monkeys[id].handle_items_roughly(worry_factor, &mut items_in_flight);
            while let Some((item, target)) = items_in_flight.pop_back() {
                monkeys[target].catch(item);
            }
        }
    }

    // Take the number of items handled by the two most active monkeys.
    let monkey_business: Vec<_> = monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted_unstable()
        .rev()
        .take(2)
        .collect();
    (monkey_business[0] as u64 * monkey_business[1] as u64).into()
}

impl Monkey {
    fn handle_items_roughly(
        &mut self,
        worry_factor: u64,
        flight_queue: &mut VecDeque<(u64, usize)>,
    ) {
        while let Some(mut item) = self.items.pop_front() {
            item = self.operation.apply(item);
            let target = self.rule.check(item);
            item %= worry_factor;
            flight_queue.push_front((item, target));
            self.inspected += 1;
        }
    }
}
