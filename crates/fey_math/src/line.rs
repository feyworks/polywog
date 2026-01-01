use crate::{
    Float, Num, Projection, Ray, Rect, Vec2, impl_approx, impl_bytemuck, impl_casts, impl_interp,
    impl_serde, rect, vec2,
};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub type LineF = Line<f32>;
pub type LineI = Line<i32>;
pub type LineU = Line<u32>;

/// A line segment connecting two points.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Line<T> {
    /// The start point.
    pub start: Vec2<T>,

    /// The end point.
    pub end: Vec2<T>,
}

impl_bytemuck!(Line);

impl_serde!(
    NAME = Line
    FIELDS = (start, end)
);

impl_approx!(
    NAME = Line
    FIELDS = (start, end)
);

impl_casts!(
    NAME = Line
    FIELDS = (start, end)
);

impl_interp!(
    NAME = Line
    FIELD_TY = Vec2
    FIELDS = (start, end)
);

/// Create a [`Line`].
#[inline]
pub const fn line<T>(start: Vec2<T>, end: Vec2<T>) -> Line<T> {
    Line { start, end }
}

impl<T> Line<T> {
    /// Create a new line.
    #[inline]
    pub const fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        line(start, end)
    }

    /// The points of the line.
    #[inline]
    pub fn points(self) -> [Vec2<T>; 2] {
        [self.start, self.end]
    }

    /// Reverse the line (swap the start and end points).
    #[inline]
    pub fn rev(self) -> Self {
        line(self.end, self.start)
    }
}

impl<T: Num> Line<T> {
    /// A line with both `start` and `end` at `(0, 0)`.
    pub const ZERO: Self = line(Vec2::ZERO, Vec2::ZERO);

    /// The vector from the line's start to end point.
    pub fn vector(&self) -> Vec2<T> {
        self.end - self.start
    }

    /// Squared length of the line.
    #[inline]
    pub fn sqr_len(&self) -> T {
        self.vector().sqr_len()
    }

    /// Rectangular bounds of the line.
    #[inline]
    pub fn bounds(&self) -> Rect<T> {
        let min = self.start.min(self.end);
        let max = self.start.max(self.end);
        rect(min.x, min.y, max.x, max.y)
    }

    /// X-position of the line's center.
    #[inline]
    pub fn center_x(&self) -> T {
        self.start.x + (self.end.x - self.start.x) / T::TWO
    }

    /// Y-position of the line's center.
    #[inline]
    pub fn center_y(&self) -> T {
        self.start.y + (self.end.y - self.start.y) / T::TWO
    }

    /// The line's center.
    #[inline]
    pub fn center(&self) -> Vec2<T> {
        self.start + self.vector() / T::TWO
    }
}

impl<T: Float> Line<T> {
    /// Length of the line.
    #[inline]
    pub fn len(&self) -> T {
        self.vector().len()
    }

    /// Axis of the line from `a` to `b`.
    #[inline]
    pub fn norm(&self) -> Vec2<T> {
        self.vector().norm()
    }

    /// Left-perpendicular axis of the line.
    #[inline]
    pub fn left_norm(&self) -> Vec2<T> {
        self.norm().turn_left()
    }

    /// Right-perpendicular axis of the line.
    #[inline]
    pub fn right_norm(&self) -> Vec2<T> {
        self.norm().turn_right()
    }

    /// Project this line onto the provided axis.
    #[inline]
    pub fn project_onto_axis(&self, axis: Vec2<T>) -> Projection<T> {
        let a = self.start.dot(axis);
        let b = self.end.dot(axis);
        let (min, max) = T::min_max(a, b);
        Projection { min, max }
    }

    /// Project the point onto this line.
    #[inline]
    pub fn project_point(&self, p: Vec2<T>) -> Vec2<T> {
        let norm = self.norm();
        self.start + norm * p.dot(norm)
    }

