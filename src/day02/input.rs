use crate::day02::Input;
use nom::{
    character::complete::{anychar, space1},
    error::Error as NomError,
    sequence::separated_pair,
    Finish, IResult,
};

const INPUT: &str = include_str!("../../input/02/input.txt");

/// Attempts to parse a line from the INPUT
fn parse_line(line: &str) -> Result<(char, char), NomError<&str>> {
    // parses lines that contain a character, one or more spaces,
    // then another character
    let (_, char_pair) = separated_pair(anychar, space1, anychar)(line).finish()?;
    Ok(char_pair)
}

/// Parse the INPUT
pub fn read() -> Input {
    // Parse the lines into (char, char) values and return the resulting
    // list. Ignores any lines that fail to parse.
    INPUT.lines().flat_map(parse_line).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input_parsing() {
        let input = read();

        let first_round = *input.first().expect("There should be at least one round!");
        let first_expected = ('B', 'X');
        assert_eq!(first_round, first_expected);

        let last_round = *input.last().expect("There should be at least one round!");
        let last_expected = ('B', 'Y');
        assert_eq!(last_round, last_expected);

        assert_eq!(input.len(), 2500);
    }
}
