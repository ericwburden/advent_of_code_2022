use crate::day03::Input;
use anyhow::{anyhow, bail, Error, Result};

const INPUT: &str = include_str!("../../input/03/input.txt");

// Today we'll do a bit of math converting ASCII characters to numbers.
// These constants are used in that math. For references, ASCII 'a' corresponds
// to a value of 97, and ASCII 'A' corresponds to a value of 65.
const LOWERCASE_OFFSET: u32 = 96; // 'a' -> 97 -  1 (priority of 'a') = 96
const CAPITAL_OFFSET: u32 = 38; // 'A' -> 65 - 26 (priority of 'A') = 38

/// A `Rucksack` represents a pack carried by an elf with two different,
/// separated sets of `Item`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rucksack(pub ItemSet, pub ItemSet);

/// Attempt to convert a line from the input into a `Rucksack`
impl TryFrom<&str> for Rucksack {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // Because the input lines contain two equal-length strings of
        // characters representing `Item`s, we need the length of each
        // half.
        let compartment_len = s.len() / 2;

        // Split the line into two equal-length strings
        let (str1, str2) = s.split_at(compartment_len);

        // Convert each string into a set of `Item`s
        let compartment1 = ItemSet::try_from(str1)?;
        let compartment2 = ItemSet::try_from(str2)?;

        // Return a `Rucksack` with two sets of items
        Ok(Rucksack(compartment1, compartment2))
    }
}

/// An `ItemSet` represents the unique items held in each compartment of a
/// `Rucksack`. The goal is to provide functionality equivalent to a
/// `HashSet<Item>` (at least as much as we need) without the overhead
/// of an actual `HashSet`. This is accomplished by assigning each type of
/// `Item` to a particular bit in the inner `u64`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemSet(pub u64);

impl ItemSet {
    /// Insert an `Item` into an `ItemSet`.
    fn insert(&mut self, item: Item) {
        // Set the bit in the `ItemSet` that corresponds to the particular type
        // of `Item`.
        self.0 |= item.0;
    }
}

/// Attempt to convert a string slice into a set of `Item`s.
impl TryFrom<&str> for ItemSet {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Start with an empty item set
        let mut item_set = ItemSet::default();

        // Convert each character in the input string into an `Item` and
        // insert each `Item` into the set
        for ch in value.chars() {
            let item = Item::try_from(ch)?;
            item_set.insert(item);
        }

        Ok(item_set)
    }
}

/// An `Item` represents a particular item carried by the elves. Because the puzzle
/// specifies that there are only 52 possible item types, each item can be uniquely
/// represented by a single set bit in a `u64` with 6 extra bits to spare. We'll set
/// bits in order of increasing priority, starting with 'a' at 2^1. This way, the
/// number of trailing zeros will be equal to priority. So, 'a' will be stored as
/// 2u64 with 1 trailing zero, and 'A' will be stored as 134217728u64 with 27 trailing
/// zeros.
///
/// Just for excessive clarity, 134217728u64 is represented in bits as:
///   0b00000000000000000000000000000000001000000000000000000000000000
#[derive(Debug, Clone, Copy)]
pub struct Item(pub u64);

/// Attempt to convert a single character into an `Item`
impl TryFrom<char> for Item {
    type Error = Error;

    #[rustfmt::skip]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        // Error if trying to make an `Item` out of any character that's not a letter
        if !value.is_alphabetic() { bail!("Cannot convert {value} to an Item!") }

        // Offset for ASCII range starts and priority offset.
        let offset = if value > 'Z' { LOWERCASE_OFFSET } else { CAPITAL_OFFSET };
        let priority = value as u32 - offset;
        let set_bit = 1 << priority; // One bit, shifted left by priority

        Ok(Item(set_bit))
    }
}

/// Read and parse the input
pub fn read() -> Input {
    // Attempt to convert each line into a `Rucksack` and return the
    // list of successful results.
    INPUT.lines().flat_map(Rucksack::try_from).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 300);

        let first = *input.first().unwrap();
        let first_expected = Rucksack::try_from("VdzVHmNpdVmBBCpmQLTNfTtMhMJnhFhTTf").unwrap();
        assert_eq!(first, first_expected);

        let last = *input.last().unwrap();
        let last_expected = Rucksack::try_from("vjWPWjWPPPWgwmfCrNvTvZ").unwrap();
        assert_eq!(last, last_expected);
    }
}
