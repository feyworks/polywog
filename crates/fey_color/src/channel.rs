use bytemuck::{Pod, Zeroable};
use fey_math::Num;

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// A value that can be used as a color channel.
///
/// This is implemented only for `u8`, `u16`, `f32`, and `f64`.
pub trait Channel: private::Sealed + Num + Zeroable + Pod + Default {
    /// The max value this channel can be (full brightness).
    ///
    /// For the integer types, this is `u8::MAX` and `u16::MAX`
    /// respectively, but for floating point types it is `1.0`.
    const CHANNEL_MAX: Self;

    /// Convert a `u8` channel value to this type's equivalent.
    fn from_u8_channel(val: u8) -> Self;

    /// Convert a `u16` channel value to this type's equivalent.
    fn from_u16_channel(val: u16) -> Self;

    /// Convert a `f32` channel value to this type's equivalent.
    fn from_f32_channel(val: f32) -> Self;

    /// Convert a `f64` channel value to this type's equivalent.
    fn from_f64_channel(val: f64) -> Self;

    /// Convert this channel to another type's equivalent.
    fn to_channel<T: Channel>(self) -> T;

    /// Unsigned normal multiplication.
    fn un_mul(self, other: Self) -> Self;

    /// Unsigned normal addition.
    fn un_add(self, other: Self) -> Self;

    /// Unsigned normal subtraction.
    fn un_sub(self, other: Self) -> Self;

    /// Convert a channel value to this type's equivalent.
    #[inline]
    fn from_channel<T: Channel>(val: T) -> Self {
        val.to_channel::<Self>()
    }

    /// Lerp between two values using `factor` and unsigned operations.
    #[inline]
    fn un_lerp<C: Channel>(self, target: Self, factor: C) -> Self {
        self.un_add(target.un_sub(self).un_mul(factor.to_channel()))
    }
}

impl Channel for u8 {
    const CHANNEL_MAX: Self = u8::MAX;

    #[inline]
    fn from_u8_channel(val: u8) -> Self {
        val
    }

    #[inline]
    fn from_u16_channel(val: u16) -> Self {
        (((val as f32) / 65535.0) * 255.0) as u8
    }

    #[inline]
    fn from_f32_channel(val: f32) -> Self {
        (val * 255.0) as u8
    }

    #[inline]
    fn from_f64_channel(val: f64) -> Self {
        (val * 255.0) as u8
    }

    #[inline]
    fn to_channel<T: Channel>(self) -> T {
        T::from_u8_channel(self)
    }

    #[inline]
    fn un_mul(self, other: Self) -> Self {
        let a = self as u16;
        let b = other as u16;
        let t = a * b + 0x80;
        (((t >> 8) + t) >> 8) as u8
    }

    #[inline]
    fn un_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }

    #[inline]
    fn un_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}

impl Channel for u16 {
    const CHANNEL_MAX: Self = u16::MAX;

    #[inline]
    fn from_u8_channel(val: u8) -> Self {
        (((val as f32) / 255.0) * 65535.0) as u16
    }

    #[inline]
    fn from_u16_channel(val: u16) -> Self {
        val
    }

    #[inline]
    fn from_f32_channel(val: f32) -> Self {
        (val * 65535.0) as u16
    }

    #[inline]
    fn from_f64_channel(val: f64) -> Self {
        (val * 65535.0) as u16
    }

    #[inline]
    fn to_channel<T: Channel>(self) -> T {
        T::from_u16_channel(self)
    }

    #[inline]
    fn un_mul(self, other: Self) -> Self {
        (self.to_channel::<f32>() * other.to_channel::<f32>()).to_channel()
    }

    #[inline]
    fn un_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }

    #[inline]
    fn un_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}

impl Channel for f32 {
    const CHANNEL_MAX: Self = 1.0;

    #[inline]
    fn from_u8_channel(val: u8) -> Self {
        (val as f32) / 255.0
    }

    #[inline]
    fn from_u16_channel(val: u16) -> Self {
        (val as f32) / 65535.0
    }

    #[inline]
    fn from_f32_channel(val: f32) -> Self {
        val
    }

    #[inline]
    fn from_f64_channel(val: f64) -> Self {
        val as f32
    }

    #[inline]
    fn to_channel<T: Channel>(self) -> T {
        T::from_f32_channel(self)
    }

    #[inline]
    fn un_mul(self, other: Self) -> Self {
        self * other
    }

    #[inline]
    fn un_add(self, other: Self) -> Self {
        (self + other).clamp(0.0, 1.0)
    }

    #[inline]
    fn un_sub(self, other: Self) -> Self {
        (self - other).clamp(0.0, 1.0)
    }
}

impl Channel for f64 {
    const CHANNEL_MAX: Self = 1.0;

    #[inline]
    fn from_u8_channel(val: u8) -> Self {
        (val as f64) / 255.0
    }

    #[inline]
    fn from_u16_channel(val: u16) -> Self {
        (val as f64) / 65535.0
    }

    #[inline]
    fn from_f32_channel(val: f32) -> Self {
        val as f64
    }

    #[inline]
    fn from_f64_channel(val: f64) -> Self {
        val
    }

    #[inline]
    fn to_channel<T: Channel>(self) -> T {
        T::from_f64_channel(self)
    }

    #[inline]
    fn un_mul(self, other: Self) -> Self {
        self * other
    }

    #[inline]
    fn un_add(self, other: Self) -> Self {
        (self + other).clamp(0.0, 1.0)
    }

    #[inline]
    fn un_sub(self, other: Self) -> Self {
        (self - other).clamp(0.0, 1.0)
    }
}
