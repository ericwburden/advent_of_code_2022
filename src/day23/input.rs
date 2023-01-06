use std::str::FromStr;
use crate::day23::Input;
use super::grid::Grid;

const INPUT: &str = include_str!("../../input/23/input.txt");

/// Parse the input!
pub fn read() -> Input {
    Grid::from_str(INPUT).unwrap()
}
