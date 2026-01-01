use crate::{
    Angle, Direction, Float, Line, Num, Polygon, Polygonal, Projection, Radians, Ray, RayHit, Rect,
    Shape, Vec2, extract_on, impl_approx, impl_casts, line, overlaps_on, rect, vec2,
};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub type CircleF = Circle<f32>;
pub type CircleI = Circle<i32>;

/// A circle, represented by a center point and radius.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Circle<T> {
    pub center: Vec2<T>,
    pub radius: T,
}

impl_approx!(
    NAME = Circle
    FIELDS = (center, radius)
);

impl_casts!(
    NAME = Circle
    FIELDS = (center, radius)
);

/// Create a [`Circle`].
#[inline]
pub const fn circle<T>(center: Vec2<T>, radius: T) -> Circle<T> {
    Circle { center, radius }
}

impl<T> Circle<T> {
    /// Create a new circle.
    #[inline]
    pub const fn new(center: Vec2<T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<T: Num> Circle<T> {
    /// A circle at `(0, 0)` with radius `0`.
    pub const ZERO: Self = circle(Vec2::ZERO, T::ZERO);

    /// A unit circle at `(0, 0)` with radius `1`.
    pub const UNIT: Self = circle(Vec2::ZERO, T::ONE);

    /// Create a circle at `(0, 0)` with the provided radius.
    #[inline]
    pub fn with_radius(radius: T) -> Self {
        circle(Vec2::ZERO, radius)
    }

    /// Diameter of the circle.
    #[inline]
    pub fn diameter(&self) -> T {
        self.radius + self.radius
    }

    /// Returns true if the two circles overlap.
    #[inline]
    pub fn overlaps(&self, other: &Circle<T>) -> bool {
        let sqr_dist = (self.center - other.center).sqr_len();
        let sqr_rad = (self.radius + other.radius) * (self.radius + other.radius);
        sqr_dist < sqr_rad
    }
}

impl<T: Float> Circle<T> {
    /// Area of the circle.
    #[inline]
    pub fn area(&self) -> T {
        T::PI * (self.radius * self.radius)
    }

    /// Circumference of the circle.
    #[inline]
    pub fn circumference(&self) -> T {
        T::TAU * self.radius
    }

    /// Check if a circle is fully contained within this one.
    #[inline]
    pub fn contains_circ(&self, other: &Circle<T>) -> bool {
        self.center.dist(other.center) + other.radius <= self.radius
    }

    /// Plot `count` points evenly distributed along the perimeter of the circle.
    pub fn hull_points_n(&self, mut count: T, angle: impl Angle<T>, mut plot: impl FnMut(Vec2<T>)) {
        let step = Radians(T::TAU / count);
        let mut angle = angle.to_radians();
        while count > T::ZERO {
            plot(self.center + angle.norm() * self.radius);
            angle = angle + step;
            count = count - T::ONE;
        }
    }

    /// Plot points along the perimeter of the circle, each `seg_len` apart.
    #[inline]
    pub fn hull_points(&self, seg_len: T, angle: impl Angle<T>, plot: impl FnMut(Vec2<T>)) {
        let count = T::max(self.circumference() / seg_len, T::THREE);
        self.hull_points_n(count, angle, plot)
    }

    /// Iterate over `count` points evenly distributed along the perimeter of the circle.
    pub fn iter_hull_points_n(
        &self,
        mut count: T,
        angle: impl Angle<T>,
    ) -> impl Iterator<Item = Vec2<T>> + '_ {
        let step = Radians(T::TAU / count);
        let mut angle = angle.to_radians();
        std::iter::from_fn(move || {
            (count > T::ZERO).then(|| {
                let p = self.center + angle.norm() * self.radius;
                angle = angle + step;
                count = count - T::ONE;
                p
            })
        })
    }

    /// Iterate over points along the perimeter of the circle, each `seg_len` apart.
    #[inline]
    pub fn iter_hull_points(
        &self,
        seg_len: T,
        angle: impl Angle<T>,
    ) -> impl Iterator<Item = Vec2<T>> + '_ {
        let count = T::max(self.circumference() / seg_len, T::THREE);
        self.iter_hull_points_n(count, angle)
    }

    /// Plot `count` edges evenly distributed along the circle's perimeter.
    pub fn hull_edges_n(&self, mut count: T, angle: impl Angle<T>, mut plot: impl FnMut(Line<T>)) {
        let step = Radians(T::TAU / count);
        let mut angle = angle.to_radians();
        let mut prev = self.center + angle.norm() * self.radius;
        while count > T::ZERO {
            angle = angle + step;
            let curr = self.center + angle.norm() * self.radius;
            plot(line(prev, curr));
            prev = curr;
            count = count - T::ONE;
        }
    }

