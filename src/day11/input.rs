use crate::day11::Input;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub rule: Rule,
    pub inspected: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    pub divisor: u64,
    pub success: usize,
    pub fail: usize,
}

mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{multispace1, newline, one_of, space1, u64},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    fn id(s: &str) -> IResult<&str, usize> {
        map(delimited(tag("Monkey "), u64, tag(":")), |n| n as usize)(s)
    }

    fn items(s: &str) -> IResult<&str, VecDeque<u64>> {
        let prefix = preceded(space1, tag("Starting items: "));
        let list = separated_list1(tag(", "), u64);
        map(preceded(prefix, list), VecDeque::from)(s)
    }

    fn add_op(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("+ "), u64), Operation::Add)(s)
    }

    fn mult_op(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("* "), u64), Operation::Mult)(s)
    }

    fn square_op(s: &str) -> IResult<&str, Operation> {
        value(Operation::Square, tag("* old"))(s)
    }

    fn op_prefix(s: &str) -> IResult<&str, &str> {
        preceded(space1, tag("Operation: new = old "))(s)
    }

    fn op(s: &str) -> IResult<&str, Operation> {
        let add = preceded(op_prefix, add_op);
        let mult = preceded(op_prefix, mult_op);
        let square = preceded(op_prefix, square_op);
        alt((add, mult, square))(s)
    }

    fn test_rule(s: &str) -> IResult<&str, Rule> {
        let (s, divisor) = preceded(space1, preceded(tag("Test: divisible by "), u64))(s)?;
        let (s, success) =
            preceded(multispace1, preceded(tag("If true: throw to monkey "), u64))(s)?;
        let (s, fail) = preceded(
            multispace1,
            preceded(tag("If false: throw to monkey "), u64),
        )(s)?;
        let rule = Rule {
            divisor,
            success: success as usize,
            fail: fail as usize,
        };
        Ok((s, rule))
    }

    fn monkey(s: &str) -> IResult<&str, Monkey> {
        let (s, id) = terminated(id, newline)(s)?;
        let (s, items) = terminated(items, newline)(s)?;
        let (s, operation) = terminated(op, newline)(s)?;
        let (s, rule) = test_rule(s)?;

        let monkey = Monkey {
            id,
            items,
            operation,
            rule,
            inspected: 0,
        };
        Ok((s, monkey))
    }

    pub fn parse(s: &str) -> Result<Vec<Monkey>> {
        let result = separated_list1(multispace1, monkey)(s);
        let (s, monkeys) = result
            .finish()
            .map_err(|e| anyhow!("Failed to parse monkeys with error {e}"))?;
        Ok(monkeys)
    }
}

const INPUT: &str = include_str!("../../input/11/input.txt");
const EXAMPLE: &str = include_str!("../../input/11/example.txt");

pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() {
        let monkeys = parser::parse(INPUT.trim()).unwrap();
        assert_eq!(monkeys.len(), 8);
    }
}
