use crate::day13::{Input, Output, PacketPair};

/// Solve Day 13, Part 1
pub fn solve(input: &Input) -> Output {
    let mut total = 0; // The total index value of proper sorted pairs

    // For each pair of packets...
    for (idx, packet_pair) in input.iter().enumerate() {
        // If it's not sorted correctly, skip.
        if !packet_pair.is_sorted() {
            continue;
        }

        // Otherwise, add the value of its index to the total
        total += (idx as u32) + 1; // Packets are 1-indexed
    }
    total.into()
}

impl PacketPair {
    /// Indicates if the first packet in a pair is less than the second
    /// packet in the pair.
    fn is_sorted(&self) -> bool {
        let Self(first, second) = self;
        first < second
    }
}
