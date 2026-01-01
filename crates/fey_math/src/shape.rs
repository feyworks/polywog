use crate::{Circle, Float, Line, Projection, Ray, RayHit, Rect, Vec2, line, rect};

/// A type that represents a convex 2D shape.
pub trait Shape<T> {
    /// Centroid of the shape.
    fn centroid(&self) -> Vec2<T>;

    /// If the point is contained within the shape.
    fn contains(&self, p: Vec2<T>) -> bool;

    /// Rectangular bounds of the shape.
    fn bounds(&self) -> Rect<T>;

    /// Project the shape onto the axis.
    fn project_onto_axis(&self, axis: Vec2<T>) -> Projection<T>;

    /// Project a point onto the outside surface of the shape.
    fn project_point(&self, p: Vec2<T>) -> Vec2<T>;

    /// Check if a ray intersects this shape.
    fn rayhit(&self, ray: &Ray<T>) -> bool;

    /// Raycast against the shape.
    fn raycast(&self, ray: &Ray<T>) -> Option<RayHit<T>>;

    /// If this shape overlaps the rectangle.
    fn overlaps_rect(&self, rect: &Rect<T>) -> bool;

    /// If this shape overlaps the circle.
    fn overlaps_circ(&self, circ: &Circle<T>) -> bool;

    /// If this shape overlaps the polygon.
    fn overlaps_poly<P: Polygonal<T>>(&self, poly: &P) -> bool;

    /// If this shape and the circle overlap, return a push-out
    /// vector that can be used to extract them from each other.
    fn extract_from_circ(&self, circ: &Circle<T>) -> Option<Vec2<T>>;

    /// If this shape and the polygon overlap, return a push-out
    /// vector that can be used to extract them from each other.
    fn extract_from_poly<P: Polygonal<T>>(&self, poly: &P) -> Option<Vec2<T>>;

    /// If this shape is convex.
    fn is_convex(&self) -> bool;
}

/// A type that represents a convex 2D polygonal shape.
pub trait Polygonal<T>: Shape<T> {
    /// Get the nearest point on this polygon to the source point.
    fn nearest_vertex(&self, source: Vec2<T>) -> Vec2<T>;

    /// Iterate all polygonal edges of the shape, returning true if
    /// any of them satisfy the conditional function provided.
    fn all_edges<F: FnMut(Line<T>) -> bool>(&self, cond: F) -> bool;

    /// Iterate all edge normals of the shape, returning true if
    /// any of them satisfy the conditional function provided.
    fn all_normals<F: FnMut(Vec2<T>) -> bool>(&self, cond: F) -> bool;

    /// Walk through every edge of the polygon.
    #[inline]
    fn visit_edges<F: FnMut(Line<T>)>(&self, mut plot: F) {
        self.all_edges(|edge| {
            plot(edge);
            true
        });
    }

    /// Walk through every normal of the polygon.
    fn visit_normals<F: FnMut(Vec2<T>)>(&self, plot: F);
}

/// Check if two shapes overlap on the provided axis.
#[inline]
pub(crate) fn overlaps_on<T: Float, A: Shape<T>, B: Shape<T>>(a: &A, b: &B, axis: Vec2<T>) -> bool {
    let a = a.project_onto_axis(axis);
    let b = b.project_onto_axis(axis);
    a.overlaps(b)
}

/// If the two shapes overlap on the provided axis. This will assign the
/// push-out distance and direction if the calculated push-out is shorter.
#[inline]
pub(crate) fn extract_on<T: Float, A: Shape<T>, B: Shape<T>>(
    a: &A,
    b: &B,
    axis: Vec2<T>,
    push_dist: &mut T,
    push_dir: &mut Vec2<T>,
) -> bool {
    let a = a.project_onto_axis(axis);
    let b = b.project_onto_axis(axis);
    if a.min < b.max && a.max > b.min {
        let dist = if T::abs(b.max - a.min) < T::abs(a.max - b.min) {
            b.max - a.min
        } else {
            b.min - a.max
        };
        if T::abs(dist) < T::abs(*push_dist) {
            *push_dist = dist;
            *push_dir = axis;
        }
        true
    } else {
        false
    }
}

impl<T: Float, S: AsRef<[Vec2<T>]>> Shape<T> for S {
    fn centroid(&self) -> Vec2<T> {
        let arr = self.as_ref();
        let mut centroid = Vec2::ZERO;
        let mut signed_area = T::ZERO;
        for i in 0..arr.len() {
            let a = arr[i];
            let b = arr[(i + 1) % arr.len()];
            let area = a.x * b.y - b.x * a.y;
            centroid += (a + b) * area;
            signed_area += area;
        }
        centroid / (signed_area * T::THREE)
    }

