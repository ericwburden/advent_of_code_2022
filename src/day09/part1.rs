use crate::day09::{Input, Motion, Output, Knot};
use std::collections::HashSet;
use std::ops::AddAssign;

pub fn solve(input: &Input) -> Output {
    // Brand new `RopeSimulator`(tm)
    let mut simulator = RopeSimulator::new();

    // For each specified motion, update the simulator
    input.iter().for_each(|motion| simulator.move_head(motion));

    // Return the number of unique tail positions from the simulator
    let unique_tail_pos = simulator.hist.len() as u32;
    unique_tail_pos.into()
}

/// A struct to encapsulate the state of the rope, with a HashSet to keep up with
/// the unique positions of the tail.
pub struct RopeSimulator {
    head: Knot,
    tail: Knot,
    hist: HashSet<Knot>,
}

impl RopeSimulator {
    /// Create a new RopeSimulator with head and tail both at the origin (0, 0) and
    /// initializing the set of tail positions to contain the initial tail position.
    fn new() -> Self {
        let head = Knot::default();
        let tail = Knot::default();
        let hist = HashSet::from([Knot::default()]);
        RopeSimulator { head, tail, hist }
    }

    /// Move the tail of the rope towards the head according to the rules given by
    /// the puzzle instructions.
    fn move_tail(&mut self) {
        let Knot(hx, hy) = self.head;
        let Knot(tx, ty) = self.tail;

        // This `use` statement means we don't have to fully quality all the
        // different `Ordering` variants below. Makes it cleaner to look at.
        use std::cmp::Ordering::*;

        // Set the position of the tail based on where the head is relative to it.
        // For example, if the head is positioned diagonally up and to the left of
        // the tail, then we'll match on `(Less, Less)` and move the tail up and
        // to the left.
        self.tail = match (hx.cmp(&tx), hy.cmp(&ty)) {
            (Less, Less) => Knot(tx - 1, ty - 1),
            (Less, Equal) => Knot(tx - 1, ty),
            (Less, Greater) => Knot(tx - 1, ty + 1),
            (Equal, Less) => Knot(tx, ty - 1),
            (Equal, Equal) => unreachable!(),
            (Equal, Greater) => Knot(tx, ty + 1),
            (Greater, Less) => Knot(tx + 1, ty - 1),
            (Greater, Equal) => Knot(tx + 1, ty),
            (Greater, Greater) => Knot(tx + 1, ty + 1),
        };

        // Add the new tail position to the set of tracked tail positions.
        self.hist.insert(self.tail);
    }

    fn move_head(&mut self, motion: &Motion) {
        // Generate a specification for moving the head. We get the number of
        // steps from the `Motion`, and the offset indicates how the `Position`
        // of the head is changed on each step.
        let (steps, offset) = match motion {
            Motion::Up(steps) => (steps, (0, -1)),
            Motion::Down(steps) => (steps, (0, 1)),
            Motion::Left(steps) => (steps, (-1, 0)),
            Motion::Right(steps) => (steps, (1, 0)),
        };

        // Now we move the head one step at a time, adjusting the head by
        // the offset each time. Any time the head is too far from the
        // tail, we adjust the tail and record its new position.
        for _ in 0..*steps {
            self.head += offset;
            if self.head.too_far(&self.tail) {
                self.move_tail();
            }
        }
    }
}
