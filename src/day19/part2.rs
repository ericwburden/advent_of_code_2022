use super::part1::Factory;
use crate::day19::{Input, Output};
use rayon::prelude::*;

pub fn solve(input: &Input) -> Output {
    input
        .par_iter()
        .take(3)
        .map(|blueprint| Factory::new(*blueprint, 32))
        .map(|factory| factory.geodes_produced())
        .product::<u32>()
        .into()
}
