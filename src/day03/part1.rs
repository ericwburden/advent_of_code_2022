use crate::day03::{Input, Item, ItemSet, Output, Rucksack};
use anyhow::{bail, Error, Result};

/// Solve Day 3, Part 1
pub fn solve(input: &Input) -> Output {
    // For each `Rucksack`, identify the one item in common between the
    // compartments, calculate that item's priority, and return the sum
    // of all unique item priorities.
    input
        .iter()
        .flat_map(|r| r.one_in_common())
        .map(|i| i.priority())
        .sum::<u32>()
        .into()
}

impl Rucksack {
    /// Attempt to identify the one `Item` in common between both compartments.
    fn one_in_common(&self) -> Result<Item> {
        self.0.intersect(self.1).try_into()
    }
}

impl ItemSet {
    /// Construct an `ItemSet` that contains only items in common between
    /// `self` and `other`. Just a bitwise _and_ on the underlying integers.
    pub fn intersect(&self, other: ItemSet) -> ItemSet {
        ItemSet(self.0 & other.0)
    }
}

/// Attempt to convert an `ItemSet` into a single `Item`.
/// Fails and returns an Error if the `ItemSet` contains more than one item.
impl TryFrom<ItemSet> for Item {
    type Error = Error;

    fn try_from(set: ItemSet) -> Result<Self, Self::Error> {
        if set.0.count_ones() > 1 {
            bail!("{set:?} contains more than one item!")
        }
        Ok(Item(set.0))
    }
}

impl Item {
    /// Calculate the priority of the `Item`. Recall each `Item` is represented
    /// by a single bit shifted left by priority, so priority is just the 
    /// number of trailing zeros.
    pub fn priority(&self) -> u32 {
        self.0.trailing_zeros()
    }
}
