use dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use winit::window::{Fullscreen, Window as WinitWindow};

use crate::math::{Vec2I, Vec2U};

use super::{DisplayMode, Monitor, VideoMode};

/// Handle to the window.
///
/// This handle can be cloned and passed around freely to give objects access to the window.
///
/// Obtained from [`Context`](super::Context).
#[derive(Clone)]
pub struct Window(pub(crate) Arc<WinitWindow>);

impl Debug for Window {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Window").finish_non_exhaustive()
    }
}

impl Window {
    /// The window title.
    #[inline]
    pub fn title(&self) -> String {
        self.0.title()
    }

    /// Set the window title.
    #[inline]
    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    /// The window scale factor.
    #[inline]
    pub fn scale_factor(&self) -> f32 {
        self.0.scale_factor() as f32
    }

    /// The inverse window scale factor.
    #[inline]
    pub fn inv_scale_factor(&self) -> f32 {
        self.scale_factor().recip()
    }

    /// The monitor the window is on.
    #[inline]
    pub fn monitor(&self) -> Option<Monitor> {
        self.0.current_monitor().map(Monitor)
    }

    /// The primary monitor.
    #[inline]
    pub fn primary_monitor(&self) -> Option<Monitor> {
        self.0.primary_monitor().map(Monitor)
    }

    /// All monitors.
    #[inline]
    pub fn monitors(&self) -> impl Iterator<Item = Monitor> {
        self.0.available_monitors().map(Monitor)
    }

    /// Center the window on the selected monitor.
    #[inline]
    pub fn center_on(&self, monitor: &Monitor) {
        let mon_pos = monitor.0.position();
        let mon_size = monitor.0.size();
        let win_size = self.0.outer_size();
        self.0.set_outer_position(PhysicalPosition::new(
            mon_pos.x + ((mon_size.width - win_size.width) / 2) as i32,
            mon_pos.y + ((mon_size.height - win_size.height) / 2) as i32,
        ));
    }

    /// The window display mode.
    #[inline]
    pub fn display_mode(&self) -> DisplayMode {
        match self.0.fullscreen() {
            Some(Fullscreen::Exclusive(mode)) => DisplayMode::FullscreenExclusive(VideoMode(mode)),
            Some(Fullscreen::Borderless(monitor)) => {
                DisplayMode::FullscreenBorderless(monitor.map(Monitor))
            }
            None => DisplayMode::Windowed(self.monitor()),
        }
    }

    /// Set the window display mode.
    #[inline]
    pub fn set_display_mode(&self, display_mode: DisplayMode) {
        match display_mode {
            DisplayMode::FullscreenExclusive(mode) => {
                self.0.set_fullscreen(Some(Fullscreen::Exclusive(mode.0)));
            }
            DisplayMode::FullscreenBorderless(monitor) => {
                self.0
                    .set_fullscreen(Some(Fullscreen::Borderless(monitor.map(|m| m.0))));
            }
            DisplayMode::Windowed(monitor) => {
                self.0.set_fullscreen(None);
                if let Some(monitor) = monitor {
                    self.center_on(&monitor);
                }
            }
        }
    }

    /// If the window is in a fullscreen mode.
    #[inline]
    pub fn is_fullscreen(&self) -> bool {
        self.0.fullscreen().is_some()
    }

    /// Set the window to borderless fullscreen mode.
    #[inline]
    pub fn set_fullscreen_borderless(&self, monitor: Option<Monitor>) {
        self.set_display_mode(DisplayMode::FullscreenBorderless(monitor));
    }

    /// Set the window to exclusive fullscreen mode.
    #[inline]
    pub fn set_fullscreen_exclusive(&self, video_mode: VideoMode) {
        self.set_display_mode(DisplayMode::FullscreenExclusive(video_mode));
    }

    /// Set the window to windowed mode.
    #[inline]
    pub fn set_windowed(&self, monitor: Option<Monitor>) {
        self.set_display_mode(DisplayMode::Windowed(monitor));
    }

    /// If the window has focus.
    #[inline]
    pub fn has_focus(&self) -> bool {
        self.0.has_focus()
    }

    /// The window position in pixels.
    #[inline]
    pub fn pixel_pos(&self) -> Option<Vec2I> {
        self.0.inner_position().ok().map(Vec2I::from)
    }

    /// The DPI-independent window position.
    #[inline]
    pub fn pos(&self) -> Option<Vec2I> {
        Some(
            self.0
                .inner_position()
                .ok()?
                .to_logical(self.0.scale_factor())
                .into(),
        )
    }

