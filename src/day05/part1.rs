use crate::day05::{Input, Output, CrateStacks, Instruction};

pub fn solve(input: &Input) -> Output {
    let (crate_stacks, instructions) = input;
    let mut crate_stacks = crate_stacks.clone();
    instructions.iter().for_each(|i| crate_stacks.execute(i));
    crate_stacks.message().into()
}

trait Execute {
    fn execute(&mut self, _: &Instruction);
}

impl Execute for CrateStacks {
    fn execute(&mut self, instruction: &Instruction) {
        let Instruction { count, origin, destination } = instruction;
        for _ in 0..*count {
            if let Some(ch) = self[(*origin - 1) as usize].pop() {
                self[(*destination - 1) as usize].push(ch);
            }
        }
    }
}

impl CrateStacks {
    pub fn message(&mut self) -> String {
        let mut out = String::new();
        for stack in self.0.iter_mut() {
            if let Some(ch) = stack.pop() {
                out.push(ch);
            }
        }
        out
    }
}
