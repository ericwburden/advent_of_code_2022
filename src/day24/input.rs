use crate::day24::Input;
use anyhow::{bail, Error, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Represents the direction in which a particular Blizzard is blowing.
/// We have impls to allow converting to/from single-bit u8 values so
/// that a set of four directions can be represented as a u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Left,
    Up,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            4 => Ok(Direction::Up),
            8 => Ok(Direction::Right),
            _ => bail!("No direction corresponding to {value}!"),
        }
    }
}

impl Direction {
    /// Convert a Direction to a single-bit u8 value
    const fn value(&self) -> u8 {
        match self {
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 4,
            Direction::Right => 8,
        }
    }

    /// Get an array of all four Directions
    const fn all() -> [Direction; 4] {
        [
            Direction::Down,
            Direction::Left,
            Direction::Up,
            Direction::Right,
        ]
    }
}

/// Represents one or more blizzards occupying a single space in the Valley.
/// Because there's only one blizzard per space in the input, there can ever
/// only be one blizzard per direction in a single space at any time after
/// the blizzards begin moving in the Valley. So, we never need more than
/// the presence/absence of each of the four directions per Blizzard space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Blizzard(u8);

impl Blizzard {
    fn from(direction: Direction) -> Self {
        Blizzard(direction.value())
    }

    fn direction(&self) -> Result<Direction> {
        Direction::try_from(self.0)
    }

    /// Add a direction to the current Blizzard
    fn add(&mut self, direction: Direction) {
        self.0 |= direction.value();
    }

    /// Check if the Blizzard space has a Blizzard blowing in a
    /// particular direction.
    fn has(&self, direction: Direction) -> bool {
        self.0 & direction.value() > 0
    }
}

/// Represents a single space in the Valley. Can either be a space with Blizzards
/// blowing in one or more directions, an impassable Wall, or an Empty space where
/// the elven expedition can walk.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Space {
    Blizzard(Blizzard),
    Wall,
    Empty,
}

/// Represents the entire Valley and all the Blizzards blowing through it at a
/// given point in time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Valley {
    pub rows: usize,
    pub cols: usize,
    pub spaces: Vec<Vec<Space>>,
}

/// Produce a new Valley from a 2D grid of Spaces.
impl From<Vec<Vec<Space>>> for Valley {
    fn from(spaces: Vec<Vec<Space>>) -> Self {
        let rows = spaces.len();
        let cols = spaces.first().map(|line| line.len()).unwrap_or_default();
        Valley { rows, cols, spaces }
    }
}

impl From<&str> for Valley {
    fn from(input: &str) -> Self {
        let mut spaces = Vec::new();
        for line in input.lines() {
            let row = line
                .chars()
                .map(|glyph| match glyph {
                    '>' => Space::Blizzard(Blizzard::from(Direction::Right)),
                    '<' => Space::Blizzard(Blizzard::from(Direction::Left)),
                    '^' => Space::Blizzard(Blizzard::from(Direction::Up)),
                    'v' => Space::Blizzard(Blizzard::from(Direction::Down)),
                    '.' => Space::Empty,
                    '#' => Space::Wall,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            spaces.push(row);
        }

        Valley::from(spaces)
    }
}

impl Valley {
    /// Produce a clone of this Valley with its state advanced by one minute. All
    /// Blizzards move forward by one space in the direction they are facing,
    /// wrapping around the Valley when they encounter a Wall.
    fn advance(&self) -> Self {
        // Creates a new mutable clone of this Valley with only Empty Spaces
        let Valley { rows, cols, spaces } = self;
        let mut new_spaces = vec![vec![Space::Empty; *cols]; *rows];
        let mut new_state = Valley::from(new_spaces);

        // For each Space in the current Valley, update the appropriate space in the
        // new state. Move Blizzards and set Walls. Skip the Empties, since all the
        // Spaces in the new state are already Empty.
        for (row, col) in (0..*rows).cartesian_product(0..*cols) {
            match spaces[row][col] {
                Space::Blizzard(blizzard) => {
                    for direction in Direction::all() {
                        if !blizzard.has(direction) {
                            continue;
                        }
                        let (new_row, new_col) = self.next_position(row, col, direction);
                        new_state.add_blizzard(new_row, new_col, direction);
                    }
                }
                Space::Wall => new_state.spaces[row][col] = Space::Wall,
                Space::Empty => continue,
            }
        }

        // Return the new Valley
        new_state
    }

    /// Given a starting row and column and a Direction to move, return the row
    /// and column where you'd end up if you moved in that Direction.
    fn next_position(&self, row: usize, col: usize, direction: Direction) -> (usize, usize) {
        match direction {
            // Wrapping for increasing rows/columns
            Direction::Down => ((row % (self.rows - 2)) + 1, col),
            Direction::Right => (row, (col % (self.cols - 2)) + 1),

            // Wrapping for decreasing rows/columns
            Direction::Left => match col - 1 {
                0 => (row, self.cols - 2),
                _ => (row, col - 1),
            },
            Direction::Up => match row - 1 {
                0 => (self.rows - 2, col),
                _ => (row - 1, col),
            },
        }
    }

    /// Given a row and column and the Direction a blizzard is blowing, add that
    /// blizzard Direction to that Space. If the Space is empty, convert it to a
    /// Blizzard. If there's already a Blizzard there, just add the new Direction
    /// to the existing Blizzard. Attempts to add a Blizzard to a Wall will
    /// definitely fail.
    fn add_blizzard(&mut self, row: usize, col: usize, direction: Direction) {
        match &mut self.spaces[row][col] {
            Space::Blizzard(v) => v.add(direction),
            Space::Wall => panic!("Tried to add a blizzard to a wall!"),
            Space::Empty => self.spaces[row][col] = Space::Blizzard(Blizzard::from(direction)),
        }
    }
}

const INPUT: &str = include_str!("../../input/24/input.txt");

/// Parse the initial Valley state from the input, then advance the state
/// minute-by-minute until we reach a state we've seen before. That's right,
/// the state of the Valley cycles! This way, we can have in memory a map
/// of which Spaces are Empty and which ones have Blizzards for each point
/// in time. This means we don't need to re-calculate each state on the fly,
/// potentially re-creating the same state multiple times. Store the Valley
/// states in a Vector where the index indicates the minute at which that
/// state is valid.
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
    use std::collections::HashSet;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    impl Display for Valley {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            for row in &self.spaces {
                for space in row.iter() {
                    match space {
                        Space::Blizzard(blizzard) => {
                            if blizzard.0.count_ones() > 1 {
                                write!(f, "X")?;
                            } else {
                                let glyph = match blizzard.direction().unwrap() {
                                    Direction::Down => 'v',
                                    Direction::Left => '<',
                                    Direction::Up => '^',
                                    Direction::Right => '>',
                                };
                                write!(f, "{glyph}")?;
                            }
                        }
                        Space::Wall => {
                            write!(f, "#")?;
                        }
                        Space::Empty => {
                            write!(f, ".")?;
                        }
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
