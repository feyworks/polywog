use crate::{Sprite, SpriteAnim, SpriteFont, SpriteGlyph, SpritePatch, SpriteSheet};
use kero::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

// Represents a packed sprite atlas.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlas<I> {
    pub sprites: Vec<AtlasSprite<I>>,
    pub sheets: Vec<AtlasSheet<I>>,
    pub fonts: Vec<AtlasFont<I>>,
    pub patches: Vec<AtlasPatch<I>>,
    pub anims: Vec<AtlasAnim<I>>,
}

/// A packed sprite.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasSprite<I> {
    pub id: I,
    pub size: Vec2U,
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// A packed sheet.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasSheet<I> {
    pub id: I,
    pub tile_size: Vec2U,
    pub size: Vec2U,
    pub tiles: Vec<Option<AtlasTile>>,
}

/// A packed sheet tile.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTile {
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// A packed font.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasFont<I> {
    pub id: I,
    pub ascent: i32,
    pub descent: i32,
    pub line_gap: i32,
    pub glyphs: Vec<AtlasGlyph>,
    pub kerning: Vec<(char, char, i32)>,
}

/// A packed font glyph.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasGlyph {
    pub chr: char,
    pub adv: i32,
    pub size: Vec2U,
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// A packed 9-patch.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasPatch<I> {
    pub id: I,
    pub outer: RectU,
    pub inner: RectU,
}

/// A packed animation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasAnim<I> {
    pub id: I,
    pub size: Vec2U,
    pub cels: Vec<AtlasCel>,
    pub frames: Vec<AnimFrame>,
    pub tags: Vec<AnimTag>,
    pub layers: Vec<AnimLayer>,
}

/// A packed animation cel.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasCel {
    pub size: Vec2U,
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// Graphics assets generated from a sprite atlas.
pub struct AtlasGraphics<I> {
    pub texture: UserDataOf<TextureHandle>,
    pub sprites: Vec<(I, Sprite)>,
    pub sheets: Vec<(I, UserDataOf<SpriteSheet>)>,
    pub fonts: Vec<(I, UserDataOf<SpriteFont>)>,
    pub patches: Vec<(I, UserDataOf<SpritePatch>)>,
    pub anims: Vec<(I, UserDataOf<SpriteAnim>)>,
}

impl<I: Eq + Hash> AtlasGraphics<I> {
    /// Convert the graphics lists into hashmaps.
    pub fn mapped(self) -> AtlasGraphicsMapped<I> {
        AtlasGraphicsMapped {
            texture: self.texture,
            sprites: self.sprites.into_iter().collect(),
            sheets: self.sheets.into_iter().collect(),
            fonts: self.fonts.into_iter().collect(),
            patches: self.patches.into_iter().collect(),
            anims: self.anims.into_iter().collect(),
        }
    }
}

/// Hash-mapped graphics assets generated from a sprite atlas.
pub struct AtlasGraphicsMapped<I> {
    pub texture: UserDataOf<TextureHandle>,
    pub sprites: HashMap<I, UserDataOf<Sprite>>,
    pub sheets: HashMap<I, UserDataOf<SpriteSheet>>,
    pub fonts: HashMap<I, UserDataOf<SpriteFont>>,
    pub patches: HashMap<I, UserDataOf<SpritePatch>>,
    pub anims: HashMap<I, UserDataOf<SpriteAnim>>,
}

impl<I> SpriteAtlas<I> {
    /// Create renderable graphics assets from this sprite atlas.
    pub fn create_graphics(
        self,
        lua: &Lua,
        texture: UserDataOf<TextureHandle>,
    ) -> AtlasGraphics<I> {
        let sprites = self
            .sprites
            .into_iter()
            .map(|sprite| {
                (
                    sprite.id,
                    UserDataOf::new(
                        lua,
                        Sprite::new(
                            texture.clone(),
                            sprite.size.to_f32(),
                            sprite.rect.to_f32(),
                            sprite.off.to_f32(),
                        ),
                    ),
                )
            })
            .collect();

        let sheets = self
            .sheets
            .into_iter()
            .map(|sheet| {
                let tile_size = sheet.tile_size.to_f32();
                (
                    sheet.id,
                    UserDataOf::new(
                        lua,
                        SpriteSheet {
                            tiles: VecGrid::with_store(
                                sheet.size,
                                sheet
                                    .tiles
                                    .into_iter()
                                    .map(|tile| {
                                        tile.map(|tile| {
                                            UserDataOf::new(
                                                lua,
                                                Sprite::new(
                                                    texture.clone(),
                                                    tile_size,
                                                    tile.rect.to_f32(),
                                                    tile.off.to_f32(),
                                                ),
                                            )
                                        })
                                    })
                                    .collect(),
                            ),
                            tile_size,
                        },
                    ),
                )
            })
            .collect();

        let fonts = self
            .fonts
            .into_iter()
            .map(|font| {
                (
                    font.id,
                    UserDataOf::new(
                        lua,
                        SpriteFont {
                            ascent: font.ascent as f32,
                            descent: font.descent as f32,
                            line_gap: font.line_gap as f32,
                            glyphs: font
                                .glyphs
                                .into_iter()
                                .map(|g| {
                                    (
                                        g.chr,
                                        UserDataOf::new(
                                            lua,
                                            SpriteGlyph {
                                                sprite: (g.size.x > 0).then(|| {
                                                    UserDataOf::new(
                                                        lua,
                                                        Sprite::new(
                                                            texture.clone(),
                                                            g.size.to_f32(),
                                                            g.rect.to_f32(),
                                                            g.off.to_f32(),
                                                        ),
                                                    )
                                                }),
                                                advance: g.adv as f32,
                                            },
                                        ),
                                    )
                                })
                                .collect(),
                            kerning: font
                                .kerning
                                .into_iter()
                                .map(|(a, b, k)| ((a, b), k as f32))
                                .collect(),
                        },
                    ),
                )
            })
            .collect();

        let patches = self
            .patches
            .into_iter()
            .map(|patch| {
                (
                    patch.id,
                    UserDataOf::new(
                        lua,
                        SpritePatch::new(
                            texture.clone(),
                            patch.outer.to_f32(),
                            patch.inner.to_f32(),
                        ),
                    ),
                )
            })
            .collect();

        let anims = self
            .anims
            .into_iter()
            .map(|anim| {
                (anim.id, {
                    UserDataOf::new(
                        lua,
                        SpriteAnim {
                            size: anim.size.to_f32(),
                            frames: anim.frames,
                            sprites: {
                                anim.cels
                                    .into_iter()
                                    .map(|cel| {
                                        UserDataOf::new(
                                            lua,
                                            Sprite::new(
                                                texture.clone(),
                                                cel.size.to_f32(),
                                                cel.rect.to_f32(),
                                                cel.off.to_f32(),
                                            ),
                                        )
                                    })
                                    .collect()
                            },
                            tags: anim.tags,
                            layers: anim.layers,
                        },
                    )
                })
            })
            .collect();

        AtlasGraphics {
            texture,
            sprites,
            sheets,
            fonts,
            patches,
            anims,
        }
    }
}
