pub mod input;
pub mod part1;
pub mod part2;
pub mod shared;

use crate::{Output, Part};
use input::Motion;
use shared::Position;

pub type Input = Vec<Motion>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, 6175);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 2578);
    }
}
