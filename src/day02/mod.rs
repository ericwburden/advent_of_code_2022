pub mod input;
pub mod part1;
pub mod part2;
pub mod shared;

use crate::{Output, Part};

// Today's input is a list of character pairs
pub type Input = Vec<(char, char)>;

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
        assert_eq!(result, 10994);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 12526);
    }
}
