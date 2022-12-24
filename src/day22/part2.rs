use super::input::{Direction, Heading, MonkeyMap, Position, Tile};
use super::part1::Walker;
use crate::day22::{Input, Output};

/// Solve Day 22, Part 2
pub fn solve(input: &Input) -> Output {
    let (board, directions) = input;
    let mut board = board.clone();
    board.wrap(); // This is the difference!
    let Some(start_pos) = board.first_path_position() else { panic!("Cannot find start position!"); };
    let mut walker = Walker::new(start_pos);
    directions
        .iter()
        .for_each(|direction| walker.follow(&board, *direction));
    walker.score().into()
}

impl MonkeyMap {
    /// My map looks like this, where the pairs of letters indicate pairs of edges
    /// that line up to form the cube if this shape were folded.
    ///
    ///            _______ _______
    ///           |   A   |   B   |
    ///           |C      |      D|
    ///           |_______|___F___|
    ///           |       |
    ///           |E     F|
    ///    _______|_______|
    ///   |   E   |       |
    ///   |C      |      D|
    ///   |_______|___G___|
    ///   |       |
    ///   |A     G|
    ///   |___B___|
    ///
    /// Yours probably doesn't. This function re-maps the connections on the linkss
    /// on the outside edges to the correct matching edge. The hard part is making
    /// sure you match up the right edges in the right order. I recommend cutting
    /// out the shape of your map on a sheet of paper, folding it into a cube, and
    /// using that cube to determine how to match up the sides. This was an incredibly
    /// tedious bit of code to write because the various joins between the open face
    /// pairs all had their own bits of uniqueness that prevented me from doing this
    /// in a loop. So, each pair of faces is being joined manually.
    fn wrap(&mut self) {
        let inner = &self.0;

        // Match up the (A) sides. All the faces are joined this way, where I
        // iterate along the positions from the matching faces in the order that
        // they match up and modify their links as needed to have each edge direct
        // the Walker to the corresponding Tile on the other face. This was made
        // even more tedious by the fact that this approach is completely different
        // that the one I'd need to take to do this for the example input, since
        // the example map is shaped differently from my input.
        let side1 = (50..100).map(|col| Position(0, col));
        let side2 = (150..200).map(|row| Position(row, 0));
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.up = if self[p2].is_path() {
                    Some((Heading::Right, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.left = if self[p1].is_path() {
                    Some((Heading::Down, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // Match up the (B) sides.
        let side1 = (100..150).map(|col| Position(0, col));
        let side2 = (0..50).map(|col| Position(199, col));
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.up = if self[p2].is_path() {
                    Some((Heading::Up, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.down = if self[p1].is_path() {
                    Some((Heading::Down, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // Match up the (C) sides.
        let side1 = (0..50).map(|row| Position(row, 50));
        let side2 = (100..150).map(|row| Position(row, 0)).rev();
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.left = if self[p2].is_path() {
                    Some((Heading::Right, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.left = if self[p1].is_path() {
                    Some((Heading::Right, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // Match up the (D) sides.
        let side1 = (0..50).map(|row| Position(row, 149));
        let side2 = (100..150).map(|row| Position(row, 99)).rev();
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.right = if self[p2].is_path() {
                    Some((Heading::Left, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.right = if self[p1].is_path() {
                    Some((Heading::Left, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // Match up the (E) sides.
        let side1 = (50..100).map(|row| Position(row, 50));
        let side2 = (0..50).map(|col| Position(100, col));
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.left = if self[p2].is_path() {
                    Some((Heading::Down, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.up = if self[p1].is_path() {
                    Some((Heading::Right, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // Match up the (F) sides.
        let side1 = (100..150).map(|col| Position(49, col));
        let side2 = (50..100).map(|row| Position(row, 99));
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.down = if self[p2].is_path() {
                    Some((Heading::Left, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.right = if self[p1].is_path() {
                    Some((Heading::Up, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // Match up the (G) sides.
        let side1 = (50..100).map(|col| Position(149, col));
        let side2 = (150..200).map(|row| Position(row, 49));
        for (p1, p2) in std::iter::zip(side1, side2) {
            if self[p1].is_path() {
                let mut links = self[p1].links();
                links.down = if self[p2].is_path() {
                    Some((Heading::Left, p2))
                } else {
                    None
                };
                self[p1] = Tile::Path(links);
            }

            if self[p2].is_path() {
                let mut links = self[p2].links();
                links.right = if self[p1].is_path() {
                    Some((Heading::Up, p1))
                } else {
                    None
                };
                self[p2] = Tile::Path(links);
            }
        }

        // And that's that! Seven slightly different edge-to-edge joins! I can't
        // tell you how many small "typo" bugs I made in these seven blocks. The
        // good news is that once the edges are forwarding correctly, walking the
        // "cube" is exactly the same as walking the map in part one.
    }
}
