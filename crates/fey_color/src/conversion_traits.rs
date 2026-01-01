use crate::{Channel, Rgb, Rgba};

/// A color that can be converted from RGB.
pub trait FromRgb<C: Channel> {
    fn from_rgb(val: Rgb<C>) -> Self;
}

/// A color that can be converted to RGB.
pub trait ToRgb<C: Channel> {
    fn to_rgb(self) -> Rgb<C>;
}

/// A color that can be converted to RGBA.
pub trait FromRgba<C: Channel> {
    fn from_rgba(val: Rgba<C>) -> Self;
}

/// A color that can be converted from RGBA.
pub trait ToRgba<C: Channel> {
    fn to_rgba(self) -> Rgba<C>;
}

/// A channel that can be converted to linear color space.
///
/// Implemented on `f32` and `f64`.
pub trait ToLinear {
    fn to_linear(self) -> Self;
}

/// A channel that can be converted from linear color space.
///
/// Implemented on `f32` and `f64`.
pub trait FromLinear {
    fn from_linear(val: Self) -> Self;
}

macro_rules! impl_linear {
    ($name:ty) => {
        impl ToLinear for $name {
            #[inline]
            fn to_linear(self) -> Self {
                if self >= 0.0031308 {
                    1.055 * self.powf(1.0 / 2.4) - 0.055
                } else {
                    12.92 * self
                }
            }
        }

        impl FromLinear for $name {
            #[inline]
            fn from_linear(val: Self) -> Self {
                if val >= 0.04045 {
                    ((val + 0.055) / (1.0 + 0.055)).powf(2.4)
                } else {
                    val / 12.92
                }
            }
        }
    };
}

impl_linear!(f32);
impl_linear!(f64);
