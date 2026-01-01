use crate::{Channel, Grey, Rgba, ToRgba};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// An alias for [`GreyAlpha<u8>`].
pub type GreyAlpha8 = GreyAlpha<u8>;

/// An alias for [`GreyAlpha<u16>`].
pub type GreyAlpha16 = GreyAlpha<u16>;

/// An alias for [`GreyAlpha<f32>`].
pub type GreyAlpha32F = GreyAlpha<f32>;

/// An alias for [`GreyAlpha<f64>`].
pub type GreyAlpha64F = GreyAlpha<f64>;

/// A 2-channel greyscale color with alpha.
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct GreyAlpha<T> {
    pub g: T,
    pub a: T,
}

fey_math::macros::impl_approx!(
    NAME = GreyAlpha
    FIELDS = (g, a)
);
fey_math::macros::impl_casts!(
    NAME = GreyAlpha
    FIELDS = (g, a)
);
fey_math::macros::impl_tuple_arr!(
    NAME = GreyAlpha
    LEN = 2
    FIELDS = (g, a)
    TUPLE = (T, T)
);

/// Create a new grey-alpha color.
#[inline]
pub const fn grey_alpha<T>(g: T, a: T) -> GreyAlpha<T> {
    GreyAlpha { g, a }
}

unsafe impl<T: Zeroable> Zeroable for GreyAlpha<T> {}
unsafe impl<T: Pod> Pod for GreyAlpha<T> {}

impl<T> GreyAlpha<T> {
    /// Create a new grey-alpha color.
    #[inline]
    pub const fn new(g: T, a: T) -> Self {
        Self { g, a }
    }
}

impl<T: Channel> GreyAlpha<T> {
    pub const TRANSPARENT: Self = Self::new(T::ZERO, T::ZERO);
    pub const BLACK: Self = Self::new(T::ZERO, T::CHANNEL_MAX);
    pub const WHITE: Self = Self::new(T::CHANNEL_MAX, T::CHANNEL_MAX);

    /// Convert from grey to grey-alpha.
    #[inline]
    pub fn from_grey<C: Channel>(val: Grey<C>) -> Self {
        val.to_grey_alpha()
    }

    /// Convert from a different channel type.
    #[inline]
    pub fn from_grey_alpha<C: Channel>(val: GreyAlpha<C>) -> Self {
        val.to_grey_alpha()
    }

    /// Convert to a different channel type.
    #[inline]
    pub fn to_grey_alpha<C: Channel>(self) -> GreyAlpha<C> {
        grey_alpha(self.g.to_channel(), self.a.to_channel())
    }

    /// Convert to a different channel type.
    #[inline]
    pub fn to_rgba<C: Channel>(self) -> Rgba<C> {
        let GreyAlpha { g, a } = self.to_grey_alpha();
        Rgba::new(g, g, g, a)
    }
}

impl GreyAlpha<u8> {
    /// Pack the color into a `u16` value.
    #[inline]
    pub const fn pack(self) -> u16 {
        (self.g as u16) << 8 | (self.a as u16)
    }

    /// Unpack the color from a `u16` value.
    #[inline]
    pub const fn unpack(packed: u16) -> Self {
        Self::new((packed >> 8) as u8, packed as u8)
    }
}

impl Display for GreyAlpha<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06x}", self.pack())
    }
}

impl<T: Channel> From<Grey<T>> for GreyAlpha<T> {
    #[inline]
    fn from(value: Grey<T>) -> Self {
        Self::new(value.0, T::CHANNEL_MAX)
    }
}

impl<T: Channel, F: Channel> ToRgba<T> for GreyAlpha<F> {
    #[inline]
    fn to_rgba(self) -> Rgba<T> {
        self.to_rgba()
    }
}
