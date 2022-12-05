use crate::day04::Input;

/// Represents a range of beach assignments for a particular elf
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssignmentRange {
    pub start: u8,
    pub stop: u8,
}

/// Convert a pair of integers to an `AssignmentRange`
impl From<(u8, u8)> for AssignmentRange {
    fn from(value: (u8, u8)) -> Self {
        let (start, stop) = value;
        AssignmentRange { start, stop }
    }
}

/// Represents a pair of elf beach cleaning assignment ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct AssignmentRangePair(pub AssignmentRange, pub AssignmentRange);

/// Convert a pair of assignment ranges into an `AssignmentRangePair`
impl From<(AssignmentRange, AssignmentRange)> for AssignmentRangePair {
    fn from(value: (AssignmentRange, AssignmentRange)) -> Self {
        let (first, second) = value;
        AssignmentRangePair(first, second)
    }
}

/// I find that I like having the `nom` parser functions bundled under a module
/// like this. It helps to namespace the parser combinators by usage and keeps
/// me from polluting the parent module with a bunch of otherwise ambiguously
/// named functions. Another option I'm playing with is bundling them as impl
/// functions under an empty struct. Not sure which I like better yet, but this has
/// the advantage of allowing me to include the `use::nom::*` imports close to where
/// the parser combinators are defined. I'll need to try it out in a few more
/// situations to get a better handle on the pros and cons.
mod arp_parser {
    use super::{AssignmentRange, AssignmentRangePair};
    use nom::{
        bytes::complete::tag, character::complete::u8, combinator::into, error::Error as NomError,
        sequence::separated_pair, Finish, IResult,
    };

    /// Nom parser for "12-22" -> (12u8, 22u8)
    fn number_pair(s: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(u8, tag("-"), u8)(s)
    }

    /// Nom parser for "12-22" -> AssignmentRange { start: 12, stop: 22 }
    fn range(s: &str) -> IResult<&str, AssignmentRange> {
        into(number_pair)(s)
    }

    /// Nom parser for "12-22,18-24" -> AssignmentRangePair(
    ///    AssignmentRange { start: 12, stop: 22 },
    ///    AssignmentRange { start: 18, stop: 24 },
    /// )
    pub fn parse(s: &str) -> Result<AssignmentRangePair, NomError<&str>> {
        let pair_parser = separated_pair(range, tag(","), range);
        let parse_result = into(pair_parser)(s);
        let (_, ranges) = parse_result.finish()?;
        Ok(ranges)
    }
}

// Keep the input file as a compile time constant string slice
const INPUT: &str = include_str!("../../input/04/input.txt");

/// Read the input by parsing each line into an `AssignmentRangePair` and discarding
/// any lines that return an Error. We'll check in the tests to make sure every line
/// is parsed.
pub fn read() -> Input {
    INPUT.lines().flat_map(arp_parser::parse).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 1000);

        let first = *input.first().unwrap();
        let first_expected = AssignmentRangePair::from((
            AssignmentRange::from((14, 50)),
            AssignmentRange::from((14, 50)),
        ));
        assert_eq!(first, first_expected);

        let last = *input.last().unwrap();
        let last_expected = AssignmentRangePair::from((
            AssignmentRange::from((98, 98)),
            AssignmentRange::from((17, 99)),
        ));
        assert_eq!(last, last_expected);
    }
}
