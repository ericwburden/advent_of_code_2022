use crate::day08::Input;

/// Represents our overall view of the trees. Really just a two-dimensional vector
/// of the input characters, converted to numbers, with the number of rows and
/// columns brought along for the ride.
#[derive(Debug)]
pub struct TreeView {
    pub row_len: usize,
    pub col_len: usize,
    pub trees: Vec<Vec<u8>>,
}

/// Parser a TreeView from the input file. No parser combinators today, since they're
/// almost entirely unnecessary for this one.
impl From<&str> for TreeView {
    fn from(input: &str) -> Self {
        let row_len = input.lines().count();
        let col_len = input.lines().next().unwrap().len();
        let line_to_nums = |s: &str| s.chars().map(|c| c as u8 - b'0').collect();
        let trees: Vec<Vec<_>> = input.lines().map(line_to_nums).collect();

        TreeView {
            row_len,
            col_len,
            trees,
        }
    }
}

const INPUT: &str = include_str!("../../input/08/input.txt");

/// Read the input file and convert it to a `TreeView`
pub fn read() -> Input {
    TreeView::from(INPUT)
}
