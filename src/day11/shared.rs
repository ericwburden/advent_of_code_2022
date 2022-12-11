use super::{Monkey, Operation, Rule};
use itertools::Itertools;

impl Operation {
    /// Apply an operation to an item's worry score.
    pub fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Add(n) => item + n,
            Operation::Mult(n) => item * n,
            Operation::Square => item * item,
        }
    }
}

impl Rule {
    /// Check an item's worry score and return which monkey ID to throw
    /// the item to.
    pub fn check(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.success
        } else {
            self.fail
        }
    }
}
