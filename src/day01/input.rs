use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

/// Read and parse the input file
pub fn read() -> Input {
    // Iterate over each empty-line separated "chunk",
    // parsing each chunk into a total calorie count
    // per Elf, returning the list of total calories per
    // Elf.
    INPUT
        .trim()
        .split("\n\n")
        .map(try_parse_elf_calories)
        .collect::<Result<_, _>>()
        .expect("Could not parse input!")
}

/// Parse a "chunk" of lines representing an individual
/// Elf's snacks into the total calories for that Elf.
fn try_parse_elf_calories(value: &str) -> Result<u32, std::num::ParseIntError> {
    // Iterate over each line, convert it to a u32, and sum the results
    let mut total = 0;
    for line in value.lines() {
        total += line.parse::<u32>()?;
    }
    Ok(total)
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
