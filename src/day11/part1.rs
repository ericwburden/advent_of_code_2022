use crate::day11::{Input, Monkey, Operation, Output, Rule};
use itertools::Itertools;
use std::collections::VecDeque;

pub fn solve(input: &Input) -> Output {
    let mut monkeys: Vec<_> = input.to_vec();
    let mut items_in_flight = VecDeque::new();
    for _ in 0..20 {
        for id in 0..monkeys.len() {
            monkeys[id].handle_items(&mut items_in_flight);
            while let Some((item, target)) = items_in_flight.pop_back() {
                monkeys[target].catch(item);
            }
        }
    }

    let monkey_business: Vec<_> = monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted_unstable()
        .rev()
        .take(2)
        .collect();
    (monkey_business[0] * monkey_business[1]).into()
}

impl Monkey {
    fn handle_items(&mut self, flight_queue: &mut VecDeque<(u64, usize)>) {
        while let Some(mut item) = self.items.pop_front() {
            item = self.operation.apply(item);
            item /= 3;
            let target = self.rule.check(item);
            flight_queue.push_front((item, target));
            self.inspected += 1;
        }
    }

    pub fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

impl Operation {
    pub fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Add(n) => item + n,
            Operation::Mult(n) => item * n,
            Operation::Square => item * item,
        }
    }
}

impl Rule {
    pub fn check(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.success
        } else {
            self.fail
        }
    }
}
