pub mod grid;
pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};
use grid::Grid;

// These values are tuned to produce the right answer in the least
// time. It's a bit of a hack, to be honest, but it works!
pub type Input = Grid<u128, 2, 176>;

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
        assert_eq!(result, 4052);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 978);
    }
}
