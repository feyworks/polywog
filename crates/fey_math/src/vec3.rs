use crate::{Num, Signed, Vec2, Vec4, impl_vec, vec2, vec4};
use std::fmt::{Display, Formatter};

pub type Vec3F = Vec3<f32>;
pub type Vec3I = Vec3<i32>;
pub type Vec3U = Vec3<u32>;

/// A 3-dimensional vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Create a [`Vec3`].
#[inline]
pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

impl_vec!(
    NAME = Vec3
    SHORT = vec3
    LEN = 3
    FIELDS = (x, y, z)
    TUPLE = (T, T, T)
);

impl<T> Vec3<T> {
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

    /// Return the vector with the y-value replaced.
    #[inline]
    pub fn with_z(self, z: T) -> Self {
        Self { z, ..self }
    }

    /// Add a fourth dimension to this vector.
    #[inline]
    pub fn with_w(self, w: T) -> Vec4<T> {
        vec4(self.x, self.y, self.z, w)
    }

    /// Swizzle components.
    #[inline]
    pub fn xzy(self) -> Self {
        vec3(self.x, self.z, self.y)
    }

    /// Swizzle components.
    #[inline]
    pub fn yxz(self) -> Self {
        vec3(self.y, self.x, self.z)
    }

    /// Swizzle components.
    #[inline]
    pub fn zxy(self) -> Self {
        vec3(self.z, self.x, self.y)
    }

    /// Swizzle components.
    #[inline]
    pub fn yzx(self) -> Self {
        vec3(self.y, self.z, self.x)
    }

    /// Swizzle components.
    #[inline]
    pub fn zyx(self) -> Self {
        vec3(self.z, self.y, self.x)
    }
}

impl<T: Copy> Vec3<T> {
    /// Reduce to a 2D vector by dropping `z`.
    #[inline]
    pub fn xy(&self) -> Vec2<T> {
        vec2(self.x, self.y)
    }
}

impl<T: Num> Vec3<T> {
    /// A unit vector representing the x-axis, equal to `(1, 0, 0)`.
    pub const X_AXIS: Self = vec3(T::ONE, T::ZERO, T::ZERO);

    /// A unit vector representing the y-axis, equal to `(0, 1, 0)`.
    pub const Y_AXIS: Self = vec3(T::ZERO, T::ONE, T::ZERO);

    /// A unit vector representing the z-axis, equal to `(0, 0, 1)`.
    pub const Z_AXIS: Self = vec3(T::ZERO, T::ZERO, T::ONE);

    /// A normalized vector pointing right, equal to `(1, 0, 0)`.
    pub const RIGHT: Self = vec3(T::ONE, T::ZERO, T::ZERO);

    /// A normalized vector pointing up, equal to `(0, 1, 0)`.
    pub const UP: Self = vec3(T::ZERO, T::ONE, T::ZERO);

    /// A normalized vector pointing forward, equal to `(0, 0, 1)`.
    pub const FORWARD: Self = vec3(T::ZERO, T::ZERO, T::ONE);
}

impl<T: Signed> Vec3<T> {
    /// A normalized vector pointing left, equal to `(-1, 0, 0)`.
    pub const LEFT: Self = vec3(T::NEG_ONE, T::ZERO, T::ZERO);

    /// A normalized vector pointing down, equal to `(0, -1, 0)`.
    pub const DOWN: Self = vec3(T::ZERO, T::NEG_ONE, T::ZERO);

    /// A normalized vector pointing backward, equal to `(0, 0, -1)`.
    pub const BACKWARD: Self = vec3(T::ZERO, T::ZERO, T::NEG_ONE);

    /// Returns the cross-product of this vector and another,
    /// equal to:
    ///
    /// `(y¹z²-y²z¹, -(x¹z²-x²z¹), x¹y²-x²y¹)`.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        vec3(
            self.y * other.z - other.y * self.z,
            -(self.x * other.z - other.x * self.z),
            self.x * other.y - other.x * self.y,
        )
    }

    /// Given the triangle `(a, b, c)`, and the interpolation values
    /// `ab` and `bc`, returns a barycentric coordinate.
    #[inline]
    pub fn barycentric(a: Self, b: Self, c: Self, t1: T, t2: T) -> Self {
        #[inline]
        fn coord<T: Signed>(a: T, b: T, c: T, ab: T, bc: T) -> T {
            a + (b - a) * ab + (c - a) * bc
        }
        vec3(
            coord(a.x, b.x, c.x, t1, t2),
            coord(a.y, b.y, c.y, t1, t2),
            coord(a.z, b.z, c.z, t1, t2),
        )
    }
}

impl<T: Display> Display for Vec3<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(", ")?;
        self.z.fmt(f)
    }
}

impl<T: Num> From<Vec2<T>> for Vec3<T> {
    #[inline]
    fn from(Vec2 { x, y }: Vec2<T>) -> Self {
        vec3(x, y, T::ZERO)
    }
}
