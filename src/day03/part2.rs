use crate::day03::{Input, Item, ItemSet, Output, Rucksack};
use anyhow::Result;
use itertools::Itertools;

pub fn solve(input: &Input) -> Output {
    let mut total = 0; // The sum of badge priorities

    // Convert each `Rucksack` to an `ItemSet` consisting of all its unique
    // `Item`s, as an iterator.
    let rucksack_sets = input.iter().map(|r| r.all_items());

    // For each chunk of three rucksack_sets in sequence...
    for chunk in &rucksack_sets.chunks(3) {
        // Produce an `ItemSet` that is the intersection of all three sets
        // in the chunk. If the chunk is empty, we get an empty `ItemSet` back.
        let intersect = chunk
            .reduce(|acc, i| acc.intersect(i))
            .unwrap_or(ItemSet(0));

        // Attempt to convert the `ItemSet` into a single Item. Panic if
        // the chunk contains more than one common item. The puzzle text
        // assures us this won't happen.
        let badge = Item::try_from(intersect).expect("{intersect:?} contains multiple Items!");

        // Add the priority of the badge to the total
        total += badge.priority();
    }

    total.into()
}

impl Rucksack {
    /// Return an `ItemSet` comprised of the items in both compartments of
    /// the `Rucksack`. Recall that this is a set, so duplicates aren't counted.
    fn all_items(&self) -> ItemSet {
        ItemSet(self.0 .0 | self.1 .0)
    }
}
