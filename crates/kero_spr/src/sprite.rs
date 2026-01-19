use std::ops::Deref;
use std::rc::Rc;

use kero::prelude::*;

/// A single renderable sprite.
#[derive(Debug, Clone)]
pub struct Sprite(Rc<SubTexture>);

impl From<SubTexture> for Sprite {
    fn from(value: SubTexture) -> Self {
        Self(Rc::new(value))
    }
}

impl Sprite {
    #[inline]
    pub fn new_ext(texture: Texture, rect: RectF, offset: Vec2F, size: Vec2F) -> Self {
        Self::from(SubTexture::new_ext(texture, rect, offset, size))
    }

    #[inline]
    pub fn new(texture: Texture, rect: RectF) -> Self {
        Self::from(SubTexture::new(texture, rect))
    }

    #[inline]
    pub fn draw_flipped(
        &self,
        draw: &mut Draw,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        draw.subtexture_at_flipped(&self.0, pos, color, mode, flip);
    }

    #[inline]
    pub fn draw_ext(&self, draw: &mut Draw, pos: impl Into<Vec2F>, color: Rgba8, mode: ColorMode) {
        draw.subtexture_at_ext(&self.0, pos, color, mode);
    }

    #[inline]
    pub fn draw(&self, draw: &mut Draw, pos: impl Into<Vec2F>) {
        draw.subtexture_at(&self.0, pos);
    }
}

impl Deref for Sprite {
    type Target = SubTexture;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
