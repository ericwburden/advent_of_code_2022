use std::ops::{BitAnd, BitOrAssign};
use super::input::Signal;

/// Implement bitwise _and_ for `Signal`s
impl BitAnd for Signal {
    type Output = Signal;

    fn bitand(self, rhs: Self) -> Self::Output {
        Signal(self.0 & rhs.0)
    }
}

/// Implement bitwise _or-assign_ for `Signal`s
impl BitOrAssign for Signal {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Represents a 'detector' for unique sequences of `Signal`s of a constant length. 
/// Holds a buffer of the signals encountered so far and inserts new signals into
/// that buffer by wrapping around the length of the buffer.
#[derive(Debug)]
pub struct SequenceDetector<const N: usize> {
    buffer: [Signal; N],
    mark: usize,
}

impl<const N: usize> SequenceDetector<N> {
    /// Create a new `SequenceDetector` with an empty buffer and a marker set to the
    /// first index of the buffer.
    pub fn new() -> Self {
        let buffer = [Signal::default(); N];
        let mark = 0;
        Self { buffer, mark }
    }

    /// Given a `Signal`, indicates whether the most recent `N` signals detected
    /// comprises a unique sequence.
    pub fn detect(&mut self, signal: Signal) -> bool {
        // Add the signal to the buffer and bump the marker over by one to 
        // receive the next signal.
        self.buffer[self.mark] = signal;
        self.mark = (self.mark + 1) % N;

        // If the marker points to an empty signal in the buffer, this means
        // the buffer isn't full yet and we definitely haven't found `N` unique
        // signal inputs.
        if self.buffer[self.mark] == Signal(0) { return false; }

        // Check the buffer for unique signals. If any duplicate `Signal`s are
        // detected, return false early. If all `Signal`s _are_ unique, return true.
        let mut bits = Signal(0);
        for bit in self.buffer.iter() {
            if *bit & bits > Signal(0) { return false; }
            bits |= *bit;
        }
        true
    }
}

impl<const N: usize> Default for SequenceDetector<N> {
    fn default() -> Self {
        Self::new()
    }
}
