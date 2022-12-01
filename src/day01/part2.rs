use crate::day01::{Input, Output};

/// Solve Day 01, Part 02
pub fn solve(input: &Input) -> Output {
    // A running total of the top three values for total
    // Elf calories found so far, descending from left to
    // right.
    let mut top_three = [u32::MIN; 3];

    // For each Elf's total snack calories...
    for calories in input.iter() {
        // Need a local (mutable) copy of the calories value
        let mut calories = *calories;

        // For each value in the current top three...
        for top_value in top_three.iter_mut() {
            // If the current Elf's total calories are greater than
            // the current top value, then swap the values. `calories`
            // now contains the previous `top_value` and the previous
            // value for `calories` is in `top_three`. Repeat for all
            // values in `top_three` until `top_three` contains the
            // largest three values of `top_three` and `calories`.
            if calories > *top_value {
                std::mem::swap(top_value, &mut calories);
            }
        }
    }

    // Return the sum of the top three calorie counts
    top_three.iter().sum::<u32>().into()
}
