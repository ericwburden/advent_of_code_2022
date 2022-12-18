use crate::day17::Input;

/// Represents a gust from the jets of gas, either to the left or right.
#[derive(Debug, Clone, Copy)]
pub enum Gust {
    Left,
    Right,
}

/// An iterator that produces gusts of gas in an repeating cycle
#[derive(Debug, Clone)]
pub struct GasJetIter {
    iter: Vec<Gust>,
    pub idx: usize,
}

/// Iterator implementation for `GasJetIter`. Just returns the values contained
/// in a repeating cycle.
impl Iterator for GasJetIter {
    type Item = Gust;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.iter[self.idx]);
        self.idx = (self.idx + 1) % self.iter.len();
        result
    }
}

/// Convert a list of `Gust`s into a `GasJetIter`
impl From<Vec<Gust>> for GasJetIter {
    fn from(iter: Vec<Gust>) -> Self {
        GasJetIter {
            iter,
            idx: Default::default(),
        }
    }
}

/// Module to wrap the parsing functions for today's puzzle
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        character::complete::char,
        combinator::{map, value},
        multi::many1,
        Finish, IResult,
    };

    /// Nom parser for '<' -> Gust::Left
    fn push_left(s: &str) -> IResult<&str, Gust> {
        value(Gust::Left, char('<'))(s)
    }

    /// Nom parser for '>' -> Gust::Right
    fn push_right(s: &str) -> IResult<&str, Gust> {
        value(Gust::Right, char('>'))(s)
    }

    /// Nom parser for a Gust in either direction
    fn push(s: &str) -> IResult<&str, Gust> {
        alt((push_left, push_right))(s)
    }

    /// Nom parser for a list of Gusts
    fn pushes(s: &str) -> IResult<&str, Vec<Gust>> {
        many1(push)(s)
    }

    /// Nom parser to map a GasJetIter to a list of Gusts
    fn gas_jet_iter(s: &str) -> IResult<&str, GasJetIter> {
        map(pushes, GasJetIter::from)(s)
    }

    /// Main parsing function, attempts to parse the input into a GasJetIter and 
    /// return it.
    pub fn parse(s: &str) -> Result<GasJetIter> {
        let (_, result) = gas_jet_iter(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

const INPUT: &str = include_str!("../../input/17/input.txt");

/// Parse that input!
pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    impl GasJetIter {
        fn len(&self) -> usize {
            self.iter.len()
        }
    }

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 10091);
    }
}
