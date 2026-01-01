use crate::{Angle, Float, Mat2, Quad, Rect, Vec2, impl_affine};

pub type Affine2F = Affine2<f32>;

/// A 2D affine matrix (translation, rotation, scaling and shear).
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Affine2<T> {
    pub matrix: Mat2<T>,
    pub translation: Vec2<T>,
}

/// Create an [`Affine2`].
#[inline]
pub const fn affine2<T>(matrix: Mat2<T>, translation: Vec2<T>) -> Affine2<T> {
    Affine2 {
        matrix,
        translation,
    }
}

impl_affine!(
    NAME = Affine2
    SHORT = affine2
    MUL_FN = mul_affine2
    MAT_TY = Mat2
    VEC_TY = Vec2
);

impl<T: Float> Affine2<T> {
    /// Create a rotation matrix.
    #[inline]
    pub fn rotation(angle: impl Angle<T>) -> Self {
        affine2(Mat2::rotation(angle), Vec2::ZERO)
    }

    /// Create a translation-rotation-scaling matrix.
    #[inline]
    pub fn trs(translation: Vec2<T>, rotation: impl Angle<T>, scale: Vec2<T>) -> Self {
        // TODO: simplify this?
        Self::translation(translation) * Self::rotation(rotation) * Self::scale(scale)
    }

    // /// Decompose the matrix into a transform.
    // #[inline]
    // pub fn decompose(self) -> Transform2D<T> {
    //     transform2d(
    //         self.translation,
    //         Radians(T::atan2(-self.matrix.y_axis.x, self.matrix.y_axis.y)),
    //         vec2(
    //             self.matrix.x_axis.len() * T::signum(self.matrix.determinant()),
    //             self.matrix.y_axis.len(),
    //         ),
    //     )
    // }

    /// Transforms a 2D vector.
    #[inline]
    pub fn transform_vec2(&self, rhs: Vec2<T>) -> Vec2<T> {
        self.matrix.transform_vec2(rhs)
    }

    /// Transforms a 2D point.
    #[inline]
    pub fn transform_pos2(&self, rhs: Vec2<T>) -> Vec2<T> {
        self.matrix.transform_vec2(rhs) + self.translation
    }

    // /// Transform the circle by the matrix, generating a polygon of `N` points.
    // /// This is useful for generating a polygonal representation of an ellipse.
    // #[inline]
    // pub fn transform_circ(&self, rhs: Circle<T>, seg_len: T) -> Polygon<T> {
    //     let mut poly = Polygon::new();
    //     rhs.hull_points(seg_len, Radians::ZERO, |p| {
    //         poly.push(self.transform_pos2(p));
    //     });
    //     poly
    // }
    //
    // /// Transform the circle by the matrix, but have it remain a circle. A true
    // /// transformation would have the circle become an ellipse, but ellipses are
    // /// not supported currently.
    // #[inline]
    // pub fn transform_circ_retain(&self, rhs: Circle<T>) -> Circle<T> {
    //     let quad = rhs.bounds().corners().map(|p| self.transform_pos2(p));
    //     let dist_ab = (quad[0] - quad[1]).sqr_len();
    //     let dist_bc = (quad[1] - quad[2]).sqr_len();
    //     let sum = quad[0] + quad[1] + quad[2] + quad[3];
    //     Circle {
    //         center: sum / T::FOUR,
    //         radius: T::min(dist_ab, dist_bc) / T::TWO,
    //     }
    // }
    //
    // /// Transforms a rectangle.
    // #[inline]
    // pub fn transform_tri(&self, mut rhs: Triangle<T>) -> Triangle<T> {
    //     for p in &mut rhs.0 {
    //         *p = self.transform_pos2(*p);
    //     }
    //     rhs
    // }
    //
    /// Transforms a rectangle.
    #[inline]
    pub fn transform_rect(&self, rhs: Rect<T>) -> Quad<T> {
        self.transform_quad(Quad::from_rect(rhs))
    }
    //
    // /// Transforms a rectangle, but have it remain a rectangle.
    // #[inline]
    // pub fn transform_rect_retain(&self, rhs: Rect<T>) -> Rect<T> {
    //     self.transform_rect(rhs).bounds()
    // }
    //
    /// Transforms a quad.
    #[inline]
    pub fn transform_quad(&self, mut rhs: Quad<T>) -> Quad<T> {
        for p in &mut rhs.0 {
            *p = self.transform_pos2(*p);
        }
        rhs
    }
    //
    // /// Transforms a polygon.
    // #[inline]
    // pub fn transform_poly(&self, rhs: &mut Polygon<T>) {
    //     rhs.transform_in_place(self);
    //     rhs
    // }
    //
    // /// Transforms a dynamic shape.
    // #[inline]
    // pub fn transform_dyn(&self, rhs: DynShape<T>, seg_len: T) -> DynShape<T> {
    //     match rhs {
    //         DynShape::Circle(sh) => DynShape::Polygon(self.transform_circ(sh, seg_len)),
    //         DynShape::Triangle(sh) => DynShape::Triangle(self.transform_tri(sh)),
    //         DynShape::Rect(sh) => DynShape::Quad(self.transform_rect(sh)),
    //         DynShape::Quad(sh) => DynShape::Quad(self.transform_quad(sh)),
    //         DynShape::Polygon(sh) => DynShape::Polygon(self.transform_poly(sh)),
    //     }
    // }
    //
    // /// Transforms a dynamic shape without changing the shape's variant. This
    // /// means that for circles and rectangles, the `transform_circ_retain()` and
    // /// `transform_rect_retain()` variations will be used.
    // #[inline]
    // pub fn transform_dyn_retain(&self, rhs: DynShape<T>) -> DynShape<T> {
    //     match rhs {
    //         DynShape::Circle(sh) => DynShape::Circle(self.transform_circ_retain(sh)),
    //         DynShape::Triangle(sh) => DynShape::Triangle(self.transform_tri(sh)),
    //         DynShape::Rect(sh) => DynShape::Rect(self.transform_rect_retain(sh)),
    //         DynShape::Quad(sh) => DynShape::Quad(self.transform_quad(sh)),
    //         DynShape::Polygon(sh) => DynShape::Polygon(self.transform_poly(sh)),
    //     }
    // }
    //
    // /// Transforms a line.
    // #[inline]
    // pub fn transform_line(&self, rhs: Line<T>) -> Line<T> {
    //     line(self.transform_pos2(rhs.start), self.transform_pos2(rhs.end))
    // }

    /// Try to invert the matrix.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let matrix = self.matrix.inverse()?;
        let translation = -matrix.transform_vec2(self.translation);
        Some(affine2(matrix, translation))
    }

    /// Multiply by another matrix.
    #[inline]
    pub fn mul_affine2(&self, rhs: &Self) -> Self {
        affine2(
            self.matrix * rhs.matrix,
            self.matrix.transform_vec2(rhs.translation) + self.translation,
        )
    }
}
