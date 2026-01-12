use crate::gfx::{Graphics, Texture, TexturePacker};
use crate::prelude::SubTexture;
use fey_font::Font as FeyFont;
use fey_math::Vec2F;
use fnv::FnvHashMap;
use std::fmt::{Debug, Formatter};
use std::path::Path;

/// A drawable font.
pub struct Font {
    size: f32,
    pixelated: bool,
    glyphs: FnvHashMap<char, Glyph>,
    kerning: FnvHashMap<(char, char), f32>,
}

impl Debug for Font {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font").finish_non_exhaustive()
    }
}

/// A drawable font glyph.
#[derive(Debug, Clone, PartialEq)]
pub struct Glyph {
    pub sub: Option<SubTexture>,
    pub adv: f32,
}

impl Font {
    pub fn new(size: f32, pixelated: bool) -> Self {
        Self {
            size,
            pixelated,
            glyphs: FnvHashMap::default(),
            kerning: FnvHashMap::default(),
        }
    }

    pub fn from_ttf_bytes(
        gfx: &Graphics,
        font: &[u8],
        size: f32,
        pixelated: bool,
        chars: impl Iterator<Item = char>,
    ) -> Result<Option<(Self, Texture)>, fey_font::FontError> {
        let font = FeyFont::from_slice(font, size)?;
        Ok(Self::pack(gfx, font, pixelated, chars))
    }

    pub fn from_ttf_file(
        gfx: &Graphics,
        path: impl AsRef<Path>,
        size: f32,
        pixelated: bool,
        chars: impl Iterator<Item = char>,
    ) -> Result<Option<(Self, Texture)>, fey_font::FontError> {
        let font = FeyFont::from_file(path, size)?;
        Ok(Self::pack(gfx, font, pixelated, chars))
    }

    fn pack(
        gfx: &Graphics,
        font: FeyFont<'_>,
        pixelated: bool,
        chars: impl Iterator<Item = char>,
    ) -> Option<(Self, Texture)> {
        let mut packer = TexturePacker::new();

        // rasterize and pack all glyphs, collect their char/advance/offset
        let chars: Vec<(char, f32, Vec2F)> = chars
            .enumerate()
            .map(|(i, chr)| {
                let g = font.char_glyph(chr);
                if chr == ' ' {
                    println!("{}", g.advance());
                }
                let raster = match pixelated {
                    true => g.rasterize_pixelated(),
                    false => g.rasterize_smooth(),
                };
                let off = match raster {
                    Some(raster) => {
                        packer.add_image(i, raster.image, None, None);
                        raster.offset
                    }
                    None => Vec2F::ZERO,
                };
                (chr, g.advance(), off)
            })
            .collect();

        // build the kerning table
        let mut kerning = FnvHashMap::default();
        for left in chars.iter().map(|(chr, _, _)| *chr) {
            for right in chars.iter().map(|(chr, _, _)| *chr) {
                let kern = font.char_kerning(left, right);
                if kern != 0.0 {
                    kerning.insert((left, right), kern);
                }
            }
        }

        // pack the atlas
        let (tex, mut subs) = packer.pack(gfx)?;

        // build the glyph list and apply offset to the subtextures
        let glyphs = chars
            .into_iter()
            .enumerate()
            .map(|(i, (chr, adv, off))| {
                let mut sub = subs.remove(&i);
                if let Some(sub) = sub.as_mut() {
                    sub.offset.x += off.x;
                    sub.offset.y -= off.y;
                };
                (chr, Glyph { sub, adv })
            })
            .collect();

        // return the packed font and its texture (in case the user wants it)
        Some((
            Self {
                size: font.size(),
                pixelated,
                glyphs,
                kerning,
            },
            tex,
        ))
    }

    #[inline]
    pub fn size(&self) -> f32 {
        self.size
    }

    #[inline]
    pub fn pixelated(&self) -> bool {
        self.pixelated
    }

    #[inline]
    pub fn set_glyph(&mut self, chr: char, glyph: Glyph) {
        self.glyphs.insert(chr, glyph);
    }

    #[inline]
    pub fn glyph(&self, chr: char) -> Option<&Glyph> {
        self.glyphs.get(&chr)
    }

    #[inline]
    pub fn set_kerning(&mut self, left: char, right: char, kerning: f32) {
        self.kerning.insert((left, right), kerning);
    }

    #[inline]
    pub fn kerning(&self, left: char, right: char) -> Option<f32> {
        self.kerning.get(&(left, right)).copied()
    }
}
