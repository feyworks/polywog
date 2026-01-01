use crate::{Cardinal, Degrees, Octal, Radians, Rotations, Vec2};

/// A type that represents a direction in 2D space.
pub trait Direction<T>: Sized {
    /// Angle of the direction in degrees.
    fn to_degrees(self) -> Degrees<T>;

    /// Angle of the direction in radians.
    fn to_radians(self) -> Radians<T>;

    /// Angle of the direction in rotations.
    fn to_rotations(self) -> Rotations<T>;

    /// Get an approximately equivalent [`Cardinal`] direction.
    fn to_cardinal(self, bias: Cardinal) -> Cardinal;

    /// Get an approximately equivalent [`Octal`] direction.
    fn to_octal(self) -> Octal;

    /// The direction from the direction of a vector.
    fn from_vec2(v: Vec2<T>) -> Self;

    /// The `(sin, cos)` pair for this direction.
    fn sin_cos(self) -> (T, T);

    /// Normal representation of this direction.
    #[inline]
    fn norm(self) -> Vec2<T> {
        let (y, x) = self.sin_cos();
        Vec2 { x, y }
    }
}
