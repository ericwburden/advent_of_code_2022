use crate::day04::{AssignmentRange, AssignmentRangePair, Input, Output};

/// Solve Day 4, Part 1
pub fn solve(input: &Input) -> Output {
    // For each assignment pair in the input, check to see if one assignment
    // complete contains another. Count only the pairs where this check returns
    // true.
    let result = input
        .iter()
        .map(|pair| pair.full_containment())
        .filter(|x| *x)
        .count() as u32;
    result.into()
}

/// Trait for determine whether one `AssignmentRange` completely contains another
trait RangeContains {
    fn contains(&self, other: &Self) -> bool;
}

impl RangeContains for AssignmentRange {
    fn contains(&self, other: &Self) -> bool {
        // Return true if one range is completely inside another
        self.start <= other.start && self.stop >= other.stop
    }
}

impl AssignmentRangePair {
    /// Return true if either range in the pair completely contains the other one.
    fn full_containment(&self) -> bool {
        let AssignmentRangePair(first, second) = self;
        first.contains(second) || second.contains(first)
    }
}
