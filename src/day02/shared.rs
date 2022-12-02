//! Common structs and functions for both parts. `Shape` and `Outcome`
//! are used in both parts and the functionality for converting them into 
//! points is unchanged between the parts.

/// Represents a 'shape' in a game of Rock, Paper, Scissors
#[derive(Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

/// Convert a `Shape` into it's score value
impl From<Shape> for u32 {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

/// Represents the outcome of a game of Rock, Paper, Scissors, from the
/// perspective of you, the player. Each variant encapsulates the shape
/// you made to achieve that outcome.
pub enum Outcome {
    Win(Shape),
    Draw(Shape),
    Lose(Shape),
}

impl Outcome {
    /// Calculate the score from a given outcome
    pub fn score(&self) -> u32 {
        match self {
            // 6 points for winning + the points for your shape
            Outcome::Win(t) => 6 + u32::from(*t),

            // 3 points for a draw + the points for your shape
            Outcome::Draw(t) => 3 + u32::from(*t),

            // 0 points for losing + the points for your shape
            Outcome::Lose(t) => u32::from(*t),
        }
    }
}
