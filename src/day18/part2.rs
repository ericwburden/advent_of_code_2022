use crate::day18::{Input, Output};
use super::part1::Offset;
use super::input::Cube;
use std::ops::RangeInclusive;
use std::collections::HashSet;

/// Solve Day 18, Part 2
pub fn solve(input: &Input) -> Output {
    // Identify the bounding box that contains all the Cubes for the lava
    // plus at least one extra in each dimension.
    let bounds = input.get_bounds();

    // Adding up surface area, like before
    let mut surface_area = 0;

    // Prepare for a Depth-First Search through all the Cubes that are within
    // the bounding box but that _aren't_ lava. Each "air" cube that touches
    // a lava cube adds one to the total observed surface area.
    let mut stack = Vec::with_capacity(input.len() * 2);
    let mut seen = HashSet::with_capacity(input.len() * 2);

    // This isn't technically a corner of the bounding box or anything, but
    // I checked my input and there isn't any lava at this location. It really
    // doesn't matter _where_ you start the search, so long as it's a Cube
    // outside the lava blob.
    let start = Cube::new(0, 0, 0);
    stack.push(start);

    // So long as there are still Cubes outside the lava blob to check...
    while let Some(cube) = stack.pop() {

        // If the cube we're on has already been explored or it falls outside
        // the bounding box, skip it.
        if seen.contains(&cube) || !bounds.contains(&cube) {
            continue;
        }

        // If the cube we're on contains lava, then we add one to our surface area
        // and move on. We don't want to explore the lava-containing cubes.
        if input.contains(&cube) {
            surface_area += 1;
            continue;
        }

        seen.insert(cube);

        // For each cube that shares a face with the current Cube, if we haven't 
        // explored it already, add it to the stack for later exploration.
        for neighbor in cube.neighbors() {
            if seen.contains(&neighbor) {
                continue;
            }
            stack.push(neighbor);
        }
    }

    surface_area.into()
}

/// Represents the 3D range that contains all the air Cubes we want to explore,
/// encapsulating the lava Cubes.
struct Bounds(RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>);

/// This trait provides a convenient way to get the bounding box of
/// a HashSet of Cubes
trait GetBounds {
    fn get_bounds(&self) -> Bounds;
}

impl GetBounds for &HashSet<Cube> {
    /// Nothing fancy here. Identify the minimim and maximum values on each axis
    /// for all the lava-containing cubes, then set the bounding box to be one
    /// greater in each dimension. This leaves a one-wide gap around the lava
    /// blob that is OK to explore.
    fn get_bounds(&self) -> Bounds {
        let (mut min_x, mut max_x) = (0, 0);
        let (mut min_y, mut max_y) = (0, 0);
        let (mut min_z, mut max_z) = (0, 0);

        for cube in self.iter() {
            let (x, y, z) = cube.inner();
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            min_z = min_z.min(z);
            max_z = max_z.max(z);
        }

        let x_range = (min_x - 1)..=(max_x + 1);
        let y_range = (min_y - 1)..=(max_y + 1);
        let z_range = (min_z - 1)..=(max_z + 1);

        Bounds(x_range, y_range, z_range)
    }
}

impl Bounds {
    /// Indicates whether a Cube lies within the Bounds
    fn contains(&self, cube: &Cube) -> bool {
        let (x, y, z) = cube.inner();
        let Bounds( x_range, y_range, z_range ) = self;
        x_range.contains(&x) && y_range.contains(&y) && z_range.contains(&z)
    }
}

