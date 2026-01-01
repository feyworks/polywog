use super::{Monitor, VideoMode};

/// A window display mode.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMode {
    /// If monitor is `None`, will center on the current monitor.
    Windowed(Option<Monitor>),

    FullscreenExclusive(VideoMode),

    /// If monitor is `None`, will fullscreen on the current monitor.
    FullscreenBorderless(Option<Monitor>),
}

impl Default for DisplayMode {
    #[inline]
    fn default() -> Self {
        Self::Windowed(None)
    }
}
