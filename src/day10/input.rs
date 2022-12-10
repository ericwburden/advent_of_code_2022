use crate::day10::Input;

/// Represents an instruction to our handheld device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

/// Module wrapping the parser for the instructions from the input.
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::i32,
        combinator::{map, value},
        multi::separated_list1,
        sequence::preceded,
        Finish, IResult,
    };

    /// Nom parser for "noop" -> Instruction:Noop
    fn noop(s: &str) -> IResult<&str, Instruction> {
        value(Instruction::Noop, tag("noop"))(s)
    }

    /// Nom parser for "addx 3" -> Instruction::Addx(3)
    fn addx(s: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("addx "), i32), Instruction::Addx)(s)
    }

    /// Nom parser for either instruction variant
    fn instruction(s: &str) -> IResult<&str, Instruction> {
        alt((noop, addx))(s)
    }

    /// Nom parser for all instruction lines -> Vec<Instruction>
    pub fn parse(s: &str) -> Result<Vec<Instruction>> {
        let result = separated_list1(tag("\n"), instruction)(s);
        let (_, instrs) = result.finish().map_err(|e| anyhow!("{e}"))?;
        Ok(instrs)
    }
}

const INPUT: &str = include_str!("../../input/10/input.txt");

/// Parse that input!
pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 145);

        let third_found = *input.get(2).unwrap();
        let third_expected = Instruction::Addx(5);
        assert_eq!(third_found, third_expected);

        let near_last_found = *input.get(141).unwrap();
        let near_last_expected = Instruction::Addx(2);
        assert_eq!(near_last_found, near_last_expected);
    }
}
