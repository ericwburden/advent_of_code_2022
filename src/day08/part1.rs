use crate::day08::{Input, Output};

/// Solve Day 8, Part 1
#[allow(clippy::needless_range_loop)]
pub fn solve(input: &Input) -> Output {
    // Start with a two-dimensional vector the same size as the input,
    // filled with `false`. So far, we can't see any trees!
    let mut visibility_map = vec![vec![false; input.col_len]; input.row_len];

    // For each row of trees...
    for row_idx in 0..input.row_len {
        // The tallest tree found so far has a height of 0
        let mut tallest = 0;

        // For each column from left to right...
        for col_idx in 0..input.col_len {
            // Take the tree at that position
            let tree = input.trees[row_idx][col_idx];

            // If it's taller than the current `tallest` OR it's on the left
            // or right edge, then set that tree to visible and update the
            // tallest found so far.
            if tree > tallest || col_idx == 0 || col_idx == (input.col_len - 1) {
                visibility_map[row_idx][col_idx] = true;
                tallest = tree;
            }
        }

        // Reset `tallest` and do the same thing again from right to left. There's
        // no need to attend to the edges again on this round.
        tallest = 0;
        for col_idx in 0..input.col_len {
            let tree = input.trees[row_idx][col_idx];
            if tree > tallest {
                visibility_map[row_idx][col_idx] = true;
                tallest = tree;
            }
        }
    }

    // Now scan the columns from top to bottom and bottom to top
    for col_idx in 0..input.col_len {
        // Same deal, except this time we'll also trigger the visibility update
        // on the top and bottom rows, as well.
        let mut tallest = 0;
        for row_idx in 0..input.row_len {
            let tree = input.trees[row_idx][col_idx];
            if tree > tallest || row_idx == 0 || row_idx == (input.row_len - 1) {
                visibility_map[row_idx][col_idx] = true;
                tallest = tree;
            }
        }

        // If you guess "reset the tallest height and scan from bottom to top",
        // the you guessed right!
        tallest = 0;
        for row_idx in (0..input.row_len).rev() {
            let tree = input.trees[row_idx][col_idx];
            if tree > tallest {
                visibility_map[row_idx][col_idx] = true;
                tallest = tree;
            }
        }
    }

    // Count the number of visible trees in the visibility map and return the count
    let found = visibility_map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|x| **x)
        .count() as u32;
    found.into()
}
