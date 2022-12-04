use crate::day04::{AssignmentRange, AssignmentRangePair, Input, Output};

/// Solve Day 4, Part 2
pub fn solve(input: &Input) -> Output {
    let result = input
        .iter()
        .map(|pair| pair.ranges_overlap())
        .filter(|x| *x)
        .count() as u32;
    result.into()
}

/// Trait for determine whether one `AssignmentRange` overlaps another
trait RangeOverlap {
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeOverlap for AssignmentRange {
    fn overlaps(&self, other: &Self) -> bool {
        // Return true if one range overlaps another
        self.start <= other.stop && self.stop >= other.start
    }
}

impl AssignmentRangePair {
    /// Return true if either range in the pair overlaps another
    fn ranges_overlap(&self) -> bool {
        let AssignmentRangePair(first, second) = self;
        first.overlaps(second) || second.overlaps(first)
    }
}
