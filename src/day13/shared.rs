use super::Packet;
use std::cmp::Ordering;

/// Partial ordering for `Packet`s. Defining this would be enough to use
/// `Packet` < `Packet`, but we won't be able to do a full sort without
/// implementing `Ord` as well.
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Defines the result of comparing one `Packet` to another, used for ordinal
/// comparisons and sorting. Returns an enum that indicates whether `self` is
/// less than, greater than, or even equal to `other`.
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*; // For syntax
        match (self, other) {
            // When comparing two packet integers, just compare them by value
            (Integer(i1), Integer(i2)) => i1.cmp(i2),

            // When comparing a packet integer to a packet list, convert the integer
            // to a single item list and compare the two lists.
            (Integer(i), List(_)) => List(vec![Integer(*i)]).cmp(other),
            (List(_), Integer(i)) => self.cmp(&List(vec![Integer(*i)])),

            // When comparing two lists, compare item by item and return the first
            // result where the two items aren't equal. If one list has more items
            // than another and all the values up to the length of the shortest list
            // are equal, then we compare the length of the lists.
            (List(l1), List(l2)) => {
                for (first, second) in l1.iter().zip(l2.iter()) {
                    let result = first.cmp(second);
                    let Ordering::Equal = result else { return result; };
                }
                l1.len().cmp(&l2.len())
            }
        }
    }
}
