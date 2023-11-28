use super::input::Signal;
use std::ops::{BitAnd, BitOrAssign, BitXorAssign};

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

impl BitXorAssign for Signal {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Signal {
    fn count_components(&self) -> u32 {
        self.0.count_ones()
    }
}

/// Represents a 'detector' for unique sequences of `Signal`s of a constant length.
/// Holds a buffer of the signals encountered so far and inserts new signals into
/// that buffer by wrapping around the length of the buffer.
#[derive(Debug)]
pub struct SequenceDetector<const N: usize> {
    buffer: [Signal; N],
    mark: usize,
    composite_signal: Signal,
}

impl<const N: usize> SequenceDetector<N> {
    /// Create a new `SequenceDetector` with an empty buffer and a marker set to the
    /// first index of the buffer.
    pub fn new() -> Self {
        let buffer = [Signal::default(); N];
        let mark = 0;
        let composite_signal = Signal::default();
        Self { buffer, mark, composite_signal }
    }

    /// Given a `Signal`, indicates whether the most recent `N` signals detected
    /// comprises a unique sequence.
    pub fn detect(&mut self, signal: Signal) -> bool {
        // Remove the oldest signal from the composite signal, add the current
        // signal to the buffer and the composite signal, and bump the marker over
        // by one to receive the next signal.
        self.composite_signal ^= self.buffer[self.mark];

        if self.composite_signal.count_components() < 3 {
            self.composite_signal |= self.buffer[self.mark];
        }
        
        self.composite_signal |= signal;
        self.buffer[self.mark] = signal;
        self.mark = (self.mark + 1) % N;

        // Return an indicator as to whether or not the buffer contains N unique
        // signals, indicated by the number of signals in the composite signal
        self.composite_signal.count_components() == N as u32
    }
}

impl<const N: usize> Default for SequenceDetector<N> {
    fn default() -> Self {
        Self::new()
    }
}
