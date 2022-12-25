use crate::day24::{Input, Output};
use super::input::{Valley, Space};
use core::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn solve(input: &Input) -> Output {
    let initial_valley_state = input.get(0).expect("Valley should have an initial state!");
    let start_col = initial_valley_state.0.first().unwrap().iter().position(|s| matches!(s, Space::Empty)).unwrap();
    let end_col = initial_valley_state.0.last().unwrap().iter().position(|s| matches!(s, Space::Empty)).unwrap();

    let start_at = Expedition(0, start_col);
    let end_at = Expedition(initial_valley_state.0.len() - 1, end_col);

    if let Some(minutes) = start_at.shortest_path(end_at, 0, input) {
        return (minutes as u32).into();
    }

    panic!("Could not find a way through the valley. Died of frostbite!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Expedition(pub usize, pub usize);

impl Expedition {
    fn available_steps(&self, valley: &Valley) -> Vec<Expedition> {
        let Expedition(row, col) = *self;
        let mut possible_steps = Vec::with_capacity(5);

        // Left neighbor
        if let Space::Empty = valley.0[row][col - 1] {
            possible_steps.push(Expedition(row, col - 1));
        }

        // Right neighbor
        if let Space::Empty = valley.0[row][col + 1] {
            possible_steps.push(Expedition(row, col + 1));
        }

        // The upward neighbor needs to account for the fact that the
        // final space is in the top row, and thus accessible.
        if row > 0 {
            if let Space::Empty = valley.0[row - 1][col] {
                possible_steps.push(Expedition(row - 1, col));
            }
        }

        // The downard neighbor needs to account for the beginning space
        // being on the last row.
        if row < (valley.0.len() - 1) {
            if let Space::Empty = valley.0[row + 1][col] {
                possible_steps.push(Expedition(row + 1, col));
            }
        }

        // Waiting is a necessary option if there's nothing in our current space
        if let Space::Empty = valley.0[row][col] {
            possible_steps.push(Expedition(row, col));
        }

        possible_steps
    }

    pub fn shortest_path(&self, target: Expedition, start_time: usize, valley_states: &[Valley]) -> Option<usize> {
        let mut open = BinaryHeap::from([(Reverse(start_time), *self)]);
        let mut minutes = HashMap::from([((*self, start_time % valley_states.len()), start_time)]);

        while let Some((Reverse(minutes_passed), expedition)) = open.pop() {
            if expedition == target {
                return Some(minutes_passed);
            }

            let valley_state_id = (minutes_passed + 1) % valley_states.len();
            let valley_state = &valley_states[valley_state_id];

            for available_step in expedition.available_steps(valley_state) {
                let next_minutes = minutes_passed + 1;
                let curr_minutes = *minutes.get(&(available_step, valley_state_id)).unwrap_or(&usize::MAX);
                if next_minutes >= curr_minutes {
                    continue;
                }
                open.push((Reverse(next_minutes), available_step));
                minutes.insert((available_step, valley_state_id), next_minutes);
            }
        }

        None
    }
}

