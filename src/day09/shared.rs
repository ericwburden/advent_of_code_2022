use std::ops::AddAssign;

/// Represents the position of a knot in x/y space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Knot(pub i32, pub i32);

/// Check if the current knot is "too far" from another knot. This means
/// it's more than one unit away in either dimension.
impl Knot {
    pub fn too_far(&self, other: &Knot) -> bool {
        let Knot(x1, y1) = self;
        let Knot(x2, y2) = other;
        x1.abs_diff(*x2) > 1 || y1.abs_diff(*y2) > 1
    }
}

/// You know what's nice? Adding an offset to a knot to 'move' the position!
impl AddAssign<(i32, i32)> for Knot {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        let Knot(x, y) = self;
        let (xd, yd) = rhs;
        *self = Knot(*x + xd, *y + yd);
    }
}
