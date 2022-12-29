use crate::day25::Input;

/// This represents one of our SNAFU numbers, which is just a String
/// in a Wrapper so we can have custom `From` implementations.
#[derive(Debug, Clone)]
pub struct Snafu(pub String);

/// Converting a line from the input into a Snafu is super complicated.
impl From<&str> for Snafu {
    fn from(line: &str) -> Self {
        Snafu(line.to_string())
    }
}

const INPUT: &str = include_str!("../../input/25/input.txt");

/// Parse that input!
pub fn read() -> Input {
    INPUT.lines().map(Snafu::from).collect::<Vec<_>>()
}
