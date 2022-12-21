use crate::day20::Input;

const INPUT: &str = include_str!("../../input/20/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .flat_map(|l| l.parse::<i64>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn check_input() {
        let input = read();
        assert_eq!(input.len(), 5000);
        assert_eq!(input[0], -8023);
        assert_eq!(input[4999],  5838);

        let input_set = input.iter().collect::<HashSet<_>>();
        assert_eq!(input.len(), input_set.len());
    }
}
