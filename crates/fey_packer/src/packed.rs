use fey_math::Vec2U;

/// An item that has been packed.
pub struct Packed<T> {
    pub data: T,
    pub pos: Vec2U,
}
