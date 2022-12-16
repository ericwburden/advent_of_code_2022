use crate::day15::Input;

/// Represents a point in the 2D plane where our sensors are located
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);

impl Point {
    // Calculate the Manhattan Distance from one Point to another Point
    pub fn distance_to(&self, other: &Self) -> usize {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }
}

/// Easily create a Point from a tuple of i32's. This is mostly here because
/// the `nom` parsers don't provide isize directly.
impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;
        Point(x as isize, y as isize)
    }
}

/// Represents one of our sensors. Encapsulates the location of the beacon it is
/// detecting and the Senso's detection range.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sensor {
    pub location: Point,
    pub beacon: Point,
    pub range: usize,
}

impl Sensor {
    fn new(location: Point, beacon: Point) -> Self {
        let range = location.distance_to(&beacon);
        Sensor {
            location,
            beacon,
            range,
        }
    }
}

/// Convert a tuple of Points into a Sensor. The first Point is the location of the
/// Sensor, the second is the location of the beacon.
impl From<(Point, Point)> for Sensor {
    fn from(value: (Point, Point)) -> Self {
        let (location, beacon) = value;
        Sensor::new(location, beacon)
    }
}

/// Internal module wrapping the `nom` parser for the input
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::take_till,
        character::{
            complete::{i32, newline},
            is_alphabetic, is_digit,
        },
        combinator::{map, recognize},
        multi::separated_list0,
        sequence::{pair, preceded},
        Finish, IResult,
    };


    /// Nom parser to skip everything that's not a number or minus sign
    fn till_number(s: &str) -> IResult<&str, &str> {
        take_till(|c: char| c.is_ascii_digit() || c == '-')(s)
    }

    /// Nom parser to parse an i32 with a bunch of cruft in front of it
    fn prefixed_number(s: &str) -> IResult<&str, i32> {
        preceded(till_number, i32)(s)
    }

    /// Nom parser to take the first two found numbers and return a Point from them
    fn point(s: &str) -> IResult<&str, Point> {
        map(pair(prefixed_number, prefixed_number), Point::from)(s)
    }

    /// Get the two Points from an input line
    fn sensor(s: &str) -> IResult<&str, Sensor> {
        map(pair(point, point), Sensor::from)(s)
    }

    /// Parse all lines from the input into a list of Sensors
    fn sensors(s: &str) -> IResult<&str, Vec<Sensor>> {
        separated_list0(newline, sensor)(s)
    }

    /// Parse the input, returns a list of Sensors
    pub fn parse(s: &str) -> Result<Vec<Sensor>> {
        let (_, result) = sensors(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

const INPUT: &str = include_str!("../../input/15/input.txt");

/// Parse that input!
pub fn read() -> Input {
    let mut input = parser::parse(INPUT).unwrap();
    input.sort_unstable();
    input
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 26);

        let first_found = *input.first().unwrap();
        let first_expected =
            Sensor::new(Point(3772068, 2853720), Point(4068389, 2345925));
        assert_eq!(first_found, first_expected);

        let last_found = *input.last().unwrap();
        let last_expected =
            Sensor::new(Point(2712265, 2155055), Point(2700909, 2519581));
        assert_eq!(last_found, last_expected);
    }
}
