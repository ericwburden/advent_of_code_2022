use crate::day07::{FileSystem, FileSystemObj, Input, Output};

/// Solve Day 7, Part 2 
pub fn solve(input: &Input) -> Output {
    // Calculate the space we need to free up as described by the puzzle
    let space_remaining = 70_000_000 - input.total_size();
    let space_needed = 30_000_000 - space_remaining;

    // Iterate through the directory sizes and find the ones whose size is at least
    // as large as the amount of space we need, and take the smallest size of those.
    input
        .get_directory_sizes()
        .into_iter()
        .filter(|x| *x >= space_needed)
        .min()
        .unwrap()
        .into()
}

impl FileSystem<'_> {
    /// Get the size of the root directory, which is the total size of the files
    /// in our file system.
    fn total_size(&self) -> u32 {
        match &self.0 {
            FileSystemObj::Dir(d) => d.borrow().size,
            FileSystemObj::File(_) => unreachable!(),
        }
    }
}