    /// The window outer position in pixels.
    #[inline]
    pub fn outer_pixel_pos(&self) -> Option<Vec2I> {
        self.0.outer_position().ok().map(Vec2I::from)
    }

    /// The DPI-independent window outer position.
    #[inline]
    pub fn outer_pos(&self) -> Option<Vec2I> {
        Some(
            self.0
                .outer_position()
                .ok()?
                .to_logical(self.0.scale_factor())
                .into(),
        )
    }

    /// Set the window outer DPI-independent position.
    #[inline]
    pub fn set_outer_pos(&self, pos: impl Into<Vec2I>) {
        let pos = pos.into();
        self.0
            .set_outer_position(LogicalPosition::new(pos.x, pos.y));
    }

    /// Set the window outer pixel position.
    #[inline]
    pub fn set_outer_pixel_pos(&self, pos: impl Into<Vec2I>) {
        let pos = pos.into();
        self.0
            .set_outer_position(PhysicalPosition::new(pos.x, pos.y));
    }

    /// The window pixel size.
    #[inline]
    pub fn pixel_size(&self) -> Vec2U {
        self.0.inner_size().into()
    }

    /// The window DPI-independent size.
    #[inline]
    pub fn size(&self) -> Vec2U {
        self.0.inner_size().to_logical(self.0.scale_factor()).into()
    }

    /// Request a DPI-independent size for the window.
    #[inline]
    pub fn request_size(&self, size: impl Into<Vec2U>) -> bool {
        let size = size.into();
        self.0
            .request_inner_size(LogicalSize::new(size.x, size.y))
            .is_none()
    }

    /// Request a pixel size for the window.
    #[inline]
    pub fn request_pixel_size(&self, size: impl Into<Vec2U>) -> bool {
        let size = size.into();
        self.0
            .request_inner_size(PhysicalSize::new(size.x, size.y))
            .is_none()
    }

    /// Outer pixel size of the window.
    #[inline]
    pub fn outer_pixel_size(&self) -> Vec2U {
        self.0.outer_size().into()
    }

    /// Outer DPI-independent size of the window.
    #[inline]
    pub fn outer_size(&self) -> Vec2U {
        self.0.outer_size().to_logical(self.0.scale_factor()).into()
    }

    /// DPI-independent center of the window.
    #[inline]
    pub fn center(&self) -> Vec2U {
        self.size() / 2
    }

    /// Pixel center of the window.
    #[inline]
    pub fn pixel_center(&self) -> Vec2U {
        self.pixel_size() / 2
    }

    /// If the window can be resized by the user.
    #[inline]
    pub fn resizable(&self) -> bool {
        self.0.is_resizable()
    }

    /// Set if the window can be resized by the user.
    #[inline]
    pub fn set_resizable(&self, resizable: bool) {
        self.0.set_resizable(resizable);
    }

    /// If the window is maximized.
    #[inline]
    pub fn maximized(&self) -> bool {
        self.0.is_maximized()
    }

    /// Set if the window is maximized.
    #[inline]
    pub fn set_maximized(&self, maximized: bool) {
        self.0.set_maximized(maximized);
    }

    /// If the window is minimized.
    #[inline]
    pub fn minimized(&self) -> Option<bool> {
        self.0.is_minimized()
    }

    /// Set if the window is minimized.
    #[inline]
    pub fn set_minimized(&self, minimized: bool) {
        self.0.set_minimized(minimized);
    }

    /// Set the window's minimum inner DPI-independent size.
    #[inline]
    pub fn set_min_inner_size(&self, size: impl Into<Option<Vec2U>>) {
        self.0
            .set_min_inner_size(size.into().map(|s| LogicalSize::new(s.x, s.y)));
    }

    /// Set the window's minimum inner pixel size.
    #[inline]
    pub fn set_min_inner_pixel_size(&self, size: impl Into<Option<Vec2U>>) {
        self.0
            .set_min_inner_size(size.into().map(|s| LogicalSize::new(s.x, s.y)));
    }

    /// Set the window's maximum inner DPI-independent size.
    #[inline]
    pub fn set_max_inner_size(&self, size: impl Into<Option<Vec2U>>) {
        self.0
            .set_max_inner_size(size.into().map(|s| LogicalSize::new(s.x, s.y)));
    }

    /// Set the window's maximum inner pixel size.
    #[inline]
    pub fn set_max_inner_pixel_size(&self, size: impl Into<Option<Vec2U>>) {
        self.0
            .set_max_inner_size(size.into().map(|s| LogicalSize::new(s.x, s.y)));
    }

    // #[inline]
    // pub fn set_cursor(&self, icon: CursorIcon) {
    //     self.0.set_cursor(icon);
    // }
}
