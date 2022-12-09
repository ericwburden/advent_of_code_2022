use std::ops::AddAssign;

/// Represents the position of a knot in x/y space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

/// Check if the current position is "too far" from another position. For knots,
/// the next knot is too far away if it's more than one unit away in either
/// dimension.
impl Position {
    pub fn too_far(&self, other: &Position) -> bool {
        let Position(x1, y1) = self;
        let Position(x2, y2) = other;
        x1.abs_diff(*x2) > 1 || y1.abs_diff(*y2) > 1
    }
}

/// You know what's nice? Adding an offset to a position to 'move' the position!
impl AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        let Position(x, y) = self;
        let (xd, yd) = rhs;
        *self = Position(*x + xd, *y + yd);
    }
}
