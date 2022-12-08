use crate::day06::{Input, Output, SequenceDetector};

pub fn solve(input: &Input) -> Output {
    let mut detector: SequenceDetector<4> = SequenceDetector::new();
    for (idx, signal) in input.iter().enumerate() {
        if detector.detect(*signal) {
            return (idx as u32 + 1).into();
        }
    }
    panic!("No start-of-packet marker detected!")
}
