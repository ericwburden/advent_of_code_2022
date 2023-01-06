use crate::day23::{Input, Output};
use std::collections::HashMap;
use super::grid::{Grid, Rules, GridBuilder};

/// Solve Day 23, Part 1
pub fn solve(input: &Input) -> Output {
    // Start with a copy of the Grid and the default order of proposal directions
    let mut state = *input;
    let mut propose_order = Rules::default();

    // Update the Grid ten times. Still not convinced that I _needed_ the builder
    // pattern here, but it does make for a nice syntax for producing new Grid
    // states. It's nice to have listed each part of the process like this.
    for _ in 0..10 {
        state = GridBuilder::init(state, propose_order)
            .identify_movers()
            .make_proposals()
            .resolve_conflicts()
            .finalize();
        propose_order.rotate();
    }

    // Count and return the number of empty spaces
    state.count_empty_spaces().into()
}

