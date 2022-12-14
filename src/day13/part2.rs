use crate::day13::{parser, Input, Output, Packet, PacketPair};

/// Solve Day 13, Part 2
pub fn solve(input: &Input) -> Output {
    use Packet::*; // For syntax

    // Define the two divider packets and put them in an array to add them
    // to the list of all the packets.
    let divider1 = List(vec![List(vec![Integer(2)])]);
    let divider2 = List(vec![List(vec![Integer(6)])]);
    let dividers = [divider1, divider2];

    // Flatten all the pairs of packets into a list of packets with the two
    // divider packets added to the end.
    let mut all_packets = input
        .iter()
        .cloned()
        .flatten()
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    // Sort all the packets! No need to do any more work than that, since
    // by implementing Ord and PartialOrd, we've done all we need to
    // sort them.
    all_packets.sort_unstable();

    // Find the indices of the two divider packets and return their product.
    let mut total = 1;
    for (idx, packet) in all_packets.iter().enumerate() {
        if dividers.contains(packet) {
            total *= (idx as u32) + 1;
        }
    }
    total.into()
}

// It's easier to flatten the packet pairs into a 1D list when we can
// iterate over the two packets contained in them.
impl IntoIterator for PacketPair {
    type Item = Packet;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        let PacketPair(first, second) = self;
        [first, second].into_iter()
    }
}
