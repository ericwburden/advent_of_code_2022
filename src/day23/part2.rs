use crate::day23::{Input, Output};
use super::grid::{Grid, Rules, GridBuilder};

/// Solve Day 23, Part 2
pub fn solve(input: &Input) -> Output {
    // Start with a copy of the Grid, the initial proposal order, and a blank Grid
    let mut state = *input;
    let mut last_state = Grid::default();
    let mut propose_order = Rules::default();
    let mut rounds = 0;

    // We'll keep two Grid states around, the current state and the last state. Any
    // time the new state matches the old state, we know that no elves moved and 
    // we can stop.
    while state != last_state {
        last_state = state;
        state = GridBuilder::init(state, propose_order)
            .identify_movers()
            .make_proposals()
            .resolve_conflicts()
            .finalize();
        propose_order.rotate();
        rounds += 1;
    }

    // Return the number of rounds it took to find a round where no changes
    // occurred.
    rounds.into()
}
