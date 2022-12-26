use super::input::{Direction, Grove, Position, Rule, Rules, Surroundings};
use crate::day23::{Input, Output};
use std::collections::HashMap;

/// Solve Day 23, Part 1
pub fn solve(input: &Input) -> Output {
    // Get a mutable Grove clone
    let mut grove = input.clone();

    // Advance to the next state ten times
    (0..10).for_each(|_| {
        grove.move_elves();
    });

    // Count the empty spaces and return the count
    grove.count_empty_spaces().into()
}

/// Represents the current status of a proposed move by an elf. When only one
/// elf has propsed to move to a particular Position, use the Move variant. If
/// two or more elves propose to move the the same space, use the Blocked
/// variant.
#[derive(Debug, Clone, Copy)]
enum Proposal {
    Move(usize),
    Blocked,
}

impl Grove {
    /// Get the Surroundings of a particular Position in the Grove. If there
    /// are no nearby elves, return None. Otherwise, return Some 3x3 array
    /// where each `true` value indicates the presence of a nearby elf.
    fn get_surroundings(&self, position: Position) -> Option<Surroundings> {
        use Direction::*;

        // Check each of the eight surrounding spaces for the presence
        // of another elf.
        let nw = self.occupied.contains(&position.offset(NorthWest));
        let no = self.occupied.contains(&position.offset(North));
        let ne = self.occupied.contains(&position.offset(NorthEast));
        let ea = self.occupied.contains(&position.offset(East));
        let se = self.occupied.contains(&position.offset(SouthEast));
        let so = self.occupied.contains(&position.offset(South));
        let sw = self.occupied.contains(&position.offset(SouthWest));
        let we = self.occupied.contains(&position.offset(West));
        let cr = true; // Used for the center space

        // If there are no nearby elves, return None
        if !(nw || no || ne || ea || se || so || sw || we) {
            return None;
        }

        // Otherwise, return the Surroundings
        Some(Surroundings::new([
            [nw, no, ne],
            [we, cr, ea],
            [sw, so, se],
        ]))
    }

    /// Gather all the proposals for moving from all the elves at the current
    /// state. Return a map of Positions and the Proposed action for each.
    fn propose_state(&self) -> HashMap<Position, Proposal> {
        let mut moves = HashMap::with_capacity(self.elves.len());

        // For each elf entry in the mapping of elf => position...
        for (elf, position) in self.elves.iter() {
            // If there are any nearby elves...
            let Some(surroundings) = self.get_surroundings(*position) else { continue; };

            // ...take the first valid proposal for moving the current elf...
            let Some(proposed) = self.rules.try_propose(position, &surroundings) else { continue; };

            // ..and try to add that proposal to the proposal mapping. If
            // this is the first elf proposing to move to the `proposed`
            // position, then add it as a Move proposal. If another elf
            // has already proposed to move to that position, block them
            // both.
            moves
                .entry(proposed)
                .and_modify(|e| *e = Proposal::Blocked)
                .or_insert(Proposal::Move(*elf));
        }
        moves
    }

    /// Simulate the elves proposing to move to their new positions then move
    /// the elves where you can. Rotate the rules so that they'll be in the
    /// correct order for the next state change.
    pub fn move_elves(&mut self) -> bool {
        let proposed_state = self.propose_state();

        // We'll identify and return whether any elves moved during this state change.
        let mut any_moved = false;

        // For each position with a proposal...
        for (position, proposal) in proposed_state.iter() {
            // If the proposal hasn't been blocked, then...
            let Proposal::Move(elf) = proposal else { continue; };

            // Move the elf to the proposed position.
            self.occupied.remove(&self.elves[elf]);
            self.elves.insert(*elf, *position);
            self.occupied.insert(self.elves[elf]);

            // Set the indicator for any elves moved to `true`
            any_moved = true;
        }

        // Rotate the rules order
        self.rules.rotate_left();

        // Return the indicator
        any_moved
    }

    /// Check the current arrangement of elves in the Grove and return two
    /// positions marking the bounds of the rectangle containing all the
    /// elves. The first Position is the top-left corner and the second
    /// Position is the bottom right corner.
    fn bounds(&self) -> (Position, Position) {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        for (xpos, ypos) in self.occupied.iter().map(|p| p.inner()) {
            min_x = min_x.min(xpos);
            max_x = max_x.max(xpos);
            min_y = min_y.min(ypos);
            max_y = max_y.max(ypos);
        }

        (Position::new(min_x, min_y), Position::new(max_x, max_y))
    }

