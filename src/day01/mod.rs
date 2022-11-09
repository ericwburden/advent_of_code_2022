pub mod input;
pub mod part1;
pub mod part2;

use crate::Part;

pub type Input = Vec<u32>;
pub type Output = u32;

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
    fn test_part_one() {
        assert_eq!(run(Part::One), 1583);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(run(Part::Two), 1627);
    }

}
