use crate::{
    Angle, Circle, Float, Line, Polygon, Polygonal, Projection, Quad, Radians, Ray, RayHit, Rect,
    Shape, Triangle, Vec2,
};

pub type DynShapeF = DynShape<f32>;

/// A circle, triangle, rect, quad, or polygon.
///
/// This itself implements [`Shape<T>`] and defers to each variant's
/// implementation of the trait's methods.
///
/// This is convenient when you want lists of shapes that can be one
/// of any of the variants, rather than all being the same. For example,
/// colliders for entities in a core engine.
#[derive(Debug, Clone)]
pub enum DynShape<T> {
    Circle(Circle<T>),
    Triangle(Triangle<T>),
    Rect(Rect<T>),
    Quad(Quad<T>),
    Polygon(Polygon<T>),
}

impl<T: Float> DynShape<T> {
    /// Returns true if this shape overlaps the other.
    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        match other {
            Self::Circle(sh) => self.overlaps_circ(sh),
            Self::Triangle(sh) => self.overlaps_poly(sh),
            Self::Rect(sh) => self.overlaps_rect(sh),
            Self::Quad(sh) => self.overlaps_poly(sh),
            Self::Polygon(sh) => self.overlaps_poly(sh),
        }
    }

    /// If this shape overlaps the other, returns a *push-out* vector
    /// that can be used to translate it so they no longer overlap.
    #[inline]
    pub fn extract_from(&self, other: &Self) -> Option<Vec2<T>> {
        match other {
            Self::Circle(sh) => self.extract_from_circ(sh),
            Self::Triangle(sh) => self.extract_from_poly(sh),
            Self::Rect(sh) => self.extract_from_poly(sh),
            Self::Quad(sh) => self.extract_from_poly(sh),
            Self::Polygon(sh) => self.extract_from_poly(sh),
        }
    }

    /// Plot the vertices of the shape. For polygons, it will be the vertices
    /// of the polygon, and for circles it will be `CAP` points distributed
    /// evenly along the circle's radius.
    pub fn hull_points(&self, circle_seg_len: T, plot: impl FnMut(Vec2<T>)) {
        match self {
            Self::Circle(sh) => sh.hull_points(circle_seg_len, Radians(T::ZERO), plot),
            Self::Triangle(sh) => sh.0.iter().copied().for_each(plot),
            Self::Rect(sh) => sh.corners().into_iter().for_each(plot),
            Self::Quad(sh) => sh.0.iter().copied().for_each(plot),
            Self::Polygon(sh) => sh.points().iter().copied().for_each(plot),
        }
    }

    /// Plot the vertices of the shape.
    pub fn hull_points_n(&self, circle_count: T, plot: impl FnMut(Vec2<T>)) {
        match self {
            Self::Circle(sh) => sh.hull_points_n(circle_count, Radians(T::ZERO), plot),
            Self::Triangle(sh) => sh.0.iter().copied().for_each(plot),
            Self::Rect(sh) => sh.corners().into_iter().for_each(plot),
            Self::Quad(sh) => sh.0.iter().copied().for_each(plot),
            Self::Polygon(sh) => sh.points().iter().copied().for_each(plot),
        }
    }

    /// Plot the edges of the shape. For polygons, it will be the edges of
    /// the polygon, and for circles it will be `CAP` edgds distributed
    /// evenly along the circle's radius.
    pub fn hull_edges(&self, circle_seg_len: T, plot: impl FnMut(Line<T>)) {
        match self {
            Self::Circle(sh) => sh.hull_edges(circle_seg_len, Radians(T::ZERO), plot),
            Self::Triangle(sh) => sh.visit_edges(plot),
            Self::Rect(sh) => sh.visit_edges(plot),
            Self::Quad(sh) => sh.visit_edges(plot),
            Self::Polygon(sh) => sh.visit_edges(plot),
        }
    }

    /// Plot the edges of the shape. For polygons, it will be the edges of
    /// the polygon, and for circles it will be `CAP` edgds distributed
    /// evenly along the circle's radius.
    pub fn hull_edges_n(&self, circle_count: T, plot: impl FnMut(Line<T>)) {
        match self {
            Self::Circle(sh) => sh.hull_edges_n(circle_count, Radians(T::ZERO), plot),
            Self::Triangle(sh) => sh.visit_edges(plot),
            Self::Rect(sh) => sh.visit_edges(plot),
            Self::Quad(sh) => sh.visit_edges(plot),
            Self::Polygon(sh) => sh.visit_edges(plot),
        }
    }

    pub fn transform_into(
        &self,
        into: &mut Self,
        circle_out: CircleOut<T>,
        rect_out: RectOut,
        f: impl FnMut(Vec2<T>) -> Vec2<T>,
    ) {
        match self {
            Self::Circle(sh) => match circle_out {
                CircleOut::Circle => {
                    *into = Self::Circle(sh.transform_by_retain(f));
                }
                CircleOut::SegCount { count, angle } => {
                    if let Self::Polygon(into) = into {
                        sh.transform_by_into_n(count, angle, into, f);
                    } else {
                        *into = Self::Polygon(sh.transform_by_n(count, angle, f));
                    }
                }
                CircleOut::SegLen { len, angle } => {
                    if let Self::Polygon(into) = into {
                        sh.transform_by_into(len, angle, into, f);
                    } else {
                        *into = Self::Polygon(sh.transform_by(len, angle, f));
                    }
                }
            },
            Self::Triangle(sh) => {
                if let Self::Triangle(into) = into {
                    *into = sh.transform_by(f);
                } else {
                    *into = Self::Triangle(sh.transform_by(f));
                }
            }
            Self::Rect(sh) => match rect_out {
                RectOut::Rect => {
                    if let Self::Rect(into) = into {
                        *into = sh.transform_by_retain(f);
                    } else {
                        *into = Self::Rect(sh.transform_by_retain(f));
                    }
                }
                RectOut::Quad => {
                    if let Self::Quad(into) = into {
                        *into = sh.transform_by(f);
                    } else {
                        *into = Self::Quad(sh.transform_by(f));
                    }
                }
            },
            Self::Quad(sh) => {
                if let Self::Quad(into) = into {
                    *into = sh.transform_by(f);
                } else {
                    *into = Self::Quad(sh.transform_by(f));
                }
            }
            Self::Polygon(sh) => {
                if let Self::Polygon(into) = into {
                    sh.transform_by_into(into, f);
                } else {
                    *into = Self::Polygon(sh.transform_by(f));
                }
            }
        }
    }
}

