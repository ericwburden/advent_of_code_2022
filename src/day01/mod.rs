pub mod input;
pub mod part1;
pub mod part2;

use crate::Part;

pub type Input = Vec<u32>;
pub type Output = u32;

pub fn run(part: Part) -> String {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input).to_string(),
        Part::Two => part2::solve(&input).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
