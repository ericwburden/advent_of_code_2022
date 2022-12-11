use crate::day11::Input;

/// Represents one of those mischeivous monkies! Contains fields
/// for the items the monkey is currently holding, the operation
/// performed when the monkey inspects an item, the rule the monkey
/// uses to decide who to throw the item to, and the number of
/// items the monkey has inspected so far.
#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub rule: Rule,
    pub inspected: u32,
}

/// Represents the operation performed to determine what happens to your
/// worry level over a particular item inspected by a monkey.
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

/// Represents the rule used by a monkey to determine which other monkey to
/// throw your item to, based on your worry level.
#[derive(Debug, Clone, Copy)]
pub struct Rule {
    pub divisor: u64,
    pub success: usize,
    pub fail: usize,
}

/// Wraps the parser combinators for parsing our input into a list of pesky monkies.
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

    /// Nom parser for "Monkey 3:" -> 3usize
    fn id(s: &str) -> IResult<&str, usize> {
        map(delimited(tag("Monkey "), u64, tag(":")), |n| n as usize)(s)
    }

    /// Nom parser for "Starting items: 1, 2, 3" -> VecDeque<[1, 2, 3]>
    fn items(s: &str) -> IResult<&str, Vec<u64>> {
        let prefix = preceded(space1, tag("Starting items: "));
        let list = separated_list1(tag(", "), u64);
        map(preceded(prefix, list), Vec::from)(s)
    }

    /// Nom parser for "+ 5" -> Operation::Add(5)
    fn add_op(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("+ "), u64), Operation::Add)(s)
    }

    /// Nom parser for "* 5" -> Operation::Mult(5)
    fn mult_op(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("* "), u64), Operation::Mult)(s)
    }

    /// Nom parser for "* old" -> Operation::Square
    fn square_op(s: &str) -> IResult<&str, Operation> {
        value(Operation::Square, tag("* old"))(s)
    }

    /// Nom parser to detect the string that comes before the operator
    /// when parsing an operation.
    fn op_prefix(s: &str) -> IResult<&str, &str> {
        preceded(space1, tag("Operation: new = old "))(s)
    }

    /// Nom parser for: 
    /// - "Operation: new = old + 5" -> Operation::Add(5)
    /// - "Operation: new = old * 5" -> Operation::Mult(5)
    /// - "Operation: new = old * old" -> Operation::Square
    fn op(s: &str) -> IResult<&str, Operation> {
        let add = preceded(op_prefix, add_op);
        let mult = preceded(op_prefix, mult_op);
        let square = preceded(op_prefix, square_op);
        alt((add, mult, square))(s)
    }

    /// Nom parser for extracting the relevant values from the three
    /// lines that describe the rules the monkey uses to determine where
    /// to throw your item, used ton construct a `Rule`. For example:
    ///
    ///   Test: divisible by 17 
    ///     If true: throw to monkey 0 
    ///     If false: throw to monkey 5
    ///
    /// becomes
    ///
    /// Rule { divisor: 17, success: 0, fail: 5 }
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

    /// Nom parser for converting a chunk of the input into a `Monkey`.
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

    /// Splits the input file into chunks based on empty lines and parses
    /// each chunk into a `Monkey`. Returns the list of `Monkey`s if
    /// successful or the relevant nom Error if not.
    pub fn parse(s: &str) -> Result<Vec<Monkey>> {
        let result = separated_list1(tag("\n\n"), monkey)(s);
        let (s, monkeys) = result
            .finish()
            .map_err(|e| anyhow!("Failed to parse monkeys with error {e}"))?;
        Ok(monkeys)
    }
}

const INPUT: &str = include_str!("../../input/11/input.txt");

/// Parse that input!
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
