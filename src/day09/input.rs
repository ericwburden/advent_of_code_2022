use crate::day09::Input;

/// Represents one of the motions specified in the input, either up,
/// down, left, or right by a given distance (or number of steps).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

/// Module wrapping the parser for today's puzzle. Produces a `Vec<Motion>`.
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
        multi::separated_list1, sequence::preceded, Finish, IResult,
    };

    /// Nom parser for "U 5" -> Motion::Up(5)
    fn up(s: &str) -> IResult<&str, Motion> {
        map(preceded(tag("U "), u8), Motion::Up)(s)
    }

    /// Nom parser for "D 5" -> Motion::Down(5)
    fn down(s: &str) -> IResult<&str, Motion> {
        map(preceded(tag("D "), u8), Motion::Down)(s)
    }

    /// Nom parser for "L 5" -> Motion::Left(5)
    fn left(s: &str) -> IResult<&str, Motion> {
        map(preceded(tag("L "), u8), Motion::Left)(s)
    }

    /// Nom parser for "R 5" -> Motion::Right(5)
    fn right(s: &str) -> IResult<&str, Motion> {
        map(preceded(tag("R "), u8), Motion::Right)(s)
    }

    /// Nom parser to take all the lines of the input and produce a vector
    /// of `Motion`s
    pub fn parse(s: &str) -> Result<Vec<Motion>> {
        let result = separated_list1(tag("\n"), alt((up, down, left, right)))(s);
        let (_, motions) = result
            .finish()
            .map_err(|_| anyhow!("Could not parse motions!"))?;
        Ok(motions)
    }
}

const INPUT: &str = include_str!("../../input/09/input.txt");

pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 2000);

        let first_found = *input.first().unwrap();
        let first_expected = Motion::Down(2);
        assert_eq!(first_found, first_expected);

        let last_found = *input.last().unwrap();
        let last_expected = Motion::Right(3);
        assert_eq!(last_found, last_expected);
    }
}
