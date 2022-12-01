use crate::day01::{Input, Output};

/// Solve Day 01, Part 01
pub fn solve(input: &Input) -> Output {
    // Get the maximum calorie count for an Elf
    // and return it as an Output::U32.
    input.iter().copied().max().unwrap().into()
}
