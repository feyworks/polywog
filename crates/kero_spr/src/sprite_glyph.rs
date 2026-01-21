use crate::Sprite;

/// A single renderable [`SpriteFont`](crate::SpriteFont) glyph.
///
/// This is just a [`Sprite`] paired with a cursor-advance.
#[derive(Debug, Clone)]
pub struct SpriteGlyph {
    /// The glyph's sprite.
    pub sprite: Option<Sprite>,

    /// The glyphs's cursor-advance.
    pub advance: f32,
}

impl SpriteGlyph {
    /// Create a new sprite glyph.
    pub fn new(sprite: impl Into<Option<Sprite>>, advance: f32) -> Self {
        Self {
            sprite: sprite.into(),
            advance,
        }
    }
}
