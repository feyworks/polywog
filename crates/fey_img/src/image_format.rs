use serde::{Deserialize, Serialize};

/// The format an image can take.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageFormat {
    Grey8,
    Grey16,
    Grey32F,
    GreyAlpha8,
    GreyAlpha16,
    GreyAlpha32F,
    Rgb8,
    Rgb16,
    Rgb32F,
    Rgba8,
    Rgba16,
    Rgba32F,
}

impl ImageFormat {
    /// How many channels the format has.
    #[inline]
    pub const fn num_channels(self) -> usize {
        match self {
            Self::Grey8 | Self::Grey16 | Self::Grey32F => 1,
            Self::GreyAlpha8 | Self::GreyAlpha16 | Self::GreyAlpha32F => 2,
            Self::Rgb8 | Self::Rgb16 | Self::Rgb32F => 3,
            Self::Rgba8 | Self::Rgba16 | Self::Rgba32F => 4,
        }
    }

    /// How many bits each component has in this format.
    #[inline]
    pub const fn bit_depth(self) -> usize {
        match self {
            Self::Grey8 | Self::GreyAlpha8 | Self::Rgb8 | Self::Rgba8 => 8,
            Self::Grey16 | Self::GreyAlpha16 | Self::Rgb16 | Self::Rgba16 => 16,
            Self::Grey32F | Self::GreyAlpha32F | Self::Rgb32F | Self::Rgba32F => 32,
        }
    }

    /// How many bytes each component has in this format.
    #[inline]
    pub const fn byte_depth(self) -> usize {
        match self {
            Self::Grey8 | Self::GreyAlpha8 | Self::Rgb8 | Self::Rgba8 => 1,
            Self::Grey16 | Self::GreyAlpha16 | Self::Rgb16 | Self::Rgba16 => 2,
            Self::Grey32F | Self::GreyAlpha32F | Self::Rgb32F | Self::Rgba32F => 4,
        }
    }

    /// How many bits are in a pixel of this format.
    #[inline]
    pub const fn bits_per_pixel(self) -> usize {
        self.num_channels() * self.bit_depth()
    }

    /// How many bytes are in a pixel of this format.
    #[inline]
    pub const fn bytes_per_pixel(self) -> usize {
        self.num_channels() * self.byte_depth()
    }
}
