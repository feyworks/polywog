use crate::{Float, Line, Num, Vec2, impl_approx, impl_bytemuck, impl_casts, impl_interp, line};
use serde::{Deserialize, Serialize};

pub type TriangleF = Triangle<f32>;
pub type TriangleI = Triangle<i32>;

/// A triangle, represented by 3 points.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Triangle<T>(pub [Vec2<T>; 3]);

impl_bytemuck!(Triangle);

impl_interp!(
    NAME = Triangle
    INDICES = [0, 1, 2]
);

impl_casts!(
    NAME = Triangle
    FIELDS = (0)
);

impl_approx!(
    NAME = Triangle
    FIELDS = (0)
);

/// Create a [`Triangle`].
#[inline]
pub const fn triangle<T>(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>) -> Triangle<T> {
    Triangle([a, b, c])
}

impl<T> Triangle<T> {
    /// Create a new triangle.
    #[inline]
    pub const fn new(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>) -> Self {
        Self([a, b, c])
    }
}

impl<T: Copy> Triangle<T> {
    /// First point of the triangle.
    #[inline]
    pub const fn a(&self) -> Vec2<T> {
        self.0[0]
    }

    /// Second point of the triangle
    #[inline]
    pub const fn b(&self) -> Vec2<T> {
        self.0[1]
    }

    /// Third point of the triangle.
    #[inline]
    pub const fn c(&self) -> Vec2<T> {
        self.0[2]
    }

    /// The triangle's 3 edges.
    #[inline]
    pub fn edges(&self) -> [Line<T>; 3] {
        [self.edge_ab(), self.edge_bc(), self.edge_ca()]
    }

    /// The triangle's a -> b edge.
    #[inline]
    pub const fn edge_ab(&self) -> Line<T> {
        line(self.a(), self.b())
    }

    /// The triangle's b -> c edge.
    #[inline]
    pub const fn edge_bc(&self) -> Line<T> {
        line(self.b(), self.c())
    }

    /// The triangle's c -> a edge.
    #[inline]
    pub const fn edge_ca(&self) -> Line<T> {
        line(self.c(), self.a())
    }
}

impl<T: Num> Triangle<T> {
    /// A triangle with all points set to `(0, 0)`.
    pub const ZERO: Self = Self([Vec2::ZERO; 3]);

    /// The vector a -> b of the triangle.
    #[inline]
    pub fn ab(&self) -> Vec2<T> {
        self.b() - self.a()
    }

    /// The vector b -> c of the triangle.
    #[inline]
    pub fn bc(&self) -> Vec2<T> {
        self.c() - self.b()
    }

    /// The vector c -> a of the triangle.
    #[inline]
    pub fn ca(&self) -> Vec2<T> {
        self.a() - self.c()
    }
}

impl<T: Float> Triangle<T> {
    /// Return the outward-facing normal of edge a -> b.
    #[inline]
    pub fn norm_ab(&self) -> Vec2<T> {
        self.ab().turn_left().norm()
    }

    /// Return the outward-facing normal of edge b -> c.
    #[inline]
    pub fn norm_bc(&self) -> Vec2<T> {
        self.bc().turn_left().norm()
    }

    /// Return the outward-facing normal of edge c -> a.
    #[inline]
    pub fn norm_ca(&self) -> Vec2<T> {
        self.ca().turn_left().norm()
    }

    #[inline]
    pub fn transform_by(&self, f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Self {
        Self(self.0.map(f))
    }
}

impl<T> AsRef<[Vec2<T>]> for Triangle<T> {
    #[inline]
    fn as_ref(&self) -> &[Vec2<T>] {
        self.0.as_slice()
    }
}

impl<T> From<(Vec2<T>, Vec2<T>, Vec2<T>)> for Triangle<T> {
    #[inline]
    fn from((a, b, c): (Vec2<T>, Vec2<T>, Vec2<T>)) -> Self {
        Self([a, b, c])
    }
}

impl<T> From<Triangle<T>> for (Vec2<T>, Vec2<T>, Vec2<T>) {
    #[inline]
    fn from(Triangle([a, b, c]): Triangle<T>) -> Self {
        (a, b, c)
    }
}

impl<T> From<[Vec2<T>; 3]> for Triangle<T> {
    #[inline]
    fn from(value: [Vec2<T>; 3]) -> Self {
        Self(value)
    }
}

impl<T> From<Triangle<T>> for [Vec2<T>; 3] {
    #[inline]
    fn from(value: Triangle<T>) -> Self {
        value.0
    }
}
