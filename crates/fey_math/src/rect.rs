use crate::{
    Circle, Float, Line, Num, Polygonal, Projection, Ray, RayHit, Shape, Signed, Vec2, extract_on,
    impl_approx, impl_bytemuck, impl_casts, impl_interp, impl_serde, impl_tuple_arr, line,
    overlaps_on, vec2,
};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::Quad;

pub type RectF = Rect<f32>;
pub type RectI = Rect<i32>;
pub type RectU = Rect<u32>;

/// A 2D axis-aligned rectangle.
///
/// Most of the methods for this struct assume that the rectangle
/// has a positive width and height, so rectangles where `T` is
/// signed may yield incorrect values for negative-sized instances.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl_tuple_arr!(
    NAME = Rect
    LEN = 4
    FIELDS = (x, y, w, h)
    TUPLE = (T, T, T, T)
);

impl_approx!(
    NAME = Rect
    FIELDS = (x, y, w, h)
);

impl_serde!(
    NAME = Rect
    FIELDS = (x, y, w, h)
);

impl_bytemuck!(Rect);

impl_casts!(
    NAME = Rect
    FIELDS = (x, y, w, h)
);

impl_interp!(
    NAME = Rect
    FIELDS = (x, y, w, h)
);

/// Create a [`Rect`].
#[inline]
pub const fn rect<T>(x: T, y: T, w: T, h: T) -> Rect<T> {
    Rect { x, y, w, h }
}

impl<T> Rect<T> {
    /// Create a new rectangle.
    #[inline]
    pub const fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }

    /// Map the rectangle from one type to another.
    #[inline]
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Rect<U> {
        rect(f(self.x), f(self.y), f(self.w), f(self.h))
    }
}

impl<T: Copy> Rect<T> {
    /// The width and height of the rectangle.
    #[inline]
    pub fn size(&self) -> Vec2<T> {
        vec2(self.w, self.h)
    }

    /// The left edge of the rectangle. Equivalent to `x`.
    #[inline]
    pub fn left(&self) -> T {
        self.x
    }

    /// The top edge of the rectangle. Equivalent to `y`.
    #[inline]
    pub fn top(&self) -> T {
        self.y
    }

    /// The top-left point of the rectangle.
    #[inline]
    pub fn top_left(&self) -> Vec2<T> {
        vec2(self.left(), self.top())
    }
}

impl<T: Num> Rect<T> {
    /// A zero-sized rectangle.
    pub const ZERO: Self = rect(T::ZERO, T::ZERO, T::ZERO, T::ZERO);

    /// A rectangle at `(0, 0)` with the provided width and height.
    #[inline]
    pub const fn sized(size: Vec2<T>) -> Self {
        rect(T::ZERO, T::ZERO, size.x, size.y)
    }

    /// A rectangle at `pos` with the `size`.
    #[inline]
    pub const fn pos_size(pos: Vec2<T>, size: Vec2<T>) -> Self {
        rect(pos.x, pos.y, size.x, size.y)
    }

    /// Right edge of the rectangle. Equivalent to `x + w`.
    #[inline]
    pub fn right(&self) -> T {
        self.x + self.w
    }

    /// Bottom edge of the rectangle. Equivalent to `y + h`.
    #[inline]
    pub fn bottom(&self) -> T {
        self.y + self.h
    }

    /// Top-right point of the rectangle.
    #[inline]
    pub fn top_right(&self) -> Vec2<T> {
        vec2(self.right(), self.top())
    }

    /// Bottom-right point of the rectangle.
    #[inline]
    pub fn bottom_right(&self) -> Vec2<T> {
        vec2(self.right(), self.bottom())
    }

    /// Bottom-left point of the rectangle.
    #[inline]
    pub fn bottom_left(&self) -> Vec2<T> {
        vec2(self.left(), self.bottom())
    }

    /// Center x-position of the rectangle.
    #[inline]
    pub fn center_x(&self) -> T {
        self.x + self.w / T::TWO
    }

    /// Cente y-position of the rectangle.
    #[inline]
    pub fn center_y(&self) -> T {
        self.y + self.h / T::TWO
    }

