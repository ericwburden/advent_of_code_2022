use crate::day17::{GasJetIter, Gust, Input, Output};

/// Solve Day 17, Part 1
pub fn solve(input: &Input) -> Output {
    // We need an owned copy of the iterator so we can use it in both parts
    let mut gas_jets = input.to_owned();
    let total_rocks = 2022;

    // Since the maximum height of any rock is 4, a chamber than can hold
    // up to rocks * 4 levels will be plenty big enough.
    let mut chamber = Chamber::with_capacity(total_rocks * 4);

    // Produce rocks of the appropriate shape in a cycle, up to `total_rocks`
    // times.
    for rock in Rock::all().iter().cycle().take(total_rocks) {
        chamber.add_rock(&mut gas_jets, *rock);
    }

    // Repor the total height of the rocks in the chamber
    (chamber.height() as u32).into()
}

// You'll note that we're storing rocks as u32 integers. This makes collision
// checking much easier, as well as adding rocks to the chamber. It does, however
// make _reading_ the code a bit more difficult. These are the 32-bit integers
// that represent the column just to the left of the left wall and the column
// just to the right of the right wall. If any part of our rock overlaps one of
// those spaces and the wind tries to push it in the corresponding direction,
// it won't move over.
const LEFT_WALL: u32 = 0x40404040;
const RIGHT_WALL: u32 = 0x01010101;

/// Represents one of the rocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rock(u32);

impl Rock {
    /// Produce a list of all five rock shapes, in order. This is what they look
    /// like in octal. Printed out in binary one byte at a time, with each byte
    /// on a new line, they look like this (using . instead of 0 to make it easier
    /// to see):
    ///
    /// ...1111.  ....1...  .....1..  ...1....  ...11...
    /// ........  ...111..  .....1..  ...1....  ...11...
    /// ........  ....1...  ...111..  ...1....  ........
    /// ........  ........  ........  ...1....  ........
    pub const fn all() -> [Self; 5] {
        [
            Self(0x0000001E), // -
            Self(0x00081C08), // +
            Self(0x0004041C), // L
            Self(0x10101010), // |
            Self(0x00001818), // #
        ]
    }

    /// Try to move a rock over to the left or right from a gas jet
    /// within the chamber.
    fn shove(&mut self, push: Gust, levels: u32) {
        // Get a copy of the rock bits
        let mut pushed = self.0;

        match push {
            // If pushed to the left and not against the wall, move the rock
            // to the left.
            Gust::Left => {
                if self.0 & LEFT_WALL == 0 {
                    pushed = self.0 << 1
                }
            }

            // If pushed to the right and not against the wall, move the rock
            // to the right.
            Gust::Right => {
                if self.0 & RIGHT_WALL == 0 {
                    pushed = self.0 >> 1
                }
            }
        }

        // If the pushed rock isn't colliding with anything in the same levels
        // as it in the chamber, then update the rock's horizontal position.
        if pushed & levels == 0 {
            self.0 = pushed
        }
    }

    /// Check to see if this rock collides with another object.
    fn collides(&self, other: u32) -> bool {
        self.0 & other > 0
    }

    /// Produce the bytes of the rock in order, skipping the empty bytes
    fn bytes(&self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b > 0)
    }
}

/// Represents the chamber where the rocks are being dropped
#[derive(Debug, Default, Clone)]
pub struct Chamber(pub Vec<u8>);

impl Chamber {
    /// Create a new chamber with a set capacity
    pub fn with_capacity(n: usize) -> Self {
        Self(Vec::with_capacity(n))
    }

    /// Report the height of the rocks in the chamber
    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// Add a new level (byte) to the chamber
    fn push(&mut self, level: u8) {
        self.0.push(level)
    }

    /// Get a 4-wide chunk of bytes from the chamber, starting at `level`.
    fn get_level_chunk(&self, level: usize) -> u32 {
        if level >= self.0.len() {
            return 0;
        }

        // Starting at `level`, take up to four bytes from the chamber, reverse
        // the production (so that the chunk is right-side up) of bytes, then
        // convert the four bytes into a single u32 by shifting existing bits
        // left and adding each new byte to the first 8 bits after the shift.
        self.0
            .iter()
            .skip(level)
            .take(4)
            .rev()
            .fold(0, |acc, byte| (acc << 8) | *byte as u32)
    }

    /// Add a rock to the top of the chamber and allow it to fall until it
    /// comes to rest on another rock or the floor.
    pub fn add_rock(&mut self, gas_jets: &mut GasJetIter, mut rock: Rock) {
        // Start the rock out three levels above the top of chamber, which is
        // the top level with any rock parts.
        let mut level = self.height() + 3;

        // Until the rock comes to rest...
        loop {
            // Get the chunk of chamber levels starting at `level`
            let levels = self.get_level_chunk(level);

            // Get the next gust from the gas jets
            let jet = gas_jets.next().unwrap();

            // Attempt to shove the rock over
            rock.shove(jet, levels);

            // If the current level is above the highest level with a rock in it
            // in the chamber, drop the level and shove the rock again.
            if level > self.height() {
                level -= 1;
                continue;
            }

            // Now get the chunk of the chamber starting one level below where
            // we started. This simulates the rock dropping down one level.
            let levels = self.get_level_chunk(level.saturating_sub(1));

            // Check to see if the rock would collide with anything if it moved down.
            let collision = rock.collides(levels);

            // If we reached the floor or would collide with another rock...
            if level == 0 || collision {
                // Add the rock to the chamber, layer by layer. Where there's a
                // chamber level already there for the layer of rock we're adding,
                // just add the layer of rock to the level. Otherwise, push the
                // layer to the top of the chamber's bytes.
                for byte in rock.bytes() {
                    if level < self.height() {
                        self.0[level] |= byte;
                    } else {
                        self.push(byte);
                    }
                    level += 1;
                }
                break;
            }

            // If the rock doesn't need to stop due to the floor or collision, keep
            // going, dropping the rock another level and going again.
            level -= 1;
        }
    }
}
