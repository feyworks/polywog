use crate::ImageFormat;
use bytemuck::{Pod, Zeroable};
use fey_color::{Channel, Grey, GreyAlpha, Rgb, Rgba};
use std::fmt::Debug;

/// A value that can be an image's pixel type.
pub trait Pixel:
    private::Sealed + Debug + Copy + Clone + Default + PartialEq + Zeroable + Pod
{
    /// Format of this pixel.
    const FORMAT: ImageFormat;

    /// How many channels this pixel has.
    const NUM_CHANNELS: usize = Self::FORMAT.num_channels();

    /// How many bits each component of this pixel has.
    const BIT_DEPTH: usize = Self::FORMAT.bit_depth();

    /// How many bytes each component of this pixel has.
    const BYTE_DEPTH: usize = Self::FORMAT.byte_depth();

    /// How many bits are in this pixel.
    const BITS_PER_PIXEL: usize = Self::FORMAT.bits_per_pixel();

    /// How many bytes are in this pixel.
    const BYTES_PER_PIXEL: usize = Self::FORMAT.bytes_per_pixel();

    /// The data type of each channel of this pixel.
    type Channel: Channel;

    /// The pixel's alpha value.
    fn alpha(&self) -> Self::Channel;

    /// Premultiply the pixel by its alpha channel.
    fn premultiply(self) -> Self;

    fn as_slice(&self) -> &[Self::Channel];

    unsafe fn from_slice_unchecked(slice: &[Self::Channel]) -> Self;
}

mod private {
    pub trait Sealed {}

    impl<T> Sealed for fey_color::Grey<T> {}
    impl<T> Sealed for fey_color::GreyAlpha<T> {}
    impl<T> Sealed for fey_color::Rgb<T> {}
    impl<T> Sealed for fey_color::Rgba<T> {}
}

macro_rules! impl_grey {
    ($col:ident, $channel:ty, $format:ident) => {
        impl Pixel for $col<$channel> {
            const FORMAT: ImageFormat = ImageFormat::$format;
            type Channel = $channel;

            #[inline]
            fn alpha(&self) -> Self::Channel {
                <$channel>::CHANNEL_MAX
            }

            #[inline]
            fn premultiply(self) -> Self {
                self
            }

            #[inline]
            fn as_slice(&self) -> &[Self::Channel] {
                bytemuck::cast_slice(bytemuck::bytes_of(self))
            }

            #[inline]
            unsafe fn from_slice_unchecked(slice: &[Self::Channel]) -> Self {
                unsafe { Self(*slice.get_unchecked(0)) }
            }
        }
    };
}

macro_rules! impl_grey_alpha {
    ($col:ident, $channel:ty, $format:ident) => {
        impl Pixel for $col<$channel> {
            const FORMAT: ImageFormat = ImageFormat::$format;
            type Channel = $channel;

            #[inline]
            fn alpha(&self) -> Self::Channel {
                self.a
            }

            #[inline]
            fn premultiply(self) -> Self {
                Self::new(self.g.un_mul(self.a), self.a)
            }

            #[inline]
            fn as_slice(&self) -> &[Self::Channel] {
                bytemuck::cast_slice(bytemuck::bytes_of(self))
            }

            #[inline]
            unsafe fn from_slice_unchecked(slice: &[Self::Channel]) -> Self {
                unsafe { Self::new(*slice.get_unchecked(0), *slice.get_unchecked(1)) }
            }
        }
    };
}

macro_rules! impl_rgb {
    ($col:ident, $channel:ty, $format:ident) => {
        impl Pixel for $col<$channel> {
            const FORMAT: ImageFormat = ImageFormat::$format;
            type Channel = $channel;

            #[inline]
            fn alpha(&self) -> Self::Channel {
                <$channel>::CHANNEL_MAX
            }

            #[inline]
            fn premultiply(self) -> Self {
                self
            }

            #[inline]
            fn as_slice(&self) -> &[Self::Channel] {
                bytemuck::cast_slice(bytemuck::bytes_of(self))
            }

            #[inline]
            unsafe fn from_slice_unchecked(slice: &[Self::Channel]) -> Self {
                unsafe {
                    Self::new(
                        *slice.get_unchecked(0),
                        *slice.get_unchecked(1),
                        *slice.get_unchecked(2),
                    )
                }
            }
        }
    };
}

macro_rules! impl_rgba {
    ($col:ident, $channel:ty, $format:ident) => {
        impl Pixel for $col<$channel> {
            const FORMAT: ImageFormat = ImageFormat::$format;
            type Channel = $channel;

            #[inline]
            fn alpha(&self) -> Self::Channel {
                self.a
            }

            #[inline]
            fn premultiply(self) -> Self {
                Self::new(
                    self.r.un_mul(self.a),
                    self.g.un_mul(self.a),
                    self.b.un_mul(self.a),
                    self.a,
                )
            }

            #[inline]
            fn as_slice(&self) -> &[Self::Channel] {
                bytemuck::cast_slice(bytemuck::bytes_of(self))
            }

            #[inline]
            unsafe fn from_slice_unchecked(slice: &[Self::Channel]) -> Self {
                unsafe {
                    Self::new(
                        *slice.get_unchecked(0),
                        *slice.get_unchecked(1),
                        *slice.get_unchecked(2),
                        *slice.get_unchecked(3),
                    )
                }
            }
        }
    };
}

impl_grey!(Grey, u8, Grey8);
impl_grey!(Grey, u16, Grey16);
impl_grey!(Grey, f32, Grey32F);
impl_grey_alpha!(GreyAlpha, u8, GreyAlpha8);
impl_grey_alpha!(GreyAlpha, u16, GreyAlpha16);
impl_grey_alpha!(GreyAlpha, f32, GreyAlpha32F);
impl_rgb!(Rgb, u8, Rgb8);
impl_rgb!(Rgb, u16, Rgb16);
impl_rgb!(Rgb, f32, Rgb32F);
impl_rgba!(Rgba, u8, Rgba8);
impl_rgba!(Rgba, u16, Rgba16);
impl_rgba!(Rgba, f32, Rgba32F);
