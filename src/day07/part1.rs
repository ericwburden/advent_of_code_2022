use crate::day07::{FileSystem, FileSystemObj, Input, Output};

/// Solve Day 7, Part 1
pub fn solve(input: &Input) -> Output {
    // From the list of directory sizes, keep only the sizes less than 100_000
    // and return the total of those directory sizes.
    input
        .get_directory_sizes()
        .iter()
        .filter(|x| **x <= 100_000)
        .sum::<u32>()
        .into()
}

impl FileSystem<'_> {
    /// Pull the sizes of each directory in the file system into a vector
    pub fn get_directory_sizes(&self) -> Vec<u32> {
        let mut sizes = Vec::new();

        // Walk the file system tree structure, adding directory sizes to `sizes`
        fn walk(obj: FileSystemObj, sizes: &mut Vec<u32>) {
            let FileSystemObj::Dir(dir) = obj else { return; };
            sizes.push(dir.borrow().size);
            for item in dir.borrow().contents.iter() {
                walk(item.clone(), sizes);
            }
        }

        // Do the walk!
        walk(self.0.clone(), &mut sizes);
        sizes
    }
}
