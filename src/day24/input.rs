use crate::day24::Input;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Left,
    Up,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Space {
    Blizzard(Vec<Direction>),
    Wall,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Valley(pub Vec<Vec<Space>>);

impl From<&str> for Valley {
    fn from(input: &str) -> Self {
        let mut valley = Vec::new();
        for line in input.lines() {
            let row = line.chars().map(|glyph| {
                match glyph {
                    '>' => Space::Blizzard(vec![Direction::Right]),
                    '<' => Space::Blizzard(vec![Direction::Left]),
                    '^' => Space::Blizzard(vec![Direction::Up]),
                    'v' => Space::Blizzard(vec![Direction::Down]),
                    '.' => Space::Empty,
                    '#' => Space::Wall,
                    _ => unreachable!(),
                }
            }).collect::<Vec<_>>();
            valley.push(row);
        }
        Valley(valley)
    }
}


impl Valley {
    fn advance(&self) -> Self {
        let rows = self.0.len();
        let cols = self.0.first().map(|row| row.len()).unwrap_or_default();
        let mut new_state = Valley(vec![vec![Space::Empty; cols]; rows]);

        for (row, col) in (0..rows).cartesian_product(0..cols) {
            match &self.0[row][col] {
                Space::Blizzard(directions) => for direction in directions {
                    match direction {
                        Direction::Down => {
                            let new_row = (row % (rows - 2)) + 1;
                            new_state.add_blizzard(new_row, col, *direction);
                        },
                        Direction::Left =>  {
                            let new_col = if (col - 1) == 0 { cols - 2 } else { col - 1 };
                            new_state.add_blizzard(row, new_col, *direction);
                        },
                        Direction::Up => {
                            let new_row = if (row - 1) == 0 { rows - 2 } else { row - 1 };
                            new_state.add_blizzard(new_row, col, *direction);
                        },
                        Direction::Right => {
                            let new_col = (col % (cols - 2)) + 1;
                            new_state.add_blizzard(row, new_col, *direction);
                        },
                    }
                },
                Space::Wall => new_state.0[row][col] = Space::Wall,
                Space::Empty => continue,
            }
        }

        new_state
    }

    fn add_blizzard(&mut self, row: usize, col: usize, direction: Direction) {
        match &mut self.0[row][col] {
            Space::Blizzard(v) => v.push(direction),
            Space::Wall => panic!("Tried to add a blizzard to a wall!"),
            Space::Empty => self.0[row][col] = Space::Blizzard(vec![direction]),
        }
    }
}



const INPUT: &str = include_str!("../../input/24/input.txt");
const EXAMPLE: &str = include_str!("../../input/24/example.txt");

pub fn read() -> Input {
    let mut valley = Valley::from(INPUT);
    let mut valley_states = Vec::new();
    let mut seen_states = HashSet::new();
    while !seen_states.contains(&valley) {
        seen_states.insert(valley.clone());
        valley_states.push(valley.clone());
        valley = valley.advance()
    }
    valley_states
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::HashSet;


    impl Display for Valley {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            for row in &self.0 {
                for space in row.iter() {
                    match space {
                        Space::Blizzard(dirs) => {
                            if dirs.len() > 1 {
                                write!(f, "X")?;
                            } else {
                                let glyph = match dirs.first().unwrap() {
                                    Direction::Down => 'v',
                                    Direction::Left => '<',
                                    Direction::Up => '^',
                                    Direction::Right => '>',
                                };
                                write!(f, "{glyph}")?;
                            }
                        },
                        Space::Wall => { write!(f, "#")?; },
                        Space::Empty => { write!(f, ".")?; },
                    }
                }
                writeln!(f)?;
            }
            write!(f, "")
        }
    }
    

    #[test]
    fn check_input() {
        let mut valley = Valley::from(INPUT);
        let mut valley_states = HashMap::new();
        let mut seen_states = HashSet::new();
        while !seen_states.contains(&valley) {
            seen_states.insert(valley.clone());
            valley_states.insert(valley_states.len(), valley.clone());
            valley = valley.advance()
        }
        println!("{}", valley_states.len());
    }
}
