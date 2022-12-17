use crate::day15::{Input, Output, Point, Sensor};
use itertools::Itertools;

/// Solve Day 15, Part 1
pub fn solve(input: &Input) -> Output {
    let row = 2_000_000; // Our hard-coded row of interest

    // Identify a RowRange for each sensor indicated the furthest left and furthest
    // right point detected by each sensor. These RowRanges may overlap, though, so
    // we need to 'condense' the ranges so we don't double-count any points. This
    // part depends on the input being sorted ahead of time, so that the RowRanges
    // come out sorted, so that all the RowRanges that can be condensed will be
    // encountered one after another. If the input isn't sorted here, you'll get a
    // different (wrong) answer.
    let mut ranges: Vec<RowRange> = Vec::new();
    for range in input.iter().flat_map(|s| s.row_range_sensed(row)) {
        // If the current range can be merged with the last range in `ranges`, do so.
        // Otherwise, just push the current range to the list.
        if let Some(last_rng) = ranges.last_mut() {
            if last_rng.overlaps(&range) {
                last_rng.merge(&range);
            }
            continue;
        }
        ranges.push(range);
    }

    // Count the number of observable positions on the target row, defined by
    // the limits of the `ranges`.
    let sensed_on_row = ranges.iter().map(|r| r.count_positions()).sum::<usize>();

    // We'll need to subtract out the number of beacons on the row, since those
    // points definitely _can_ contain a beacon.
    let beacons_on_row = input
        .iter()
        .filter_map(|s| s.beacon_on_row(row))
        .unique()
        .count();

    let definitely_not_beacons = sensed_on_row - beacons_on_row;
    (definitely_not_beacons as u32).into()
}

/// Represents a range of points on a given row of the scan. Includes the start and
/// end points on that row.
#[derive(Debug)]
struct RowRange(isize, isize);

impl RowRange {
    /// Does this range overlap another?
    fn overlaps(&self, other: &Self) -> bool {
        other.1 >= self.0 && self.1 >= other.0
    }

    /// Merge this range with another
    fn merge(&mut self, other: &Self) {
        *self = RowRange(self.0.min(other.0), self.1.max(other.1));
    }

    /// Count the number of positions in this range
    fn count_positions(&self) -> usize {
        self.0.abs_diff(self.1) + 1
    }
}

impl Sensor {
    /// Indicates if the sensor can detect the given Point
    pub fn can_detect(&self, point: &Point) -> bool {
        self.location.distance_to(point) <= self.range
    }

    /// Workhorse of part one. Identifies and returns the range of positions
    /// that can be detected by this sensor on the indicated row, as a RowRange.
    fn row_range_sensed(&self, row: isize) -> Option<RowRange> {
        let distance_to_row = self.location.1.abs_diff(row);
        if distance_to_row > self.range {
            return None;
        }

        // The spread indicates how much of the Manhattan distance for detection
        // is remaining to 'spread' out to the left and right. Essentially half
        // the width of the detection zone on this row.
        let spread = self.range - distance_to_row;
        let range_start = self.location.0.saturating_sub_unsigned(spread);
        let range_end = self.location.0.saturating_add_unsigned(spread);
        Some(RowRange(range_start, range_end))
    }

    /// If the beacon is on the given row, return the location of the beacon.
    /// Otherwise, return None.
    fn beacon_on_row(&self, row: isize) -> Option<Point> {
        if self.beacon.1 == row {
            return Some(self.beacon);
        }
        None
    }
}