    /// Center point of the rectangle.
    #[inline]
    pub fn center(&self) -> Vec2<T> {
        vec2(self.center_x(), self.center_y())
    }

    /// Center point along the rectangle's top edge.
    #[inline]
    pub fn top_center(&self) -> Vec2<T> {
        vec2(self.center_x(), self.top())
    }

    /// Center point along the rectangle's bottom edge.
    #[inline]
    pub fn bottom_center(&self) -> Vec2<T> {
        vec2(self.center_x(), self.bottom())
    }

    /// Center point along the rectangle's right edge.
    #[inline]
    pub fn right_center(&self) -> Vec2<T> {
        vec2(self.right(), self.center_y())
    }

    /// Center point along the rectangle's left edge.
    #[inline]
    pub fn left_center(&self) -> Vec2<T> {
        vec2(self.left(), self.center_y())
    }

    /// Area of the rectangle.
    #[inline]
    pub fn area(&self) -> T {
        self.w * self.h
    }

    /// Length of the rectangle's perimeter.
    #[inline]
    pub fn perimeter(&self) -> T {
        self.w + self.w + self.h + self.h
    }

    /// Returns true if this rectangle contains the point.
    #[inline]
    pub fn contains(&self, p: Vec2<T>) -> bool {
        p.x >= self.x && p.y >= self.y && p.x < self.right() && p.y < self.bottom()
    }

    /// Translate the rectangle.
    #[inline]
    pub fn translate(&self, amount: &Vec2<T>) -> Self {
        rect(self.x + amount.x, self.y + amount.y, self.w, self.h)
    }

    /// Top edge segment of the rectangle.
    #[inline]
    pub fn top_edge(&self) -> Line<T> {
        line(self.top_left(), self.top_right())
    }

    /// Right edge segment of the rectangle.
    #[inline]
    pub fn right_edge(&self) -> Line<T> {
        line(self.top_right(), self.bottom_right())
    }

    /// Bottom edge segment of the rectangle.
    #[inline]
    pub fn bottom_edge(&self) -> Line<T> {
        line(self.bottom_right(), self.bottom_left())
    }

    /// Left edge segment of the rectangle.
    #[inline]
    pub fn left_edge(&self) -> Line<T> {
        line(self.bottom_left(), self.top_left())
    }

    /// The rectangle's 4 corner points.
    #[inline]
    pub fn corners(&self) -> [Vec2<T>; 4] {
        let r = self.right();
        let b = self.bottom();
        [
            vec2(self.x, self.y),
            vec2(r, self.y),
            vec2(r, b),
            vec2(self.x, b),
        ]
    }

    /// The rectangle's 4 edges.
    #[inline]
    pub fn edges(&self) -> [Line<T>; 4] {
        let [a, b, c, d] = self.corners();
        [line(a, b), line(b, c), line(c, d), line(d, a)]
    }

    /// Inflate the rectangle by the amount.
    #[inline]
    pub fn inflate(self, amount: impl Into<Vec2<T>>) -> Self {
        let amount = amount.into();
        rect(
            self.x - amount.x,
            self.y - amount.y,
            self.w + amount.x + amount.x,
            self.h + amount.y + amount.y,
        )
    }

    /// Absolute left bounds of the rectangle.
    #[inline]
    pub fn min_x(&self) -> T {
        T::min(self.x, self.right())
    }

    /// Absolute top bounds of the rectangle.
    #[inline]
    pub fn min_y(&self) -> T {
        T::min(self.y, self.bottom())
    }

    /// Absolute right bounds of the rectangle.
    #[inline]
    pub fn max_x(&self) -> T {
        T::max(self.x, self.right())
    }

    /// Absolute bottom bounds of the rectangle.
    #[inline]
    pub fn max_y(&self) -> T {
        T::max(self.y, self.bottom())
    }

    /// Absolute top-left point of the rectangle.
    #[inline]
    pub fn min_pos(&self) -> Vec2<T> {
        vec2(self.min_x(), self.min_y())
    }

    /// Absolute bottom-right point of the rectangle.
    #[inline]
    pub fn max_pos(&self) -> Vec2<T> {
        vec2(self.max_x(), self.max_y())
    }

