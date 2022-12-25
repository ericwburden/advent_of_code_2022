use crate::day24::{Input, Output};
use super::input::Space;
use super::part1::Expedition;

pub fn solve(input: &Input) -> Output {
    let initial_valley_state = input.get(0).expect("Valley should have an initial state!");
    let start_col = initial_valley_state.0.first().unwrap().iter().position(|s| matches!(s, Space::Empty)).unwrap();
    let end_col = initial_valley_state.0.last().unwrap().iter().position(|s| matches!(s, Space::Empty)).unwrap();

    let start_at = Expedition(0, start_col);
    let end_at = Expedition(initial_valley_state.0.len() - 1, end_col);

    let Some(minutes) = start_at.shortest_path(end_at, 0, input) else { panic!("Could not find a way through the valley. Died of frostbite!"); };
    let Some(minutes) = end_at.shortest_path(start_at, minutes, input) else { panic!("Could not find a way back! Died in a blizzard!"); };
    let Some(minutes) = start_at.shortest_path(end_at, minutes, input) else { panic!("Could not find a way back! Died of embarassment!"); };
    (minutes as u32).into()
}
