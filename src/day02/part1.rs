use super::shared::{Outcome, Shape};
use crate::day02::{Input, Output};

/// Solve part one
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

    /// Attempt to convert an input character into a `Shape`. Yes, we know
    /// that there will be no other characters, but I like to practice good
    /// input hygiene when I can.
    fn try_into_shape(&self) -> Result<Shape, Self::Error> {
        match self {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
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

    #[rustfmt::skip] // I _like_ my pretty match statement below
    fn try_into_outcome(&self) -> Result<Outcome, Self::Error> {
        // Attempt to convert both characters into their respective `Shape`
        let (ch1, ch2) = self;
        let opponent = ch1.try_into_shape()?;
        let player = ch2.try_into_shape()?;

        // Based on the shapes, determine who won and return the `Outcome`
        use Shape::*;
        let result = match (opponent, player) {
            (Rock,     Rock)     => Outcome::Draw(player),
            (Rock,     Paper)    => Outcome::Win(player),
            (Rock,     Scissors) => Outcome::Lose(player),
            (Paper,    Rock)     => Outcome::Lose(player),
            (Paper,    Paper)    => Outcome::Draw(player),
            (Paper,    Scissors) => Outcome::Win(player),
            (Scissors, Rock)     => Outcome::Win(player),
            (Scissors, Paper)    => Outcome::Lose(player),
            (Scissors, Scissors) => Outcome::Draw(player),
        };
        Ok(result)
    }
}
