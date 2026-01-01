use crate::{Num, Vec2, impl_approx, impl_bytemuck, impl_casts};
use serde::{Deserialize, Serialize};

pub type RayF = Ray<f32>;

/// A ray with an origin and direction.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Ray<T> {
    pub origin: Vec2<T>,
    pub direction: Vec2<T>,
}

impl_bytemuck!(Ray);

impl_approx!(
    NAME = Ray
    FIELDS = (origin, direction)
);

impl_casts!(
    NAME = Ray
    FIELDS = (origin, direction)
);

/// Create a [`Ray`].
#[inline]
pub const fn ray<T>(origin: Vec2<T>, direction: Vec2<T>) -> Ray<T> {
    Ray { origin, direction }
}

impl<T> Ray<T> {
    /// Create a new ray.
    #[inline]
    pub const fn new(origin: Vec2<T>, direction: Vec2<T>) -> Self {
        ray(origin, direction)
    }
}

impl<T: Num> Ray<T> {
    /// Get a point at the distance `dist` along the ray.
    #[inline]
    pub fn point(&self, dist: T) -> Vec2<T> {
        self.origin + self.direction * dist
    }
}
