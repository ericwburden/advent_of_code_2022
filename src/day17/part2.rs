use crate::day17::{Input, Output};
use part1::{Chamber, Shape};
use std::collections::HashMap;

use super::part1;

pub fn solve(input: &Input) -> Output {
    let mut gas_jets = input.to_owned();
    // let total_rocks = 1_000_000_000_000;
    let total_rocks = 2022;
    let mut rocks_added = 0;
    // let mut seen = HashMap::new();
    let mut chamber = Chamber::default();
    let mut total_height = 0;
    let mut rock_types = Shape::all().into_iter().cycle();

    while rocks_added <= total_rocks {
        let rock = rock_types.next().unwrap();
        chamber.add_rock(&mut gas_jets, rock);
        rocks_added += 1;
        // if chamber.height() < 8 {
        //     continue;
        // }

        // let state = (chamber.skyline(), rock, gas_jets.idx);
        // if let Some((prev_rocks_added, prev_height)) = seen.get(&state) {
        //     let rocks_in_repeat: usize = rocks_added - prev_rocks_added;
        //     let repeats: usize = (total_rocks - rocks_added) / rocks_in_repeat;
        //     rocks_added += rocks_in_repeat * repeats;
        //     total_height += repeats * (chamber.height() - prev_height);
        //     seen.clear();
        //     continue;
        // }
        // seen.insert(state, (rocks_added, chamber.height()));
    }

    (chamber.height() as u32 + total_height as u32).into()
}

use std::fmt::{Display, Formatter, Result as FmtResult};
impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for byte in self.0.iter().rev() {
            writeln!(f, "{:0>7b}", byte);
        }
        write!(f, "")
    }
}

impl Chamber {
    fn skyline(&self) -> Option<u64> {
        if self.height() < 8 {
            return None;
        }
        let result = self
            .0
            .iter()
            .rev()
            .take(8)
            .fold(0u64, |acc, byte| (acc << 8) | *byte as u64);
        Some(result)
    }
}