    /// If this rectangle contains the other.
    #[inline]
    pub fn contains_rect(&self, r: &Self) -> bool {
        r.x >= self.x && r.y >= self.y && r.right() <= self.right() && r.bottom() <= self.bottom()
    }

    /// If this rectangle overlaps the other.
    #[inline]
    pub fn overlaps(&self, r: &Self) -> bool {
        self.x < r.right() && self.y < r.bottom() && self.right() > r.x && self.bottom() > r.y
    }

    /// If this rectangle overlaps the other, returns a rectangle
    /// representing the overlapping region.
    #[inline]
    pub fn overlap(&self, r: &Self) -> Option<Self> {
        let min = self.top_left().max(r.top_left());
        let max = self.bottom_right().min(r.bottom_right());
        if max.x > min.x && max.y > min.y {
            Some(rect(min.x, min.y, max.x - min.x, max.y - min.y))
        } else {
            None
        }
    }

    /// Return a rectangle that minimally encapsulates this rectangle and the other.
    /// This is useful if you have a lot of rectangles (or bounds) and want to find
    /// the minimal sum boundary that contains them all.
    #[inline]
    pub fn conflate(&self, r: &Self) -> Self {
        let min = self.min_pos().min(r.min_pos());
        let max = self.max_pos().max(r.max_pos());
        rect(min.x, min.y, max.x - min.x, max.y - min.y)
    }

    /// Return a rectangle that is this rectangle clamped inside of the provided
    /// outer rectangle. If this rectangle is larger than the outer by either
    /// dimension, it will be shrunk to fit.
    pub fn clamp_inside(&self, outer: &Self) -> Self {
        let mut rect = *self;
        if rect.right() > outer.right() {
            rect.x -= rect.right() - outer.right();
        }
        if rect.bottom() > outer.bottom() {
            rect.y -= rect.bottom() - outer.bottom();
        }
        if rect.x < outer.x {
            rect.w -= outer.x - rect.x;
            rect.x = outer.x;
        }
        if rect.y < outer.y {
            rect.h -= outer.y - rect.y;
            rect.y = outer.y;
        }
        rect
    }
}

impl<T: Signed> Rect<T> {
    /// If the rectangle has a non-negative size.
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.w >= T::ZERO && self.h >= T::ZERO
    }

    /// If the rectangle has a negative width or height, invert it on those
    /// axes and return a corrected version with non-negative dimensions
    #[inline]
    pub fn non_neg(mut self) -> Self {
        if self.w < T::ZERO {
            self.x += self.w;
            self.w = -self.w;
        }
        if self.h < T::ZERO {
            self.y += self.h;
            self.h = -self.h;
        }
        self
    }
}

impl<T: Float> Rect<T> {
    /// Transform the rectangle, producing a quad.
    #[inline]
    pub fn transform_by(&self, f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Quad<T> {
        Quad(self.corners().map(f))
    }

    /// Transform the rectangle, but retain the type.
    #[inline]
    pub fn transform_by_retain(&self, mut f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Rect<T> {
        //let center = self.transform_by(f).centroid();
        //self.transform_by(f).bounds()
        //let orig = f(Vec2::ZERO);
        self + f(Vec2::ZERO)
    }
}

impl<T: Display> Display for Rect<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(", ")?;
        self.w.fmt(f)?;
        f.write_str(", ")?;
        self.h.fmt(f)
    }
}

impl<T: Float> Shape<T> for Rect<T> {
    #[inline]
    fn centroid(&self) -> Vec2<T> {
        self.center()
    }

    #[inline]
    fn contains(&self, p: Vec2<T>) -> bool {
        p.x >= self.x && p.y >= self.y && p.x < self.right() && p.y < self.bottom()
    }

    #[inline]
    fn bounds(&self) -> Rect<T> {
        *self
    }

    #[inline]
    fn project_onto_axis(&self, axis: Vec2<T>) -> Projection<T> {
        let dot1 = self.top_left().dot(axis);
        let dot2 = self.top_right().dot(axis);
        let dot3 = self.bottom_right().dot(axis);
        let dot4 = self.bottom_left().dot(axis);
        let min = T::min(dot1, T::min(dot2, T::min(dot3, dot4)));
        let max = T::max(dot1, T::max(dot2, T::max(dot3, dot4)));
        Projection { min, max }
    }

