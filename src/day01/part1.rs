use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input.windows(2).filter(|x| x[0] < x[1]).count() as u32
}
