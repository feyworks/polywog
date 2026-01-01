use crate::{Channel, FromRgb, Rgb, Rgba, ToRgb, ToRgba};
use bytemuck::{Pod, Zeroable};
use fey_math::Float;
use serde::{Deserialize, Serialize};

/// An alias for [`Hsl<f32>`].
pub type HslF = Hsl<f32>;

/// A color represented by hue, saturation, and lightness.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct Hsl<T> {
    /// The color's hue, represented by 0-360º on the color wheel.
    pub h: T,

    /// The color's saturation, from 0 (greyscale) to 1 (full saturation).
    pub s: T,

    /// The color's lightness, from 0 (black) to 1 (white).
    pub l: T,
}

/// Create a new HSL color.
#[inline]
pub const fn hsl<T>(h: T, s: T, l: T) -> Hsl<T> {
    Hsl { h, s, l }
}

unsafe impl<T: Zeroable> Zeroable for Hsl<T> {}
unsafe impl<T: Pod> Pod for Hsl<T> {}

impl<T> Hsl<T> {
    /// Create a new HSL color.
    #[inline]
    pub const fn new(h: T, s: T, l: T) -> Self {
        hsl(h, s, l)
    }
}

impl<T: Channel + Float, F: Channel> FromRgb<F> for Hsl<T> {
    fn from_rgb(val: Rgb<F>) -> Self {
        let Rgb { r, g, b }: Rgb<T> = val.to_rgb();
        let (r, g, b) = (r / T::NUM_255, g / T::NUM_255, b / T::NUM_255);
        let min = T::min(T::min(r, g), b);
        let max = T::max(T::max(r, g), b);
        let chroma = max - min;
        let l = (max + min) * T::HALF;
        let h = if chroma == T::ZERO {
            T::ZERO
        } else if max == r {
            (g - b) / chroma % (T::TWO + T::TWO + T::TWO)
        } else if max == g {
            (b - r) / chroma + T::TWO
        } else {
            (r - g) / chroma + T::TWO + T::TWO
        } * T::NUM_60;
        let s = if chroma == T::ZERO || l == T::ZERO || l == T::ONE {
            T::ZERO
        } else {
            (max - l) / T::min(l, T::ONE - l)
        };
        hsl(h, s, l)
    }
}

impl<T: Channel, F: Channel + Float> ToRgb<T> for Hsl<F> {
    fn to_rgb(self) -> Rgb<T> {
        let Hsl { h, s, l } = self;

        let mut chroma = (F::ONE - F::abs(F::TWO * l - F::ONE)) * s * F::NUM_255;
        let mut x = chroma * (F::ONE - F::abs(h / F::NUM_60 % F::TWO - F::ONE));
        let min = l * F::NUM_255 - chroma / F::TWO;
        chroma += min;
        x += min;

        let (r, g, b) = if h <= F::NUM_60 {
            (chroma, x, min)
        } else if h <= F::NUM_120 {
            (x, chroma, min)
        } else if h <= F::NUM_180 {
            (min, chroma, x)
        } else if h <= F::NUM_240 {
            (min, x, chroma)
        } else if h <= F::NUM_300 {
            (x, min, chroma)
        } else {
            (chroma, min, x)
        };

        Rgb::new(r, g, b).to_rgb()
    }
}

impl<T: Channel, F: Channel + Float> ToRgba<T> for Hsl<F> {
    #[inline]
    fn to_rgba(self) -> Rgba<T> {
        let Rgb { r, g, b } = self.to_rgb();
        Rgba::new(r, g, b, T::CHANNEL_MAX)
    }
}
