pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};
use input::{Offset, Point};
use part1::CaveMap;
use std::collections::HashSet;

pub type Input = HashSet<Point>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, 625);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 25193);
    }
}
