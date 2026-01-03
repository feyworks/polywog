//! Font loading and glyph rasterization.

use ab_glyph::InvalidFont;
use ab_glyph::{Font as AbFont, FontRef, FontVec, ScaleFont};
use fey_color::GreyAlpha8;
use fey_grid::GridMut;
use fey_img::{Image, Pixel};
use fey_math::{Vec2, vec2};
use std::io::BufRead;
use std::path::Path;
use thiserror::Error;

/// A glyph ID.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GlyphId(u16);

impl GlyphId {
    pub const NUL: Self = Self(0);
}

#[derive(Debug)]
enum FontData<'a> {
    Ref(FontRef<'a>),
    Vec(FontVec),
}

/// A font file loaded from memory, with an assigned size.
#[derive(Debug)]
pub struct Font<'a> {
    font: FontData<'a>,
    size: f32,
    pt_size: f32,
}

impl<'a> Font<'a> {
    /// Load a font from a slice of bytes.
    pub fn from_slice(data: &'a [u8], size: f32) -> Result<Self, FontError> {
        let font = FontRef::try_from_slice(data)?;
        let pt_size = (font.height_unscaled() * size) / font.units_per_em().unwrap();
        Ok(Self {
            font: FontData::Ref(font),
            size,
            pt_size,
        })
    }

    /// Load a font from an owned vector of bytes.
    pub fn from_vec(data: Vec<u8>, size: f32) -> Result<Self, FontError> {
        let font = FontVec::try_from_vec(data)?;
        let pt_size = (font.height_unscaled() * size) / font.units_per_em().unwrap();
        Ok(Self {
            font: FontData::Vec(font),
            size,
            pt_size,
        })
    }

    /// Load a font from a reader.
    pub fn from_read<R: BufRead>(mut r: R, size: f32) -> Result<Self, FontError> {
        let mut data = Vec::new();
        _ = r.read_to_end(&mut data)?;
        Self::from_vec(data, size)
    }

    /// Load a font from a file.
    pub fn from_file<P: AsRef<Path>>(path: P, size: f32) -> Result<Self, FontError> {
        let data = std::fs::read(path)?;
        Self::from_vec(data, size)
    }

    /// Size the font was loaded with.
    pub const fn size(&self) -> f32 {
        self.size
    }

    /// The font ascender (how high it can rise above the baseline).
    #[inline]
    pub fn ascent(&self) -> f32 {
        match &self.font {
            FontData::Ref(f) => f.as_scaled(self.pt_size).ascent(),
            FontData::Vec(f) => f.as_scaled(self.pt_size).ascent(),
        }
    }

    /// The font descender (how low it can drop below the baseline).
    #[inline]
    pub fn descent(&self) -> f32 {
        match &self.font {
            FontData::Ref(f) => f.as_scaled(self.pt_size).descent(),
            FontData::Vec(f) => f.as_scaled(self.pt_size).descent(),
        }
    }

    /// The font's height (equal to `ascent - descent`).
    #[inline]
    pub fn height(&self) -> f32 {
        self.ascent() - self.descent()
    }

    /// How much space to put between printed lines.
    #[inline]
    pub fn line_gap(&self) -> f32 {
        match &self.font {
            FontData::Ref(f) => f.as_scaled(self.pt_size).line_gap(),
            FontData::Vec(f) => f.as_scaled(self.pt_size).line_gap(),
        }
    }

    /// How many glyphs are available in the font.
    #[inline]
    pub fn glyph_count(&self) -> usize {
        match &self.font {
            FontData::Ref(f) => f.glyph_count(),
            FontData::Vec(f) => f.glyph_count(),
        }
    }

    /// Get the glyph ID associated with a character.
    #[inline]
    pub fn char_id(&self, chr: char) -> GlyphId {
        GlyphId(match &self.font {
            FontData::Ref(f) => f.glyph_id(chr).0,
            FontData::Vec(f) => f.glyph_id(chr).0,
        })
    }

    /// Iterate through all glyph IDs in the font.
    #[inline]
    pub fn glyph_ids(&self) -> impl Iterator<Item = GlyphId> {
        (0..(self.glyph_count() as u16)).map(GlyphId)
    }

    /// Collect a list of all glyph/char pairs available.
    #[inline]
    pub fn glyph_chars(&self) -> Vec<(GlyphId, char)> {
        match &self.font {
            FontData::Ref(f) => f
                .codepoint_ids()
                .map(|(id, chr)| (GlyphId(id.0), chr))
                .collect(),
            FontData::Vec(f) => f
                .codepoint_ids()
                .map(|(id, chr)| (GlyphId(id.0), chr))
                .collect(),
        }
    }

