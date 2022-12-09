use crate::day09::{Input, Motion, Output, Position};
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::AddAssign;

pub fn solve(input: &Input) -> Output {
    // Brand new `RopeSimulator`(tm)
    let mut simulator: RopeSimulator<10> = RopeSimulator::new();

    // For each specified motion, update the simulator
    input.iter().for_each(|motion| simulator.move_head(motion));

    // Return the number of unique tail positions from the simulator
    let unique_tail_pos = simulator.hist.len() as u32;
    unique_tail_pos.into()
}

// New and improved! I realize we only _needed_ to support a rope with 10
// knots, but at this point, why not make it *const generic*?
pub struct RopeSimulator<const N: usize> {
    knots: [Position; N],
    hist: HashSet<Position>,
}

impl<const N: usize> RopeSimulator<N> {
    // New RopeSimulator, this time we're keeping all the knots in an array
    // of length N (10 for our case). Note that the order of these knots
    // matters, since `knots[0]` will be the head and `knots[N - 1]` will be
    // the tail.
    fn new() -> Self {
        let knots = [Position::default(); N];
        let hist = HashSet::from([Position::default()]);
        RopeSimulator { knots, hist }
    }

    // This time, instead of hard-coding the head and the tail, we pass in
    // the index of the `leader` knot and the `follower` knot. For our
    // implementation, follower == leader + 1;
    fn follow(&mut self, leader: usize, follower: usize) {
        let Position(hx, hy) = self.knots[leader];
        let Position(tx, ty) = self.knots[follower];

        // The logic here is exactly the same as for the first part
        use std::cmp::Ordering::*;
        self.knots[follower] = match (hx.cmp(&tx), hy.cmp(&ty)) {
            (Less, Less) => Position(tx - 1, ty - 1),
            (Less, Equal) => Position(tx - 1, ty),
            (Less, Greater) => Position(tx - 1, ty + 1),
            (Equal, Less) => Position(tx, ty - 1),
            (Equal, Equal) => unreachable!(),
            (Equal, Greater) => Position(tx, ty + 1),
            (Greater, Less) => Position(tx + 1, ty - 1),
            (Greater, Equal) => Position(tx + 1, ty),
            (Greater, Greater) => Position(tx + 1, ty + 1),
        };

        // Now we need to check to be sure the `follower` is in the tail
        // slot before we record its `Position`.
        if follower == N - 1 {
            self.hist.insert(self.knots[N - 1]);
        }
    }

    fn move_head(&mut self, motion: &Motion) {
        // Generate a specification for moving the head. We get the number of
        // steps from the `Motion`, and the offset indicates how the `Position`
        // of the head is changed on each step.
        let (reps, offset) = match motion {
            Motion::Up(reps) => (reps, (0, -1)),
            Motion::Down(reps) => (reps, (0, 1)),
            Motion::Left(reps) => (reps, (-1, 0)),
            Motion::Right(reps) => (reps, (1, 0)),
        };

        // For each step in the motion, move the first knot in the `knots` array
        // (that's the head), then move down the array of knots, updating each
        // knot in sequence based on the position of the previous knot.
        for _ in 0..*reps {
            self.knots[0] += offset;

            // Note the `tuple_windows()` method from the `itertools` crate. Handy
            // crate, that `itertools`.
            for (leader, follower) in (0..N).tuple_windows() {
                // If the first knot is too far away from the knot behind it, move
                // the follower.
                if self.knots[leader].too_far(&self.knots[follower]) {
                    self.follow(leader, follower);
                }
            }
        }
    }
}
