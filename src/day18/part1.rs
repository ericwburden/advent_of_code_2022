use super::input::Cube;
use crate::day18::{Input, Output};
use std::collections::HashSet;
use std::ops::Add;

/// Solve Day 18, Part 1
pub fn solve(input: &Input) -> Output {
    let mut surface_area = 0;

    // For each cube in the input, look at each cube that shares
    // a face with it. For every one of those neighboring cubes that
    // is not lava, add one to the total surface area.
    for cube in input.iter() {
        for neighbor in cube.neighbors() {
            if !input.contains(&neighbor) {
                surface_area += 1;
            }
        }
    }

    surface_area.into()
}

/// Represents an offset used to adjust a Cube location
#[derive(Debug, Clone, Copy)]
pub struct Offset(i32, i32, i32);

impl Offset {
    /// All the offsets of interest. Provides the offsets that will result
    /// in a Cube that shares a face with an existing Cube.
    const fn all() -> [Offset; 6] {
        [
            Offset(-1, 0, 0),
            Offset(1, 0, 0),
            Offset(0, -1, 0),
            Offset(0, 1, 0),
            Offset(0, 0, -1),
            Offset(0, 0, 1),
        ]
    }

    /// Convenience!
    fn new(x: i32, y: i32, z: i32) -> Self {
        Offset(x, y, z)
    }

    /// Convenience!
    fn inner(&self) -> (i32, i32, i32) {
        let Offset(x, y, z) = self;
        (*x, *y, *z)
    }
}

/// Allows for adding an Offset to a Cube to get a Cube offset from the original
impl Add<Offset> for Cube {
    type Output = Cube;

    fn add(self, offset: Offset) -> Self::Output {
        let (cx, cy, cz) = self.inner();
        let (ox, oy, oz) = offset.inner();
        let x = cx + ox;
        let y = cy + oy;
        let z = cz + oz;

        Cube::new(x, y, z)
    }
}

impl Cube {
    /// Get all the neighboring Cubes that share a face with this one.
    pub fn neighbors(&self) -> [Cube; 6] {
        let mut cubes = [Cube::default(); 6];
        let offsets = Offset::all();
        for (slot, offset) in cubes.iter_mut().zip(offsets.iter()) {
            *slot = *self + *offset;
        }
        cubes
    }
}
