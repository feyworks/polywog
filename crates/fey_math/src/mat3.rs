use crate::{Angle, Float, Mat2, Mat4, Num, Signed, Vec2, Vec3, impl_mat, vec2, vec3};
use std::ops::Mul;

pub type Mat3F = Mat3<f32>;

/// A 3x3 column major matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Mat3<T> {
    pub x_axis: Vec3<T>,
    pub y_axis: Vec3<T>,
    pub z_axis: Vec3<T>,
}

/// Create a [`Mat3`].
#[inline]
pub const fn mat3<T>(x_axis: Vec3<T>, y_axis: Vec3<T>, z_axis: Vec3<T>) -> Mat3<T> {
    Mat3 {
        x_axis,
        y_axis,
        z_axis,
    }
}

impl_mat!(
    NAME = Mat3
    SHORT = mat3
    VEC_TY = Vec3
    MUL_FN = mul_mat3
    FIELDS = (x_axis, y_axis, z_axis)
    CONSTS = (X_AXIS, Y_AXIS, Z_AXIS)
);

impl<T: Copy> Mat3<T> {
    /// Create from a 4x4 matrix, discarding the 4th row and column.
    #[inline]
    pub const fn from_mat4(m: &Mat4<T>) -> Self {
        mat3(m.x_axis.xyz(), m.y_axis.xyz(), m.z_axis.xyz())
    }

    // Transpose the matrix.
    #[inline]
    pub const fn transpose(&self) -> Self {
        mat3(
            vec3(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            vec3(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            vec3(self.x_axis.z, self.y_axis.z, self.z_axis.z),
        )
    }
}

impl<T: Num> Mat3<T> {
    /// Create a translation matrix.
    #[inline]
    pub fn translation(amount: impl Into<Vec2<T>>) -> Self {
        mat3(Vec3::X_AXIS, Vec3::Y_AXIS, amount.into().with_z(T::ONE))
    }

    /// Create a scaling matrix.
    #[inline]
    pub fn scale(scale: impl Into<Vec3<T>>) -> Self {
        let scale = scale.into();
        mat3(
            vec3(scale.x, T::ZERO, T::ZERO),
            vec3(T::ZERO, scale.y, T::ZERO),
            vec3(T::ZERO, T::ZERO, scale.z),
        )
    }

    /// Transforms a 2D position.
    #[inline]
    pub fn transform_pos2(&self, rhs: Vec2<T>) -> Vec2<T> {
        vec2(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.z_axis.x,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.z_axis.y,
        )
    }

    /// Transforms a 2D vector.
    #[inline]
    pub fn transform_vec2(&self, rhs: Vec2<T>) -> Vec2<T> {
        vec2(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y,
        )
    }

    /// Transforms a 3D vector.
    #[inline]
    pub fn transform_vec3(&self, rhs: Vec3<T>) -> Vec3<T> {
        vec3(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.z_axis.x * rhs.z,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.z_axis.y * rhs.z,
            self.x_axis.z * rhs.x + self.y_axis.z * rhs.y + self.z_axis.z * rhs.z,
        )
    }

    /// Multiply by another matrix.
    #[inline]
    pub fn mul_mat3(&self, rhs: &Self) -> Self {
        mat3(
            self.transform_vec3(rhs.x_axis),
            self.transform_vec3(rhs.y_axis),
            self.transform_vec3(rhs.z_axis),
        )
    }
}

impl<T: Signed> Mat3<T> {
    /// Returns the determinant.
    #[inline]
    pub fn determinant(&self) -> T {
        self.z_axis.dot(self.x_axis.cross(self.y_axis))
    }
}

impl<T: Float> Mat3<T> {
    /// Invert the matrix.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot(tmp2);
        if det == T::ZERO {
            return None;
        }
        let inv_det = Vec3::splat(T::ONE / det);
        Some(mat3(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose())
    }

    /// Create a matrix rotation around `axis` by the `angle`.
    #[inline]
    pub fn axis_angle(axis: Vec3<T>, angle: impl Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (xsin, ysin, zsin) = axis.mul(sin).into();
        let (x, y, z) = axis.into();
        let (x2, y2, z2) = axis.mul(axis).into();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        mat3(
            vec3(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            vec3(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            vec3(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        )
    }

    /// Create a rotation matrix around the x-axis.
    #[inline]
    pub fn rotation_x(angle: impl Angle<T>) -> Self {
        let (s, c) = angle.sin_cos();
        mat3(Vec3::X_AXIS, vec3(T::ZERO, c, s), vec3(T::ZERO, -s, c))
    }

    /// Create a rotation matrix around the y-axis.
    #[inline]
    pub fn rotation_y(angle: impl Angle<T>) -> Self {
        let (s, c) = angle.sin_cos();
        mat3(vec3(c, T::ZERO, -s), Vec3::Y_AXIS, vec3(s, T::ZERO, c))
    }

    /// Create a rotation matrix around the z-axis.
    #[inline]
    pub fn rotation_z(angle: impl Angle<T>) -> Self {
        let (s, c) = angle.sin_cos();
        mat3(vec3(c, s, T::ZERO), vec3(-s, c, T::ZERO), Vec3::Z_AXIS)
    }
}

impl<T: Num> From<Mat2<T>> for Mat3<T> {
    #[inline]
    fn from(Mat2 { x_axis, y_axis }: Mat2<T>) -> Self {
        mat3(x_axis.into(), y_axis.into(), Vec3::Z_AXIS)
    }
}
