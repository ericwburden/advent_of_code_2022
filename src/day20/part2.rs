use crate::day20::{Input, Output};
use super::part1::MixingDecryptor;

/// Solve Day 20, Part 2
///
/// No real changes here, other than multiplying each value by some hefty
/// prime-looking number and mixing the list ten times. Just call me 
/// Sir Mix-A-Lot.
pub fn solve(input: &Input) -> Output {
    let mod_input: Vec<_> = input.iter().map(|x| x * 811589153).collect();
    let mut decryptor = MixingDecryptor::from(mod_input);
    (0..10).for_each(|_| decryptor.mix());
    decryptor.grove_coordinates_sum().into()
}
