use crate::day14::{Input, Offset, Output, Point};
use std::collections::HashSet;

/// Solve Day 14, Part 1
pub fn solve(input: &Input) -> Output {
    // Turn that set of impassable points into a `CaveMap`
    let mut cave_map = CaveMap::new(input.clone());

    // We'll drop up to 10K grains of sand. This is overkill, but I'd rather
    // use a `for` loop here instead of a `loop`. Mostly so I have the grain
    // number in the loop without maintaining a separate variable and mutating
    // it each loop.
    for grains in 1..10_000 {
        // When we find the first grain of sand that falls into the infinite
        // abyss, we stop and return the current grain count minus one as
        // the number of grains _before_ this poor soul was lost to the void.
        if let GrainStatus::LostToTheAbyss = cave_map.add_sand() {
            return (grains - 1).into();
        }
    }

    // If we ever get here, something has gone horribly wrong
    unreachable!()
}

/// This enum represents the status of a grain of sand flowing down from the
/// ceiling. May represent the result of a single step or the entire flow of
/// that grain from start to finish.
#[derive(Debug)]
enum GrainStatus {
    MovedTo(Point),
    StoppedAt(Point),
    LostToTheAbyss,
}

/// Represents the map of our cave. We're keeping up with a set of the points in
/// space that sand can't flow through in `obstacles`, the point where the sand
/// enters the map in `entrypoint`, and the y-index of the lowest point containing
/// rock in `depth`. Past that is the timeless void.
#[derive(Debug, Clone)]
pub struct CaveMap {
    pub obstacles: HashSet<Point>,
    pub entrypoint: Point,
    pub depth: u32,
}

impl CaveMap {
    /// Create a new `CaveMap` from a set of points representing impassable points
    /// in the cave (for sand).
    pub fn new(obstacles: HashSet<Point>) -> Self {
        // Find the lowest y-coordinate for any rock
        let depth = obstacles
            .iter()
            .map(|point| point.1)
            .max()
            .unwrap_or_default();

        // Could have been a constant...
        let entrypoint = Point(500, 0);

        CaveMap {
            obstacles,
            entrypoint,
            depth,
        }
    }

    /// Add one grain of sand to the map and follow it as it flows down. Returns
    /// the final status of the grain.
    fn add_sand(&mut self) -> GrainStatus {
        let mut sand = self.entrypoint; // Sand flows in from here

        // Infinite loop!!! It'll stop eventually (we hope).
        loop {
            // Try to flow the grain of sand down one step
            let sand_flow = self.try_move_sand(sand);

            // Do different things depending on what happened when the grain of sand
            // attempted to move down one step.
            match sand_flow {
                // If the grain moved to a new point, update the grain and keep going
                GrainStatus::MovedTo(point) => sand = point,

                // If the grain stopped moving, add it as an obstacle to the cave
                // and break the loop, returning this final status of the grain.
                GrainStatus::StoppedAt(point) => {
                    self.obstacles.insert(point);
                    break sand_flow;
                }

                // If the grain of sand is now tumbling through the dark, slowly
                // forgetting what the sun looks like, break the loop and return
                // this depressing (for the grain of sand) result.
                GrainStatus::LostToTheAbyss => break sand_flow,
            }
        }
    }

    /// Try to move a grain of sand from it's current position downwards.
    fn try_move_sand(&self, sand: Point) -> GrainStatus {
        // A grain of sand can try to move down, down-left, and down-right, in
        // that order.
        let offsets = [Offset(0, 1), Offset(-1, 1), Offset(1, 1)];

        // Try each step in order...
        for offset in offsets {
            // The position that we want the grain of sand to take
            let try_pos = sand + offset;

            // If there's an obstacle there, then try moving another direction
            if self.obstacles.contains(&try_pos) {
                continue;
            }

            // If we're at the y-coordinate representing the depth, then we know
            // there is no hope for the grain of sand and it will spin down into
            // darkness. Return this status.
            if sand.1 >= self.depth {
                return GrainStatus::LostToTheAbyss;
            }

            // If the grain were able to move to the new position, return that result.
            return GrainStatus::MovedTo(try_pos);
        }

        // If all three directions were tried and failed, this grain can go no
        // further. Return that status.
        GrainStatus::StoppedAt(sand)
    }
}
