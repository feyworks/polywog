use crate::{Channel, FromLinear, FromRgb, Grey, Rgba, ToLinear, ToRgb, ToRgba};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// An alias for [`Rgb<u8>`].
pub type Rgb8 = Rgb<u8>;

/// An alias for [`Rgb<u16>`].
pub type Rgb16 = Rgb<u16>;

/// An alias for [`Rgb<f32>`].
pub type Rgb32F = Rgb<f32>;

/// An alias for [`Rgb<f64>`].
pub type Rgb64F = Rgb<f64>;

/// A 3-channel RGB color.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

fey_math::impl_approx!(
    NAME = Rgb
    FIELDS = (r, g, b)
);
fey_math::impl_casts!(
    NAME = Rgb
    FIELDS = (r, g, b)
);
fey_math::impl_tuple_arr!(
    NAME = Rgb
    LEN = 3
    FIELDS = (r, g, b)
    TUPLE = (T, T, T)
);

unsafe impl<T: Zeroable> Zeroable for Rgb<T> {}
unsafe impl<T: Pod> Pod for Rgb<T> {}

impl<T> Rgb<T> {
    /// Create a new RGB color.
    #[inline]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    /// Upgrade to an RGBA color with the provided alpha value.
    #[inline]
    pub fn with_a(self, a: T) -> Rgba<T> {
        Rgba::new(self.r, self.g, self.b, a)
    }
}

impl<T: Channel> Rgb<T> {
    pub const BLACK: Self = Self::splat(T::ZERO);
    pub const WHITE: Self = Self::splat(T::CHANNEL_MAX);
    pub const RED: Self = Self::new(T::CHANNEL_MAX, T::ZERO, T::ZERO);
    pub const GREEN: Self = Self::new(T::ZERO, T::CHANNEL_MAX, T::ZERO);
    pub const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::CHANNEL_MAX);
    pub const FUCHSIA: Self = Self::new(T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX);
    pub const CYAN: Self = Self::new(T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const YELLOW: Self = Self::new(T::CHANNEL_MAX, T::CHANNEL_MAX, T::ZERO);

    /// Create a color with all components set to the same value.
    #[inline]
    pub const fn splat(val: T) -> Self {
        Self::new(val, val, val)
    }

    /// Convert from grey to RGB.
    #[inline]
    pub fn from_grey<C: Channel>(val: Grey<C>) -> Self {
        val.to_rgb()
    }

    /// Calculate the [redmean difference](https://en.wikipedia.org/wiki/Color_difference) between two colors.
    pub fn diff_f32(&self, other: &Self) -> f32 {
        let [r1, g1, b1] = [self.r, self.g, self.b].map(|x| x.to_channel::<f32>());
        let [r2, g2, b2] = [other.r, other.g, other.b].map(|x| x.to_channel::<f32>());
        let rm = (r1 + r2) * 0.5;
        ((2.0 + rm / 256.0) * (r1 - r2).powi(2)
            + 4.0 * (g1 - g2).powi(2)
            + (2.0 + (255.0 - rm) / 256.0) * (b1 - b2).powi(2))
        .sqrt()
    }

    /// Calculate the [redmean difference](https://en.wikipedia.org/wiki/Color_difference) between two colors.
    pub fn diff_f64(&self, other: &Self) -> f64 {
        let [r1, g1, b1] = [self.r, self.g, self.b].map(|x| x.to_channel::<f64>());
        let [r2, g2, b2] = [other.r, other.g, other.b].map(|x| x.to_channel::<f64>());
        let rm = (r1 + r2) * 0.5;
        ((2.0 + rm / 256.0) * (r1 - r2).powi(2)
            + 4.0 * (g1 - g2).powi(2)
            + (2.0 + (255.0 - rm) / 256.0) * (b1 - b2).powi(2))
        .sqrt()
    }

    /// Unsigned-normalize multiply all channels by the value.
    #[inline]
    pub fn un_mul(self, a: T) -> Self {
        Self::new(self.r.un_mul(a), self.g.un_mul(a), self.b.un_mul(a))
    }
}

impl Rgb<u8> {
    /// Pack the color into a `u32` value.
    #[inline]
    pub const fn pack(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    /// Unpack the color from a `u32` value.
    #[inline]
    pub const fn unpack(packed: u32) -> Self {
        Self::new((packed >> 16) as u8, (packed >> 8) as u8, packed as u8)
    }
}

impl Debug for Rgb<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#08x}", self.pack())
    }
}

impl Display for Rgb<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

macro_rules! impl_debug {
    ($name:ty) => {
        impl Debug for Rgb<$name> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Rgba")
                    .field("r", &self.r)
                    .field("g", &self.g)
                    .field("b", &self.b)
                    .finish()
            }
        }
    };
}

impl_debug!(u16);
impl_debug!(f32);
impl_debug!(f64);

impl<T: Channel> From<Grey<T>> for Rgb<T> {
    #[inline]
    fn from(value: Grey<T>) -> Self {
        Self::new(value.0, value.0, value.0)
    }
}

impl<T: Channel> FromRgb<T> for Rgb<T> {
    #[inline]
    fn from_rgb(val: Rgb<T>) -> Self {
        val.to_rgb()
    }
}

impl<T: Channel, F: Channel> ToRgb<T> for Rgb<F> {
    #[inline]
    fn to_rgb(self) -> Rgb<T> {
        Rgb::new(
            self.r.to_channel(),
            self.g.to_channel(),
            self.b.to_channel(),
        )
    }
}

impl<T: Channel, F: Channel> ToRgba<T> for Rgb<F> {
    fn to_rgba(self) -> Rgba<T> {
        let Rgb { r, g, b } = self.to_rgb();
        Rgba::new(r, g, b, T::CHANNEL_MAX)
    }
}

impl<T: ToLinear> ToLinear for Rgb<T> {
    #[inline]
    fn to_linear(self) -> Self {
        Self::new(self.r.to_linear(), self.g.to_linear(), self.b.to_linear())
    }
}

impl<T: FromLinear> FromLinear for Rgb<T> {
    #[inline]
    fn from_linear(Self { r, g, b }: Self) -> Self {
        Self::new(T::from_linear(r), T::from_linear(g), T::from_linear(b))
    }
}
