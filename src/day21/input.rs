use crate::day21::Input;
use anyhow::{bail, Error};
use std::collections::HashMap;

/// Represents the names of each monkey. I originally wanted to use
/// string references as the monkey labels, but that causes borrow
/// checker issues with mutating the Environment (defined below),
/// so I'm using a small, cheap array of characters instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label([char; 4]);

/// Try to convert a string slice to a Label. Only works for string
/// slices with exactly four characters.
impl TryFrom<&str> for Label {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            bail!("Can only convert a 4-character string to Label")
        }
        let mut out = ['\0'; 4];
        for (idx, ch) in value.chars().enumerate() {
            out[idx] = ch;
        }
        Ok(Label(out))
    }
}

/// Represents a Value being passed from monkey to monkey. Either a reference
/// to another monkey, or a raw number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ref(Label),
    Raw(i128),
}

/// Converts a string slice into a Value::Ref. Just convenient to have.
impl TryFrom<&str> for Value {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::Ref(Label::try_from(value)?))
    }
}

/// Represents one of the mathematical expressions being evaluated by a
/// monkey. Every monkey is evaluating an expression, even if it's just
/// to report a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Val(Value),
}

/// Represents the environment in which monkey math is being evaluated. Keeps
/// track of each variable (monkey) and their current assigned Expression.
#[derive(Debug, Clone)]
pub struct Environment(pub HashMap<Label, Expression>);

/// Wraps the parsing functions for today's input.
pub mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, i128, newline},
        combinator::{map, map_res, verify},
        multi::separated_list0,
        sequence::separated_pair,
        Finish, IResult,
    };

    /// Nom parser for "humn" -> Label(['h', 'u', 'm', 'n'])
    fn label(s: &str) -> IResult<&str, Label> {
        map_res(alpha1, Label::try_from)(s)
    }

    /// Nom parser for "humn" -> Value::Ref(Label(['h', 'u', 'm', 'n']))
    fn value_ref(s: &str) -> IResult<&str, Value> {
        map(label, Value::Ref)(s)
    }

    /// Nom parser for "15" -> Value::Raw(15)
    fn value_raw(s: &str) -> IResult<&str, Value> {
        map(i128, Value::Raw)(s)
    }

    /// Nom parser for monkey addition expressions
    fn expr_add(s: &str) -> IResult<&str, (Label, Expression)> {
        let (s, lbl) = label(s)?;
        let (s, _) = tag(": ")(s)?;
        let (s, (v1, v2)) = separated_pair(value_ref, tag(" + "), value_ref)(s)?;
        Ok((s, (lbl, Expression::Add(v1, v2))))
    }

    /// Nom parser for monkey subtraction expressions
    fn expr_sub(s: &str) -> IResult<&str, (Label, Expression)> {
        let (s, lbl) = label(s)?;
        let (s, _) = tag(": ")(s)?;
        let (s, (v1, v2)) = separated_pair(value_ref, tag(" - "), value_ref)(s)?;
        Ok((s, (lbl, Expression::Sub(v1, v2))))
    }

    /// Nom parser for monkey multiplication expressions
    fn expr_mul(s: &str) -> IResult<&str, (Label, Expression)> {
        let (s, lbl) = label(s)?;
        let (s, _) = tag(": ")(s)?;
        let (s, (v1, v2)) = separated_pair(value_ref, tag(" * "), value_ref)(s)?;
        Ok((s, (lbl, Expression::Mul(v1, v2))))
    }

    /// Nom parser for monkey division expressions
    fn div_expr(s: &str) -> IResult<&str, (Label, Expression)> {
        let (s, lbl) = label(s)?;
        let (s, _) = tag(": ")(s)?;
        let (s, (v1, v2)) = separated_pair(value_ref, tag(" / "), value_ref)(s)?;
        Ok((s, (lbl, Expression::Div(v1, v2))))
    }

    /// Nom parser for monkey value expressions
    fn val_expr(s: &str) -> IResult<&str, (Label, Expression)> {
        let (s, lbl) = label(s)?;
        let (s, _) = tag(": ")(s)?;
        let (s, value) = value_raw(s)?;
        Ok((s, (lbl, Expression::Val(value))))
    }

    /// Nom parser for all variants of an Expression
    fn expression(s: &str) -> IResult<&str, (Label, Expression)> {
        alt((val_expr, expr_add, expr_sub, expr_mul, div_expr))(s)
    }

    /// Nom parser for a newline-separated list of Expressions
    fn expressions(s: &str) -> IResult<&str, HashMap<Label, Expression>> {
        let (s, exprs) = separated_list0(newline, expression)(s)?;
        let exprs = exprs.into_iter().collect::<HashMap<_, _>>();
        Ok((s, exprs))
    }

    /// Parses the input file into an Environment
    pub fn parse(s: &str) -> Result<Environment> {
        let (_, result) = expressions(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(Environment(result))
    }
}

const INPUT: &str = include_str!("../../input/21/input.txt");

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
        println!("{input:?}");
        assert_eq!(input.0.len(), 1929);
    }
}