    /// Get `count` edges evenly distributed along the circle's perimeter.
    #[inline]
    pub fn hull_edges(&self, seg_len: T, angle: impl Angle<T>, plot: impl FnMut(Line<T>)) {
        let count = T::max(self.circumference() / seg_len, T::THREE);
        self.hull_edges_n(count, angle, plot)
    }

    /// Iterate over `count` edges evenly distributed along the circle's perimeter.
    pub fn iter_hull_edges_n(
        &self,
        mut count: T,
        angle: impl Angle<T>,
    ) -> impl Iterator<Item = Line<T>> + '_ {
        let step = Radians(T::TAU / count);
        let mut angle = angle.to_radians();
        let mut prev = self.center + angle.norm() * self.radius;
        std::iter::from_fn(move || {
            (count > T::ZERO).then(|| {
                angle = angle + step;
                let curr = self.center + angle.norm() * self.radius;
                let line = line(prev, curr);
                prev = curr;
                count = count - T::ONE;
                line
            })
        })
    }

    #[inline]
    pub fn iter_hull_edges(
        &self,
        seg_len: T,
        angle: impl Angle<T>,
    ) -> impl Iterator<Item = Line<T>> + '_ {
        let count = T::max(self.circumference() / seg_len, T::THREE);
        self.iter_hull_edges_n(count, angle)
    }

    #[inline]
    fn raycast_len(&self, ray: &Ray<T>) -> Option<T> {
        let diff = self.center - ray.origin;
        let sqr_dist = diff.sqr_len();

        let radius = self.radius * self.radius;
        if sqr_dist < radius {
            return None;
        }

        let dist_along_ray = ray.direction.dot(diff);
        if dist_along_ray < T::ZERO {
            return None;
        }

        let dist = radius + dist_along_ray * dist_along_ray - sqr_dist;
        if dist < T::ZERO {
            return None;
        }

        Some(dist_along_ray - T::sqrt(dist))
    }

    #[inline]
    pub fn transform_by_into(
        &self,
        seg_len: T,
        angle: impl Angle<T>,
        into: &mut Polygon<T>,
        f: impl FnMut(Vec2<T>) -> Vec2<T>,
    ) {
        let count = T::max(self.circumference() / seg_len, T::THREE);
        self.transform_by_into_n(count, angle, into, f);
    }

    #[inline]
    pub fn transform_by_into_n(
        &self,
        seg_count: T,
        angle: impl Angle<T>,
        into: &mut Polygon<T>,
        mut f: impl FnMut(Vec2<T>) -> Vec2<T>,
    ) {
        into.clear();
        into.reserve(seg_count.to_usize() + 1);
        self.hull_points_n(seg_count, angle, |p| {
            into.push(f(p));
        });
    }

    #[inline]
    pub fn transform_by(
        &self,
        seg_len: T,
        angle: impl Angle<T>,
        f: impl FnMut(Vec2<T>) -> Vec2<T>,
    ) -> Polygon<T> {
        let mut poly = Polygon::new();
        self.transform_by_into(seg_len, angle, &mut poly, f);
        poly
    }

    #[inline]
    pub fn transform_by_n(
        &self,
        count: T,
        angle: impl Angle<T>,
        f: impl FnMut(Vec2<T>) -> Vec2<T>,
    ) -> Polygon<T> {
        let mut poly = Polygon::new();
        self.transform_by_into_n(count, angle, &mut poly, f);
        poly
    }

    #[inline]
    pub fn transform_by_retain(&self, f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Self {
        let quad = self.bounds().corners().map(f);
        let ab = quad[0].dist(quad[1]);
        let bc = quad[1].dist(quad[2]);
        Self::new(quad.centroid(), T::min(ab, bc) * T::HALF)
    }

    #[inline]
    pub fn suggest_seg_count(&self) -> T {
        T::max(T::floor(T::sqrt(self.diameter()) * T::PI), T::THREE)
    }

    #[inline]
    pub fn suggest_seg_count_f(&self, f: impl FnMut(Vec2<T>) -> Vec2<T>) -> T {
        let quad = self.bounds().corners().map(f);
        let ab = quad[0].dist(quad[1]);
        let bc = quad[1].dist(quad[2]);
        let diam = T::min(ab, bc);
        T::max(T::floor(T::sqrt(diam) * T::PI), T::THREE)
    }
}

impl<T: Float> Shape<T> for Circle<T> {
    #[inline]
    fn centroid(&self) -> Vec2<T> {
        self.center
    }

    #[inline]
    fn contains(&self, p: Vec2<T>) -> bool {
        let diff = self.center - p;
        diff.sqr_len() < (self.radius * self.radius)
    }

    #[inline]
    fn bounds(&self) -> Rect<T> {
        let diam = self.diameter();
        rect(
            self.center.x - self.radius,
            self.center.y - self.radius,
            diam,
            diam,
        )
    }

