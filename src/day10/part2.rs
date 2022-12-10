use crate::day10::{Input, Instruction, Output};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Solve Day 10, Part 2
pub fn solve(input: &Input) -> Output {
    // Boot up a new model of device and run all the instructions on it.
    let mut device = Device::new();
    input
        .iter()
        .for_each(|instruction| device.execute(instruction));

    // Collect the prettified pixel display into a string and return it
    PrettyPixels(device.pixels).to_string().into()
}

/// Represents a new-fangled computer with a display. We'll keep track of pixels
/// lit in a boolean array and worry about displaying them later.
struct Device {
    register: i32,
    cycle: usize,
    pixels: [bool; 240],
}

impl Device {
    fn new() -> Self {
        Device {
            register: 1,
            cycle: 0,
            pixels: [false; 240],
        }
    }

    // Execute a NOOP instruction. We'll leverage these instructions to update the
    // pixels based on the current sprite position.
    fn execute_noop(&mut self) {
        // Calculate the three-wide position of the sprite.
        let sprite_range = (self.register - 1)..=(self.register + 1);

        // The current line position is the cycle wrapped to 40-width lines. So, on
        // cycle 40, we've wrapped around to the first pixel of the second line.
        let line_pos = (self.cycle % 40) as i32;

        // If the current line position is within the sprite, set the corresponding
        // pixel in our array of pixels to `true`.
        if sprite_range.contains(&line_pos) {
            self.pixels[self.cycle] = true;
        }

        self.cycle += 1;
    }

    /// Execute an ADDX instruction. Once again, we leverage the NOOP instructions here
    /// to update the cycle count and the pixels. This time, we need to update the
    /// register _after_ both NOOPs, since pixel drawing happens at the _beginning_
    /// of each cycle.
    fn execute_addx(&mut self, value: i32) {
        self.execute_noop();
        self.execute_noop();
        self.register += value;
    }

    /// Dispatch for instruction execution. Calls the appropriate execute method
    /// depending on the instruction provided.
    fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Noop => self.execute_noop(),
            Instruction::Addx(v) => self.execute_addx(*v),
        }
    }
}

/// This is how we worry about displaying pixels. Using the newtype syntax, we can
/// implement a standard library trait on a (wrapped) standard library data structure.
/// In this case, we want to implement `Display` for our array of pixels so we can
/// convert it to a string that can show us the answer to part two.
struct PrettyPixels([bool; 240]);

#[rustfmt::skip]
impl Display for PrettyPixels {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (idx, pixel) in self.0.iter().enumerate() {
            // Wrap the pixel lines to a width of 40 characters
            if (idx % 40 == 0) && idx > 0 { writeln!(f)?; }

            // If the pixel is lit, print a '#', other wise print a space
            let glyph = if *pixel { "#" } else { " " };
            write!(f, "{glyph}")?;
        }

        write!(f, "") // Finish the print results
    }
}