    /// Retrieve glyph data associated with the glyph ID.
    #[inline]
    pub fn glyph(&self, id: GlyphId) -> Glyph<'_> {
        let glyph = ab_glyph::GlyphId(id.0).with_scale(self.pt_size);
        Glyph { font: self, glyph }
    }

    /// Retrieve the glyph data associated with the character.
    #[inline]
    pub fn char_glyph(&self, chr: char) -> Glyph<'_> {
        self.glyph(self.char_id(chr))
    }

    /// How much extra to advance the cursor when printing `right`
    /// after having just printed a `left`.
    #[inline]
    pub fn kerning(&self, left: GlyphId, right: GlyphId) -> f32 {
        let [left, right] = [left, right].map(|id| ab_glyph::GlyphId(id.0));
        match &self.font {
            FontData::Ref(f) => f.as_scaled(self.pt_size).kern(left, right),
            FontData::Vec(f) => f.as_scaled(self.pt_size).kern(left, right),
        }
    }

    /// How much extra to advance the cursor when printing `right`
    /// after having just printed a `left`.
    #[inline]
    pub fn char_kerning(&self, left: char, right: char) -> f32 {
        self.kerning(self.char_id(left), self.char_id(right))
    }
}

/// A font glyph.
#[derive(Debug)]
pub struct Glyph<'a> {
    font: &'a Font<'a>,
    glyph: ab_glyph::Glyph,
}

impl Glyph<'_> {
    /// The glyph's ID.
    #[inline]
    pub fn id(&self) -> GlyphId {
        GlyphId(self.glyph.id.0)
    }

    /// How much to advance the cursor after printing the glyph.
    #[inline]
    pub fn advance(&self) -> f32 {
        let id = self.glyph.id;
        match &self.font.font {
            FontData::Ref(f) => f.as_scaled(self.font.pt_size).h_advance(id),
            FontData::Vec(f) => f.as_scaled(self.font.pt_size).h_advance(id),
        }
    }

    /// How much to horizontally offset the glyph from the cursor position.
    #[inline]
    pub fn left_side_bearing(&self) -> f32 {
        let id = self.glyph.id;
        match &self.font.font {
            FontData::Ref(f) => f.as_scaled(self.font.pt_size).h_side_bearing(id),
            FontData::Vec(f) => f.as_scaled(self.font.pt_size).h_side_bearing(id),
        }
    }

    /// Rasterize the glyph, generating an image.
    pub fn rasterize<P: Pixel, F: FnMut(f32) -> P>(&self, mut f: F) -> Option<RasterizedGlyph<P>> {
        let outlined = match &self.font.font {
            FontData::Ref(f) => f
                .as_scaled(self.font.pt_size)
                .outline_glyph(self.glyph.clone()),
            FontData::Vec(f) => f
                .as_scaled(self.font.pt_size)
                .outline_glyph(self.glyph.clone()),
        }?;
        let bounds = outlined.px_bounds();
        let w = bounds.width().ceil() as u32;
        let h = bounds.height().ceil() as u32;
        let mut image = Image::new_vec((w, h), P::default());
        outlined.draw(|x, y, a| {
            image.set(x, y, f(a));
        });
        Some(RasterizedGlyph {
            image,
            offset: vec2(bounds.min.x, -bounds.min.y),
        })
    }

    /// Rasterize the glyph, generating a greyscale-alpha image where every pixel is
    /// either fully transparent or fully opaque white.
    #[inline]
    pub fn rasterize_pixelated(&self) -> Option<RasterizedGlyph<GreyAlpha8>> {
        self.rasterize(|a| {
            if a > 0.5 {
                GreyAlpha8::WHITE
            } else {
                GreyAlpha8::TRANSPARENT
            }
        })
    }

    /// Rasterize the glyph, generating a smooth greyscale-alpha image.
    #[inline]
    pub fn rasterize_smooth(&self) -> Option<RasterizedGlyph<GreyAlpha8>> {
        self.rasterize(|a| GreyAlpha8::new(255, (a * 255.0) as u8))
    }
}

/// A rasterized glyph with a drawing offset.
#[derive(Debug, Clone)]
pub struct RasterizedGlyph<P: Pixel> {
    pub image: Image<P>,
    pub offset: Vec2<f32>,
}

#[derive(Debug, Error)]
pub enum FontError {
    #[error("{0}")]
    Invalid(#[from] InvalidFont),

    #[error("{0}")]
    Io(#[from] std::io::Error),
}