    #[inline]
    fn project_onto_axis(&self, axis: Vec2<T>) -> Projection<T> {
        let min = axis.dot(self.center - axis * self.radius);
        let max = axis.dot(self.center + axis * self.radius);
        Projection { min, max }
    }

    #[inline]
    fn project_point(&self, p: Vec2<T>) -> Vec2<T> {
        if self.center == p {
            p
        } else {
            self.center + (p - self.center).norm() * self.radius
        }
    }

    #[inline]
    fn rayhit(&self, ray: &Ray<T>) -> bool {
        self.raycast_len(ray).is_some()
    }

    #[inline]
    fn raycast(&self, ray: &Ray<T>) -> Option<RayHit<T>> {
        self.raycast_len(ray).map(|distance| {
            let normal = (ray.point(distance) - self.center).norm();
            RayHit { normal, distance }
        })
    }

    #[inline]
    fn overlaps_rect(&self, rect: &Rect<T>) -> bool {
        overlaps_on(self, rect, Vec2::X_AXIS)
            && overlaps_on(self, rect, Vec2::Y_AXIS)
            && overlaps_on(
                self,
                rect,
                (rect.nearest_vertex(self.center) - self.center).norm(),
            )
    }

    #[inline]
    fn overlaps_circ(&self, circ: &Circle<T>) -> bool {
        let sqr_dist = (self.center - circ.center).sqr_len();
        let sqr_rad = (self.radius + circ.radius) * (self.radius + circ.radius);
        sqr_dist < sqr_rad
    }

    #[inline]
    fn overlaps_poly<P: Polygonal<T>>(&self, poly: &P) -> bool {
        poly.all_normals(|axis| overlaps_on(self, poly, axis))
            && overlaps_on(
                self,
                poly,
                (poly.nearest_vertex(self.center) - self.center).norm(),
            )
    }

    fn extract_from_circ(&self, circ: &Circle<T>) -> Option<Vec2<T>> {
        let offset = self.center - circ.center;
        let sqr_dist = offset.sqr_len();
        let sqr_rad = (self.radius + circ.radius) * (self.radius + circ.radius);
        (sqr_dist < sqr_rad).then(|| {
            let d = T::sqrt(sqr_rad) - T::sqrt(sqr_dist);
            offset.norm() * d
        })
    }

    fn extract_from_poly<P: Polygonal<T>>(&self, poly: &P) -> Option<Vec2<T>> {
        let mut dist = T::MAX;
        let mut dir = Vec2::ZERO;
        (poly.all_normals(|axis| extract_on(self, poly, axis, &mut dist, &mut dir))
            && extract_on(
                self,
                poly,
                (self.center - poly.nearest_vertex(self.center)).norm(),
                &mut dist,
                &mut dir,
            ))
        .then(|| dir * dist)
    }

    #[inline]
    fn is_convex(&self) -> bool {
        self.radius > T::ZERO
    }
}

// ---------- ADD ----------

impl<T: Add<T, Output = T>> Add<Vec2<T>> for Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        circle(self.center + rhs, self.radius)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<Vec2<T>> for &Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        circle(self.center + rhs, self.radius)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Vec2<T>> for Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        circle(self.center + rhs, self.radius)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Vec2<T>> for &Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        circle(self.center + rhs, self.radius)
    }
}

// ---------- ADD ASSIGN ----------

impl<T: AddAssign<T>> AddAssign<Vec2<T>> for Circle<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.center += rhs;
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<&Vec2<T>> for Circle<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec2<T>) {
        self.center += rhs;
    }
}

// ---------- SUB ----------

impl<T: Sub<T, Output = T>> Sub<Vec2<T>> for Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        circle(self.center - rhs, self.radius)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<Vec2<T>> for &Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        circle(self.center - rhs, self.radius)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<&Vec2<T>> for Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        circle(self.center - rhs, self.radius)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub<&Vec2<T>> for &Circle<T> {
    type Output = Circle<T>;

    #[inline]
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        circle(self.center - rhs, self.radius)
    }
}

// ---------- SUB ASSIGN ----------

impl<T: Copy + SubAssign<T>> SubAssign<Vec2<T>> for Circle<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.center -= rhs;
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<&Vec2<T>> for Circle<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec2<T>) {
        self.center -= rhs;
    }
}

// ---------- FROM ----------

impl<T> From<(Vec2<T>, T)> for Circle<T> {
    #[inline]
    fn from((center, radius): (Vec2<T>, T)) -> Self {
        Self { center, radius }
    }
}

impl<T> From<(T, T, T)> for Circle<T> {
    #[inline]
    fn from((x, y, radius): (T, T, T)) -> Self {
        Self::new(vec2(x, y), radius)
    }
}
