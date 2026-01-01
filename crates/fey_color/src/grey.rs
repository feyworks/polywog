use crate::{Channel, GreyAlpha, Rgb, Rgba, ToRgb, ToRgba, grey_alpha};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

/// An alias for [`Grey<u8>`].
pub type Grey8 = Grey<u8>;

/// An alias for [`Grey<u16>`].
pub type Grey16 = Grey<u16>;

/// An alias for [`Grey<f32>`].
pub type Grey32F = Grey<f32>;

/// An alias for [`Grey<f64>`].
pub type Grey64F = Grey<f64>;

/// A single-channel greyscale color.
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Grey<T>(pub T);

fey_math::macros::impl_approx!(
    NAME = Grey
    FIELDS = (0)
);
fey_math::macros::impl_casts!(
    NAME = Grey
    FIELDS = (0)
);

unsafe impl<T: Zeroable> Zeroable for Grey<T> {}
unsafe impl<T: Pod> Pod for Grey<T> {}

/// Create a new grey color.
#[inline]
pub const fn grey<T>(g: T) -> Grey<T> {
    Grey(g)
}

impl<T: Channel> Grey<T> {
    pub const BLACK: Self = Self(T::ZERO);
    pub const WHITE: Self = Self(T::CHANNEL_MAX);

    /// Create a new grey color.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Convert from a different channel type.
    #[inline]
    pub fn from_grey<C: Channel>(val: Grey<C>) -> Self {
        val.to_grey()
    }

    /// Convert to a different channel type.
    #[inline]
    pub fn to_grey<C: Channel>(self) -> Grey<C> {
        Grey(self.0.to_channel())
    }

    /// Convert from grey to grey-alpha.
    #[inline]
    pub fn to_grey_alpha<C: Channel>(self) -> GreyAlpha<C> {
        grey_alpha(self.0.to_channel(), C::CHANNEL_MAX)
    }

    /// Convert to a different channel type.
    #[inline]
    pub fn to_rgb<C: Channel>(self) -> Rgb<C> {
        let v = self.0.to_channel();
        Rgb::new(v, v, v)
    }

    /// Convert to a different channel type.
    #[inline]
    pub fn to_rgba<C: Channel>(self) -> Rgba<C> {
        let Rgb { r, g, b } = self.to_rgb();
        Rgba::new(r, g, b, C::CHANNEL_MAX)
    }
}

impl<T: Channel, F: Channel> ToRgb<T> for Grey<F> {
    #[inline]
    fn to_rgb(self) -> Rgb<T> {
        self.to_rgb()
    }
}

impl<T: Channel, F: Channel> ToRgba<T> for Grey<F> {
    #[inline]
    fn to_rgba(self) -> Rgba<T> {
        self.to_rgba()
    }
}
