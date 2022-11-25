pub mod input;
pub mod part1;

use crate::Part;

pub type Input = u8;
pub type Output = u8;


pub fn run(part: Part) -> String {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input).to_string(),
        Part::Two => String::from("No part 2 for Day 25!"),
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
}
