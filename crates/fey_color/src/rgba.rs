use crate::{Channel, FromRgb, FromRgba, Grey, GreyAlpha, Rgb, ToRgba, abgr};

use bytemuck::{Pod, Zeroable};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

/// An alias for [`Rgba<u8>`].
pub type Rgba8 = Rgba<u8>;

/// An alias for [`Rgba<u16>`].
pub type Rgba16 = Rgba<u16>;

/// An alias for [`Rgba<f32>`].
pub type Rgba32F = Rgba<f32>;

/// An alias for [`Rgba<f64>`].
pub type Rgba64F = Rgba<f64>;

/// A 4-channel RGBA color.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

fey_math::impl_approx!(
    NAME = Rgba
    FIELDS = (r, g, b, a)
);
fey_math::impl_casts!(
    NAME = Rgba
    FIELDS = (r, g, b, a)
);
fey_math::impl_tuple_arr!(
    NAME = Rgba
    LEN = 4
    FIELDS = (r, g, b, a)
    TUPLE = (T, T, T, T)
);

unsafe impl<T: Zeroable> Zeroable for Rgba<T> {}
unsafe impl<T: Pod> Pod for Rgba<T> {}

impl<T> Rgba<T> {
    /// Create a new RGBA color.
    #[inline]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: Channel> Rgba<T> {
    pub const TRANSPARENT: Self = Self::splat(T::ZERO);
    pub const BLACK: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::CHANNEL_MAX);
    pub const WHITE: Self = Self::splat(T::CHANNEL_MAX);
    pub const RED: Self = Self::new(T::CHANNEL_MAX, T::ZERO, T::ZERO, T::CHANNEL_MAX);
    pub const GREEN: Self = Self::new(T::ZERO, T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX);
    pub const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const FUCHSIA: Self = Self::new(T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const CYAN: Self = Self::new(T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const YELLOW: Self = Self::new(T::CHANNEL_MAX, T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX);

    /// Create a color with all components set to the same value.
    #[inline]
    pub const fn splat(val: T) -> Self {
        Self::new(val, val, val, val)
    }

    /// Convert from grey to RGBA.
    #[inline]
    pub fn from_grey<C: Channel>(val: Grey<C>) -> Self {
        val.to_rgba()
    }

    /// Convert from grey-alpha to RGBA.
    #[inline]
    pub fn from_grey_alpha<C: Channel>(val: GreyAlpha<C>) -> Self {
        val.to_rgba()
    }

    /// Unsigned-normalize multiply all channels by the value.
    #[inline]
    pub fn un_mul(self, a: T) -> Self {
        Self::new(
            self.r.un_mul(a),
            self.g.un_mul(a),
            self.b.un_mul(a),
            self.a.un_mul(a),
        )
    }

    /// Unsigned-normalize multiply all channels by the value.
    #[inline]
    pub fn mul_color(self, Self { r, g, b, a }: Self) -> Self {
        Self::new(
            self.r.un_mul(r),
            self.g.un_mul(g),
            self.b.un_mul(b),
            self.a.un_mul(a),
        )
    }

    /// Unsigned-normalize add all channels by the value.
    #[inline]
    pub fn add_color(self, Self { r, g, b, a }: Self) -> Self {
        Self::new(
            self.r.un_add(r),
            self.g.un_add(g),
            self.b.un_add(b),
            self.a.un_add(a),
        )
    }

    /// Unsigned-normalize subtract all channels by the value.
    #[inline]
    pub fn sub_color(self, Self { r, g, b, a }: Self) -> Self {
        Self::new(
            self.r.un_sub(r),
            self.g.un_sub(g),
            self.b.un_sub(b),
            self.a.un_sub(a),
        )
    }
}

impl Rgba<u8> {
    pub const ALICE_BLUE: Self = abgr(0xfffff8f0);
    pub const ANTIQUE_WHITE: Self = abgr(0xffd7ebfa);
    pub const AQUA: Self = abgr(0xffffff00);
    pub const AQUAMARINE: Self = abgr(0xffd4ff7f);
    pub const AZURE: Self = abgr(0xfffffff0);
    pub const BEIGE: Self = abgr(0xffdcf5f5);
    pub const BISQUE: Self = abgr(0xffc4e4ff);
    pub const BLANCHED_ALMOND: Self = abgr(0xffcdebff);
    pub const BLUE_VIOLET: Self = abgr(0xffe22b8a);
    pub const BROWN: Self = abgr(0xff2a2aa5);
    pub const BURLY_WOOD: Self = abgr(0xff87b8de);
    pub const CADET_BLUE: Self = abgr(0xffa09e5f);
    pub const CHARTREUSE: Self = abgr(0xff00ff7f);
    pub const CHOCOLATE: Self = abgr(0xff1e69d2);
    pub const CORAL: Self = abgr(0xff507fff);
    pub const CORNFLOWER_BLUE: Self = abgr(0xffed9564);
    pub const CORNSILK: Self = abgr(0xffdcf8ff);
    pub const CRIMSON: Self = abgr(0xff3c14dc);
    pub const DARK_BLUE: Self = abgr(0xff8b0000);
    pub const DARK_CYAN: Self = abgr(0xff8b8b00);
    pub const DARK_GOLDENROD: Self = abgr(0xff0b86b8);
    pub const DARK_GRAY: Self = abgr(0xffa9a9a9);
    pub const DARK_GREEN: Self = abgr(0xff006400);
    pub const DARK_KHAKI: Self = abgr(0xff6bb7bd);
    pub const DARK_MAGENTA: Self = abgr(0xff8b008b);
    pub const DARK_OLIVE_GREEN: Self = abgr(0xff2f6b55);
    pub const DARK_ORANGE: Self = abgr(0xff008cff);
    pub const DARK_ORCHID: Self = abgr(0xffcc3299);
    pub const DARK_RED: Self = abgr(0xff00008b);
    pub const DARK_SALMON: Self = abgr(0xff7a96e9);
    pub const DARK_SEA_GREEN: Self = abgr(0xff8bbc8f);
    pub const DARK_SLATE_BLUE: Self = abgr(0xff8b3d48);
    pub const DARK_SLATE_GRAY: Self = abgr(0xff4f4f2f);
    pub const DARK_TURQUOISE: Self = abgr(0xffd1ce00);
    pub const DARK_VIOLET: Self = abgr(0xffd30094);
    pub const DEEP_PINK: Self = abgr(0xff9314ff);
    pub const DEEP_SKY_BLUE: Self = abgr(0xffffbf00);
    pub const DIM_GRAY: Self = abgr(0xff696969);
    pub const DODGER_BLUE: Self = abgr(0xffff901e);
    pub const FIREBRICK: Self = abgr(0xff2222b2);
    pub const FLORAL_WHITE: Self = abgr(0xfff0faff);
    pub const FOREST_GREEN: Self = abgr(0xff228b22);
    pub const GAINSBORO: Self = abgr(0xffdcdcdc);
    pub const GHOST_WHITE: Self = abgr(0xfffff8f8);
    pub const GOLD: Self = abgr(0xff00d7ff);
    pub const GOLDENROD: Self = abgr(0xff20a5da);
    pub const GRAY: Self = abgr(0xff808080);
    pub const GREEN_YELLOW: Self = abgr(0xff2fffad);
    pub const HONEYDEW: Self = abgr(0xfff0fff0);
    pub const HOT_PINK: Self = abgr(0xffb469ff);
    pub const INDIAN_RED: Self = abgr(0xff5c5ccd);
    pub const INDIGO: Self = abgr(0xff82004b);
    pub const IVORY: Self = abgr(0xfff0ffff);
    pub const KHAKI: Self = abgr(0xff8ce6f0);
    pub const LAVENDER: Self = abgr(0xfffae6e6);
    pub const LAVENDER_BLUSH: Self = abgr(0xfff5f0ff);
    pub const LAWN_GREEN: Self = abgr(0xff00fc7c);
    pub const LEMON_CHIFFON: Self = abgr(0xffcdfaff);
    pub const LIGHT_BLUE: Self = abgr(0xffe6d8ad);
    pub const LIGHT_CORAL: Self = abgr(0xff8080f0);
    pub const LIGHT_CYAN: Self = abgr(0xffffffe0);
    pub const LIGHT_GOLDENROD_YELLOW: Self = abgr(0xffd2fafa);
    pub const LIGHT_GRAY: Self = abgr(0xffd3d3d3);
    pub const LIGHT_GREEN: Self = abgr(0xff90ee90);
    pub const LIGHT_PINK: Self = abgr(0xffc1b6ff);
    pub const LIGHT_SALMON: Self = abgr(0xff7aa0ff);
    pub const LIGHT_SEA_GREEN: Self = abgr(0xffaab220);
    pub const LIGHT_SKY_BLUE: Self = abgr(0xffface87);
    pub const LIGHT_SLATE_GRAY: Self = abgr(0xff998877);
    pub const LIGHT_STEEL_BLUE: Self = abgr(0xffdec4b0);
    pub const LIGHT_YELLOW: Self = abgr(0xffe0ffff);
    pub const LIME_GREEN: Self = abgr(0xff32cd32);
    pub const LINEN: Self = abgr(0xffe6f0fa);
    pub const MAROON: Self = abgr(0xff000080);
    pub const MEDIUM_AQUAMARINE: Self = abgr(0xffaacd66);
    pub const MEDIUM_BLUE: Self = abgr(0xffcd0000);
    pub const MEDIUM_ORCHID: Self = abgr(0xffd355ba);
    pub const MEDIUM_PURPLE: Self = abgr(0xffdb7093);
    pub const MEDIUM_SEA_GREEN: Self = abgr(0xff71b33c);
    pub const MEDIUM_SLATE_BLUE: Self = abgr(0xffee687b);
    pub const MEDIUM_SPRING_GREEN: Self = abgr(0xff9afa00);
    pub const MEDIUM_TURQUOISE: Self = abgr(0xffccd148);
    pub const MEDIUM_VIOLET_RED: Self = abgr(0xff8515c7);
    pub const MIDNIGHT_BLUE: Self = abgr(0xff701919);
    pub const MINT_CREAM: Self = abgr(0xfffafff5);
    pub const MISTY_ROSE: Self = abgr(0xffe1e4ff);
    pub const MOCCASIN: Self = abgr(0xffb5e4ff);
    pub const MONO_GAME_ORANGE: Self = abgr(0xff003ce7);
    pub const NAVAJO_WHITE: Self = abgr(0xffaddeff);
    pub const NAVY: Self = abgr(0xff800000);
    pub const OLD_LACE: Self = abgr(0xffe6f5fd);
    pub const OLIVE: Self = abgr(0xff008080);
    pub const OLIVE_DRAB: Self = abgr(0xff238e6b);
    pub const ORANGE: Self = abgr(0xff00a5ff);
    pub const ORANGE_RED: Self = abgr(0xff0045ff);
    pub const ORCHID: Self = abgr(0xffd670da);
    pub const PALE_GOLDENROD: Self = abgr(0xffaae8ee);
    pub const PALE_GREEN: Self = abgr(0xff98fb98);
    pub const PALE_TURQUOISE: Self = abgr(0xffeeeeaf);
    pub const PALE_VIOLET_RED: Self = abgr(0xff9370db);
    pub const PAPAYA_WHIP: Self = abgr(0xffd5efff);
    pub const PEACH_PUFF: Self = abgr(0xffb9daff);
    pub const PERU: Self = abgr(0xff3f85cd);
    pub const PINK: Self = abgr(0xffcbc0ff);
    pub const PLUM: Self = abgr(0xffdda0dd);
    pub const POWDER_BLUE: Self = abgr(0xffe6e0b0);
    pub const PURPLE: Self = abgr(0xff800080);
    pub const ROSY_BROWN: Self = abgr(0xff8f8fbc);
    pub const ROYAL_BLUE: Self = abgr(0xffe16941);
    pub const SADDLE_BROWN: Self = abgr(0xff13458b);
    pub const SALMON: Self = abgr(0xff7280fa);
    pub const SANDY_BROWN: Self = abgr(0xff60a4f4);
    pub const SEA_GREEN: Self = abgr(0xff578b2e);
    pub const SEA_SHELL: Self = abgr(0xffeef5ff);
    pub const SIENNA: Self = abgr(0xff2d52a0);
    pub const SILVER: Self = abgr(0xffc0c0c0);
    pub const SKY_BLUE: Self = abgr(0xffebce87);
    pub const SLATE_BLUE: Self = abgr(0xffcd5a6a);
    pub const SLATE_GRAY: Self = abgr(0xff908070);
    pub const SNOW: Self = abgr(0xfffafaff);
    pub const SPRING_GREEN: Self = abgr(0xff7fff00);
    pub const STEEL_BLUE: Self = abgr(0xffb48246);
    pub const TAN: Self = abgr(0xff8cb4d2);
    pub const TEAL: Self = abgr(0xff808000);
    pub const THISTLE: Self = abgr(0xffd8bfd8);
    pub const TOMATO: Self = abgr(0xff4763ff);
    pub const TURQUOISE: Self = abgr(0xffd0e040);
    pub const VIOLET: Self = abgr(0xffee82ee);
    pub const WHEAT: Self = abgr(0xffb3def5);
    pub const WHITE_SMOKE: Self = abgr(0xfff5f5f5);
    pub const YELLOW_GREEN: Self = abgr(0xff32cd9a);

    /// Pack the color into a `u32` value.
    #[inline]
    pub const fn pack(self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }

    /// Unpack the color from a `u32` value.
    #[inline]
    pub const fn unpack(packed: u32) -> Self {
        Self::new(
            (packed >> 24) as u8,
            (packed >> 16) as u8,
            (packed >> 8) as u8,
            packed as u8,
        )
    }
}

impl Debug for Rgba<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#10x}", self.pack())
    }
}

