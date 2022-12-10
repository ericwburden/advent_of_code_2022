use crate::day10::{Input, Instruction, Output};

/// Solve Day 10, Part 1
pub fn solve(input: &Input) -> Output {
    // Instantiate a new device, run all the instructions on it, then return
    // the total signal strength from that device.
    let mut device = Device::new();
    input
        .iter()
        .for_each(|instruction| device.execute(instruction));
    device.signal_strength.into()
}

/// Represents our handheld computing device, complete with signal strength
/// accumulator!
struct Device {
    register: i32,
    cycle: usize,
    signal_strength: i32,
}

impl Device {
    fn new() -> Self {
        Device {
            register: 1,
            cycle: 1,
            signal_strength: 0,
        }
    }

    // Execute a NOOP instruction. We'll leverage these instructions to update the
    // signal strength based on the current cycle count.
    fn execute_noop(&mut self) {
        self.cycle += 1;
        let multiple_of_20 = self.cycle % 20 == 0;
        let odd_multiple = (self.cycle / 20) % 2 == 1;

        // We'll update the signal strength on cycles 20, 60, 100, 140, etc.
        // These are all odd multiples of 20, that is, (20 * 1), (20 * 3), (20 * 5), etc.
        // So, we check that the current cycle count is a multiple of 20 and that the
        // current cycle count divided by 20 is an odd number.
        if multiple_of_20 && odd_multiple {
            self.signal_strength += (self.cycle as i32) * self.register;
        }
    }

    /// Execute an ADDX instruction. We leverage the NOOP instructions here to update
    /// the cycle count and the signal strength, if necessary. It's important that
    /// the register be updated between the NOOP instructions, since the puzzle states
    /// signal strength is checked _during_ the cycle and not after.
    fn execute_addx(&mut self, value: i32) {
        self.execute_noop();
        self.register += value;
        self.execute_noop();
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
