use fey_math::Vec2U;

/// An item to be packed.
pub struct Item<T> {
    pub size: Vec2U,
    pub data: T,
}

impl<T> Item<T> {
    /// Creates a new item to be packed.
    pub const fn new(size: Vec2U, data: T) -> Self {
        Self { size, data }
    }
}
