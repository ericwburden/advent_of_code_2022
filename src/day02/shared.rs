//! Common structs and functions for both parts. `Throw` and `Outcome`
//! are used in both parts and the functionality for converting them into 
//! points is unchanged between the parts.

/// Represents a 'throw' in a game of Rock, Paper, Scissors
#[derive(Clone, Copy)]
pub enum Throw {
    Rock,
    Paper,
    Scissors,
}

/// Convert a `Throw` into it's score value
impl From<Throw> for u32 {
    fn from(throw: Throw) -> Self {
        match throw {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }
}

/// Represents the outcome of a game of Rock, Paper, Scissors, from the
/// perspective of you, the player. Each variant encapsulates the throw
/// you made to achieve that outcome.
pub enum Outcome {
    Win(Throw),
    Draw(Throw),
    Lose(Throw),
}

impl Outcome {
    /// Calculate the score from a given outcome
    pub fn score(&self) -> u32 {
        match self {
            // 6 points for winning + the points for your throw
            Outcome::Win(t) => 6 + u32::from(*t),

            // 3 points for a draw + the points for your throw
            Outcome::Draw(t) => 3 + u32::from(*t),

            // 0 points for losing + the points for your throw
            Outcome::Lose(t) => u32::from(*t),
        }
    }
}
