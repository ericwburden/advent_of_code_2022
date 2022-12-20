use crate::day18::Input;

/// Represents a 1x1x1 cube in 3D space
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube(i32, i32, i32);

// Just some convenience functions for working with Cubes
impl Cube {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Cube(x, y, z)
    }

    pub fn inner(&self) -> (i32, i32, i32) {
        let Cube(x, y, z) = self;
        (*x, *y, *z)
    }
}

/// Convert a tuple of i32s into a Cube
impl From<(i32, i32, i32)> for Cube {
    fn from(value: (i32, i32, i32)) -> Self {
        let (x, y, z) = value;
        Cube(x, y, z)
    }
}

/// Module wrapping the parsing functions for today's puzzle input
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i32, newline},
        combinator::map,
        multi::separated_list0,
        sequence::{terminated, tuple},
        Finish, IResult,
    };
    use std::collections::HashSet;

    /// Nom parser for ("5," || "5") -> 5i32
    fn number(s: &str) -> IResult<&str, i32> {
        alt((terminated(i32, tag(",")), i32))(s)
    }

    /// Nom parser for "1,2,3" -> (1, 2, 3)
    fn coordinates(s: &str) -> IResult<&str, (i32, i32, i32)> {
        // Using tuple instead of separated_list1 here because I want
        // to avoid allocating the Vec
        tuple((number, number, number))(s)
    }

    /// Nom parser for "1,2,3" -> Cube(1, 2, 3)
    fn cube(s: &str) -> IResult<&str, Cube> {
        map(coordinates, Cube::from)(s)
    }

    /// Parses a list of input lines into a list of Cubes
    fn cubes(s: &str) -> IResult<&str, Vec<Cube>> {
        separated_list0(newline, cube)(s)
    }

    /// Parses the input file into a HashSet of Cubes
    pub fn parse(s: &str) -> Result<HashSet<Cube>> {
        let (_, result) = cubes(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result.into_iter().collect::<HashSet<_>>())
    }
}

const INPUT: &str = include_str!("../../input/18/input.txt");

/// Parse that input!
pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::parser::*;
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 2881);
        assert!(input.contains(&Cube(16, 8, 6)));
        assert!(input.contains(&Cube(8, 20, 10)));
    }
}
