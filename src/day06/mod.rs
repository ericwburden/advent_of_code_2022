pub mod input;
pub mod part1;
pub mod part2;
pub mod shared;

use crate::{Output, Part};
use input::Signal;
use shared::SequenceDetector;

pub type Input = Box<dyn Iterator<Item = Signal>>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(input::read()),
        Part::Two => part2::solve(input::read()),
    }
}

pub fn run_both() -> (Output, Output) {
    (part1::solve(input::read()), part2::solve(input::read()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, 1647);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 2447);
    }
}
