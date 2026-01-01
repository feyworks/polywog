use crate::{Vec2, impl_approx, impl_bytemuck, impl_casts};
use serde::{Deserialize, Serialize};

pub type RayHitF = RayHit<f32>;

/// A raycast hit on the surface of a shape.
///
/// Contains the `distance` along the ray the hit occurred, and the
/// `normal` of the edge the ray intersected.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RayHit<T> {
    pub normal: Vec2<T>,
    pub distance: T,
}

impl<T> RayHit<T> {
    /// Create a new raycast hit.
    #[inline]
    pub const fn new(normal: Vec2<T>, distance: T) -> Self {
        Self { normal, distance }
    }
}

impl_bytemuck!(RayHit);

impl_approx!(
    NAME = RayHit
    FIELDS = (normal, distance)
);

impl_casts!(
    NAME = RayHit
    FIELDS = (normal, distance)
);
