use crate::day08::{Input, Output};
use std::cmp::{max, min};

/// Solve Day 8, Part 2
pub fn solve(input: &Input) -> Output {
    // This time, we'll instantiate a 2D vector the same size and shape as our
    // view of the trees, just filled with zeros!
    let mut scenic_score_map = vec![vec![0u32; input.col_len]; input.row_len];

    // Pre-computing a list of all the row/column indices so we only have
    // one `for` loop below instead of two. Keeps the nesting down.
    let indices = (0..input.row_len).flat_map(|r| (0..input.col_len).map(move |c| (r, c)));

    // For each tree location in the forest...
    for (row_idx, col_idx) in indices {

        // If we're on an edge of the map, just skip it. It'll be zero regardless.
        if row_idx == 0
            || col_idx == 0
            || row_idx == (input.row_len - 1)
            || col_idx == (input.col_len - 1)
        {
            continue;
        }

        // The tree at our current location
        let tree = input.trees[row_idx][col_idx];

        // From our tree, loop up and count the number of trees visible. We do this
        // by iterating over the positions in the same column and in rows above our
        // current tree until we either reach the edge or hit a tree our own height.
        let mut can_see_up = 0;
        for seek_idx in (0..row_idx).rev() {
            let found = input.trees[seek_idx][col_idx];
            can_see_up += 1;
            if found >= tree {
                break;
            }
        }

        // Same deal, just looking down.
        let mut can_see_down = 0;
        for seek_idx in (row_idx + 1)..input.row_len {
            let found = input.trees[seek_idx][col_idx];
            can_see_down += 1;
            if found >= tree {
                break;
            }
        }

        // Same deal, just looking left
        let mut can_see_left = 0;
        for seek_idx in (0..col_idx).rev() {
            let found = input.trees[row_idx][seek_idx];
            can_see_left += 1;
            if found >= tree {
                break;
            }
        }

        // Same deal, just looking right
        let mut can_see_right = 0;
        for seek_idx in (col_idx + 1)..input.col_len {
            let found = input.trees[row_idx][seek_idx];
            can_see_right += 1;
            if found >= tree {
                break;
            }
        }

        // Calculate the scenic score of this tree as the product of all the
        // counts from looking up, down, left, and right
        scenic_score_map[row_idx][col_idx] =
            can_see_up * can_see_down * can_see_left * can_see_right;
    }

    // Now we just loop over the 2D map and return the largest score found
    scenic_score_map
        .iter()
        .flat_map(|row| row.iter())
        .copied()
        .max()
        .unwrap()
        .into()
}
