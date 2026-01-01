use crate::gfx::{SubTexture, TextureFormat};
use crate::grid::VecGrid;
use crate::math::{Numeric, RectU, Vec2U};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use wgpu::{
    Device, Extent3d, Origin3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
    TextureDescriptor, TextureDimension, TextureUsages,
};

/// Handle to a drawable 2D texture.
///
/// This handle can be cloned and passed around freely to give objects access to the texture.
///
/// Textures are created from [`Graphics`](super::Graphics).
#[derive(Clone)]
pub struct Texture(pub(crate) Arc<Inner>);

impl Debug for Texture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Texture").finish_non_exhaustive()
    }
}

impl PartialEq for Texture {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for Texture {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0))
    }
}

#[derive(Debug)]
pub(crate) struct Inner {
    pub texture: wgpu::Texture,
    queue: Queue,
    size: Vec2U,
    format: TextureFormat,
}

impl Texture {
    pub(crate) fn new(
        device: &Device,
        queue: Queue,
        size: Vec2U,
        format: TextureFormat,
        surface: bool,
    ) -> Self {
        let mut usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;
        if surface {
            usage |= TextureUsages::RENDER_ATTACHMENT;
        }
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: format.into(),
            usage,
            view_formats: &[],
        });
        Self(Arc::new(Inner {
            texture,
            queue,
            size,
            format,
        }))
    }

    pub(crate) fn upload_bytes(&self, data: &[u8]) {
        assert_eq!(data.len(), self.size_in_bytes());
        let (width, height) = self.0.size.into();
        let bytes_per_row = Some(self.0.format.bytes_per_pixel().to_u32() * width);
        let rows_per_image = Some(height);
        self.0.queue.write_texture(
            TexelCopyTextureInfo {
                texture: &self.0.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row,
                rows_per_image,
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    // pub fn upload_pixels<P: TexturePixel>(&self, pixels: &[P]) -> Result<(), TextureUploadError> {
    //     if P::TEXTURE_FORMAT != self.format() {
    //         return Err(TextureUploadError::FormatMismatch {
    //             expected: self.format(),
    //             got: P::TEXTURE_FORMAT,
    //         });
    //     }
    //     if pixels.len() < self.pixel_count() {
    //         return Err(TextureUploadError::InsufficientPixels {
    //             expected: self.pixel_count(),
    //             got: pixels.len(),
    //         });
    //     }
    //     self.upload_bytes(cast_slice(pixels));
    //     Ok(())
    // }

    // pub fn upload_img<P: TexturePixel, S: AsRef<[P::Channel]>>(
    //     &self,
    //     img: &Image<P, S>,
    // ) -> Result<(), TextureUploadError> {
    //     if self.size() == img.size() {
    //         self.upload_pixels(img.pixels())
    //     } else {
    //         Err(TextureUploadError::InvalidSize {
    //             expected: self.size(),
    //             got: img.size(),
    //         })
    //     }
    // }

    /// Size of the texture in pixels.
    #[inline]
    pub fn size(&self) -> Vec2U {
        self.0.size
    }

    /// Width of the texture in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.size.x
    }

    /// Height of the texture in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.0.size.y
    }

    /// The texture's format.
    #[inline]
    pub fn format(&self) -> TextureFormat {
        self.0.format
    }

    /// How many pixels are in the texture.
    #[inline]
    pub fn pixel_count(&self) -> usize {
        let size = self.0.size.to_usize();
        size.x * size.y
    }

    /// The texture's total size in bytes.
    #[inline]
    pub fn size_in_bytes(&self) -> usize {
        self.pixel_count() * self.0.format.bytes_per_pixel()
    }

    /// Create a sub-texture from a region of this texture.
    #[inline]
    pub fn sub(&self, rect: impl Into<RectU>) -> SubTexture {
        SubTexture::new(self.clone(), rect.into().to_f32())
    }

    /// Split the texture into a grid of tiles.
    #[inline]
    pub fn split_into_tiles(&self, tile_size: impl Into<Vec2U>) -> VecGrid<SubTexture> {
        let tile_size = tile_size.into();
        let grid_size = self.size() / tile_size;
        VecGrid::new_from(grid_size, |tile| {
            self.sub(RectU::pos_size(tile * tile_size, tile_size))
        })
    }
}

// /// An error uploading data to a texture.
// #[derive(Debug, thiserror::Error)]
// pub enum TextureUploadError {
//     #[error("tried to upload pixels of type {expected:?} to texture of type {got:?}")]
//     FormatMismatch {
//         expected: TextureFormat,
//         got: TextureFormat,
//     },
//
//     #[error("tried to upload {got:?} pixels to texture that requires at least {expected:?}")]
//     InsufficientPixels { expected: usize, got: usize },
//
//     #[error("tried to upload an image of size ({got}) to a texture of size ({expected})")]
//     InvalidSize { expected: Vec2U, got: Vec2U },
// }

impl AsRef<Texture> for Texture {
    #[inline]
    fn as_ref(&self) -> &Texture {
        self
    }
}
