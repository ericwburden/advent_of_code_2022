use crate::day22::Input;
use itertools::Itertools;
use std::ops::{Index, IndexMut};

/// Represents a position on the "map", from a top-down perspective. A
/// Position is given as (row, column).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Position(pub usize, pub usize);

/// Indicates the heading of the current movement on the map, from a
/// top-down perspective.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Heading {
    Up,
    Right,
    Down,
    Left,
}

/// Represents the links from one point on the map to another. This way, each
/// point on the map knows what the heading and position will be for someone
/// who moves in a given direction from that point.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Links {
    pub up: Option<(Heading, Position)>,
    pub right: Option<(Heading, Position)>,
    pub down: Option<(Heading, Position)>,
    pub left: Option<(Heading, Position)>,
}

/// Represents a tile on the map
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Void,
    Path(Links),
    Wall,
}

impl Tile {
    /// Sometimes, it's nice to have a function to let us know if the current
    /// Tile is a Tile::Path or not.
    pub fn is_path(&self) -> bool {
        matches!(self, Tile::Path(_))
    }
}

/// Represents a movement direction, given on the last line of the input file.
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    TurnLeft,
    TurnRight,
    Forward(u32),
}

/// Represents the tiles on the map given to us by the monkeys, with the added
/// metadata from us added in.
#[derive(Debug, Clone)]
pub struct MonkeyMap(pub Vec<Vec<Tile>>);

/// Namespacing for the parsers used in today's puzzle.
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_till, take_while},
        character::complete::{alphanumeric0, newline, u32},
        combinator::{map, value},
        multi::{many0, many1, separated_list0},
        sequence::{pair, separated_pair},
        Finish, IResult,
    };

    /// Nom parser for " " -> Tile::Void
    fn void(s: &str) -> IResult<&str, Tile> {
        value(Tile::Void, tag(" "))(s)
    }

    /// Nom parser for "." -> Tile::Path()
    fn passable(s: &str) -> IResult<&str, Tile> {
        value(Tile::Path(Links::default()), tag("."))(s)
    }

    /// Nom parser for "#" -> Tile::Wall
    fn wall(s: &str) -> IResult<&str, Tile> {
        value(Tile::Wall, tag("#"))(s)
    }

    /// Nom parser for all Tile variants
    fn tile(s: &str) -> IResult<&str, Tile> {
        alt((void, passable, wall))(s)
    }

    /// Nom parser for a line of Tiles in the input
    fn tile_line(s: &str) -> IResult<&str, Vec<Tile>> {
        many1(tile)(s)
    }

    /// Nom parser for all the tiles in the input
    pub fn monkey_map(s: &str) -> IResult<&str, MonkeyMap> {
        map(separated_list0(newline, tile_line), MonkeyMap)(s)
    }

    /// Nom parser for "10" -> Direction::Forward(10)
    fn forward(s: &str) -> IResult<&str, Direction> {
        map(u32, Direction::Forward)(s)
    }

    /// Nom parser for "L" -> Direction::TurnLeft
    fn left(s: &str) -> IResult<&str, Direction> {
        value(Direction::TurnLeft, tag("L"))(s)
    }

    /// Nom parser for "R" -> Direction::TurnRight
    fn right(s: &str) -> IResult<&str, Direction> {
        value(Direction::TurnRight, tag("R"))(s)
    }

    /// Nom parser for all Direction variants
    fn direction(s: &str) -> IResult<&str, Direction> {
        alt((forward, left, right))(s)
    }

    /// Nom parser for the entire list of Directions given in the last line of
    /// the input.
    pub fn directions(s: &str) -> IResult<&str, Vec<Direction>> {
        many0(direction)(s)
    }

    /// Nom parser for both parts of the input, separated by an empty line.
    fn both_parts(s: &str) -> IResult<&str, (MonkeyMap, Vec<Direction>)> {
        separated_pair(monkey_map, tag("\n\n"), directions)(s)
    }

    /// Entrypoint for parser combinators, parses the input file into a
    /// MonkeyMap and list of Directions.
    pub fn parse(s: &str) -> Result<(MonkeyMap, Vec<Direction>)> {
        let (_, result) = both_parts(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

/// It's convenient to be able to index into the MonkeyMap using a Position,
/// since Position is used to represent a point on the MonkeyMap.
impl Index<Position> for MonkeyMap {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        let Position(row, col) = index;
        &self.0[row][col]
    }
}

/// It's also convenient to be able to access these indices mutably.
impl IndexMut<Position> for MonkeyMap {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let Position(row, col) = index;
        &mut self.0[row][col]
    }
}

