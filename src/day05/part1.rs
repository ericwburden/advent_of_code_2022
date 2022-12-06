use crate::day05::{CrateStack, CrateStacks, Input, Instruction, Output};

/// Solve Day 05, Part 1
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

/// Implement methods for popping crates from the top of a stack and
/// pushing new crates to the top of a stack.
impl CrateStack {
    /// To push a new crate to the `CrateStack`, just add it to the index past the
    /// last crate (just _happens_ to be `height`) and bump up the height.
    pub fn push(&mut self, ch: char) {
        self.crates[self.height] = ch;
        self.height += 1;
    }

    /// To pop a crate from the `CrateStack`, bump the height down and return the
    /// character from the top of the stack. Note, this doesn't actually _remove_
    /// the character from the top of the stack, but the next crate pushed to
    /// this stack will overwrite it. So long as we stick to pushing and popping,
    /// we won't have any issues.
    pub fn pop(&mut self) -> char {
        if self.height == 0 {
            return '.';
        }
        self.height -= 1;
        self.crates[self.height]
    }
}

impl CrateStacks {
    /// Convenience method to pop from one stack out of all the stacks. Which
    /// stack to pop from is given by `crate_idx` as indicated by an `Instruction`.
    pub fn pop_from(&mut self, crate_idx: u8) -> char {
        if !(1..=9).contains(&crate_idx) {
            return '.';
        }
        self[(crate_idx as usize)].borrow_mut().pop()
    }

    /// Convenience method to push to one stack out of all the stacks. Which
    /// stack to push to is given by `crate_idx` as indicated by an `Instruction`.
    pub fn push_to(&mut self, crate_idx: u8, ch: char) {
        if !(1..=9).contains(&crate_idx) {
            return;
        }
        self[(crate_idx as usize)].borrow_mut().push(ch);
    }

    /// Fetches the top character (crate) from each stack and builds a String
    /// out of them.
    pub fn message(&mut self) -> String {
        let mut out = String::new();
        for stack in self.0.iter_mut() {
            let top_crate = stack.borrow_mut().pop();
            out.push(top_crate);
        }
        out
    }
}

/// This is the trait for executing instructions on a `CrateStacks`. Using a trait
/// here so to allow for different functionality between parts one and two.
trait Execute {
    fn execute(&mut self, _: &Instruction);
}

impl Execute for CrateStacks {
    /// Really just boils down to pop crates from one stack and push them onto
    /// another, as many times as the `Instruction` says to.
    fn execute(&mut self, instruction: &Instruction) {
        let Instruction {
            count,
            origin,
            destination,
        } = instruction;
        for _ in 0..*count {
            let top_crate = self.pop_from(*origin);
            self.push_to(*destination, top_crate);
        }
    }
}
