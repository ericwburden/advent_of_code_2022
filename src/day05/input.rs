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
use std::cell::RefCell;
use std::ops::{Index, IndexMut};

/// Represents a stack of crates. Each crate is represented by the character
/// given in the input. A stack is a 64-length array (enough room for all the
/// crates if need be) that keeps track of the current height of the stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrateStack {
    pub crates: [char; 64],
    pub height: usize,
}

/// By default, all crates are set to '.', indicating an empty space in the
/// buffer. I "opted" not to use options here because the code was already
/// getting complex enough without it. There are a number of other truly
/// advisable safety features that I've left out of today's puzzle implementation.
impl Default for CrateStack {
    fn default() -> Self {
        Self {
            crates: ['.'; 64],
            height: 0,
        }
    }
}

/// Represents the collected stacks of crates. We know we have 9 total stacks from
/// the input, so this data structure can accommodate up to 9 stacks of crates. 
/// Each stack of crates is wrapped in `RefCell` to facilitate moving crates directly
/// from one stack to the other without needing a buffer in between. We'll use this
/// functionality in part two.
#[derive(Debug, Default, Clone)]
pub struct CrateStacks(pub [RefCell<CrateStack>; 9]);

/// Implement indexing into the `CrateStacks` to get a particular crate. Since stack
/// numbers are given 1-9, we can adjust the index here to allow for 1-indexing for
/// getting a particular stack of crates.
impl Index<usize> for CrateStacks {
    type Output = RefCell<CrateStack>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}

/// Same as `Index`, just mutable! This is really the important one, but we need
/// both.
impl IndexMut<usize> for CrateStacks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}

/// This time, I'm bundling all the parsers under an empty struct
struct CrateStackParser();

impl CrateStackParser {
    /// Nom parser to parse "[A]" -> 'A'
    fn crate_label(s: &str) -> IResult<&str, char> {
        let mut crate_char = satisfy(|c| c.is_ascii_uppercase());
        delimited(tag("["), crate_char, tag("]"))(s)
    }

    /// Nom parser to parse "[A]     [B] [C]" -> [Some('A'), None, Some('B'), Some('C')]
    fn crate_row(s: &str) -> IResult<&str, Vec<Option<char>>> {
        let mut maybe_crate = map(Self::crate_label, Some);
        let mut empty_space = value(None, tag("   "));
        let mut crate_or_empty = alt((maybe_crate, empty_space));
        separated_list1(tag(" "), crate_or_empty)(s)
    }

    /// Nom parser to parse multiple newline-separated rows of crates into a list
    /// of rows, specified by `crate_row`.
    fn crate_rows(s: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
        separated_list1(tag("\n"), Self::crate_row)(s)
    }

    /// Parses the first section of the input into a `CrateStacks`, where each 
    /// `CrateStack` contained includes the crates from each column of the input.
    fn parse(s: &str) -> Result<CrateStacks> {
        let (_, rows) = Self::crate_rows(s).map_err(|_| anyhow!("Cannot parse crate rows!"))?;
        let mut stacks = CrateStacks::default();

        for row in rows.iter().rev() {
            for (idx, maybe_crate) in row.iter().enumerate() {
                if let Some(label) = maybe_crate {
                    stacks[idx + 1].borrow_mut().push(*label);
                }
            }
        }

        Ok(stacks)
    }
}


/// An Instruction represents the instructions to move one or more crates from
/// one stack to another. Includes how many crates to move, which stack to move
/// them from, and which stack to move them to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub count: u8,
    pub origin: u8,
    pub destination: u8,
}

/// Conveniently convert a three-tuple of numbers to an `Instruction`
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

/// I've bundled the parser functions for parsing `Instruction`s into this empty struct
struct InstructionParser();

impl InstructionParser {
    /// Nom parser for a string of non-digit characters
    fn not_number(s: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_alphabetic() || c.is_whitespace())(s)
    }

    /// Nom parser to convert "move 1" -> 1u8 or " from 2" -> 2u8
    fn labeled_u8(s: &str) -> IResult<&str, u8> {
        preceded(
            take_while(|c: char| c.is_alphabetic() || c.is_whitespace()),
            u8,
        )(s)
    }

    /// Nom parser to convert "move 1 from 2 to 3" -> (1, 2, 3)
    fn instruction(s: &str) -> IResult<&str, Instruction> {
        into(tuple((
            Self::labeled_u8,
            Self::labeled_u8,
            Self::labeled_u8,
        )))(s)
    }

    /// Parse a line of instruction into an `Instruction`
    fn parse(s: &str) -> Result<Instruction> {
        let (_, result) =
            Self::instruction(s).map_err(|_| anyhow!("Cannot parse line to an Instruction!"))?;
        Ok(result)
    }
}

/// Include the input as a constant string slice
const INPUT: &str = include_str!("../../input/05/input.txt");

/// Read the input from the file (string) and parse it.
pub fn read() -> Input {
    // Split the input on the empty line
    let (mut first_chunk, mut second_chunk) = INPUT.split_once("\n\n").unwrap();

    // Parse the first section into a `CrateStacks`
    let crate_stacks = CrateStackParser::parse(first_chunk).expect("Failed to parse crate stacks!");

    // Parse the second section into a list of `Instruction`s
    let instructions = second_chunk
        .lines()
        .flat_map(InstructionParser::parse)
        .collect();

    // Return the pair of parsed input sections
    (crate_stacks, instructions)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Used to instantiate `CrateStack`s from character arrays. Really only used 
    /// for testing, but handy for that.
    impl<const N: usize> From<[char; N]> for CrateStack {
        fn from(value: [char; N]) -> Self {
            let mut stack = CrateStack::default();
            for ch in value.iter() {
                stack.push(*ch);
            }
            stack
        }
    }

    #[test]
    fn check_input() {
        let (crate_stacks, instructions) = read();

        // Check the crate stacks first
        let crate_stacks_first_expected =
            CrateStack::from(['Q', 'W', 'P', 'S', 'Z', 'R', 'H', 'D']);
        let crate_stacks_last_expected = CrateStack::from(['W', 'P', 'V', 'M', 'B', 'H']);
        assert_eq!(
            crate_stacks[1].borrow().to_owned(),
            crate_stacks_first_expected
        );
        assert_eq!(
            crate_stacks[9].borrow().to_owned(),
            crate_stacks_last_expected
        );

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
