use crate::{Channel, FromLinear, FromRgb, Rgb, ToLinear, ToRgb};

/// An alias for [`Oklab<f32>`].
pub type OklabF = Oklab<f32>;

/// An Oklab color.
///
/// See: <https://bottosson.github.io/posts/oklab>
///
/// > *A perceptual color space is desirable when doing many kinds of image processing. It is useful for things like:*
/// >
/// > - *Turning an image grayscale, while keeping the perceived lightness the same*
/// > - *Increasing the saturation of colors, while maintaining perceived hue and lightness*
/// > - *Creating smooth and uniform looking transitions between colors*
pub struct Oklab<T> {
    pub l: T,
    pub a: T,
    pub b: T,
}

/// Create a new Oklab color.
#[inline]
pub const fn oklab<T>(l: T, a: T, b: T) -> Oklab<T> {
    Oklab { l, a, b }
}

impl<T> Oklab<T> {
    /// Create a new Oklab color.
    #[inline]
    pub const fn new(l: T, a: T, b: T) -> Self {
        oklab(l, a, b)
    }
}

macro_rules! impl_from_to_rgb {
    ($name:ty) => {
        impl<T: Channel + ToLinear> FromRgb<T> for Oklab<$name> {
            #[inline]
            fn from_rgb(val: Rgb<T>) -> Self {
                let Rgb { r, g, b }: Rgb<$name> = val.to_rgb().to_linear();
                let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
                let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
                let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
                oklab(
                    0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
                    1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
                    0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
                )
            }
        }

        impl<T: Channel + FromLinear> ToRgb<T> for Oklab<$name> {
            #[inline]
            fn to_rgb(self) -> Rgb<T> {
                let l = self.l + 0.3963377774 * self.a + 0.2158037573 * self.b;
                let m = self.l - 0.1055613458 * self.a - 0.0638541728 * self.b;
                let s = self.l - 0.0894841775 * self.a - 1.2914855480 * self.b;
                let l = l * l * l;
                let m = m * m * m;
                let s = s * s * s;
                Rgb::from_linear(
                    Rgb::new(
                        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
                        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
                        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
                    )
                    .to_rgb(),
                )
            }
        }
    };
}

impl_from_to_rgb!(f32);
impl_from_to_rgb!(f64);
