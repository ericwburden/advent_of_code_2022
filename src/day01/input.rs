use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

/// Read and parse the input file
pub fn read() -> Input {
    // Iterate over each empty-line separated "chunk",
    // parsing each chunk into a total calorie count
    // per Elf, returning the list of total calories per
    // Elf.
    INPUT.trim().split("\n\n").map(parse_elf_calories).collect()
}

/// Parse a "chunk" of lines representing an individual
/// Elf's snacks into the total calories for that Elf.
fn parse_elf_calories(value: &str) -> u32 {
    // Iterate over each line, convert it to a u32 (ignoring any
    // that fail to parse), and summing the results.
    value.lines().flat_map(|l| l.parse::<u32>()).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        let first = *input.first().unwrap();
        assert_eq!(first, 44656);

        let last = *input.last().unwrap();
        assert_eq!(last, 48165);

        assert_eq!(input.len(), 264);
    }
}
