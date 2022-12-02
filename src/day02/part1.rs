use super::shared::{Outcome, Throw};
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

/// Trait for converting a character from the input into a `Throw`.
/// I'm using a Trait here so that I can use the same function names
/// in the two different parts but have them behave differently, while 
/// sharing some base functionality between parts.
trait TryIntoThrow {
    type Error;
    fn try_into_throw(&self) -> Result<Throw, Self::Error>;
}

impl TryIntoThrow for char {
    type Error = &'static str;

    /// Attempt to convert an input character into a `Throw`. Yes, we know
    /// that there will be no other characters, but I like to practice good
    /// input hygiene when I can.
    fn try_into_throw(&self) -> Result<Throw, Self::Error> {
        match self {
            'A' | 'X' => Ok(Throw::Rock),
            'B' | 'Y' => Ok(Throw::Paper),
            'C' | 'Z' => Ok(Throw::Scissors),
            _ => Err("Character cannot be converted to `Throw`!"),
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
        // Attempt to convert both characters into their respective `Throw`
        let (ch1, ch2) = self;
        let opponent = ch1.try_into_throw()?;
        let player = ch2.try_into_throw()?;

        // Based on the throws, determine who won and return the `Outcome`
        use Throw::*;
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