impl Display for Rgba<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

macro_rules! impl_debug {
    ($name:ty) => {
        impl Debug for Rgba<$name> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Rgba")
                    .field("r", &self.r)
                    .field("g", &self.g)
                    .field("b", &self.b)
                    .field("a", &self.a)
                    .finish()
            }
        }
    };
}

impl_debug!(u16);
impl_debug!(f32);
impl_debug!(f64);

impl<T: Channel> From<Grey<T>> for Rgba<T> {
    #[inline]
    fn from(value: Grey<T>) -> Self {
        Self::new(value.0, value.0, value.0, T::CHANNEL_MAX)
    }
}

impl<T: Channel> From<GreyAlpha<T>> for Rgba<T> {
    #[inline]
    fn from(GreyAlpha { g, a }: GreyAlpha<T>) -> Self {
        Self::new(g, g, g, a)
    }
}

impl<T: Channel> From<Rgb<T>> for Rgba<T> {
    #[inline]
    fn from(Rgb { r, g, b }: Rgb<T>) -> Self {
        Self::new(r, g, b, T::CHANNEL_MAX)
    }
}

impl<T: Channel, F: Channel> ToRgba<T> for Rgba<F> {
    #[inline]
    fn to_rgba(self) -> Rgba<T> {
        Rgba::new(
            self.r.to_channel(),
            self.g.to_channel(),
            self.b.to_channel(),
            self.a.to_channel(),
        )
    }
}

impl<T: Channel, F: Channel> FromRgba<F> for Rgba<T> {
    #[inline]
    fn from_rgba(val: Rgba<F>) -> Self {
        val.to_rgba()
    }
}

impl<T: Channel, F: Channel> FromRgb<F> for Rgba<T> {
    #[inline]
    fn from_rgb(val: Rgb<F>) -> Self {
        val.to_rgba()
    }
}

impl From<u32> for Rgba<u8> {
    #[inline]
    fn from(value: u32) -> Self {
        Self::unpack(value)
    }
}

impl Serialize for Rgba<u8> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("#{:08X}", self.pack()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Rgba<u8> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        if str.chars().next() != Some('#') {
            return Err(D::Error::custom(format!(
                "invalid color string {str:?}, must be a hex code starting with '#'"
            )));
        }
        u32::from_str_radix(&str[1..], 16)
            .map(Self::unpack)
            .map_err(D::Error::custom)
    }
}
