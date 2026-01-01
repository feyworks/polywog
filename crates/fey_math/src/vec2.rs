use crate::{Float, Num, Signed, Vec3, impl_vec, vec3};
use dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use std::fmt::{Display, Formatter};

pub type Vec2F = Vec2<f32>;
pub type Vec2I = Vec2<i32>;
pub type Vec2U = Vec2<u32>;

/// A 2-dimensional vector.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// Create a [`Vec2`].
#[inline]
pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

impl_vec!(
    NAME = Vec2
    SHORT = vec2
    LEN = 2
    FIELDS = (x, y)
    TUPLE = (T, T)
);

impl<T> Vec2<T> {
    /// Swizzle components.
    #[inline]
    pub fn yx(self) -> Self {
        vec2(self.y, self.x)
    }

    /// Return the vector with the x-value replaced.
    #[inline]
    pub fn with_x(self, x: T) -> Self {
        Self { x, ..self }
    }

    /// Return the vector with the y-value replaced.
    #[inline]
    pub fn with_y(self, y: T) -> Self {
        Self { y, ..self }
    }

    /// Add a third dimension to this vector.
    #[inline]
    pub fn with_z(self, z: T) -> Vec3<T> {
        vec3(self.x, self.y, z)
    }
}

impl<T: Num> Vec2<T> {
    /// A normalized vector pointing right, equal to `(1, 0)`.
    pub const RIGHT: Self = vec2(T::ONE, T::ZERO);

    /// A normalized vector pointing down, equal to `(0, 1)`.
    pub const DOWN: Self = vec2(T::ZERO, T::ONE);

    /// A normalized vector pointing south, equal to `(0, 1)`.
    pub const SOUTH: Self = vec2(T::ZERO, T::ONE);

    /// A normalized vector pointing east, equal to `(1, 0)`.
    pub const EAST: Self = vec2(T::ONE, T::ZERO);

    /// A unit vector representing the x-axis, equal to `(1, 0)`.
    pub const X_AXIS: Self = vec2(T::ONE, T::ZERO);

    /// A unit vector representing the y-axis, equal to `(0, 1)`.
    pub const Y_AXIS: Self = vec2(T::ZERO, T::ONE);

    /// Returns the cross-product of this vector and another,
    /// equal to `(x1×y2-y1×x2)`.
    #[inline]
    pub fn cross(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }

    /// Given the triangle `(a, b, c)`, and the interpolation values
    /// `ab` and `bc`, returns a barycentric coordinate.
    #[inline]
    pub fn barycentric(a: Self, b: Self, c: Self, t1: T, t2: T) -> Self {
        #[inline]
        fn coord<T: Num>(a: T, b: T, c: T, ab: T, bc: T) -> T {
            a + (b - a) * ab + (c - a) * bc
        }
        vec2(coord(a.x, b.x, c.x, t1, t2), coord(a.y, b.y, c.y, t1, t2))
    }
}

impl<T: Signed> Vec2<T> {
    pub const CARDINAL_DIRS: [Self; 4] = [Self::EAST, Self::SOUTH, Self::WEST, Self::NORTH];

    /// A normalized vector pointing left, equal to `(-1, 0)`.
    pub const LEFT: Self = vec2(T::NEG_ONE, T::ZERO);

    /// A normalized vector pointing up, equal to `(0, -1)`.
    pub const UP: Self = vec2(T::ZERO, T::NEG_ONE);

    /// A normalized vector pointing west, equal to `(-1, 0)`.
    pub const WEST: Self = vec2(T::NEG_ONE, T::ZERO);

    /// A normalized vector pointing north, equal to `(0, -1)`.
    pub const NORTH: Self = vec2(T::ZERO, T::NEG_ONE);

    /// Rotates the vector 90° clockwise and returns the result.
    #[inline]
    pub fn turn_right(self) -> Self {
        vec2(-self.y, self.x)
    }

    /// Rotates the vector 90° counter-clockwise and returns the result.
    #[inline]
    pub fn turn_left(self) -> Self {
        vec2(self.y, -self.x)
    }
}

impl<T: Float> Vec2<T> {
    pub const OCTAL_DIRS: [Self; 8] = [
        Self::EAST,
        Self::SOUTH_EAST,
        Self::SOUTH,
        Self::SOUTH_WEST,
        Self::WEST,
        Self::NORTH_WEST,
        Self::NORTH,
        Self::NORTH_EAST,
    ];

    /// A normalized vector pointing south-east, equal to `(1/√2, 1/√2)`.
    pub const SOUTH_EAST: Self = vec2(T::ONE_OVER_SQRT_2, T::ONE_OVER_SQRT_2);

    /// A normalized vector pointing south-east, equal to `(-1/√2, 1/√2)`.
    pub const SOUTH_WEST: Self = vec2(T::NEG_ONE_OVER_SQRT_2, T::ONE_OVER_SQRT_2);

    /// A normalized vector pointing south-east, equal to `(-1/√2, -1/√2)`.
    pub const NORTH_WEST: Self = vec2(T::NEG_ONE_OVER_SQRT_2, T::NEG_ONE_OVER_SQRT_2);

    /// A normalized vector pointing south-east, equal to `(1/√2, -1/√2)`.
    pub const NORTH_EAST: Self = vec2(T::ONE_OVER_SQRT_2, T::NEG_ONE_OVER_SQRT_2);
}

impl<T: Display> Display for Vec2<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)
    }
}

impl<T> From<PhysicalPosition<T>> for Vec2<T> {
    #[inline]
    fn from(value: PhysicalPosition<T>) -> Self {
        vec2(value.x, value.y)
    }
}

impl<T> From<PhysicalSize<T>> for Vec2<T> {
    #[inline]
    fn from(value: PhysicalSize<T>) -> Self {
        vec2(value.width, value.height)
    }
}

impl<T> From<LogicalPosition<T>> for Vec2<T> {
    #[inline]
    fn from(value: LogicalPosition<T>) -> Self {
        vec2(value.x, value.y)
    }
}

impl<T> From<LogicalSize<T>> for Vec2<T> {
    #[inline]
    fn from(value: LogicalSize<T>) -> Self {
        vec2(value.width, value.height)
    }
}
