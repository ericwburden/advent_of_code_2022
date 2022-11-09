use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input.windows(4).filter(|x| x[0] < x[3]).count() as u32
}
