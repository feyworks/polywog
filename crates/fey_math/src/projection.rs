use crate::{Num, impl_approx, impl_bytemuck, impl_casts, impl_serde, impl_tuple_arr};

pub type ProjectionF = Projection<f32>;

/// Represents the projection of a 2D shape on an axis.
///
/// This primitive does not contain the axis itself, merely
/// the start and end bounds of the projection. This is used
/// in overlap checks for convex shapes.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Projection<T> {
    pub min: T,
    pub max: T,
}

impl_bytemuck!(Projection);

impl_tuple_arr!(
    NAME = Projection
    LEN = 2
    FIELDS = (min, max)
    TUPLE = (T, T)
);

impl_approx!(
    NAME = Projection
    FIELDS = (min, max)
);

impl_serde!(
    NAME = Projection
    FIELDS = (min, max)
);

impl_casts!(
    NAME = Projection
    FIELDS = (min, max)
);

impl<T> Projection<T> {
    /// Create a new projection.
    #[inline]
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T: Num> Projection<T> {
    /// Returns true if this projection overlaps the other.
    #[inline]
    pub fn overlaps(&self, other: Projection<T>) -> bool {
        self.min < other.max && self.max > other.min
    }

    /// If this projection overlaps the other, returns the amount
    /// which it overlaps.
    #[inline]
    pub fn overlap(&self, other: Projection<T>) -> Option<T> {
        (self.min < other.max && self.max > other.min).then(|| self.max - other.min)
    }

    /// Length of the projection.
    #[inline]
    pub fn len(&self) -> T {
        self.max - self.min
    }
}
