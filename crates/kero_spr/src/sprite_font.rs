use crate::SpriteGlyph;
use fnv::FnvHashMap;
use kero::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SpriteFont(Rc<Inner>);

#[derive(Debug)]
struct Inner {
    ascent: f32,
    descent: f32,
    line_gap: f32,
    glyphs: FnvHashMap<char, SpriteGlyph>,
    kerning: FnvHashMap<(char, char), f32>,
}

impl SpriteFont {
    pub fn new(
        ascent: f32,
        descent: f32,
        line_gap: f32,
        glyphs: FnvHashMap<char, SpriteGlyph>,
        kerning: FnvHashMap<(char, char), f32>,
    ) -> Self {
        Self(Rc::new(Inner {
            ascent,
            descent,
            line_gap,
            glyphs,
            kerning,
        }))
    }

    #[inline]
    pub fn ascent(&self) -> f32 {
        self.0.ascent
    }

    #[inline]
    pub fn descent(&self) -> f32 {
        self.0.descent
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.0.ascent - self.0.descent
    }

    /// The font's line-height.
    #[inline]
    pub fn line_height(&self) -> f32 {
        self.height() + self.0.line_gap
    }

    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.0.line_gap
    }

    #[inline]
    pub fn glyph(&self, chr: char) -> Option<&SpriteGlyph> {
        self.0.glyphs.get(&chr)
    }

    #[inline]
    pub fn kerning(&self, left: char, right: char) -> f32 {
        self.0.kerning.get(&(left, right)).copied().unwrap_or(0.0)
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
            } else if let Some(g) = self.0.glyphs.get(&chr) {
                w += g.advance();
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
    pub fn text_size(&self, text: &str, use_line_gap: bool) -> Vec2F {
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

    pub fn draw_text_ext(
        &mut self,
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
            } else if let Some(g) = self.glyph(chr).or_else(|| self.glyph('\0')) {
                if let Some(sub) = g.sub() {
                    draw.subtexture_at_ext(sub, pos, color, mode);
                }
                pos.x += g.advance();
            } else {
                println!("no glyph for: [{}]", chr);
            }
        }
    }

    #[inline]
    pub fn draw_text(&mut self, draw: &mut Draw, text: &str, pos: impl Into<Vec2F>, color: Rgba8) {
        self.draw_text_ext(draw, text, pos, color, ColorMode::MULT);
    }
}
