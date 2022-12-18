pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};
use input::{GasJetIter, Push};

pub type Input = GasJetIter;

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
        assert_eq!(result, 3147);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
        // 3155385382 is low
    }
}
