use crate::img::ImageFormat;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, FromRepr, VariantArray};

/// The format of a texture.
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    FromRepr,
    EnumCount,
    VariantArray,
)]
pub enum TextureFormat {
    /// Each pixel is one `u8` greyscale value.
    R8,

    /// Each pixel is one `u16` greyscale value.
    R16,

    /// Each pixel is one `f32` greyscale value.
    R32F,

    /// Each pixel is a `u8` grey-alpha pair.
    Rg8,

    /// Each pixel is a `u16` grey-alpha pair.
    Rg16,

    /// Each pixel is a `f32` grey-alpha pair.
    Rg32F,

    /// Each pixel is a sequence of 4 `u8` RGBA values.
    Rgba8,

    /// Each pixel is a sequence of 4 `u16` RGBA values.
    Rgba16,

    /// Each pixel is a sequence of 4 `f32` RGBA values.
    Rgba32F,
}

impl TextureFormat {
    /// This texture format's image format equivalent.
    #[inline]
    pub const fn image_format(self) -> ImageFormat {
        match self {
            Self::R8 => ImageFormat::Grey8,
            Self::R16 => ImageFormat::Grey16,
            Self::R32F => ImageFormat::Grey32F,
            Self::Rg8 => ImageFormat::GreyAlpha8,
            Self::Rg16 => ImageFormat::GreyAlpha16,
            Self::Rg32F => ImageFormat::GreyAlpha32F,
            Self::Rgba8 => ImageFormat::Rgba8,
            Self::Rgba16 => ImageFormat::Rgba16,
            Self::Rgba32F => ImageFormat::Rgba32F,
        }
    }

    /// How many channels the texture format has.
    #[inline]
    pub const fn num_channels(self) -> usize {
        self.image_format().num_channels()
    }

    /// How many bits each component has in this texture format.
    #[inline]
    pub const fn bit_depth(self) -> usize {
        self.image_format().bit_depth()
    }

    /// How many bytes each component has in this texture format.
    #[inline]
    pub const fn byte_depth(self) -> usize {
        self.image_format().byte_depth()
    }

    /// How many bits are in a pixel of this texture format.
    #[inline]
    pub const fn bits_per_pixel(self) -> usize {
        self.image_format().bits_per_pixel()
    }

    /// How many bytes are in a pixel of this texture format.
    #[inline]
    pub const fn bytes_per_pixel(self) -> usize {
        self.image_format().bytes_per_pixel()
    }
}

impl Into<wgpu::TextureFormat> for TextureFormat {
    #[inline]
    fn into(self) -> wgpu::TextureFormat {
        type Format = wgpu::TextureFormat;
        match self {
            Self::R8 => Format::R8Unorm,
            Self::R16 => Format::R16Unorm,
            Self::R32F => Format::R32Float,
            Self::Rg8 => Format::Rg8Unorm,
            Self::Rg16 => Format::Rg16Unorm,
            Self::Rg32F => Format::Rg32Float,
            Self::Rgba8 => Format::Rgba8Unorm,
            Self::Rgba16 => Format::Rgba16Unorm,
            Self::Rgba32F => Format::Rgba32Float,
        }
    }
}
