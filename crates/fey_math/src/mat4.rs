use crate::{Angle, Float, Mat2, Mat3, Num, Vec2, Vec3, Vec4, impl_mat, vec2, vec3, vec4};
use std::ops::{Add, Mul, Sub};

pub type Mat4F = Mat4<f32>;

/// A 4x4 column major matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Mat4<T> {
    pub x_axis: Vec4<T>,
    pub y_axis: Vec4<T>,
    pub z_axis: Vec4<T>,
    pub w_axis: Vec4<T>,
}

/// Create a [`Mat4`].
#[inline]
pub const fn mat4<T>(
    x_axis: Vec4<T>,
    y_axis: Vec4<T>,
    z_axis: Vec4<T>,
    w_axis: Vec4<T>,
) -> Mat4<T> {
    Mat4 {
        x_axis,
        y_axis,
        z_axis,
        w_axis,
    }
}

impl_mat!(
    NAME = Mat4
    SHORT = mat4
    VEC_TY = Vec4
    MUL_FN = mul_mat4
    FIELDS = (x_axis, y_axis, z_axis, w_axis)
    CONSTS = (X_AXIS, Y_AXIS, Z_AXIS, W_AXIS)
);

impl<T: Copy> Mat4<T> {
    /// Transpose the matrix.
    #[inline]
    pub const fn transpose(&self) -> Self {
        mat4(
            vec4(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            vec4(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            vec4(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            vec4(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        )
    }
}

impl<T: Num> Mat4<T> {
    pub fn translation(amount: impl Into<Vec3<T>>) -> Self {
        mat4(
            Vec4::X_AXIS,
            Vec4::Y_AXIS,
            Vec4::Z_AXIS,
            amount.into().with_w(T::ONE),
        )
    }

    pub fn scale(scale: impl Into<Vec3<T>>) -> Self {
        let scale = scale.into();
        mat4(
            vec4(scale.x, T::ZERO, T::ZERO, T::ZERO),
            vec4(T::ZERO, scale.y, T::ZERO, T::ZERO),
            vec4(T::ZERO, T::ZERO, scale.z, T::ZERO),
            Vec4::W_AXIS,
        )
    }
}

impl<T: Float> Mat4<T> {
    /// Create a matrix representing a rotation around an axis.
    pub fn axis_angle(axis: impl Into<Vec3<T>>, angle: impl Angle<T>) -> Self {
        let axis = axis.into();
        let (sin, cos) = angle.sin_cos();
        let axis_sin = axis * sin;
        let axis_sq = axis * axis;
        let omc = T::ONE - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;
        mat4(
            vec4(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
                T::ZERO,
            ),
            vec4(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                T::ZERO,
            ),
            vec4(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                T::ZERO,
            ),
            Vec4::W_AXIS,
        )
    }

    /// Create a matrix rotating around the x-axis.
    #[inline]
    pub fn rotation_x(angle: impl Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        mat4(
            Vec4::X_AXIS,
            vec4(T::ZERO, cos, sin, T::ZERO),
            vec4(T::ZERO, -sin, cos, T::ZERO),
            Vec4::W_AXIS,
        )
    }

    /// Create a matrix rotation around the y-axis.
    #[inline]
    pub fn rotation_y(angle: impl Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        mat4(
            vec4(cos, T::ZERO, -sin, T::ZERO),
            Vec4::Y_AXIS,
            vec4(sin, T::ZERO, cos, T::ZERO),
            Vec4::W_AXIS,
        )
    }

    /// Create a matrix rotating around the z-axis.
    #[inline]
    pub fn rotation_z(angle: impl Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        mat4(
            vec4(cos, sin, T::ZERO, T::ZERO),
            vec4(-sin, cos, T::ZERO, T::ZERO),
            Vec4::Z_AXIS,
            Vec4::W_AXIS,
        )
    }

    /// Returns the determinant.
    #[inline]
    pub fn determinant(&self) -> T {
        let (m00, m01, m02, m03) = self.x_axis.into();
        let (m10, m11, m12, m13) = self.y_axis.into();
        let (m20, m21, m22, m23) = self.z_axis.into();
        let (m30, m31, m32, m33) = self.w_axis.into();
        let a2323 = m22 * m33 - m23 * m32;
        let a1323 = m21 * m33 - m23 * m31;
        let a1223 = m21 * m32 - m22 * m31;
        let a0323 = m20 * m33 - m23 * m30;
        let a0223 = m20 * m32 - m22 * m30;
        let a0123 = m20 * m31 - m21 * m30;
        m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
            - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
            + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
            - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
    }

    /// Inverts the matrix.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let (m00, m01, m02, m03) = self.x_axis.into();
        let (m10, m11, m12, m13) = self.y_axis.into();
        let (m20, m21, m22, m23) = self.z_axis.into();
        let (m30, m31, m32, m33) = self.w_axis.into();

        let coef00 = m22 * m33 - m32 * m23;
        let coef02 = m12 * m33 - m32 * m13;
        let coef03 = m12 * m23 - m22 * m13;

        let coef04 = m21 * m33 - m31 * m23;
        let coef06 = m11 * m33 - m31 * m13;
        let coef07 = m11 * m23 - m21 * m13;

        let coef08 = m21 * m32 - m31 * m22;
        let coef10 = m11 * m32 - m31 * m12;
        let coef11 = m11 * m22 - m21 * m12;

        let coef12 = m20 * m33 - m30 * m23;
        let coef14 = m10 * m33 - m30 * m13;
        let coef15 = m10 * m23 - m20 * m13;

        let coef16 = m20 * m32 - m30 * m22;
        let coef18 = m10 * m32 - m30 * m12;
        let coef19 = m10 * m22 - m20 * m12;

        let coef20 = m20 * m31 - m30 * m21;
        let coef22 = m10 * m31 - m30 * m11;
        let coef23 = m10 * m21 - m20 * m11;

        let fac0 = vec4(coef00, coef00, coef02, coef03);
        let fac1 = vec4(coef04, coef04, coef06, coef07);
        let fac2 = vec4(coef08, coef08, coef10, coef11);
        let fac3 = vec4(coef12, coef12, coef14, coef15);
        let fac4 = vec4(coef16, coef16, coef18, coef19);
        let fac5 = vec4(coef20, coef20, coef22, coef23);

        let vec0 = vec4(m10, m00, m00, m00);
        let vec1 = vec4(m11, m01, m01, m01);
        let vec2 = vec4(m12, m02, m02, m02);
        let vec3 = vec4(m13, m03, m03, m03);

        let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
        let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
        let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
        let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

        let sign_a = vec4(T::ONE, T::NEG_ONE, T::ONE, T::NEG_ONE);
        let sign_b = vec4(T::NEG_ONE, T::ONE, T::NEG_ONE, T::ONE);

        let inverse = mat4(
            inv0.mul(sign_a),
            inv1.mul(sign_b),
            inv2.mul(sign_a),
            inv3.mul(sign_b),
        );

        let col0 = vec4(
            inverse.x_axis.x,
            inverse.y_axis.x,
            inverse.z_axis.x,
            inverse.w_axis.x,
        );

        let dot0 = self.x_axis.mul(col0);
        let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

        if dot1 == T::ZERO {
            return None;
        }

        let rcp_det = T::ONE / dot1;
        Some(inverse.mul(rcp_det))
    }

    /// Creates an orthographic perspective matrix.
    #[inline]
    pub fn ortho(left: T, right: T, bottom: T, top: T, z_near: T, z_far: T) -> Self {
        mat4(
            vec4(T::TWO / (right - left), T::ZERO, T::ZERO, T::ZERO),
            vec4(T::ZERO, T::TWO / (top - bottom), T::ZERO, T::ZERO),
            vec4(T::ZERO, T::ZERO, T::ONE / (z_near - z_far), T::ZERO),
            vec4(
                (left + right) / (left - right),
                (top + bottom) / (bottom - top),
                z_near / (z_near - z_far),
                T::ONE,
            ),
        )
    }

    /// Creates an orthographic perspective matrix with the size.
    #[inline]
    pub fn ortho_size(size: impl Into<Vec2<T>>) -> Self {
        let size = size.into();
        Self::ortho(T::ZERO, size.x, size.y, T::ZERO, T::ZERO, T::ONE)
    }

    /// Transforms a 2D point.
    #[inline]
    pub fn transform_pos2(&self, rhs: Vec2<T>) -> Vec2<T> {
        vec2(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.w_axis.x,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.w_axis.y,
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

    /// Transforms a 3D point.
    #[inline]
    pub fn transform_pos3(&self, rhs: Vec3<T>) -> Vec3<T> {
        vec3(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.z_axis.x * rhs.z + self.w_axis.x,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.z_axis.y * rhs.z + self.w_axis.y,
            self.x_axis.z * rhs.x + self.y_axis.z * rhs.y + self.z_axis.z * rhs.z + self.w_axis.z,
        )
    }

    /// Transforms a 4D vector.
    #[inline]
    pub fn transform_vec4(&self, rhs: Vec4<T>) -> Vec4<T> {
        vec4(
            self.x_axis.x * rhs.x
                + self.y_axis.x * rhs.y
                + self.z_axis.x * rhs.z
                + self.w_axis.x * rhs.w,
            self.x_axis.y * rhs.x
                + self.y_axis.y * rhs.y
                + self.z_axis.y * rhs.z
                + self.w_axis.y * rhs.w,
            self.x_axis.z * rhs.x
                + self.y_axis.z * rhs.y
                + self.z_axis.z * rhs.z
                + self.w_axis.z * rhs.w,
            self.x_axis.w * rhs.x
                + self.y_axis.w * rhs.y
                + self.z_axis.w * rhs.z
                + self.w_axis.w * rhs.w,
        )
    }

    /// Multiply by another matrix.
    #[inline]
    pub fn mul_mat4(&self, rhs: &Self) -> Self {
        mat4(
            self.transform_vec4(rhs.x_axis),
            self.transform_vec4(rhs.y_axis),
            self.transform_vec4(rhs.z_axis),
            self.transform_vec4(rhs.w_axis),
        )
    }
}

impl<T: Num> From<Mat2<T>> for Mat4<T> {
    #[inline]
    fn from(Mat2 { x_axis, y_axis }: Mat2<T>) -> Self {
        mat4(x_axis.into(), y_axis.into(), Vec4::Z_AXIS, Vec4::W_AXIS)
    }
}

impl<T: Num> From<Mat3<T>> for Mat4<T> {
    #[inline]
    fn from(
        Mat3 {
            x_axis,
            y_axis,
            z_axis,
        }: Mat3<T>,
    ) -> Self {
        mat4(x_axis.into(), y_axis.into(), z_axis.into(), Vec4::W_AXIS)
    }
}
