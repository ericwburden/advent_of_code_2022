use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    INPUT
        .split('\n')
        .filter_map(|l| l.parse::<u32>().ok())
        .collect()
}
