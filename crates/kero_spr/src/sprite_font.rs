use fnv::FnvHashMap;
use kero::prelude::*;

use crate::SpriteGlyph;

// A collection of glyphs to be rendered as text.
///
/// In addition to glyphs, fonts come with metrics that are
/// used to print out text correctly (eg. advance, kerning, etc.)
#[derive(Debug, Clone)]
pub struct SpriteFont {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub glyphs: FnvHashMap<char, SpriteGlyph>,
    pub kerning: FnvHashMap<(char, char), f32>,
}

impl SpriteFont {
    pub const DEFAULT_CHARS: &'static str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789*+=-–—<>_#&@%^~$.,!¡?¿:;`'\"‘’“”«»|/\\()[]{}ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÑÒÓÔÕÖÙÚÛÜàáâãäåæçèéêëìíîïñòóôõöùúûüŸÿßẞŒœ‚„°©®™¢€£¥•…‹›";

    /// Create a new empty font.
    pub fn new(ascent: f32, descent: f32, line_gap: f32) -> Self {
        Self {
            ascent,
            descent,
            line_gap,
            glyphs: FnvHashMap::default(),
            kerning: FnvHashMap::default(),
        }
    }

    /// The font's height.
    #[inline]
    pub fn height(&self) -> f32 {
        self.ascent - self.descent
    }

    /// The font's line-height.
    #[inline]
    pub fn line_height(&self) -> f32 {
        self.height() + self.line_gap
    }

    /// Get a reference to the glyph corresponding to the provided character.
    #[inline]
    pub fn glyph(&self, chr: char) -> Option<&SpriteGlyph> {
        self.glyphs.get(&chr)
    }

    /// Get the kerning value for the left-right character pair.
    #[inline]
    pub fn kerning(&self, left: char, right: char) -> Option<f32> {
        self.kerning.get(&(left, right)).copied()
    }

    /// Get the width of the provided text when rendered in this font.
    #[inline]
    pub fn text_width(&self, text: &str) -> f32 {
        let mut w: f32 = 0.0;
        let mut max_w: f32 = 0.0;
        for chr in text.chars() {
            if chr == '\n' {
                max_w = max_w.max(w);
                w = 0.0
            } else if let Some(g) = self.glyphs.get(&chr) {
                w += g.get().advance;
            }
        }
        max_w.max(w)
    }

    /// Get the height of the provided text when rendered in this font.
    #[inline]
    pub fn text_height(&self, text: &str, use_line_gap: bool) -> f32 {
        use_line_gap
            .then_some(self.line_height())
            .unwrap_or(self.height())
            * (text.lines().count() as f32)
    }

    /// Get the size of the provided text when rendered in this font.
    #[inline]
    pub fn text_size(&self, text: &str, use_line_gap: bool) -> Vec2<f32> {
        vec2(self.text_width(text), self.text_height(text, use_line_gap))
    }

    /// Generate a string that transforms `text` and inserts newlines so
    /// that it wraps inside a container with the provided `width`. The
    /// amount of lines in the resulting text is returned.
    pub fn word_wrap(&self, width: f32, text: &str, output: &mut String) -> usize {
        output.clear();
        let mut x = 0.0;
        let mut lines = 1;
        let space_w = self.text_width(" ");
        let mut first = true;
        for word in text.split_whitespace() {
            let word_w = self.text_width(word);
            if first {
                x += word_w;
                first = false;
            } else if x + space_w + word_w > width {
                x = word_w;
                lines += 1;
                output.push('\n');
            } else {
                x += space_w + word_w;
                output.push(' ');
            }
            output.push_str(word);
        }
        lines
    }

    pub fn draw_ext(
        &self,
        draw: &mut Draw,
        text: &str,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        let mut pos = pos.into();
        let left = pos.x;
        for chr in text.chars() {
            if chr == '\n' {
                pos.x = left;
                pos.y += self.line_height();
            } else if let Some(g) = self.glyphs.get(&chr).or_else(|| self.glyphs.get(&'\0')) {
                if let Some(spr) = g.sprite.as_ref() {
                    spr.draw_ext(draw, pos, color, mode);
                }
                pos.x += g.advance;
            } else {
                println!("no glyph for: [{}]", chr);
            }
        }
    }

    pub fn draw(&self, draw: &mut Draw, text: &str, pos: impl Into<Vec2F>, color: Rgba8) {
        self.draw_ext(draw, text, pos, color, ColorMode::MULT);
    }
}