    /// Check if the ray hits this line.
    #[inline]
    pub fn rayhit(&self, ray: &Ray<T>) -> bool {
        // TODO: can we skip some of the math to just get confirmation?
        self.raycast(ray).is_some()
    }

    /// Cast a ray against this line. If it intersects the line, return
    /// the distance along the ray that the intersection occurred.
    #[inline]
    pub fn raycast(&self, ray: &Ray<T>) -> Option<T> {
        let perp = self.vector().turn_left();
        let dot = ray.direction.dot(perp);
        let n0 = T::ZERO;
        if dot != n0 {
            let a = self.start - ray.origin;
            let t = perp.dot(a) / dot;
            let s = ray.direction.turn_left().dot(a) / dot;
            (t >= n0 && s >= n0 && s <= T::ONE).then(|| t)
        } else {
            None
        }
    }

    #[inline]
    pub fn transform_by(&self, mut f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Self {
        Self::new(f(self.start), f(self.end))
    }
}

impl<T> From<(T, T, T, T)> for Line<T> {
    #[inline]
    fn from((x1, y1, x2, y2): (T, T, T, T)) -> Self {
        line(vec2(x1, y1), vec2(x2, y2))
    }
}

impl<T> From<(Vec2<T>, Vec2<T>)> for Line<T> {
    #[inline]
    fn from((start, end): (Vec2<T>, Vec2<T>)) -> Self {
        Self { start, end }
    }
}

impl<T> From<Line<T>> for (Vec2<T>, Vec2<T>) {
    #[inline]
    fn from(Line { start, end }: Line<T>) -> Self {
        (start, end)
    }
}

impl<T> From<[Vec2<T>; 2]> for Line<T> {
    #[inline]
    fn from([start, end]: [Vec2<T>; 2]) -> Self {
        Self { start, end }
    }
}

impl<T> From<Line<T>> for [Vec2<T>; 2] {
    #[inline]
    fn from(Line { start, end }: Line<T>) -> Self {
        [start, end]
    }
}

// ---------- ADD ----------

impl<T: Copy + Add<T, Output = T>> Add<Vec2<T>> for Line<T> {
    type Output = Line<T>;

    #[inline]
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        line(self.start + rhs, self.end + rhs)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<Vec2<T>> for &Line<T> {
    type Output = Line<T>;

    #[inline]
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        line(self.start + rhs, self.end + rhs)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Vec2<T>> for Line<T> {
    type Output = Line<T>;

    #[inline]
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        line(self.start + rhs, self.end + rhs)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Vec2<T>> for &Line<T> {
    type Output = Line<T>;

    #[inline]
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        line(self.start + rhs, self.end + rhs)
    }
}

// ---------- ADD ASSIGN ----------

impl<T: Copy + AddAssign<T>> AddAssign<Vec2<T>> for Line<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.start += rhs;
        self.end += rhs;
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<&Vec2<T>> for Line<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec2<T>) {
        self.start += rhs;
        self.end += rhs;
    }
}

// ---------- SUB ----------

impl<T: Copy + Sub<T, Output = T>> Sub<Vec2<T>> for Line<T> {
    type Output = Line<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        line(self.start - rhs, self.end - rhs)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<Vec2<T>> for &Line<T> {
    type Output = Line<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        line(self.start - rhs, self.end - rhs)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<&Vec2<T>> for Line<T> {
    type Output = Line<T>;

    #[inline]
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        line(self.start - rhs, self.end - rhs)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<&Vec2<T>> for &Line<T> {
    type Output = Line<T>;

    #[inline]
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        line(self.start - rhs, self.end - rhs)
    }
}

// ---------- SUB ASSIGN ----------

impl<T: Copy + SubAssign<T>> SubAssign<Vec2<T>> for Line<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.start -= rhs;
        self.end -= rhs;
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<&Vec2<T>> for Line<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec2<T>) {
        self.start -= rhs;
        self.end -= rhs;
    }
}
