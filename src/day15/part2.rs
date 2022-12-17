use crate::day15::{Input, Output, Point, Sensor};
use itertools::Itertools;

/// Solve Day 15, Part 2
///
/// Well, I think this is my first "what the heck is he doing" comment of the year.
/// Yay! So, this solution relies on some assumptions, the biggest of which is that
/// the beacon we're searching for will lie one space outside the range of a Sensor.
/// Why can we assume that? Well, the only possible way that a gap of larger than
/// one could contain the beacon is if the beacon were located in one of the four
/// corner points of the allowed range. That would kind of suck, though, so it's
/// probably not how it is. If it turns out to be the case, then we only have four
/// possible answers to try, so it's a win-win. So, given that the beacon most
/// likely lies in a one-wide gap between sensor ranges, we can treat that gap like
/// a line. The other assumption is that there's more than one one-wide gap.
/// Otherwise, again, we'd have multiple unscanned spaces. That other gap would
/// necessarily have a slope with opposite sign of the first gap, making an X with
/// the center on the beacon. X marks the spot! So, if we can identify all the
/// one-wide gaps in the range of the sensors, we can identify all the places where
/// these gap-lines intersect. If we can find one intersection that can't be
/// detected by any sensor, that's the one we want.
pub fn solve(input: &Input) -> Output {
    // Identify all the diagonal gaps between sensor detection ranges that are only
    // one space wide.
    let mut diagonal_gaps = Vec::new();
    for (sensor1, sensor2) in input.iter().tuple_combinations() {
        let Some(gap) = sensor1.gap_size(sensor2) else { continue; };
        if gap == 1 {
            diagonal_gaps.push(sensor1.diagonal_between(sensor2));
        }
    }

    // Identify all the points where these one-wide gaps intersect.
    let intersects = diagonal_gaps
        .iter()
        .tuple_combinations()
        .flat_map(|(diag1, diag2)| diag1.intersect(diag2))
        .unique()
        .collect_vec();

    // Check the intersections against all the sensors to identify the intersection
    // that cannot be detected by any Sensor. Now, for my input, there was only one
    // intersection that made it this far, but this check should make the solution
    // more robust for other inputs.
    'outer: for intersect in intersects {
        for sensor in input.iter() {
            if sensor.can_detect(&intersect) {
                continue 'outer;
            }
        }
        return intersect.tuning_frequency().into();
    }

    // Freak out if we can't find an intersection that can't be detected.
    panic!("Could not find the beacon!");
}

/// Represents a diagonal line. The Positive variant indicates a line with a slope
/// of 1 and the Negative variant indicates a line with a slope of -1.
#[derive(Debug)]
enum Diagonal {
    Positive(isize),
    Negative(isize),
}

impl Diagonal {
    /// Identify the point where two Diagonal lines intersect.
    fn intersect(&self, other: &Self) -> Option<Point> {
        // It's simple geometry! Which explains why it was so hard for me
        // to implement. Uses the formula for the two lines to calculate the
        // intersecting point, with some shortcuts because we know the slope
        // will either be positive or negative one for both lines, and if
        // the lines have the same slope, they're parallel and we can bail.
        use Diagonal::*;
        let (neg, pos) = match (self, other) {
            (Positive(pos), Negative(neg)) => (neg, pos),
            (Negative(neg), Positive(pos)) => (neg, pos),
            (Positive(_), Positive(_)) => return None,
            (Negative(_), Negative(_)) => return None,
        };
        let x = (neg - pos) / 2;
        let y = x + pos;
        Some(Point(x, y))
    }
}

impl Sensor {
    /// Find the size of the gap in the sensor range between two Sensors.
    /// If there's no gap (they overlap), return None.
    fn gap_size(&self, other: &Self) -> Option<usize> {
        let distance = self.location.distance_to(&other.location);
        let total_range = self.range + other.range;
        if total_range >= distance {
            return None;
        }
        Some(distance - total_range - 1)
    }

    /// Calculate the formula for the line that lies in the gap between two
    /// Sensor detection ranges. The line will lie diagonally just outside
    /// the range of `self`.
    fn diagonal_between(&self, other: &Self) -> Diagonal {
        let Point(x1, y1) = self.location;
        let Point(x2, y2) = other.location;
        let offset = self.range + 1;

        // Here, we identify two points on the diagonal line. We'll pick points just
        // outside the cardinal direction points of the `self` sensor range.
        let (p1x, p1y) = if x2 > x1 {
            (x1.saturating_add_unsigned(offset), y1)
        } else {
            (x1.saturating_sub_unsigned(offset), y1)
        };
        let (p2x, p2y) = if y2 > y1 {
            (x1, y1.saturating_add_unsigned(offset))
        } else {
            (x1, y1.saturating_sub_unsigned(offset))
        };

        // We know that the slope will either be 1 or -1, since these lines
        // are diagonals.
        let slope = (p2x - p1x) / (p2y - p1y);
        let intercept = p1y - (slope * p1x);
        if slope > 0 {
            Diagonal::Positive(intercept)
        } else {
            Diagonal::Negative(intercept)
        }
    }
}

impl Point {
    /// Calculate the tuning frequency the way the puzzle told us to.
    fn tuning_frequency(&self) -> u64 {
        (4_000_000 * self.0 as u64) + self.1 as u64
    }
}
