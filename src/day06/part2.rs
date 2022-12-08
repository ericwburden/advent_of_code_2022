use crate::day06::{Input, Output, SequenceDetector};

/// Solve Day 6, Part 2
pub fn solve(input: &Input) -> Output {
    // Instantiate a detector for sequences of length 14
    let mut detector: SequenceDetector<14> = SequenceDetector::new();

    // Pass each `Signal` in the input to the detector. Return early
    // with the index (plus one) if a unique sequence is detected.
    for (idx, signal) in input.iter().enumerate() {
        if detector.detect(*signal) {
            return (idx as u32 + 1).into();
        }
    }
    panic!("No start-of-message marker detected!")
}