    #[inline]
    fn project_point(&self, p: Vec2<T>) -> Vec2<T> {
        let projections = [
            self.top_edge().project_point(p),
            self.right_edge().project_point(p),
            self.bottom_edge().project_point(p),
            self.left_edge().project_point(p),
        ];
        // Find which edge projected point is nearest to the origin.
        let (i, _) = projections
            .iter()
            .enumerate()
            .map(|(i, proj)| (i, proj.sqr_dist(p)))
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        projections[i]
    }

    #[inline]
    fn rayhit(&self, ray: &Ray<T>) -> bool {
        self.edges().iter().any(|edge| edge.raycast(ray).is_some())
    }

    fn raycast(&self, ray: &Ray<T>) -> Option<RayHit<T>> {
        // Raycast against each of our edges.
        let edges = self.edges();
        let hit_dists = [
            edges[0].raycast(ray).map(|d| (0, d)),
            edges[1].raycast(ray).map(|d| (1, d)),
            edges[2].raycast(ray).map(|d| (2, d)),
            edges[3].raycast(ray).map(|d| (3, d)),
        ];

        // TODO: come back to this, need to find out if edge raycasts are 2-sided?? this seems weird
        let crossings = hit_dists.iter().flatten().count();
        if crossings > 0 && (crossings % 2) == 0 {
            hit_dists
                .iter()
                .flatten()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|&(i, d)| RayHit::new(edges[i].norm().turn_left(), d))
        } else {
            None
        }
    }

    #[inline]
    fn overlaps_rect(&self, rect: &Rect<T>) -> bool {
        self.x < rect.right()
            && self.y < rect.bottom()
            && self.right() > rect.x
            && self.bottom() > rect.y
    }

    #[inline]
    fn overlaps_circ(&self, circ: &Circle<T>) -> bool {
        circ.overlaps_poly(self)
    }

    #[inline]
    fn overlaps_poly<P: Polygonal<T>>(&self, poly: &P) -> bool {
        self.all_normals(|axis| overlaps_on(self, poly, axis))
            && poly.all_normals(|axis| overlaps_on(self, poly, axis))
    }

    #[inline]
    fn extract_from_circ(&self, circ: &Circle<T>) -> Option<Vec2<T>> {
        circ.extract_from_poly(self).map(|p| -p)
    }

    #[inline]
    fn extract_from_poly<P: Polygonal<T>>(&self, poly: &P) -> Option<Vec2<T>> {
        let mut dist = T::MAX;
        let mut dir = Vec2::ZERO;
        (self.all_normals(|axis| extract_on(self, poly, axis, &mut dist, &mut dir))
            && poly.all_normals(|axis| extract_on(self, poly, axis, &mut dist, &mut dir)))
        .then(|| dir * dist)
    }

    #[inline]
    fn is_convex(&self) -> bool {
        self.w > T::ZERO && self.h > T::ZERO
    }
}

impl<T: Float> Polygonal<T> for Rect<T> {
    #[inline]
    fn nearest_vertex(&self, source: Vec2<T>) -> Vec2<T> {
        self.corners().nearest_vertex(source)
    }

    #[inline]
    fn all_edges<F: FnMut(Line<T>) -> bool>(&self, mut cond: F) -> bool {
        cond(self.right_edge())
            && cond(self.bottom_edge())
            && cond(self.left_edge())
            && cond(self.top_edge())
    }

    #[inline]
    fn all_normals<F: FnMut(Vec2<T>) -> bool>(&self, mut cond: F) -> bool {
        cond(Vec2::RIGHT) && cond(Vec2::DOWN) && cond(Vec2::LEFT) && cond(Vec2::UP)
    }

    #[inline]
    fn visit_normals<F: FnMut(Vec2<T>)>(&self, mut plot: F) {
        plot(Vec2::RIGHT);
        plot(Vec2::DOWN);
        plot(Vec2::LEFT);
        plot(Vec2::UP);
    }
}

