use crate::day06::{Input, Output, ToBitMarker, SequenceDetector};

pub fn solve(input: &Input) -> Output {
    let mut detector: SequenceDetector<14> = SequenceDetector::new();
    for (idx, ch) in input.chars().enumerate() {
        if detector.detect(ch) { return (idx as u32 + 1).into(); }
    }
    panic!("No start-of-packet marker detected!")
}
