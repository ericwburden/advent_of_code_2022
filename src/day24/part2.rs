use super::input::Space;
use super::part1::Expedition;
use crate::day24::{Input, Output};

/// Solve Day 24, Part 2
pub fn solve(input: &Input) -> Output {
    let first_state = input.get(0).expect("Valley should have an initial state!");

    // All examples and input have the start Space at index (0, 1) and the end
    // Space on the bottom row, next to the last column.
    let start_at = Expedition(0, 1);
    let end_at = Expedition(first_state.rows - 1, first_state.cols - 2);

    // Start at zero minutes and move from the start to the end.
    let Some(minutes) = start_at.shortest_path(end_at, 0, input) else {
        panic!("Could not find a way through the valley. Died of frostbite!"); 
    };

    // Turn around and head back to the start.
    let Some(minutes) = end_at.shortest_path(start_at, minutes, input) else {
        panic!("Could not find a way back! Died in a blizzard!"); 
    };

    // Then turn around and head back to the end again.
    let Some(minutes) = start_at.shortest_path(end_at, minutes, input) else {
        panic!("Could not find a way back! Died of embarassment!"); 
    };

    // Return the total number of minutes it took to travel all that way.
    (minutes as u32).into()
}
