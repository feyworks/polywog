use crate::{Channel, FromRgb, Rgb, Rgba, ToRgb, ToRgba};
use bytemuck::{Pod, Zeroable};
use fey_math::Float;
use serde::{Deserialize, Serialize};

/// An alias for [`Hsv<f32>`].
pub type HsvF = Hsv<f32>;

/// A color represented by hue, saturation, and value.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct Hsv<T> {
    /// The color's hue, represented by 0-360º on the color wheel.
    pub h: T,

    /// The color's saturation, from 0 (greyscale) to 1 (full saturation).
    pub s: T,

    /// The color's value, from 0 (black) to 1 (full color value).
    pub v: T,
}

/// Create a new HSV color.
#[inline]
pub const fn hsv<T>(h: T, s: T, v: T) -> Hsv<T> {
    Hsv { h, s, v }
}

unsafe impl<T: Zeroable> Zeroable for Hsv<T> {}
unsafe impl<T: Pod> Pod for Hsv<T> {}

impl<T> Hsv<T> {
    /// Create a new HSV color.
    #[inline]
    pub const fn new(h: T, s: T, v: T) -> Self {
        hsv(h, s, v)
    }
}

impl<T: Channel + Float, F: Channel> FromRgb<F> for Hsv<T> {
    fn from_rgb(val: Rgb<F>) -> Self {
        let Rgb { r, g, b }: Rgb<T> = val.to_rgb();

        let min = T::min(T::min(r, g), b);
        let max = T::max(T::max(r, g), b);
        let delta = max - min;

        let v = max;
        let s = match max > T::from_f64_channel(0.001) {
            true => delta / max,
            false => T::ZERO,
        };
        let h = if delta == T::ZERO {
            T::ZERO
        } else {
            if r == max {
                (g - b) / delta
            } else if g == max {
                T::TWO + (b - r) / delta
            } else {
                T::TWO + T::TWO + (r - g) / delta
            }
        };

        let h = (h * T::NUM_60 + T::NUM_360) % T::NUM_360;

        hsv(h, s, v)
    }
}

impl<T: Channel, F: Channel + Float> ToRgb<T> for Hsv<F> {
    fn to_rgb(self) -> Rgb<T> {
        let c = self.v * self.s;
        let x = c * (F::ONE - F::abs(((self.h / F::NUM_60) % F::TWO) - F::ONE));
        let m = self.v - c;
        let (r, g, b) = match (self.h / F::NUM_60).to_u8() {
            0 => ((c + m), (x + m), m),
            1 => ((x + m), (c + m), m),
            2 => (m, (c + m), (x + m)),
            3 => (m, (x + m), (c + m)),
            4 => ((x + m), m, (c + m)),
            _ => ((c + m), m, (x + m)),
        };
        Rgb::new(r, g, b).to_rgb()
    }
}

impl<T: Channel, F: Channel + Float> ToRgba<T> for Hsv<F> {
    #[inline]
    fn to_rgba(self) -> Rgba<T> {
        let Rgb { r, g, b }: Rgb<T> = self.to_rgb();
        Rgba::new(r, g, b, T::CHANNEL_MAX)
    }
}
