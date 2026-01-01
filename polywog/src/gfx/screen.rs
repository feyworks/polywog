use crate::color::Rgba8;
use crate::core::{Context, Window};
use crate::gfx::{Draw, Surface, TextureFormat};
use crate::math::{Numeric, RectF, Vec2F, Vec2U};

/// The screen scaling method.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScreenMode {
    /// The screen buffer will always be `size`, but scaled up as large as possible to fit the
    /// window. If `fractional` is true, it will scale as much as possible, otherwise it will only
    /// scale to integer multiples, preventing pixel distortion. The screen will be framed in the
    /// center of the window.
    Frame { size: Vec2U, fractional: bool },

    /// The screen buffer will be the size of the window's backbuffer divided by `scale`. This
    /// allows you to `scale` up pixels but still have the screen fill the entire window.
    Fill { scale: f32 },
}

impl ScreenMode {
    /// Create a new [`Thumbnail`](ScreenMode::Frame) screen buffer.
    #[inline]
    pub fn frame(size: impl Into<Vec2U>, fractional: bool) -> Self {
        Self::Frame {
            size: size.into(),
            fractional,
        }
    }

    /// Create a new [`Scaled`](ScreenMode::Fill) screen buffer.
    #[inline]
    pub const fn fill(scale: f32) -> Self {
        Self::Fill { scale }
    }
}

/// A scaled screen buffer, useful for pixel art games.
#[derive(Debug)]
pub struct Screen {
    surface: Surface,
    pub mode: ScreenMode,
    scr_rect: RectF,
    win_rect: RectF,
    scale: f32,
    mouse_pos: Vec2F,
}

fn surface_size(window: &Window, scale: f32) -> Vec2U {
    assert!(scale > 0.0);
    let scale = scale * window.scale_factor();
    window.pixel_size() / scale.to_u32()
}

impl Screen {
    /// Create a new screen with the provided mode.
    pub fn new(ctx: &Context, mode: ScreenMode) -> Self {
        let size = match mode {
            ScreenMode::Frame { size, .. } => {
                assert_ne!(size.x, 0);
                assert_ne!(size.y, 0);
                size
            }
            ScreenMode::Fill { scale } => surface_size(&ctx.window, scale),
        };
        let surface = ctx.graphics.create_surface(size, TextureFormat::Rgba8);
        let mut screen = Self {
            surface,
            mode,
            scr_rect: RectF::ZERO,
            win_rect: RectF::ZERO,
            scale: 0.0,
            mouse_pos: Vec2F::ZERO,
        };
        screen.update(ctx);
        screen
    }

    /// Create a new screen in [`Frame`](ScreenMode::Frame) mode.
    pub fn new_frame(ctx: &Context, size: impl Into<Vec2U>, fractional: bool) -> Self {
        Self::new(ctx, ScreenMode::frame(size, fractional))
    }

    /// Create a new screen in [`Fill`](ScreenMode::Fill) mode.
    pub fn new_fill(ctx: &Context, scale: f32) -> Self {
        Self::new(ctx, ScreenMode::fill(scale))
    }

    /// The screen's target surface.
    #[inline]
    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    /// The screen's surface size.
    #[inline]
    pub fn size(&self) -> Vec2U {
        self.surface.size()
    }

    /// Width of the screen's surface.
    #[inline]
    pub fn width(&self) -> u32 {
        self.surface.width()
    }

    /// Height of the screen's surface.
    #[inline]
    pub fn height(&self) -> u32 {
        self.surface.height()
    }

    // #[inline]
    // pub fn screen_rect(&self) -> &RectF {
    //     &self.scr_rect
    // }

    /// Rectangle where the screen will be drawn to the window.
    #[inline]
    pub fn window_rect(&self) -> &RectF {
        &self.win_rect
    }

    /// Scale of the screen's pixels.
    #[inline]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Position of the mouse on screen, accounting for scaling/framing.
    #[inline]
    pub fn mouse_pos(&self) -> Vec2F {
        self.mouse_pos
    }

    /// X-position of the mouse on screen, accounting for scaling/framing.
    #[inline]
    pub fn mouse_x(&self) -> f32 {
        self.mouse_pos.x
    }

    /// Y-position of the mouse on screen, accounting for scaling/framing.
    #[inline]
    pub fn mouse_y(&self) -> f32 {
        self.mouse_pos.y
    }

    /// Update the screen, which will update the mouse position. If the screen needs to update its
    /// surface, it will replace its existing surface with a new resized one.
    #[inline]
    pub fn update(&mut self, ctx: &Context) {
        let (scr_size, fractional) = match self.mode {
            ScreenMode::Frame { size, fractional } => (size, fractional),
            ScreenMode::Fill { scale } => (surface_size(&ctx.window, scale), true),
        };
        assert_ne!(scr_size, Vec2U::ZERO);
        if self.surface.size() != scr_size {
            self.surface = ctx.graphics.create_surface(scr_size, TextureFormat::Rgba8);
        }
        let scr_size = scr_size.to_f32();

        self.scr_rect = RectF::sized(scr_size);

        let win_size = ctx.window.size().to_f32();
        let (win_rect, scale) = RectF::sized(win_size).fitted(scr_size, fractional);
        self.win_rect = win_rect;
        self.scale = scale;

        self.mouse_pos = win_rect.map_pos(ctx.mouse.pos(), &self.scr_rect).round();
    }

    /// Map a window position to an on-screen position.
    #[inline]
    pub fn map_pos(&self, pos: Vec2F) -> Vec2F {
        self.win_rect.map_pos(pos, &self.scr_rect)
    }

    /// Make this screen the drawing surface.
    #[inline]
    pub fn set_as_draw_surface(&self, draw: &mut Draw, clear_color: impl Into<Option<Rgba8>>) {
        draw.set_surface(Some(self.surface.clone()), clear_color.into());
    }

    /// Draw this screen to the window.
    #[inline]
    pub fn draw_to_window(&self, draw: &mut Draw, clear_color: impl Into<Option<Rgba8>>) {
        draw.set_surface(None, clear_color.into());
        draw.textured_quad(&self.surface, self.win_rect);
    }
}