impl MonkeyMap {
    /// Build up the links to other Tiles on each Tile::Path in the MonkeyMap. This
    /// way, we can check a given Tile for the heading and position of the tile in
    /// each of the four cardinal directions.
    fn map_positions(&mut self) {
        let rows = self.0.len();

        // Iterating over the indices keeps borrow checker wrangling to a minimum
        // here. We need to get the number of columns individually for each row,
        // since each row has a variable number of columns. The input doesn't have
        // spaces at the ends of lines where the map doesn't span to the end of the
        // line.
        for row in 0..rows {
            let cols = self.0.get(row).map(|v| v.len()).unwrap_or_default();
            for col in 0..cols {
                if let Tile::Path(mut links) = self.0[row][col] {
                    let position = Position(row, col);
                    links.up = self.find_next_up(position);
                    links.right = self.find_next_right(position);
                    links.down = self.find_next_down(position);
                    links.left = self.find_next_left(position);
                    self[position] = Tile::Path(links);
                }
            }
        }
    }

    /// From a given position, identify the Heading/Position of the Tile that can
    /// be achieved by moving up from the current position. Accounts for wrapping
    /// around the map when moving up into an unmarked space.
    fn find_next_up(&self, position: Position) -> Option<(Heading, Position)> {
        let Position(row, col) = position;
        let inner_iter = self.0.iter();

        // This is a bit of a gnarly iterator chain, but the ultimate outcome is that
        // it produces an iterator that starts at the current position and moves along
        // the current column by row, in reverse, and returns the first position found
        // that isn't a Tile::Void, which provides the wrapping functionality. This
        // is probably the gnarliest of the four functions, since it requires iterating
        // by column and in reverse. The rest of the `find_next_*()` functions are
        // all variations on this one.
        let (found_row, found_tile) = inner_iter
            .enumerate()
            .rev()
            .map(|(row_idx, row)| (row_idx, row.get(col).unwrap_or(&Tile::Void)))
            .cycle()
            .skip(self.0.len() - row)
            .find(|(_, tile)| !matches!(tile, Tile::Void))?;
        if found_tile.is_path() {
            return Some((Heading::Up, Position(found_row, col)));
        }
        None
    }

    /// From a given position, identify the Heading/Position of the Tile that can
    /// be achieved by moving down from the current position. Accounts for wrapping
    /// around the map when moving up into an unmarked space.
    fn find_next_down(&self, position: Position) -> Option<(Heading, Position)> {
        let Position(row, col) = position;
        let inner_iter = self.0.iter();
        let (found_row, found_tile) = inner_iter
            .enumerate()
            .map(|(row_idx, row)| (row_idx, row.get(col).unwrap_or(&Tile::Void)))
            .cycle()
            .skip(row + 1)
            .find(|(_, tile)| **tile != Tile::Void)?;
        if found_tile.is_path() {
            return Some((Heading::Down, Position(found_row, col)));
        }
        None
    }

    /// From a given position, identify the Heading/Position of the Tile that can
    /// be achieved by moving left from the current position. Accounts for wrapping
    /// around the map when moving up into an unmarked space.
    fn find_next_left(&self, position: Position) -> Option<(Heading, Position)> {
        let Position(row, col) = position;
        let row_of_tiles = self.0.get(row)?;
        let (found_col, found_tile) = row_of_tiles
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(row_of_tiles.len() - col)
            .find(|(_, tile)| **tile != Tile::Void)?;
        if found_tile.is_path() {
            return Some((Heading::Left, Position(row, found_col)));
        }
        None
    }

    fn find_next_right(&self, position: Position) -> Option<(Heading, Position)> {
        let Position(row, col) = position;
        let row_of_tiles = self.0.get(row)?;

        // And this one is probably the least gnarly. Which obviously means it was
        // the one I wrote last. I really should have written these in reverse,
        // because writing the "up" iterator chain as a PITA and I could have
        // used this one as a template. Ah, well.
        let (found_col, found_tile) = row_of_tiles
            .iter()
            .enumerate()
            .cycle()
            .skip(col + 1)
            .find(|(_, tile)| **tile != Tile::Void)?;
        if found_tile.is_path() {
            return Some((Heading::Right, Position(row, found_col)));
        }
        None
    }
}

const INPUT: &str = include_str!("../../input/22/input.txt");

/// Parse that input!
pub fn read() -> Input {
    let (mut board, directions) = parser::parse(INPUT).unwrap();
    board.map_positions();
    (board, directions)
}