    fn contains(&self, p: Vec2<T>) -> bool {
        let arr = self.as_ref();
        for i in 0..arr.len() {
            let a = arr[i];
            let b = arr[(i + 1) % arr.len()];
            if (b - a).cross(p - a) <= T::ZERO {
                return false;
            }
        }
        true
    }

    fn bounds(&self) -> Rect<T> {
        let arr = self.as_ref();
        let mut min = Vec2::splat(T::MAX);
        let mut max = Vec2::splat(T::MIN);
        for p in arr {
            min = p.min(min);
            max = p.max(max);
        }
        rect(min.x, min.y, max.x - min.x, max.y - min.y)
    }

    fn project_onto_axis(&self, axis: Vec2<T>) -> Projection<T> {
        let arr = self.as_ref();
        let mut min = T::MAX;
        let mut max = T::MIN;
        for p in arr {
            let dot = p.dot(axis);
            min = T::min(min, dot);
            max = T::max(max, dot);
        }
        Projection { min, max }
    }

    fn project_point(&self, p: Vec2<T>) -> Vec2<T> {
        let arr = self.as_ref();
        let mut min_dist = T::MAX;
        let mut min_proj = Vec2::ZERO;
        for i in 0..arr.len() {
            let edge = line(arr[i], arr[(i + 1) % arr.len()]);
            let proj = edge.project_point(p);
            let dist = proj.sqr_dist(p);
            if dist < min_dist {
                min_dist = dist;
                min_proj = proj;
            }
        }
        min_proj
    }

    fn rayhit(&self, ray: &Ray<T>) -> bool {
        let arr = self.as_ref();
        for i in 0..arr.len() {
            let edge = line(arr[i], arr[(i + 1) % arr.len()]);
            if edge.raycast(ray).is_some() {
                return true;
            }
        }
        false
    }

    fn raycast(&self, ray: &Ray<T>) -> Option<RayHit<T>> {
        let arr = self.as_ref();
        let mut distance = T::MAX;
        let mut crossings = 0;
        let mut normal = Vec2::ZERO;
        for i in 0..arr.len() {
            let edge = line(arr[i], arr[(i + 1) % arr.len()]);
            if let Some(d) = edge.raycast(ray) {
                crossings += 1;
                if d < distance {
                    distance = d;
                    normal = edge.vector();
                }
            }
        }
        (crossings > 0 && (crossings % 2) == 0)
            .then(|| RayHit::new(normal.norm().turn_left(), distance))
    }

    #[inline]
    fn overlaps_rect(&self, rect: &Rect<T>) -> bool {
        rect.overlaps_poly(self)
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

    fn extract_from_poly<P: Polygonal<T>>(&self, poly: &P) -> Option<Vec2<T>> {
        let mut dist = T::MAX;
        let mut dir = Vec2::ZERO;
        (self.all_normals(|axis| extract_on(self, poly, axis, &mut dist, &mut dir))
            && poly.all_normals(|axis| extract_on(self, poly, axis, &mut dist, &mut dir)))
        .then(|| dir * dist)
    }

    fn is_convex(&self) -> bool {
        let p = self.as_ref();
        for i in 0..p.len() {
            let a = p[i];
            let b = p[(i + 1) % p.len()];
            let c = p[(i + 2) % p.len()];
            if (b - a).cross(c - b) < T::ZERO {
                return false;
            }
        }
        true
    }
}

impl<T: Float, S: AsRef<[Vec2<T>]>> Polygonal<T> for S {
    fn nearest_vertex(&self, source: Vec2<T>) -> Vec2<T> {
        let arr = self.as_ref();
        let mut min_dist = arr[0].dist(source);
        let mut min_i = 0;
        for i in 1..arr.len() {
            let dist = arr[i].dist(source);
            if dist < min_dist {
                min_dist = dist;
                min_i = i;
            }
        }
        arr[min_i]
    }

    #[inline]
    fn all_edges<F: FnMut(Line<T>) -> bool>(&self, mut cond: F) -> bool {
        let arr = self.as_ref();
        (0..arr.len()).all(|i| cond(line(arr[i], arr[(i + 1) % arr.len()])))
    }

    #[inline]
    fn all_normals<F: FnMut(Vec2<T>) -> bool>(&self, mut cond: F) -> bool {
        self.all_edges(|edge| cond(edge.left_norm()))
    }

    #[inline]
    fn visit_normals<F: FnMut(Vec2<T>)>(&self, mut plot: F) {
        self.all_edges(|edge| {
            plot(edge.left_norm());
            true
        });
    }
}
