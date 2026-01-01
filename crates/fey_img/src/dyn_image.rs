use crate::{
    ImageError, ImageFormat, ImageGrey8, ImageGrey16, ImageGrey32F, ImageGreyAlpha8,
    ImageGreyAlpha16, ImageGreyAlpha32F, ImageRgb8, ImageRgb16, ImageRgb32F, ImageRgba8,
    ImageRgba16, ImageRgba32F,
};
use fey_color::ToRgba;
use fey_grid::Grid;
use fey_math::{Vec2U, vec2};
use png::{BitDepth, ColorType, Decoder};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek};
use std::path::Path;

/// A dynamic image which can be any of the possible pixel formats.
///
/// This is useful for when you want to store an image, but it could be in
/// any format (such as when you load a PNG, which can produce images of
/// several different types).
///
/// The convenience method [`to_rgba8()`](DynImage::to_rgba8) is provided which
/// will infallibly convert from any of the potential image types.
#[derive(Clone)]
pub enum DynImage {
    Grey8(ImageGrey8),
    Grey16(ImageGrey16),
    Grey32F(ImageGrey32F),
    GreyAlpha8(ImageGreyAlpha8),
    GreyAlpha16(ImageGreyAlpha16),
    GreyAlpha32F(ImageGreyAlpha32F),
    Rgb8(ImageRgb8),
    Rgb16(ImageRgb16),
    Rgb32F(ImageRgb32F),
    Rgba8(ImageRgba8),
    Rgba16(ImageRgba16),
    Rgba32F(ImageRgba32F),
}

impl DynImage {
    /// The image's pixel format.
    pub fn format(&self) -> ImageFormat {
        match self {
            Self::Grey8(_) => ImageFormat::Grey8,
            Self::Grey16(_) => ImageFormat::Grey16,
            Self::Grey32F(_) => ImageFormat::Grey32F,
            Self::GreyAlpha8(_) => ImageFormat::GreyAlpha8,
            Self::GreyAlpha16(_) => ImageFormat::GreyAlpha16,
            Self::GreyAlpha32F(_) => ImageFormat::GreyAlpha32F,
            Self::Rgb8(_) => ImageFormat::Rgb8,
            Self::Rgb16(_) => ImageFormat::Rgb16,
            Self::Rgb32F(_) => ImageFormat::Rgb32F,
            Self::Rgba8(_) => ImageFormat::Rgba8,
            Self::Rgba16(_) => ImageFormat::Rgba16,
            Self::Rgba32F(_) => ImageFormat::Rgba32F,
        }
    }

    /// The image's size.
    pub fn size(&self) -> Vec2U {
        match self {
            Self::Grey8(img) => img.size(),
            Self::Grey16(img) => img.size(),
            Self::Grey32F(img) => img.size(),
            Self::GreyAlpha8(img) => img.size(),
            Self::GreyAlpha16(img) => img.size(),
            Self::GreyAlpha32F(img) => img.size(),
            Self::Rgb8(img) => img.size(),
            Self::Rgb16(img) => img.size(),
            Self::Rgb32F(img) => img.size(),
            Self::Rgba8(img) => img.size(),
            Self::Rgba16(img) => img.size(),
            Self::Rgba32F(img) => img.size(),
        }
    }

