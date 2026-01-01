use crate::{Affine2, Angle, Float, Mat3, Num, Vec2, Vec3, impl_affine};

pub type Affine3F = Affine3<f32>;

/// A 3D affine matrix (translation, rotation, and scaling).
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Affine3<T> {
    pub matrix: Mat3<T>,
    pub translation: Vec3<T>,
}

impl_affine!(
    NAME = Affine3
    SHORT = affine3
    MUL_FN = mul_affine3
    MAT_TY = Mat3
    VEC_TY = Vec3
);

/// Create an [`Affine3`].
#[inline]
pub const fn affine3<T>(matrix: Mat3<T>, translation: Vec3<T>) -> Affine3<T> {
    Affine3 {
        matrix,
        translation,
    }
}

impl<T: Float> Affine3<T> {
    /// Create a rotation matrix along the x-axis.
    #[inline]
    pub fn rotation_x(angle: impl Angle<T>) -> Self {
        affine3(Mat3::rotation_x(angle), Vec3::ZERO)
    }

    /// Create a rotation matrix along the y-axis.
    #[inline]
    pub fn rotation_y(angle: impl Angle<T>) -> Self {
        affine3(Mat3::rotation_y(angle), Vec3::ZERO)
    }

    /// Create a rotation matrix along the z-axis.
    #[inline]
    pub fn rotation_z(angle: impl Angle<T>) -> Self {
        affine3(Mat3::rotation_z(angle), Vec3::ZERO)
    }

    /// Transforms a 2D vector.
    #[inline]
    pub fn transform_vec2(&self, rhs: Vec2<T>) -> Vec2<T> {
        self.matrix.transform_vec2(rhs)
    }

    /// Transforms a 2D point.
    #[inline]
    pub fn transform_pos2(&self, rhs: Vec2<T>) -> Vec2<T> {
        self.matrix.transform_pos2(rhs) + self.translation.xy()
    }

    /// Transforms a 3D vector.
    #[inline]
    pub fn transform_vec3(&self, rhs: Vec3<T>) -> Vec3<T> {
        self.matrix.transform_vec3(rhs)
    }

    /// Transforms a 3D point.
    #[inline]
    pub fn transform_pos3(&self, rhs: Vec3<T>) -> Vec3<T> {
        self.matrix.transform_vec3(rhs) + self.translation
    }

    /// Try to invert the matrix.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let matrix = self.matrix.inverse()?;
        let translation = -matrix.transform_vec3(self.translation);
        Some(affine3(matrix, translation))
    }

    /// Multiply by another matrix.
    #[inline]
    pub fn mul_affine3(&self, rhs: &Self) -> Self {
        affine3(
            self.matrix * rhs.matrix,
            self.matrix.transform_vec3(rhs.translation) + self.translation,
        )
    }
}

impl<T: Num> From<Affine2<T>> for Affine3<T> {
    #[inline]
    fn from(
        Affine2 {
            matrix,
            translation,
        }: Affine2<T>,
    ) -> Self {
        affine3(matrix.into(), translation.into())
    }
}
