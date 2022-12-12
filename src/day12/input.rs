use crate::day12::Input;
use std::collections::HashMap;

/// Represents a hill on the map. Wraps the hill height and indicates
/// if it's a start, end, or just plain old hill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hill {
    Start(u8),
    End(u8),
    Hill(u8),
}

/// Convert characters to `Hill`s
impl From<char> for Hill {
    fn from(value: char) -> Self {
        match value {
            'S' => Hill::Start(0),
            'E' => Hill::End(25),
            c if c.is_ascii_lowercase() => Hill::Hill(value as u8 - b'a'),
            _ => unreachable!(),
        }
    }
}

impl Hill {
    // Pull the height from a hill
    fn height(&self) -> u8 {
        match self {
            Hill::Start(h) => *h,
            Hill::End(h) => *h,
            Hill::Hill(h) => *h,
        }
    }

    // Indicate whether the `other` hill can be reached from the
    // current hill, considering elevation only.
    fn can_reach(&self, other: &Hill) -> bool {
        // From the current hill, we can reach hills that are at most one
        // elevation level above the hill we're on.
        other.height().saturating_sub(self.height()) <= 1
    }
}

// Type alias we'll use to refer to hills that can be reached from the current hill
type Neighbors = [Option<(usize, usize)>; 4];

/// Represents a map of all the hills in the area. Includes the two-dimensional
/// listing of hills, the positions of the starting hill and the end hill, and
/// a hashmap that will serve as an adjacency list for hills and the neighbors
/// that can be reached from them.
pub struct HillMap {
    pub hills: Vec<Vec<Hill>>,
    pub graph: HashMap<(usize, usize), Neighbors>,
    pub start_at: (usize, usize),
    pub end_at: (usize, usize),
}

/// Convert the input string into a `HillMap`.
impl From<&str> for HillMap {
    fn from(value: &str) -> Self {
        // Start by building up the 2D vector of hills, in the same shape as the
        // characters in the input.
        let hills: Vec<Vec<_>> = value
            .lines()
            .map(|row| row.chars().map(Hill::from).collect())
            .collect();

        // Now we convert the vector of hills into a mapping we can use for a
        // nice graph algorithm (more easily). We'll precompute which neighbors
        // are reachable here to save processing later.
        let mut graph = HashMap::new();

        // Make sure we know the dimensions of the grid
        let last_row = hills.len().saturating_sub(1);
        let last_col = hills
            .first()
            .map(|r| r.len())
            .unwrap_or_default()
            .saturating_sub(1);

        // Prepare to identify the start and end locations
        let mut start_at = (0, 0);
        let mut end_at = (0, 0);

        // For each row and column in our vector of hills...
        for (row_idx, row) in hills.iter().enumerate() {
            for (col_idx, hill) in row.iter().enumerate() {
                // Create and fill in the array of neighbors in order of direction
                // from up, left, down, and right. I'm using arrays here because I
                // believe in my heart that arrays are more efficient that vectors,
                // and I won't be convinced otherwise! (today) We're doing our
                // bounds checking here as well, just to save on possible edge cases
                // later on.
                let mut neighbors = [None; 4];
                if row_idx > 0 && hill.can_reach(&hills[row_idx - 1][col_idx]) {
                    neighbors[0] = Some((row_idx - 1, col_idx));
                }
                if col_idx > 0 && hill.can_reach(&hills[row_idx][col_idx - 1]) {
                    neighbors[1] = Some((row_idx, col_idx - 1));
                }
                if row_idx < last_row && hill.can_reach(&hills[row_idx + 1][col_idx]) {
                    neighbors[2] = Some((row_idx + 1, col_idx));
                }
                if col_idx < last_col && hill.can_reach(&hills[row_idx][col_idx + 1]) {
                    neighbors[3] = Some((row_idx, col_idx + 1));
                }

                // When we encounter the start and end hills, we mark those as special.
                if let Hill::Start(_) = hill {
                    start_at = (row_idx, col_idx);
                }
                if let Hill::End(_) = hill {
                    end_at = (row_idx, col_idx);
                }
                graph.insert((row_idx, col_idx), neighbors);
            }
        }

        // All done!
        HillMap {
            hills,
            graph,
            start_at,
            end_at,
        }
    }
}

const INPUT: &str = include_str!("../../input/12/input.txt");

/// Parse that input!
pub fn read() -> Input {
    HillMap::from(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let hill_map = HillMap::from(INPUT);
        assert_eq!(
            hill_map.hills.iter().flat_map(|r| r.iter()).count(),
            hill_map.graph.keys().len()
        );
        assert_eq!(hill_map.hills[20][0], Hill::Start(0));
        assert_eq!(hill_map.hills[20][132], Hill::End(25));
        assert_eq!(
            *hill_map.graph.get(&(20, 0)).unwrap(),
            [Some((19, 0)), None, Some((21, 0)), Some((20, 1))]
        );
    }
}
