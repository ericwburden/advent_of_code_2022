pub trait ToBitMarker {
    fn to_bit_marker(&self) -> u32;
}

impl ToBitMarker for char {
    fn to_bit_marker(&self) -> u32 {
        if !self.is_ascii_lowercase() {
            return 0;
        }
        let shift = (*self as u32) - 97;
        1 << shift
    }
}

#[derive(Debug)]
pub struct SequenceDetector<const N: usize> {
    buffer: [char; N],
    mark: usize,
}

impl<const N: usize> SequenceDetector<N> {
    pub fn new() -> Self {
        let buffer = ['\0'; N];
        let mark = 0;
        Self { buffer, mark }
    }

    pub fn detect(&mut self, ch: char) -> bool {
        self.buffer[self.mark] = ch;
        self.mark = (self.mark + 1) % N;

        if self.buffer[self.mark] == '\0' { return false; }

        let mut bits = 0;
        for bit in self.buffer.iter().map(|c| c.to_bit_marker()) {
            if bit & bits > 0 { return false; }
            bits |= bit;
        }
        true
    }
}

impl<const N: usize> Default for SequenceDetector<N> {
    fn default() -> Self {
        Self::new()
    }
}
