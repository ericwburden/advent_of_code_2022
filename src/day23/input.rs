use crate::day23::Input;
use std::collections::{HashMap, HashSet};
use std::iter::{Enumerate, Map};
use std::str::Chars;

/// Represents the position of an Elf in the Grove
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(isize, isize);

/// Some convenience functions for working with the newtype Position.
/// I'm giving this approach a test-drive, since I don't really love
/// how Rust needs you to make the struct members public in order to
/// make a new struct, like Position(x, y). This way, I can just make
/// a public function, instead.
impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn inner(&self) -> (isize, isize) {
        (self.0, self.1)
    }
}

/// Represents the cardinal and secondary directions that matter for
/// today's puzzle.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Represents the presence/absence of other elves in the eight spaces
/// surrounding a given position. The middle element of the middle array
/// is pretty much ignored.
#[derive(Debug, Clone, Copy)]
pub struct Surroundings([[bool; 3]; 3]);

impl Surroundings {
    pub fn new(surroundings: [[bool; 3]; 3]) -> Self {
        Surroundings(surroundings)
    }

    pub fn inner(&self) -> [[bool; 3]; 3] {
        self.0
    }
}

/// Represents an indicator for which direction the elf should look when
/// determining which direction to move.
#[derive(Debug, Clone, Copy)]
pub enum Rule {
    North,
    South,
    East,
    West,
}

/// Represents the four rules in the order they should be considered.
#[derive(Debug, Clone)]
pub struct Rules([Rule; 4]);

impl Rules {
    pub fn new(rules: [Rule; 4]) -> Self {
        Rules(rules)
    }

    pub fn inner(&self) -> [Rule; 4] {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut [Rule; 4] {
        &mut self.0
    }
}

/// Represents the current status of a proposed move by an elf. When only one
/// elf has propsed to move to a particular Position, use the Move variant. If
/// two or more elves propose to move the the same space, use the Blocked
/// variant.
#[derive(Debug, Clone, Copy)]
pub enum Proposal {
    Move(usize),
    Blocked,
}

/// Represents the entire program state, the Grove that the elves are spreading
/// out in. Contains a mapping from elf ID to their current position, a set of
/// all the currently occupied positions, and the rules in the order they should
/// be considered.
#[derive(Debug, Clone)]
pub struct Grove {
    pub elves: HashMap<usize, Position>,
    pub occupied: HashSet<Position>,
    pub proposed: HashMap<Position, Proposal>,
    pub rules: Rules,
}

/// Our parsing function for today. Today's input seemed like it would be more
/// trouble to parse with `nom` that just by iterating over the characters.
impl<'a> From<&'a str> for Grove {
    fn from(input: &str) -> Self {
        // Build up the map of elf ID => elf position.
        let mut elves = HashMap::new();
        for (row_idx, row) in input.lines().enumerate() {
            for (col_idx, glyph) in row.chars().enumerate() {
                if glyph == '.' {
                    continue;
                }
                let position = Position(col_idx as isize, row_idx as isize);
                let elf_id = elves.len();
                elves.insert(elf_id, position);
            }
        }

        // The set of all occupied positions. This is the one we'll use to determine
        // what other elves are in the vicinity of each elf as it proposes its move.
        let occupied = elves.values().copied().collect::<HashSet<_>>();

        // The list of four rules in the initial order
        let rules = Rules([Rule::North, Rule::South, Rule::West, Rule::East]);

        // Maintaining a single list of proposed moves on the Grove
        let proposed = HashMap::with_capacity(occupied.len());

        Grove {
            elves,
            occupied,
            rules,
            proposed,
        }
    }
}

const INPUT: &str = include_str!("../../input/23/input.txt");

/// Parse the input!
pub fn read() -> Input {
    Grove::from(INPUT)
}
