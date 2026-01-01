use crate::{Num, Vec2, Vec3, impl_vec, vec2, vec3};
use std::fmt::{Display, Formatter};

pub type Vec4F = Vec4<f32>;
pub type Vec4I = Vec4<i32>;
pub type Vec4U = Vec4<u32>;

/// A 4-dimensional vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

/// Create a [`Vec4`].
#[inline]
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

impl_vec!(
    NAME = Vec4
    SHORT = vec4
    LEN = 4
    FIELDS = (x, y, z, w)
    TUPLE = (T, T, T, T)
);

impl<T> Vec4<T> {
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

    /// Return the vector with the z-value replaced.
    #[inline]
    pub fn with_z(self, z: T) -> Self {
        Self { z, ..self }
    }

    /// Return the vector with the w-value replaced.
    #[inline]
    pub fn with_w(self, w: T) -> Self {
        Self { w, ..self }
    }
}

impl<T: Copy> Vec4<T> {
    /// Reduce to a 2D vector by dropping `z` and `w`.
    #[inline]
    pub const fn xy(self) -> Vec2<T> {
        vec2(self.x, self.y)
    }

    /// Reduce to a 3D vector by dropping `w`.
    #[inline]
    pub const fn xyz(self) -> Vec3<T> {
        vec3(self.x, self.y, self.z)
    }

    /// Swizzle components.
    #[inline]
    pub fn yxzw(self) -> Self {
        vec4(self.y, self.x, self.z, self.w)
    }

    /// Swizzle components.
    #[inline]
    pub fn zxyw(self) -> Self {
        vec4(self.z, self.x, self.y, self.w)
    }

    /// Swizzle components.
    #[inline]
    pub fn wxzy(self) -> Self {
        vec4(self.w, self.x, self.z, self.y)
    }

    /// Swizzle components.
    #[inline]
    pub fn yxwz(self) -> Self {
        vec4(self.y, self.x, self.w, self.z)
    }

    /// Swizzle components.
    #[inline]
    pub fn yzxw(self) -> Self {
        vec4(self.y, self.z, self.x, self.w)
    }

    /// Swizzle components.
    #[inline]
    pub fn zyxw(self) -> Self {
        vec4(self.z, self.y, self.x, self.w)
    }

    /// Swizzle components.
    #[inline]
    pub fn wzxy(self) -> Self {
        vec4(self.w, self.z, self.x, self.y)
    }

    /// Swizzle components.
    #[inline]
    pub fn yzwx(self) -> Self {
        vec4(self.y, self.z, self.w, self.x)
    }

    /// Swizzle components.
    #[inline]
    pub fn zywx(self) -> Self {
        vec4(self.z, self.y, self.w, self.x)
    }

    /// Swizzle components.
    #[inline]
    pub fn wzyx(self) -> Self {
        vec4(self.w, self.z, self.y, self.x)
    }

    /// Swizzle components.
    #[inline]
    pub fn ywzx(self) -> Self {
        vec4(self.y, self.w, self.z, self.x)
    }
}

impl<T: Num> Vec4<T> {
    /// A unit vector representing the x-axis, equal to `(1, 0, 0, 0)`.
    pub const X_AXIS: Self = vec4(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// A unit vector representing the y-axis, equal to `(0, 1, 0, 0)`.
    pub const Y_AXIS: Self = vec4(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// A unit vector representing the z-axis, equal to `(0, 0, 1, 0)`.
    pub const Z_AXIS: Self = vec4(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// A unit vector representing the w-axis, equal to `(0, 0, 0, 1)`.
    pub const W_AXIS: Self = vec4(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: Display> Display for Vec4<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(", ")?;
        self.z.fmt(f)?;
        f.write_str(", ")?;
        self.w.fmt(f)
    }
}

impl<T: Num> From<Vec2<T>> for Vec4<T> {
    #[inline]
    fn from(Vec2 { x, y }: Vec2<T>) -> Self {
        vec4(x, y, T::ZERO, T::ZERO)
    }
}

impl<T: Num> From<Vec3<T>> for Vec4<T> {
    #[inline]
    fn from(Vec3 { x, y, z }: Vec3<T>) -> Self {
        vec4(x, y, z, T::ZERO)
    }
}
