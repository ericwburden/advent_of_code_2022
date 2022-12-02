use super::shared::{Outcome, Throw};
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

    /// Attempt to convert an input character into a `Throw`. This time,
    /// we know that 'X', 'Y', and 'Z' do not represent throws, so we don't
    /// try to convert them.
    fn try_into_throw(&self) -> Result<Throw, Self::Error> {
        match self {
            'A' => Ok(Throw::Rock),
            'B' => Ok(Throw::Paper),
            'C' => Ok(Throw::Scissors),
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

    #[rustfmt::skip] // I _still like_ my pretty match statement below
    fn try_into_outcome(&self) -> Result<Outcome, Self::Error> {
        // Now, we only convert the first character into a `Throw`
        let (ch1, result) = self;
        let opponent = ch1.try_into_throw()?;

        // Using the mapping that 'X' means we lose, 'Y' means we draw, and 
        // 'Z' means we win, determine the outcome of the game and what throw
        // you the player need to make to achieve that outcome, and return 
        // the `Outcome`.
        use Throw::*;
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
