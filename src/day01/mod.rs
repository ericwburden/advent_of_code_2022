pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

// Input for today is a vector of numbers, each of which represents the
// total number of calories carried by each Elf. In my first stab at this,
// I used a wrapper around a `Vec<Vec<u32>>`, keeping track of each item.
// Turns out, all I needed was the total calories per elf!
pub type Input = Vec<u32>;

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
        assert_eq!(result, 69795);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 208437);
    }
}
