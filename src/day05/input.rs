use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use crate::day05::Input;
use anyhow::{anyhow, bail, Error, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{digit1, satisfy, u8},
    combinator::{into, map, not, opt, recognize, value, verify},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

#[derive(Debug, Clone)]
pub struct CrateStacks(pub [RefCell<Vec<char>>; 9]);

impl Default for CrateStacks {
    fn default() -> Self {
        let stack = RefCell::new(Vec::with_capacity(64));
        CrateStacks([stack.clone(), stack.clone(), stack.clone(),
        stack.clone(), stack.clone(), stack.clone(),
        stack.clone(), stack.clone(), stack])
    }
}

impl Index<usize> for CrateStacks {
    type Output = RefCell<Vec<char>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for CrateStacks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

struct CrateStackParser();

impl CrateStackParser {
    fn crate_label(s: &str) -> IResult<&str, char> {
        let mut crate_char = satisfy(|c| c.is_ascii_uppercase());
        delimited(tag("["), crate_char, tag("]"))(s)
    }

    fn crate_row(s: &str) -> IResult<&str, Vec<Option<char>>> {
        let mut maybe_crate = map(Self::crate_label, Some);
        let mut empty_space = value(None, tag("   "));
        let mut crate_or_empty = alt((maybe_crate, empty_space));
        separated_list1(tag(" "), crate_or_empty)(s)
    }

    fn crate_rows(s: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
        separated_list1(tag("\n"), Self::crate_row)(s)
    }

    fn parse(s: &str) -> Result<CrateStacks> {
        let (_, rows) = Self::crate_rows(s).map_err(|_| anyhow!("Cannot parse crate rows!"))?;
        let mut stacks = CrateStacks::default();

        for row in rows.iter().rev() {
            for (idx, maybe_crate) in row.iter().enumerate() {
                if let Some(label) = maybe_crate {
                    stacks[idx].borrow_mut().push(*label);
                }
            }
        }

        Ok(stacks)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub count: u8,
    pub origin: u8,
    pub destination: u8,
}

impl From<(u8, u8, u8)> for Instruction {
    fn from(value: (u8, u8, u8)) -> Self {
        let (count, origin, destination) = value;
        Instruction {
            count,
            origin,
            destination,
        }
    }
}

struct InstructionParser();

impl InstructionParser {
    fn not_number(s: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_alphabetic() || c.is_whitespace())(s)
    }

    fn labeled_u8(s: &str) -> IResult<&str, u8> {
        preceded(
            take_while(|c: char| c.is_alphabetic() || c.is_whitespace()),
            u8,
        )(s)
    }

    fn instruction(s: &str) -> IResult<&str, Instruction> {
        into(tuple((
            Self::labeled_u8,
            Self::labeled_u8,
            Self::labeled_u8,
        )))(s)
    }

    fn parse(s: &str) -> Result<Instruction> {
        let (_, result) =
            Self::instruction(s).map_err(|_| anyhow!("Cannot parse line to an Instruction!"))?;
        Ok(result)
    }
}

const INPUT: &str = include_str!("../../input/05/input.txt");

pub fn read() -> Input {
    let (mut first_chunk, mut second_chunk) = INPUT.split_once("\n\n").unwrap();
    let crate_stacks = CrateStackParser::parse(first_chunk).expect("Failed to parse crate stacks!");
    let instructions = second_chunk.lines().flat_map(InstructionParser::parse).collect();
    (crate_stacks, instructions)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let (crate_stacks, instructions) = read();

        // Check the crate stacks first
        let crate_stacks_first_expected = vec!['Q', 'W', 'P', 'S', 'Z', 'R', 'H', 'D'];
        let crate_stacks_last_expected = vec!['W', 'P', 'V', 'M', 'B', 'H'];
        assert_eq!(crate_stacks[0].borrow().to_owned(), crate_stacks_first_expected);
        assert_eq!(crate_stacks[8].borrow().to_owned(), crate_stacks_last_expected);

        // Check the instructions
        assert_eq!(instructions.len(), 503);

        let first_instruction = *instructions.first().unwrap();
        let first_instruction_expected = Instruction::from((1, 3, 9));
        assert_eq!(first_instruction, first_instruction_expected);

        let last_instruction = *instructions.last().unwrap();
        let last_instruction_expected = Instruction::from((1, 1, 9));
        assert_eq!(last_instruction, last_instruction_expected);
    }
}
