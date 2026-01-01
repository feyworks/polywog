use crate::{
    Float, Line, Num, Rect, Vec2, impl_approx, impl_bytemuck, impl_casts, impl_interp, line,
};
use serde::{Deserialize, Serialize};

pub type QuadF = Quad<f32>;
pub type QuadI = Quad<i32>;

/// A quad, represented by 4 points.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Quad<T>(pub [Vec2<T>; 4]);

impl_bytemuck!(Quad);

impl_interp!(
    NAME = Quad
    INDICES = [0, 1, 2, 3]
);

impl_casts!(
    NAME = Quad
    FIELDS = (0)
);

impl_approx!(
    NAME = Quad
    FIELDS = (0)
);

/// Create a [`Quad`].
#[inline]
pub const fn quad<T>(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>, d: Vec2<T>) -> Quad<T> {
    Quad([a, b, c, d])
}

impl<T> Quad<T> {
    /// Create a new quad.
    #[inline]
    pub const fn new(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>, d: Vec2<T>) -> Self {
        Self([a, b, c, d])
    }
}

impl<T: Copy> Quad<T> {
    /// First point of the quad.
    #[inline]
    pub const fn a(&self) -> Vec2<T> {
        self.0[0]
    }

    /// Second point of the quad
    #[inline]
    pub const fn b(&self) -> Vec2<T> {
        self.0[1]
    }

    /// Third point of the quad.
    #[inline]
    pub const fn c(&self) -> Vec2<T> {
        self.0[2]
    }

    /// Third point of the quad.
    #[inline]
    pub const fn d(&self) -> Vec2<T> {
        self.0[3]
    }

    /// The quad's 4 edges.
    #[inline]
    pub fn edges(&self) -> [Line<T>; 4] {
        [
            self.edge_ab(),
            self.edge_bc(),
            self.edge_cd(),
            self.edge_da(),
        ]
    }

    /// The quad's a -> b edge.
    #[inline]
    pub const fn edge_ab(&self) -> Line<T> {
        line(self.a(), self.b())
    }

    /// The quad's b -> c edge.
    #[inline]
    pub const fn edge_bc(&self) -> Line<T> {
        line(self.b(), self.c())
    }

    /// The quad's c -> d edge.
    #[inline]
    pub const fn edge_cd(&self) -> Line<T> {
        line(self.c(), self.d())
    }

    /// The quad's d -> a edge.
    #[inline]
    pub const fn edge_da(&self) -> Line<T> {
        line(self.d(), self.a())
    }
}

impl<T: Num> Quad<T> {
    /// A quad with all points set to `(0, 0)`.
    pub const ZERO: Self = Self([Vec2::ZERO; 4]);

    /// Create a quad equivalent to the provided rectangle.
    pub fn from_rect(rect: Rect<T>) -> Self {
        Self(rect.corners())
    }

    /// The vector a -> b of the quad.
    #[inline]
    pub fn ab(&self) -> Vec2<T> {
        self.a() - self.b()
    }

    /// The vector b -> c of the quad.
    #[inline]
    pub fn bc(&self) -> Vec2<T> {
        self.c() - self.b()
    }

    /// The vector c -> d of the quad.
    #[inline]
    pub fn cd(&self) -> Vec2<T> {
        self.d() - self.c()
    }

    /// The vector d -> a of the quad.
    #[inline]
    pub fn da(&self) -> Vec2<T> {
        self.a() - self.d()
    }
}

impl<T: Float> Quad<T> {
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
    pub fn norm_cd(&self) -> Vec2<T> {
        self.cd().turn_left().norm()
    }

    /// Return the outward-facing normal of edge c -> a.
    #[inline]
    pub fn norm_da(&self) -> Vec2<T> {
        self.da().turn_left().norm()
    }

    /// Create a quad representing a "line segment" from `start` to
    /// `end`, with the thickness of `width`. Useful for rendering.
    #[inline]
    pub fn line(line: impl Into<Line<T>>, width: T) -> Self {
        let line = line.into();
        let n2 = T::ONE + T::ONE;
        let off = (line.end - line.start).turn_left().norm() * (width / n2);
        Self([
            line.start + off,
            line.end + off,
            line.end - off,
            line.start - off,
        ])
    }

    /// Create a quad representing a "line segment" from `start` to
    /// `end`, with the provided start and end thickness. Useful for rendering.
    #[inline]
    pub fn stroke(line: impl Into<Line<T>>, start_width: T, end_width: T) -> Self {
        let line = line.into();
        let off = (line.end - line.start).turn_left().norm();
        let off_a = off * (start_width * T::HALF);
        let off_b = off * (end_width * T::HALF);
        Self([
            line.start + off_a,
            line.end + off_b,
            line.end - off_b,
            line.start - off_a,
        ])
    }

    #[inline]
    pub fn transform_by(&self, f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Self {
        Self(self.0.map(f))
    }
}

impl<T> AsRef<[Vec2<T>]> for Quad<T> {
    #[inline]
    fn as_ref(&self) -> &[Vec2<T>] {
        self.0.as_slice()
    }
}

impl<T> From<(Vec2<T>, Vec2<T>, Vec2<T>, Vec2<T>)> for Quad<T> {
    #[inline]
    fn from((a, b, c, d): (Vec2<T>, Vec2<T>, Vec2<T>, Vec2<T>)) -> Self {
        Self([a, b, c, d])
    }
}

impl<T> From<Quad<T>> for (Vec2<T>, Vec2<T>, Vec2<T>, Vec2<T>) {
    #[inline]
    fn from(Quad([a, b, c, d]): Quad<T>) -> Self {
        (a, b, c, d)
    }
}

impl<T> From<[Vec2<T>; 4]> for Quad<T> {
    #[inline]
    fn from(value: [Vec2<T>; 4]) -> Self {
        Self(value)
    }
}

impl<T> From<Quad<T>> for [Vec2<T>; 4] {
    #[inline]
    fn from(value: Quad<T>) -> Self {
        value.0
    }
}

impl<T: Num> From<Rect<T>> for Quad<T> {
    #[inline]
    fn from(value: Rect<T>) -> Self {
        Self::from_rect(value)
    }
}
