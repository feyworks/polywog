use crate::{Float, Line, Num, Numeric, Quad, Rect, Triangle, Vec2, line};
use serde::{Deserialize, Serialize};

pub type PolygonF = Polygon<f32>;
pub type PolygonI = Polygon<i32>;

/// A convex polygon.
///
/// The `N` represents how many points the polygon can have before it needs
/// to allocate onto the heap. If you know your shapes will never have more
/// than `N` edges, you can set it to that value and avoid heap allocations.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon<T>(Vec<Vec2<T>>);

impl<T: Num> Polygon<T> {
    /// Creates a new empty polygon.
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new empty polygon with at least the specified `capacity`.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Create a new polygon using the array of points.
    #[inline]
    pub fn from_arr<const LEN: usize>(arr: [Vec2<T>; LEN]) -> Self {
        Self(Vec::from(arr))
    }

    /// Create a new polygon using the slice of points.
    #[inline]
    pub fn from_slice<const LEN: usize>(slice: &[Vec2<T>]) -> Self {
        Self(Vec::from(slice))
    }

    /// Create a new polygon using the array of points.
    #[inline]
    pub fn from_vec(vec: Vec<Vec2<T>>) -> Self {
        Self(vec)
    }

    // Create a polygon representation of the provided rect.
    #[inline]
    pub fn from_rect(rect: Rect<T>) -> Self {
        Self::from_arr(rect.corners())
    }

    /// Create a polygon representation of the provided triangle.
    #[inline]
    pub fn from_tri(tri: Triangle<T>) -> Self {
        Self::from_arr(tri.0)
    }

    /// Create a polygon representation of the provided quad.
    #[inline]
    pub fn from_quad(quad: Quad<T>) -> Self {
        Self::from_arr(quad.0)
    }

    /// How many points the polygon has.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// If the polygon has no points.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The polygon's allocated capacity.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Add a point to the polygon.
    #[inline]
    pub fn push(&mut self, p: Vec2<T>) {
        self.0.push(p);
    }

    /// Remove the last point from the polygon and return it.
    #[inline]
    pub fn pop(&mut self) -> Option<Vec2<T>> {
        self.0.pop()
    }

    /// Reserve enough space for at least `capacity` points.
    #[inline]
    pub fn reserve(&mut self, capacity: usize) {
        if let Some(add) = capacity.checked_sub(self.capacity()) {
            self.0.reserve(add);
        }
    }

    /// Reference to the polygon's points.
    #[inline]
    pub fn points(&self) -> &[Vec2<T>] {
        self.0.as_slice()
    }

    /// Mutable reference to the polygon's points.
    #[inline]
    pub fn points_mut(&mut self) -> &mut [Vec2<T>] {
        self.0.as_mut_slice()
    }

    /// Remove all points from the polygon.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Remove the point at `index`.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Vec2<T> {
        self.0.remove(index)
    }

    /// Insert a point at `index`.
    #[inline]
    pub fn insert(&mut self, index: usize, p: Vec2<T>) {
        self.0.insert(index, p);
    }

    /// Convert the polygon into a `Vec` of points.
    #[inline]
    pub fn to_vec(self) -> Vec<Vec2<T>> {
        self.0
    }

    /// Resize the polygon, using the provided function to create new points.
    #[inline]
    pub fn resize_with<F: FnMut() -> Vec2<T>>(&mut self, new_size: usize, f: F) {
        self.0.resize_with(new_size, f);
    }

    /// Translate all points in the polygon.
    #[inline]
    pub fn translate(&mut self, amount: Vec2<T>) {
        for p in self.points_mut() {
            *p += amount;
        }
    }

    /// Get the *nth* edge of the polygon. A polygon has the
    /// same amount of edges as vertices, so if this index exceeds
    /// `len()`, then `None` will be returned.
    #[inline]
    pub fn edge(&self, index: usize) -> Option<Line<T>> {
        let points = self.points();
        (index < points.len()).then(|| line(points[index], points[(index + 1) % self.len()]))
    }

    /// An iterator over all the polygon's edges.
    #[inline]
    pub fn edges(&self) -> impl Iterator<Item = Line<T>> + '_ {
        let len = self.len();
        let points = self.points();
        (0..len).map(move |i| {
            let j = (i + 1) % len;
            line(points[i], points[j])
        })
    }
}

impl<T: Float> Polygon<T> {
    #[inline]
    pub fn transform_in_place_by(&mut self, mut f: impl FnMut(Vec2<T>) -> Vec2<T>) {
        for p in self.points_mut() {
            *p = f(*p);
        }
    }

    #[inline]
    pub fn transform_by_into(&self, into: &mut Self, f: impl FnMut(Vec2<T>) -> Vec2<T>) {
        into.clear();
        into.extend(self.points().iter().copied().map(f));
    }

    #[inline]
    pub fn transform_by(&self, f: impl FnMut(Vec2<T>) -> Vec2<T>) -> Self {
        let mut poly = Self::new();
        self.transform_by_into(&mut poly, f);
        poly
    }

