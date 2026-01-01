use crate::gfx::Texture;
use crate::math::{Numeric, RectF, Vec2F};

/// A drawable portion of a texture.
#[derive(Debug, Clone, PartialEq)]
pub struct SubTexture {
    pub texture: Texture,
    pub rect: RectF,
    pub offset: Vec2F,
    pub size: Vec2F,
    pub coords: [Vec2F; 4],
}

impl SubTexture {
    /// Create a new subtexture from the rectangular sub-region of a texture's pixels.
    /// You can also provide a rendering offset and virtual size for the subtexture.
    #[inline]
    pub fn new_ext(texture: Texture, rect: RectF, offset: Vec2F, size: Vec2F) -> Self {
        let tex_size = texture.size().to_f32();
        let coords = rect.corners().map(|p| p / tex_size);
        Self {
            texture,
            rect,
            offset,
            size,
            coords,
        }
    }

    /// Create a new subtexture from the rectangular sub-region of a texture's pixels.
    #[inline]
    pub fn new(texture: Texture, rect: impl Into<RectF>) -> Self {
        let rect = rect.into();
        Self::new_ext(texture, rect, Vec2F::ZERO, rect.size())
    }
}

impl From<(Texture, RectF)> for SubTexture {
    #[inline]
    fn from((texture, rect): (Texture, RectF)) -> Self {
        Self::new(texture, rect)
    }
}

impl AsRef<SubTexture> for SubTexture {
    #[inline]
    fn as_ref(&self) -> &SubTexture {
        self
    }
}

impl AsMut<SubTexture> for SubTexture {
    #[inline]
    fn as_mut(&mut self) -> &mut SubTexture {
        self
    }
}
