use super::input::{Direction, Heading, Links, MonkeyMap, Position, Tile};
use crate::day22::{Input, Output};

/// Solve Day 22, Part 1
pub fn solve(input: &Input) -> Output {
    // This bit is just so I can convert the reference to the input into a
    // mutable map. I pass in input as a reference because it keeps me from
    // accidentally mutating it in Part 1 and spending hours wondering why
    // my solution for Part 2 doesn't work on the input I forgot I had mutated.
    let (monkey_map, directions) = input;
    let mut board = monkey_map.clone();

    // Start at the first path Tile on the first row, facing right
    let Some(start_pos) = board.first_path_position() else { panic!("Cannot find start position!"); };
    let mut walker = Walker::new(start_pos);

    // Follow each direction
    directions
        .iter()
        .for_each(|direction| walker.follow(&board, *direction));

    // Let the walker calculate its own score and return it
    walker.score().into()
}

impl MonkeyMap {
    /// Finds the position of the first Tile::Path in the MonkeyMap,
    /// in reading order (left to right, top to bottom), if there is
    /// one.
    pub fn first_path_position(&self) -> Option<Position> {
        let first_row = self.0.first()?;
        let first_col = first_row
            .iter()
            .position(|tile| matches!(tile, Tile::Path(_)))?;
        Some(Position(0, first_col))
    }
}

impl Tile {
    /// Get the links associated with the Tile. If it's a Tile::Void
    /// or Tile::Wall, the default is a Links with no links filled in.
    /// That doesn't actually come up much, though.
    pub fn links(&self) -> Links {
        match self {
            Tile::Path(links) => *links,
            _ => Links::default(),
        }
    }
}

/// Represents a "walker" walking the path on the MonkeyMap.
#[derive(Debug)]
pub struct Walker(pub Heading, pub Position);

impl Walker {
    pub fn new(position: Position) -> Self {
        Walker(Heading::Right, position)
    }

    /// Lets the Walker calculate it's own score according to the puzzle description.
    pub fn score(&self) -> u32 {
        let Walker(heading, position) = self;
        let Position(row, col) = position;
        let heading_mod = match heading {
            Heading::Right => 0,
            Heading::Down => 1,
            Heading::Left => 2,
            Heading::Up => 3,
        };
        ((*row as u32 + 1) * 1000) + ((*col as u32 + 1) * 4) + heading_mod
    }

    /// Given a direction and a reference to the map, attempt to follow the
    /// direction by moving or turning the Walker.
    pub fn follow(&mut self, map: &MonkeyMap, direction: Direction) {
        let Walker(heading, position) = self;
        let tile = map[*position];
        let Tile::Path(links) = tile else { return; };

        use Direction::*;
        use Heading::*;
        match (heading, direction) {
            // Turning is easy, just match and update the Walker with the new heading
            (Up, TurnLeft) => *self = Walker(Left, *position),
            (Up, TurnRight) => *self = Walker(Right, *position),

            (Right, TurnLeft) => *self = Walker(Up, *position),
            (Right, TurnRight) => *self = Walker(Down, *position),

            (Down, TurnLeft) => *self = Walker(Right, *position),
            (Down, TurnRight) => *self = Walker(Left, *position),

            (Left, TurnLeft) => *self = Walker(Down, *position),
            (Left, TurnRight) => *self = Walker(Up, *position),

            // Moving forward is a bit tougher, but not too serious.
            (heading, Forward(n)) => {
                // As many times as we're suppose to move forward...
                for _ in 0..n {
                    // Unpack the walker again since we're going to modify it
                    // directly in subsequent loops.
                    let Walker(heading, position) = self;

                    // Here's where having the Links stored on the Tiles comes in
                    // handy. We can just query the links on the current Tile for
                    // the heading and position we should be at if we move in the
                    // indicated direction from the current Tile.
                    let mut links = map[*position].links();
                    let link = match heading {
                        Up => links.up,
                        Right => links.right,
                        Down => links.down,
                        Left => links.left,
                    };

                    // If there's no link in the indicated direction, just stop.
                    // Otherwise, update the Walker and keep going.
                    let Some((heading, position)) = link else { break; };
                    *self = Walker(heading, position)
                }
            }
        };
    }
}
