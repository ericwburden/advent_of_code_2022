use crate::day13::Input;

/// Represnts a Packet. Packet data consists of lists and integer (that's what the
/// puzzle says, anyway).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

/// Represents a pair of packets. Riveting stuff!
#[derive(Debug, Clone)]
pub struct PacketPair(pub Packet, pub Packet);

/// Here's where the magic happens. This module wraps the parsers for the list of
/// packet pairs presented in the input.
pub mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u8},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair},
        Finish, IResult,
    };

    /// Nom parser for "2" -> Packet::Integer(2)
    fn integer(s: &str) -> IResult<&str, Packet> {
        map(u8, Packet::Integer)(s)
    }

    /// This is where the recursion happens. This parser parses lists of
    /// packet information into their appropriate packet representation.
    /// Parses all of the following:
    ///   - "[]" -> Packet::List([])
    ///   - "[5]" -> Packet::List([Packet::Integer(5)])
    ///   - "[1, 2]" -> Packet::List([Packet::Integer(1), Packet::Integer(2)])
    ///   - "[1, [2]]" -> Packet::List([Packet::Integer(1), Packet::List([Packet::Integer(2)])])
    fn list(s: &str) -> IResult<&str, Packet> {
        let list_contents = separated_list0(tag(","), packet);
        map(delimited(tag("["), list_contents, tag("]")), Packet::List)(s)
    }

    /// Packet data consists of lists and integers. This parser will parse a `Packet`
    /// from a list or things or from a single integer.
    fn packet(s: &str) -> IResult<&str, Packet> {
        alt((integer, list))(s)
    }

    /// Parses two packets separated by a newline into a `PacketPair`
    fn packet_pair(s: &str) -> IResult<&str, PacketPair> {
        let (s, (first, second)) = separated_pair(packet, newline, packet)(s)?;
        Ok((s, PacketPair(first, second)))
    }

    /// Parses a list of packet pairs separated by an empty line into a `Vec<PacketPair>`
    pub fn parse(s: &str) -> Result<Vec<PacketPair>> {
        let result = separated_list1(tag("\n\n"), packet_pair)(s).finish();
        let (_, pair_list) = result.map_err(|e| anyhow!("{e}"))?;
        Ok(pair_list)
    }
}

const INPUT: &str = include_str!("../../input/13/input.txt");

/// Parse that input!
pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    const EXAMPLE: &str = include_str!("../../input/13/example.txt");

    impl Display for Packet {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Packet::Integer(i) => write!(f, "{i}"),
                Packet::List(l) => {
                    write!(f, "[")?;
                    for (idx, p) in l.iter().enumerate() {
                        write!(f, "{p}")?;
                        if idx < l.len() - 1 {
                            write!(f, ",")?;
                        }
                    }
                    write!(f, "]")
                }
            }
        }
    }

    impl Display for PacketPair {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let PacketPair(first, second) = self;
            writeln!(f, "{first}")?;
            writeln!(f, "{second}")
        }
    }

    #[test]
    fn check_input() {
        // To test, I'm just reading and parsing the input, then converting it back
        // into a string and comparing it to the original.
        let input = parser::parse(INPUT).unwrap();
        let from_parsed: String = input
            .iter()
            .map(|p| p.to_string())
            .intersperse(String::from("\n"))
            .collect();
        assert_eq!(INPUT, from_parsed);
    }
}
