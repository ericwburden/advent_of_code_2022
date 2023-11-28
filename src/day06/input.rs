use anyhow::{bail, Error};

use crate::day06::Input;

const INPUT: &str = include_str!("../../input/06/input.txt");

/// Read in the input by converting each character to a `Signal` and returning
/// the list.
pub fn read() -> Input {
    Box::new(INPUT.chars().flat_map(Signal::try_from))
}

/// Represents a single signal received on our device. Each character from the
/// input string can be represented as a `u32` with a single set bit, offset
/// from the left in alphabetic order. For example:
///  
///  'a' -> Signal(00000000000000000000000000000001)
///  'b' -> Signal(00000000000000000000000000000010)
///  'x' -> Signal(00000000100000000000000000000000)
///  'z' -> Signal(00000010000000000000000000000000)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signal(pub u32);

impl TryFrom<char> for Signal {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        /// Anything besides a lowercase letter cannot make a Signal
        if !value.is_ascii_lowercase() {
            bail!("Cannot convert {value} to a `Signal`!");
        }

        /// Set a single bit and shift it left by the letter offset, return
        /// as a Signal
        let shift = (value as u32) - 97;
        Ok(Signal(1 << shift))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let input: Vec<_> = read().collect();
        assert_eq!(input.len(), 4095);

        let first_signal = *input.first().unwrap();
        let first_expected = Signal::try_from('d').unwrap();
        assert_eq!(first_signal, first_expected);

        let last_signal = *input.last().unwrap();
        let last_expected = Signal::try_from('j').unwrap();
        assert_eq!(last_signal, last_expected);
    }
}
