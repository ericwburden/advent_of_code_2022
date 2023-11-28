pub mod input;
pub mod part1;

use crate::{Output, Part};
use input::Snafu;

pub type Input = Vec<Snafu>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => String::from("No part 2 for Day 25!").into(),
    }
}

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (
        part1::solve(&input),
        Output::String(String::from("Nothing to see here..")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, "2-=102--02--=1-12=22");
    }
}
