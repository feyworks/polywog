use kero::gfx::SubTexture;

use kero::prelude::*;

/// A single renderable sprite.
///
/// Texture coordinates of sprites are paired with offsets, so the
/// sprite can be positioned ("framed"), which allows you do trim
/// the sprite, but still render it as if it was its full size.
#[derive(Debug, Clone)]
pub struct Sprite {
    pub sub: SubTexture,
}

impl Sprite {
    /// Create a new sprite from the rectangular sub-region of a texture's pixels.
    /// You can also provide a rendering offset and virtual size for the subtexture.
    #[inline]
    pub fn new_ext(texture: Texture, rect: RectF, offset: Vec2F, size: Vec2F) -> Self {
        Self {
            sub: SubTexture::new_ext(texture, rect, offset, size),
        }
    }

    /// Create a new sprite from the rectangular sub-region of a texture's pixels.
    #[inline]
    pub fn new(texture: Texture, rect: impl Into<RectF>) -> Self {
        Self {
            sub: SubTexture::new(texture, rect),
        }
    }

    /// Draw this sprite at the provided posiiton.
    #[inline]
    pub fn draw_flipped(
        &self,
        draw: &mut Draw,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        draw.subtexture_at_flipped(&self.sub, pos, color, mode, flip);
    }

    /// Draw this sprite at the provided position.
    #[inline]
    pub fn draw_ext(&self, draw: &mut Draw, pos: impl Into<Vec2F>, color: Rgba8, mode: ColorMode) {
        draw.subtexture_at_ext(&self.sub, pos, color, mode);
    }

    /// Draw this sprite at the provided position.
    #[inline]
    pub fn draw(&self, draw: &mut Draw, pos: impl Into<Vec2F>) {
        draw.subtexture_at(&self.sub, pos);
    }
}
