use super::part1::Factory;
use crate::day19::{Input, Output};
use rayon::prelude::*;

/// Solve Day 19, Part 2
pub fn solve(input: &Input) -> Output {
    // That's right, it's basically the same as part one, with the slight
    // modifications that we're only examining the first three blueprints,
    // keeping the number of geodes per Blueprint as opposed to the quality
    // level, and multiplying instead of adding the results.
    input
        .par_iter()
        .take(3)
        .map(|blueprint| Factory::new(*blueprint, 32))
        .map(|factory| factory.geodes_produced())
        .product::<u32>()
        .into()
}
