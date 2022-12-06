use std::ops::RangeFrom;

use crate::day05::{CrateStack, CrateStacks, Input, Instruction, Output};

/// Solve Day 05, Part 2
pub fn solve(input: &Input) -> Output {
    // Split up the input into the stacks of crates and the instructions,
    // then clone the crate stacks struct so we can have a mutable copy.
    let (crate_stacks, instructions) = input;
    let mut crate_stacks = crate_stacks.clone();

    // Execute each instruction on the `CrateStacks`, and return the
    // String resulting from the top crate in each stack
    instructions.iter().for_each(|i| crate_stacks.execute(i));
    crate_stacks.message().into()
}

impl CrateStack {
    /// Move multiple crates from one stack to another without changing their order.
    /// We do this by setting the new height of the stack to remove crates from, and
    /// taking crates from that stack starting with that new height and moving them
    /// to the other stack in a "bottoms-up" approach.
    fn transfer_many(&mut self, other: &mut Self, n: u8) {
        self.height -= n as usize;
        for offset in 0..(n as usize) {
            let mut crate_to_move = &mut self.crates[self.height + offset];
            let mut crate_destination = &mut other.crates[other.height + offset];
            std::mem::swap(crate_to_move, crate_destination);
        }
        other.height += n as usize;
    }
}

impl CrateStacks {
    /// Convenience function to specify transferring many crates from one stack
    /// to another. Essentially takes all the parameters from an `Instruction`
    /// to do it.
    pub fn transfer_many_between(&mut self, origin: u8, destination: u8, n: u8) {
        let mut origin_stack = self[origin as usize].borrow_mut();
        let mut destination_stack = self[destination as usize].borrow_mut();
        origin_stack.transfer_many(&mut destination_stack, n);
    }
}

/// This is the trait for executing instructions on a `CrateStacks`. Using a trait
/// here so to allow for different functionality between parts one and two.
trait Execute {
    fn execute(&mut self, _: &Instruction);
}

impl Execute for CrateStacks {
    /// Not much happening in this function other than unpacking the `Instruction`
    /// and passing its parameters to `CrateStacks.transfer_many_between()`.
    fn execute(&mut self, instruction: &Instruction) {
        let Instruction {
            count,
            origin,
            destination,
        } = instruction;
        self.transfer_many_between(*origin, *destination, *count);
    }
}
