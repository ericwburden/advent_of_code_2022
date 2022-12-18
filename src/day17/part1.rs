use crate::day17::{GasJetIter, Input, Output, Push};

pub fn solve(input: &Input) -> Output {
    let mut gas_jets = input.to_owned();
    let total_rocks = 2022;
    let mut chamber = Chamber(Vec::with_capacity(total_rocks * 4));
    for rock in Shape::all().iter().cycle().take(total_rocks) {
        chamber.add_rock(&mut gas_jets, *rock);
    }
    (chamber.height() as u32).into()
}

const LEFT_WALL: u32 = 0x40404040;
const RIGHT_WALL: u32 = 0x01010101;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shape(u32);

impl Shape {
    pub const fn all() -> [Self; 5] {
        [
            Self(0x0000001E),
            Self(0x00081C08),
            Self(0x0004041C),
            Self(0x10101010),
            Self(0x00001818),
        ]
    }

    fn push(&mut self, push: Push, chamber: u32) {
        let mut pushed = self.0;
        match push {
            Push::Left => {
                if self.0 & LEFT_WALL == 0 {
                    pushed = self.0 << 1
                }
            }
            Push::Right => {
                if self.0 & RIGHT_WALL == 0 {
                    pushed = self.0 >> 1
                }
            }
        }

        if pushed & chamber == 0 {
            self.0 = pushed
        }
    }

    fn collides(&self, other: u32) -> bool {
        self.0 & other > 0
    }

    fn bytes(&self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b > 0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Chamber(pub Vec<u8>);

impl Chamber {
    pub fn height(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, level: u8) {
        self.0.push(level)
    }

    fn get_level_chunk(&self, level: usize) -> u32 {
        if level >= self.0.len() {
            return 0;
        }
        self.0
            .iter()
            .skip(level)
            .take(4)
            .rev()
            .fold(0, |acc, byte| (acc << 8) | *byte as u32)
    }

    pub fn add_rock(&mut self, gas_jets: &mut GasJetIter, mut rock: Shape) {
        let mut level = self.height() + 3;

        loop {
            let levels = self.get_level_chunk(level);
            let jet = gas_jets.next().unwrap();
            rock.push(jet, levels);

            if level > self.height() {
                level -= 1;
                continue;
            }

            let levels = self.get_level_chunk(level.saturating_sub(1));
            let collision = rock.collides(levels);
            if level == 0 || collision {
                for byte in rock.bytes() {
                    if level < self.height() {
                        self.0[level] |= byte;
                    } else {
                        self.push(byte);
                    }
                    level += 1;
                }
                return;
            }
            level -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    impl Display for Shape {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let bytes: Vec<_> = self.bytes().collect();
            for byte in bytes.into_iter().rev() {
                writeln!(f, "{:0>7b}", byte);
            }
            write!(f, "")
        }
    }

    // impl Display for Chamber {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    //         for byte in self.0.iter().rev() {
    //             writeln!(f, "{:0>7b}", byte);
    //         }
    //         write!(f, "")
    //     }
    // }

    // #[test]
    // fn playground() {
    //     let mut gas_jets = super::super::input::read();
    //     println!("{gas_jets:?}");
    //     let total_rocks = 3;
    //     let mut chamber = Chamber(Vec::with_capacity(total_rocks * 4));
    //     for rock in Shape::all().iter().cycle().take(total_rocks) {
    //         chamber.add_rock(&mut gas_jets, *rock);
    //     }
    //     println!("{chamber}");
    //     println!("{}", chamber.height());
    // }
}
