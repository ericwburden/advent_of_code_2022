use crate::day14::{CaveMap, Input, Offset, Output, Point};
use std::collections::{HashSet, VecDeque};

/// Solve Day 14, Part 2
pub fn solve(input: &Input) -> Output {
    // Turn that set of impassable points into a `CaveMap`
    let cave_map = CaveMap::new(input.clone());

    // Now turn that `CaveMap` into a `FillMap` to determine how
    // much sand it takes to fill the pile.
    let mut fill_map = FillMap::from(cave_map);

    // Pour sand into the cave until it fills up to the entrypoint
    // and report the number of grains it took to do so.
    fill_map.sand_capacity().into()
}

/// A slight variation on the `CaveMap`. Mostly using a new struct for different
/// behaviors more than different data types.
#[derive(Debug, Clone)]
pub struct FillMap {
    obstacles: HashSet<Point>,
    entrypoint: Point,
    depth: u32,
}

impl FillMap {
    /// Unpack a `CaveMap` into a `FillMap`
    fn from(grid_map: CaveMap) -> Self {
        // Get the attributes from the `CaveMap`
        let CaveMap {
            obstacles,
            entrypoint,
            depth,
        } = grid_map;

        // Adjust the depth to represent the floor. Hey, look, there's that grain of
        // sand we thought was gone forever, breathing a huge sigh of relief. Good
        // for him!
        let depth = depth + 2;

        // Now it's a `FillMap`!
        FillMap {
            obstacles,
            entrypoint,
            depth,
        }
    }

    /// From a given Point, return an array indicating which points a grain of sand
    /// can flow into (e.g., that aren't blocked by an obstacle or the floor).
    fn get_neighbors(&self, point: Point) -> [Option<Point>; 3] {
        // The same three potential moves as the first part
        let offsets = [Offset(0, 1), Offset(-1, 1), Offset(1, 1)];

        // Array to hold the neighbors that can be moved to
        let mut neighbors = [None; 3];

        // For each possible offset...
        for (idx, offset) in offsets.iter().enumerate() {
            // The position we might move to.
            let try_pos = point + *offset;

            // If there's an obstacle there, skip it. Can't move there.
            if self.obstacles.contains(&try_pos) {
                continue;
            }

            // If there's floor there, skip it. Can't move there.
            if try_pos.1 >= self.depth {
                continue;
            }

            // Otherwise, we can move there. Add this point to our neighbors array.
            neighbors[idx] = Some(try_pos);
        }

        // Return the list of neighbors
        neighbors
    }

    /// Calculate the number of sand grains it'll take to fill in the pile and
    /// block off the entrypoint. Using Dijkstra's Algorithm! Nah, just kidding,
    /// it's a breadth-first search.
    fn sand_capacity(&self) -> u32 {
        let mut queue = VecDeque::from([self.entrypoint]);
        let mut visited = HashSet::new();
        let mut counted = 0; // Keep up with the number of grains

        // So long as we've got positions to try moving _from_...
        while let Some(point) = queue.pop_back() {
            // If we've visited this space before, skip it. Been here, done that.
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point); // Mark `point` as visited
            counted += 1; // Count this grain of sand

            // For each reachable neighbor point from the current point
            for neighbor in self.get_neighbors(point).iter().flatten() {
                // If we've visited that point before, skip it.
                if visited.contains(neighbor) {
                    continue;
                }

                // Add that point to the list of points to visit
                queue.push_front(*neighbor);
            }
        }

        counted // Return the number of grains of sand we counted
    }
}
