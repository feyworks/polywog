use crate::color::{Grey, GreyAlpha, Rgba};
use crate::gfx::TextureFormat;
use bytemuck::{Pod, Zeroable};

mod private {
    use crate::color::{Grey, GreyAlpha, Rgba};
    use crate::img::Pixel;

    pub trait Sealed: Pixel {}

    impl Sealed for Grey<u8> {}
    impl Sealed for Grey<u16> {}
    impl Sealed for Grey<f32> {}
    impl Sealed for GreyAlpha<u8> {}
    impl Sealed for GreyAlpha<u16> {}
    impl Sealed for GreyAlpha<f32> {}
    impl Sealed for Rgba<u8> {}
    impl Sealed for Rgba<u16> {}
    impl Sealed for Rgba<f32> {}
}

/// A color type that can be used in textures.
pub trait TexturePixel: private::Sealed + Zeroable + Pod {
    /// This color type's texture format.
    const TEXTURE_FORMAT: TextureFormat;
}

macro_rules! impl_pixel {
    ($color:ty, $format:ident) => {
        impl TexturePixel for $color {
            const TEXTURE_FORMAT: TextureFormat = TextureFormat::$format;
        }
    };
}

impl_pixel!(Grey<u8>, R8);
impl_pixel!(Grey<u16>, R16);
impl_pixel!(Grey<f32>, R32F);
impl_pixel!(GreyAlpha<u8>, Rg8);
impl_pixel!(GreyAlpha<u16>, Rg16);
impl_pixel!(GreyAlpha<f32>, Rg32F);
impl_pixel!(Rgba<u8>, Rgba8);
impl_pixel!(Rgba<u16>, Rgba16);
impl_pixel!(Rgba<f32>, Rgba32F);