    /// Count all the empty spaces in the smallest rectangle containing all
    /// the elves.
    fn count_empty_spaces(&self) -> u32 {
        let (min_pos, max_pos) = self.bounds();
        let (min_x, min_y) = min_pos.inner();
        let (max_x, max_y) = max_pos.inner();

        let mut empty_spaces = 0;
        for xpos in min_x..=max_x {
            for ypos in min_y..=max_y {
                let position = Position::new(xpos, ypos);
                if self.occupied.contains(&position) {
                    continue;
                }
                empty_spaces += 1;
            }
        }
        empty_spaces
    }
}

impl Position {
    /// Offset the position by one space in any of the eight directions.
    fn offset(&self, direction: Direction) -> Position {
        let (mut x, mut y) = self.inner();
        match direction {
            Direction::North => y -= 1,
            Direction::East => x += 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,

            Direction::NorthEast => {
                y -= 1;
                x += 1;
            }
            Direction::SouthEast => {
                y += 1;
                x += 1;
            }
            Direction::SouthWest => {
                y += 1;
                x -= 1;
            }
            Direction::NorthWest => {
                y -= 1;
                x -= 1;
            }
        };
        Position::new(x, y)
    }
}

impl Rules {
    /// Rotate the rules such that the current first rule is moved to the
    /// end and the other rules are all moved toward the front of the list.
    fn rotate_left(&mut self) {
        self.inner_mut().rotate_left(1);
    }
}

/// Since I had two structs that were both implementing a `try_propose()` method,
/// I decided to make it a Trait.
trait TryPropose {
    fn try_propose(&self, _: &Position, _: &Surroundings) -> Option<Position>;
}

impl TryPropose for Rule {
    /// Given this Rule and references to a Position and its Surroundings, identify
    /// whether the elf at Position should propose to move and to which Position
    /// he/she should propose to move. Return None if the elf should not propose
    /// to move according to the Rule.
    fn try_propose(&self, position: &Position, surroundings: &Surroundings) -> Option<Position> {
        let [[nw, n, ne], [w, x, e], [sw, s, se]] = surroundings.inner();
        let direction = match self {
            Rule::North if !(nw || n || ne) => Some(Direction::North),
            Rule::South if !(sw || s || se) => Some(Direction::South),
            Rule::East if !(ne || e || se) => Some(Direction::East),
            Rule::West if !(nw || w || sw) => Some(Direction::West),
            _ => None,
        }?;
        Some(position.offset(direction))
    }
}

impl TryPropose for Rules {
    /// Try to get the elf to propose to move by checking each Rule in order.
    /// If the elf can't move for any of the four rules, then return None.
    /// Otherwise, return the first Position that the elf should propose to
    /// move to.
    fn try_propose(&self, position: &Position, surroundings: &Surroundings) -> Option<Position> {
        self.inner()
            .iter()
            .flat_map(|rule| rule.try_propose(position, surroundings))
            .next()
    }
}

#[cfg(test)]
mod test {
    //! I decided to keep this test module just because I like these Display
    //! trait implementations. I find I often implement Display in the testing
    //! modules for these puzzles, I just tend to delete the intermediate
    //! test modules when I'm done with them. It's usually easier in Rust
    //! to output intermediate results using tests.
    use super::*;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    impl Display for Surroundings {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            for line in self.inner() {
                write!(f, "| ")?;
                for space in line {
                    let glyph = if space { '#' } else { '.' };
                    write!(f, "{glyph} ")?;
                }
                writeln!(f, "|")?;
            }
            write!(f, "")
        }
    }

    impl Display for Grove {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let (min_pos, max_pos) = self.bounds();
            let (min_x, min_y) = min_pos.inner();
            let (max_x, max_y) = max_pos.inner();
            for ypos in min_y..=max_y {
                write!(f, "| ")?;
                for xpos in min_x..=max_x {
                    let pos = Position::new(xpos, ypos);
                    let glyph = if self.occupied.contains(&pos) {
                        '#'
                    } else {
                        '.'
                    };
                    write!(f, "{glyph}")?;
                }
                writeln!(f, " |")?;
            }
            write!(f, "")
        }
    }
}
