use crate::{Angle, Float, Mat3, Num, Signed, Vec2, impl_mat, vec2};

pub type Mat2F = Mat2<f32>;

/// A 2x2 column major matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Mat2<T> {
    pub x_axis: Vec2<T>,
    pub y_axis: Vec2<T>,
}

/// Create a [`Mat2`].
#[inline]
pub const fn mat2<T>(x_axis: Vec2<T>, y_axis: Vec2<T>) -> Mat2<T> {
    Mat2 { x_axis, y_axis }
}

impl_mat!(
    NAME = Mat2
    SHORT = mat2
    VEC_TY = Vec2
    MUL_FN = mul_mat2
    FIELDS = (x_axis, y_axis)
    CONSTS = (X_AXIS, Y_AXIS)
);

impl<T: Copy> Mat2<T> {
    /// Create from a 3x3 matrix, discarding the 2nd row and column.
    #[inline]
    pub fn from_mat3(m: &Mat3<T>) -> Self {
        mat2(m.x_axis.xy(), m.y_axis.xy())
    }

    /// Transpose the matrix.
    #[inline]
    pub const fn transpose(&self) -> Self {
        mat2(
            vec2(self.x_axis.x, self.y_axis.x),
            vec2(self.x_axis.y, self.y_axis.y),
        )
    }
}

impl<T: Num> Mat2<T> {
    /// Returns the determinant.
    #[inline]
    pub fn determinant(&self) -> T {
        self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
    }

    /// Transforms a 2D vector.
    #[inline]
    pub fn transform_vec2(&self, rhs: Vec2<T>) -> Vec2<T> {
        vec2(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y,
        )
    }

    /// Multiply by another matrix.
    #[inline]
    pub fn mul_mat2(&self, rhs: &Self) -> Self {
        mat2(
            self.transform_vec2(rhs.x_axis),
            self.transform_vec2(rhs.y_axis),
        )
    }

    /// Create a scaling matrix.
    #[inline]
    pub fn scale(scale: impl Into<Vec2<T>>) -> Self {
        let scale = scale.into();
        mat2(vec2(scale.x, T::ZERO), vec2(T::ZERO, scale.y))
    }
}

impl<T: Signed> Mat2<T> {
    /// Create a rotation matrix.
    #[inline]
    pub fn rotation(angle: impl Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        mat2(vec2(cos, sin), vec2(-sin, cos))
    }

    /// Create a scale + rotation matrix.
    #[inline]
    pub fn scale_rotation(scale: impl Into<Vec2<T>>, angle: impl Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        let scale = scale.into();
        mat2(
            vec2(cos * scale.x, sin * scale.x),
            vec2(-sin * scale.y, cos * scale.y),
        )
    }
}

impl<T: Float> Mat2<T> {
    /// Try to invert the matrix.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let inv_det = {
            let det = self.determinant();
            if det == T::ZERO {
                return None;
            }
            T::ONE / det
        };
        Some(mat2(
            vec2(self.y_axis.y * inv_det, self.x_axis.y * -inv_det),
            vec2(self.y_axis.x * -inv_det, self.x_axis.x * inv_det),
        ))
    }
}