macro_rules! delegate {
    ($this:ident, $call:ident, $($arg:ident),*) => {
        match $this {
            Self::Circle(sh) => sh.$call($($arg),*),
            Self::Triangle(sh) => sh.$call($($arg),*),
            Self::Rect(sh) => sh.$call($($arg),*),
            Self::Quad(sh) => sh.$call($($arg),*),
            Self::Polygon(sh) => sh.$call($($arg),*),
        }
    }
}

impl<T: Float> Shape<T> for DynShape<T> {
    #[inline]
    fn centroid(&self) -> Vec2<T> {
        delegate!(self, centroid,)
    }

    #[inline]
    fn contains(&self, p: Vec2<T>) -> bool {
        delegate!(self, contains, p)
    }

    #[inline]
    fn bounds(&self) -> Rect<T> {
        delegate!(self, bounds,)
    }

    #[inline]
    fn project_onto_axis(&self, axis: Vec2<T>) -> Projection<T> {
        delegate!(self, project_onto_axis, axis)
    }

    #[inline]
    fn project_point(&self, p: Vec2<T>) -> Vec2<T> {
        delegate!(self, project_point, p)
    }

    #[inline]
    fn rayhit(&self, ray: &Ray<T>) -> bool {
        delegate!(self, rayhit, ray)
    }

    #[inline]
    fn raycast(&self, ray: &Ray<T>) -> Option<RayHit<T>> {
        delegate!(self, raycast, ray)
    }

    #[inline]
    fn overlaps_rect(&self, rect: &Rect<T>) -> bool {
        delegate!(self, overlaps_rect, rect)
    }

    #[inline]
    fn overlaps_circ(&self, circ: &Circle<T>) -> bool {
        delegate!(self, overlaps_circ, circ)
    }

    #[inline]
    fn overlaps_poly<P: Polygonal<T>>(&self, poly: &P) -> bool {
        delegate!(self, overlaps_poly, poly)
    }

    #[inline]
    fn extract_from_circ(&self, circ: &Circle<T>) -> Option<Vec2<T>> {
        delegate!(self, extract_from_circ, circ)
    }

    #[inline]
    fn extract_from_poly<P: Polygonal<T>>(&self, poly: &P) -> Option<Vec2<T>> {
        delegate!(self, extract_from_poly, poly)
    }

    #[inline]
    fn is_convex(&self) -> bool {
        delegate!(self, is_convex,)
    }
}

impl<T> From<Circle<T>> for DynShape<T> {
    #[inline]
    fn from(value: Circle<T>) -> Self {
        Self::Circle(value)
    }
}

impl<T> From<Triangle<T>> for DynShape<T> {
    #[inline]
    fn from(value: Triangle<T>) -> Self {
        Self::Triangle(value)
    }
}

impl<T> From<Rect<T>> for DynShape<T> {
    #[inline]
    fn from(value: Rect<T>) -> Self {
        Self::Rect(value)
    }
}

impl<T> From<Quad<T>> for DynShape<T> {
    #[inline]
    fn from(value: Quad<T>) -> Self {
        Self::Quad(value)
    }
}

impl<T> From<Polygon<T>> for DynShape<T> {
    #[inline]
    fn from(value: Polygon<T>) -> Self {
        Self::Polygon(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CircleOut<T> {
    Circle,
    SegCount { count: T, angle: Radians<T> },
    SegLen { len: T, angle: Radians<T> },
}

impl<T> CircleOut<T> {
    #[inline]
    pub fn seg_count(count: T, angle: impl Angle<T>) -> Self {
        Self::SegCount {
            count,
            angle: angle.to_radians(),
        }
    }

    #[inline]
    pub fn seg_len(len: T, angle: impl Angle<T>) -> Self {
        Self::SegLen {
            len,
            angle: angle.to_radians(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RectOut {
    Rect,
    Quad,
}
