use crate::day17::Input;

#[derive(Debug, Clone, Copy)]
pub enum Push {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct GasJetIter {
    iter: Vec<Push>,
    pub idx: usize,
}

impl GasJetIter {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Iterator for GasJetIter {
    type Item = Push;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.iter[self.idx]);
        self.idx = (self.idx + 1) % self.iter.len();
        result
    }
}

impl From<Vec<Push>> for GasJetIter {
    fn from(iter: Vec<Push>) -> Self {
        GasJetIter {
            iter,
            idx: Default::default(),
        }
    }
}

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

    fn push_left(s: &str) -> IResult<&str, Push> {
        value(Push::Left, char('<'))(s)
    }

    fn push_right(s: &str) -> IResult<&str, Push> {
        value(Push::Right, char('>'))(s)
    }

    fn push(s: &str) -> IResult<&str, Push> {
        alt((push_left, push_right))(s)
    }

    fn pushes(s: &str) -> IResult<&str, Vec<Push>> {
        many1(push)(s)
    }

    fn gas_jet_iter(s: &str) -> IResult<&str, GasJetIter> {
        map(pushes, GasJetIter::from)(s)
    }

    pub fn parse(s: &str) -> Result<GasJetIter> {
        let (_, result) = gas_jet_iter(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

const INPUT: &str = include_str!("../../input/17/input.txt");
const EXAMPLE: &str = include_str!("../../input/17/example.txt");

pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 10091);
    }
}
