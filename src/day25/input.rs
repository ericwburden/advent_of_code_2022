use crate::day25::Input;

#[derive(Debug, Clone)]
pub struct Snafu(pub String);

impl From<&str> for Snafu {
    fn from(line: &str) -> Self {
        Snafu(line.to_string())
    }
}

const INPUT: &str = include_str!("../../input/25/input.txt");

pub fn read() -> Input {
    INPUT.lines().map(Snafu::from).collect::<Vec<_>>()
}
