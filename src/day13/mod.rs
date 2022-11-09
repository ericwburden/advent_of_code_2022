pub mod input;
pub mod part1;
pub mod part2;

use crate::Part;

pub type Input = u8;
pub type Output = u8;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}