impl<T: Float> Rect<T> {
    pub fn fitted(&self, size: Vec2<T>, fractional: bool) -> (Self, T) {
        let scale = self.size() / size;
        let mut scale = T::min(scale.x, scale.y);
        if !fractional && scale > T::ONE {
            scale = T::floor(scale);
        }
        let new_size = size * scale;
        let pos = self.top_left() + ((self.size() - new_size) * T::HALF).floor();
        let rect = Rect::pos_size(pos, new_size);
        (rect, scale)
    }

    pub fn map_pos(&self, pos: Vec2<T>, target: &Rect<T>) -> Vec2<T> {
        target.min_pos() + target.size() * ((pos - self.min_pos()) / self.size().abs())
    }
}

// ---------- ADD ----------

impl<T: Add<T, Output = T>> Add<Vec2<T>> for Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        rect(self.x + rhs.x, self.y + rhs.y, self.w, self.h)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<Vec2<T>> for &Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        rect(self.x + rhs.x, self.y + rhs.y, self.w, self.h)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Vec2<T>> for Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        rect(self.x + rhs.x, self.y + rhs.y, self.w, self.h)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Vec2<T>> for &Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        rect(self.x + rhs.x, self.y + rhs.y, self.w, self.h)
    }
}

// ---------- ADD ASSIGN ----------

impl<T: AddAssign<T>> AddAssign<Vec2<T>> for Rect<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<&Vec2<T>> for Rect<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// ---------- SUB ----------

impl<T: Sub<T, Output = T>> Sub<Vec2<T>> for Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        rect(self.x - rhs.x, self.y - rhs.y, self.w, self.h)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<Vec2<T>> for &Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        rect(self.x - rhs.x, self.y - rhs.y, self.w, self.h)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<&Vec2<T>> for Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        rect(self.x - rhs.x, self.y - rhs.y, self.w, self.h)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<&Vec2<T>> for &Rect<T> {
    type Output = Rect<T>;

    #[inline]
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        rect(self.x - rhs.x, self.y - rhs.y, self.w, self.h)
    }
}

// ---------- SUB ASSIGN ----------

impl<T: SubAssign<T>> SubAssign<Vec2<T>> for Rect<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<&Vec2<T>> for Rect<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// ---------- SCALAR/VEC2 OPS ----------

macro_rules! impl_ops {
    ($op:ident $op_fn:ident $assign:ident $assign_fn:ident) => {
        impl<T: Num> std::ops::$op<T> for Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $op_fn(self, rhs: T) -> Self::Output {
                rect(
                    self.x.$op_fn(rhs),
                    self.y.$op_fn(rhs),
                    self.w.$op_fn(rhs),
                    self.h.$op_fn(rhs),
                )
            }
        }

        impl<T: Num> std::ops::$op<T> for &Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $op_fn(self, rhs: T) -> Self::Output {
                rect(
                    self.x.$op_fn(rhs),
                    self.y.$op_fn(rhs),
                    self.w.$op_fn(rhs),
                    self.h.$op_fn(rhs),
                )
            }
        }

        impl<T: Num> std::ops::$op<Vec2<T>> for Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $op_fn(self, rhs: Vec2<T>) -> Self::Output {
                rect(
                    self.x.$op_fn(rhs.x),
                    self.y.$op_fn(rhs.x),
                    self.w.$op_fn(rhs.y),
                    self.h.$op_fn(rhs.y),
                )
            }
        }

        impl<T: Num> std::ops::$op<Vec2<T>> for &Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $op_fn(self, rhs: Vec2<T>) -> Self::Output {
                rect(
                    self.x.$op_fn(rhs.x),
                    self.y.$op_fn(rhs.x),
                    self.w.$op_fn(rhs.y),
                    self.h.$op_fn(rhs.y),
                )
            }
        }

        impl<T: Num> std::ops::$assign<T> for Rect<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: T) {
                self.x.$assign_fn(rhs);
                self.y.$assign_fn(rhs);
                self.w.$assign_fn(rhs);
                self.h.$assign_fn(rhs);
            }
        }
    };
}

impl_ops!(Mul mul MulAssign mul_assign);
impl_ops!(Div div DivAssign div_assign);
impl_ops!(Rem rem RemAssign rem_assign);