    /// The raw bytes of the image.
    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::Grey8(img) => img.bytes(),
            Self::Grey16(img) => img.bytes(),
            Self::Grey32F(img) => img.bytes(),
            Self::GreyAlpha8(img) => img.bytes(),
            Self::GreyAlpha16(img) => img.bytes(),
            Self::GreyAlpha32F(img) => img.bytes(),
            Self::Rgb8(img) => img.bytes(),
            Self::Rgb16(img) => img.bytes(),
            Self::Rgb32F(img) => img.bytes(),
            Self::Rgba8(img) => img.bytes(),
            Self::Rgba16(img) => img.bytes(),
            Self::Rgba32F(img) => img.bytes(),
        }
    }

    /// Convert this image to an [`ImageRgba8`].
    ///
    /// This function may result in loss of image precision if the contained pixel
    /// format is higher precision (eg. [`Rgba16`](Self::Rgba16)).
    pub fn to_rgba8(self) -> ImageRgba8 {
        match self {
            Self::Grey8(img) => img.map(|p| p.to_rgba()),
            Self::Grey16(img) => img.map(|p| p.to_rgba()),
            Self::Grey32F(img) => img.map(|p| p.to_rgba()),
            Self::GreyAlpha8(img) => img.map(|p| p.to_rgba()),
            Self::GreyAlpha16(img) => img.map(|p| p.to_rgba()),
            Self::GreyAlpha32F(img) => img.map(|p| p.to_rgba()),
            Self::Rgb8(img) => img.map(|p| p.to_rgba()),
            Self::Rgb16(img) => img.map(|p| p.to_rgba()),
            Self::Rgb32F(img) => img.map(|p| p.to_rgba()),
            Self::Rgba8(img) => img,
            Self::Rgba16(img) => img.map(|p| p.to_rgba()),
            Self::Rgba32F(img) => img.map(|p| p.to_rgba()),
        }
    }

    /// Load a PNG image file.
    pub fn load_png_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        Self::load_png(BufReader::new(File::open(path)?))
    }

    /// Load a PNG image from in-memory bytes.
    pub fn load_png_from_memory(bytes: &[u8]) -> Result<Self, ImageError> {
        Self::load_png(Cursor::new(bytes))
    }

    /// Load a PNG image.
    pub fn load_png<R: Read + Seek>(r: R) -> Result<Self, ImageError> {
        let decoder = Decoder::new(BufReader::new(r));
        let mut reader = decoder.read_info()?;

        let mut buf = Vec::new();
        buf.resize(
            reader
                .output_buffer_size()
                .expect("could not calculate output buffer size"),
            0,
        );
        let info = reader.next_frame(&mut buf)?;

        let size = vec2(info.width, info.height);

        match info.bit_depth {
            BitDepth::Eight => Ok(match info.color_type {
                ColorType::Grayscale => ImageGrey8::from_raw(size, buf).into(),
                ColorType::GrayscaleAlpha => ImageGreyAlpha8::from_raw(size, buf).into(),
                ColorType::Rgb => ImageRgb8::from_raw(size, buf).into(),
                ColorType::Rgba => ImageRgba8::from_raw(size, buf).into(),
                ColorType::Indexed => {
                    let pal = reader.info().palette.as_ref().unwrap().as_ref();
                    let buf = buf
                        .into_iter()
                        .map(|i| i as usize)
                        .map(|i| [pal[i * 3], pal[i * 3 + 1], pal[i * 3 + 2]])
                        .flatten()
                        .collect();
                    ImageRgb8::from_raw(size, buf).into()
                }
            }),
            BitDepth::Sixteen => {
                let buf = buf
                    .chunks_exact(2)
                    .map(|p| u16::from_le_bytes([p[0], p[1]]))
                    .collect();
                Ok(match info.color_type {
                    ColorType::Grayscale => ImageGrey16::from_raw(size, buf).into(),
                    ColorType::GrayscaleAlpha => ImageGreyAlpha16::from_raw(size, buf).into(),
                    ColorType::Rgb => ImageRgb16::from_raw(size, buf).into(),
                    ColorType::Rgba => ImageRgba16::from_raw(size, buf).into(),
                    ColorType::Indexed => unreachable!(),
                })
            }
            _ => Err(ImageError::UnsupportedBitDepth(info.bit_depth as usize)),
        }
    }

    /// Premultiply the image (if it has an alpha channel).
    #[inline]
    pub fn premultiply(&mut self) {
        match self {
            Self::GreyAlpha8(img) => img.premultiply(),
            Self::GreyAlpha16(img) => img.premultiply(),
            Self::GreyAlpha32F(img) => img.premultiply(),
            Self::Rgba8(img) => img.premultiply(),
            Self::Rgba16(img) => img.premultiply(),
            Self::Rgba32F(img) => img.premultiply(),
            _ => {}
        }
    }
}

macro_rules! impl_from {
    ($name:ty, $variant:ident) => {
        impl From<$name> for DynImage {
            #[inline]
            fn from(value: $name) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_from!(ImageGrey8, Grey8);
impl_from!(ImageGrey16, Grey16);
impl_from!(ImageGrey32F, Grey32F);
impl_from!(ImageGreyAlpha8, GreyAlpha8);
impl_from!(ImageGreyAlpha16, GreyAlpha16);
impl_from!(ImageGreyAlpha32F, GreyAlpha32F);
impl_from!(ImageRgb8, Rgb8);
impl_from!(ImageRgb16, Rgb16);
impl_from!(ImageRgb32F, Rgb32F);
impl_from!(ImageRgba8, Rgba8);
impl_from!(ImageRgba16, Rgba16);
impl_from!(ImageRgba32F, Rgba32F);
