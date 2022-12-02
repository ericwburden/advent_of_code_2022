use super::shared::{Outcome, Shape};
use crate::day02::{Input, Output};

/// Solve part two
pub fn solve(input: &Input) -> Output {
    // For each pair of characters in the input, convert each to an `Outcome`,
    // calculate the score of that `Outcome`, and return the total as an `Output`.
    input
        .iter()
        .flat_map(|pair| pair.try_into_outcome())
        .map(|outcome| outcome.score())
        .sum::<u32>()
        .into()
}

/// Trait for converting a character from the input into a `Shape`.
/// I'm using a Trait here so that I can use the same function names
/// in the two different parts but have them behave differently, while 
/// sharing some base functionality between parts.
trait TryIntoShape {
    type Error;
    fn try_into_shape(&self) -> Result<Shape, Self::Error>;
}

impl TryIntoShape for char {
    type Error = &'static str;

    /// Attempt to convert an input character into a `Shape`. This time,
    /// we know that 'X', 'Y', and 'Z' do not represent shapes, so we don't
    /// try to convert them.
    fn try_into_shape(&self) -> Result<Shape, Self::Error> {
        match self {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err("Character cannot be converted to `Shape`!"),
        }
    }
}

/// Trait for converting a pair of characters from the input into an `Outcome`.
/// Same deal as the other trait above.
trait TryIntoOutcome {
    type Error;
    fn try_into_outcome(&self) -> Result<Outcome, Self::Error>;
}

impl TryIntoOutcome for (char, char) {
    type Error = &'static str;

    #[rustfmt::skip] // I _still like_ my pretty match statement below
    fn try_into_outcome(&self) -> Result<Outcome, Self::Error> {
        // Now, we only convert the first character into a `Shape`
        let (ch1, result) = self;
        let opponent = ch1.try_into_shape()?;

        // Using the mapping that 'X' means we lose, 'Y' means we draw, and 
        // 'Z' means we win, determine the outcome of the game and what shape
        // you the player need to make to achieve that outcome, and return 
        // the `Outcome`.
        use Shape::*;
        match (opponent, result) {
            (Rock,     'Y') => Ok(Outcome::Draw(Rock)),
            (Rock,     'Z') => Ok(Outcome::Win(Paper)),
            (Rock,     'X') => Ok(Outcome::Lose(Scissors)),
            (Paper,    'X') => Ok(Outcome::Lose(Rock)),
            (Paper,    'Y') => Ok(Outcome::Draw(Paper)),
            (Paper,    'Z') => Ok(Outcome::Win(Scissors)),
            (Scissors, 'Z') => Ok(Outcome::Win(Rock)),
            (Scissors, 'X') => Ok(Outcome::Lose(Paper)),
            (Scissors, 'Y') => Ok(Outcome::Draw(Scissors)),
            (_, _) => Err("Cannot convert character pair to an Outcome!"),
        }
    }
}
