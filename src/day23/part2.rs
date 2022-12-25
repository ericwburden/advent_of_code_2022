use crate::day23::{Input, Output};

/// Solve Day 23, Part 2
pub fn solve(input: &Input) -> Output {
    // Get a mutable clone of the Grove.
    let mut grove = input.clone();

    // Count the number of rounds it takes for the Grove to
    // reach a state where no elves need to move from one 
    // state to the next.
    let mut rounds = 1;
    while grove.move_elves() { rounds += 1; }

    // Return the number of rounds taken.
    rounds.into()
}
