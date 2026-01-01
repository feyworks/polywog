use crate::{Image, ImageError, Pixel};
use fey_color::{Grey8, Grey16, GreyAlpha8, GreyAlpha16, Rgb8, Rgb16, Rgba8, Rgba16};
use fey_grid::Grid;
use png::{BitDepth, ColorType, Encoder};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// A PNG-compatible pixel type.
///
/// Only images with this pixel type can be loaded from PNG files. The
/// supported pixel types are:
///
/// - [`Grey8`]
/// - [`Grey16`]
/// - [`GreyAlpha8`]
/// - [`GreyAlpha16`]
/// - [`Rgb8`]
/// - [`Rgb16`]
/// - [`Rgba8`]
/// - [`Rgba16`]
pub trait PngPixel: Pixel {
    /// Bit depth of the pixel.
    fn bit_depth() -> BitDepth;

    /// Color type of the pixel.
    fn color_type() -> ColorType;
}

macro_rules! impl_png_pixel {
    ($type:ty, $depth:ident, $color:ident) => {
        impl PngPixel for $type {
            #[inline]
            fn bit_depth() -> BitDepth {
                BitDepth::$depth
            }

            #[inline]
            fn color_type() -> ColorType {
                ColorType::$color
            }
        }
    };
}

impl_png_pixel!(Grey8, Eight, Grayscale);
impl_png_pixel!(Grey16, Sixteen, Grayscale);
impl_png_pixel!(GreyAlpha8, Eight, GrayscaleAlpha);
impl_png_pixel!(GreyAlpha16, Sixteen, GrayscaleAlpha);
impl_png_pixel!(Rgb8, Eight, Rgb);
impl_png_pixel!(Rgb16, Sixteen, Rgb);
impl_png_pixel!(Rgba8, Eight, Rgba);
impl_png_pixel!(Rgba16, Sixteen, Rgba);

impl<Px: PngPixel, S: AsRef<[Px::Channel]>> Image<Px, S> {
    /// Save the image as a PNG.
    pub fn save_png<W: Write>(&self, w: W) -> Result<(), ImageError> {
        let size = self.size();
        let mut enc = Encoder::new(w, size.x as u32, size.y as u32);
        enc.set_depth(Px::bit_depth());
        enc.set_color(Px::color_type());
        let mut writer = enc.write_header()?;
        writer.write_image_data(self.bytes())?;
        Ok(())
    }

    /// Save the image as a PNG file.
    #[inline]
    pub fn save_png_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self.save_png(BufWriter::new(File::create(path)?))
    }
}