    // /// Transform the polygon by the matrix.
    // #[inline]
    // pub fn transform_in_place(&mut self, mat: &Affine2<T>) {
    //     for p in self.points_mut() {
    //         *p = mat.transform_pos2(*p);
    //     }
    // }
    //
    // /// Transform the polygon, storing the results in the provided one.
    // #[inline]
    // pub fn transform_into(&mut self, mat: &Affine2<T>, into: &mut Self) {
    //     into.clear();
    //     into.reserve(self.len());
    //     for p in self.points() {
    //         into.push(mat.transform_pos2(*p));
    //     }
    // }
}

impl<T: Num> FromIterator<Vec2<T>> for Polygon<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Vec2<T>>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T: Num> Extend<Vec2<T>> for Polygon<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = Vec2<T>>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T> IntoIterator for Polygon<T> {
    type Item = Vec2<T>;
    type IntoIter = std::vec::IntoIter<Vec2<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> AsRef<[Vec2<T>]> for Polygon<T> {
    #[inline]
    fn as_ref(&self) -> &[Vec2<T>] {
        self.0.as_ref()
    }
}

impl<T> AsMut<[Vec2<T>]> for Polygon<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Vec2<T>] {
        self.0.as_mut()
    }
}

impl<T: Num, const S: usize> From<[Vec2<T>; S]> for Polygon<T> {
    #[inline]
    fn from(value: [Vec2<T>; S]) -> Self {
        Self::from_arr(value)
    }
}

impl<T: Num> From<Vec<Vec2<T>>> for Polygon<T> {
    #[inline]
    fn from(value: Vec<Vec2<T>>) -> Self {
        Self::from_vec(value)
    }
}

impl<T: Num> From<Rect<T>> for Polygon<T> {
    #[inline]
    fn from(value: Rect<T>) -> Self {
        Self::from_rect(value)
    }
}

impl<T: Num> From<Triangle<T>> for Polygon<T> {
    #[inline]
    fn from(value: Triangle<T>) -> Self {
        Self::from_tri(value)
    }
}

impl<T: Num> From<Quad<T>> for Polygon<T> {
    #[inline]
    fn from(value: Quad<T>) -> Self {
        Self::from_quad(value)
    }
}

impl<T: Num> Numeric for Polygon<T> {
    type AsU8 = Polygon<u8>;
    type AsU16 = Polygon<u16>;
    type AsU32 = Polygon<u32>;
    type AsU64 = Polygon<u64>;
    type AsU128 = Polygon<u128>;
    type AsUSize = Polygon<usize>;
    type AsI8 = Polygon<i8>;
    type AsI16 = Polygon<i16>;
    type AsI32 = Polygon<i32>;
    type AsI64 = Polygon<i64>;
    type AsI128 = Polygon<i128>;
    type AsISize = Polygon<isize>;
    type AsF32 = Polygon<f32>;
    type AsF64 = Polygon<f64>;

    #[inline]
    fn to_u8(self) -> Self::AsU8 {
        self.into_iter().map(Vec2::to_u8).collect()
    }

    #[inline]
    fn to_u16(self) -> Self::AsU16 {
        self.into_iter().map(Vec2::to_u16).collect()
    }

    #[inline]
    fn to_u32(self) -> Self::AsU32 {
        self.into_iter().map(Vec2::to_u32).collect()
    }

    #[inline]
    fn to_u64(self) -> Self::AsU64 {
        self.into_iter().map(Vec2::to_u64).collect()
    }

    #[inline]
    fn to_u128(self) -> Self::AsU128 {
        self.into_iter().map(Vec2::to_u128).collect()
    }

    #[inline]
    fn to_usize(self) -> Self::AsUSize {
        self.into_iter().map(Vec2::to_usize).collect()
    }

    #[inline]
    fn to_i8(self) -> Self::AsI8 {
        self.into_iter().map(Vec2::to_i8).collect()
    }

    #[inline]
    fn to_i16(self) -> Self::AsI16 {
        self.into_iter().map(Vec2::to_i16).collect()
    }

    #[inline]
    fn to_i32(self) -> Self::AsI32 {
        self.into_iter().map(Vec2::to_i32).collect()
    }

    #[inline]
    fn to_i64(self) -> Self::AsI64 {
        self.into_iter().map(Vec2::to_i64).collect()
    }

    #[inline]
    fn to_i128(self) -> Self::AsI128 {
        self.into_iter().map(Vec2::to_i128).collect()
    }

    #[inline]
    fn to_isize(self) -> Self::AsISize {
        self.into_iter().map(Vec2::to_isize).collect()
    }

    #[inline]
    fn to_f32(self) -> Self::AsF32 {
        self.into_iter().map(Vec2::to_f32).collect()
    }

    #[inline]
    fn to_f64(self) -> Self::AsF64 {
        self.into_iter().map(Vec2::to_f64).collect()
    }
}
