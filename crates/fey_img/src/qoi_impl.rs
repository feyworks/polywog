use crate::qoi_impl::private::Sealed;
use crate::{DynImage, ImageError, ImageRgb8, ImageRgba8};
use fey_grid::Grid;
use fey_math::{Numeric, Vec2U, vec2};
use qoi::Channels;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

impl DynImage {
    /// Load a QOI image. This will only ever return either a `DynImage::Rgb8`
    /// or `DynImage::Rgba8`, and so will always be Rgba8 convertible.
    pub fn load_qoi(mut r: impl Read) -> Result<Self, ImageError> {
        let mut bytes = Vec::new();
        r.read_to_end(&mut bytes)?;
        let (header, bytes) = qoi::decode_to_vec(&bytes)?;
        let size = vec2(header.width, header.height);
        Ok(match header.channels {
            Channels::Rgb => Self::Rgb8(ImageRgb8::from_raw(size, bytes)),
            Channels::Rgba => Self::Rgba8(ImageRgba8::from_raw(size, bytes)),
        })
    }

    /// Load a QOI image from file.
    #[inline]
    pub fn load_qoi_from_file(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Self::load_qoi(BufReader::new(File::open(path)?))
    }
}

#[inline]
fn save_qoi(bytes: &[u8], size: Vec2U, mut w: impl Write) -> Result<(), ImageError> {
    let size = size.to_u32();
    let bytes = qoi::encode_to_vec(bytes, size.x, size.y)?;
    w.write_all(&bytes)?;
    Ok(())
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::ImageRgb8 {}
    impl Sealed for crate::ImageRgba8 {}
}

/// An image that can be encoded as Qoi.
pub trait EncodeAsQoi: Sealed {
    /// Save a QOI image.
    fn save_qoi(&self, w: impl Write) -> Result<(), ImageError>;

    /// Save a QOI image to file.
    #[inline]
    fn save_qoi_to_file(&self, path: impl AsRef<Path>) -> Result<(), ImageError> {
        self.save_qoi(File::create(path)?)
    }
}

impl EncodeAsQoi for ImageRgb8 {
    #[inline]
    fn save_qoi(&self, w: impl Write) -> Result<(), ImageError> {
        save_qoi(self.channels(), self.size(), w)
    }
}

impl EncodeAsQoi for ImageRgba8 {
    #[inline]
    fn save_qoi(&self, w: impl Write) -> Result<(), ImageError> {
        save_qoi(self.channels(), self.size(), w)
    }
}
