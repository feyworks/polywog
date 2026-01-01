use crate::core::VideoMode;
use crate::math::{Vec2I, Vec2U};
use std::fmt::{Debug, Formatter};
use winit::monitor::MonitorHandle;

/// Handle to a monitor.
///
/// This handle can be cloned and passed around freely to give objects access to the monitor.
///
/// Can be obtained from a [`Window`](super::Window).
#[derive(Clone, PartialEq)]
pub struct Monitor(pub(crate) MonitorHandle);

impl Debug for Monitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Monitor").finish_non_exhaustive()
    }
}

impl Monitor {
    /// The monitor name.
    #[inline]
    pub fn name(&self) -> Option<String> {
        self.0.name()
    }

    /// The monitor's dpi-independent size.
    #[inline]
    pub fn size(&self) -> Vec2U {
        self.0.size().to_logical(self.0.scale_factor()).into()
    }

    /// The monitor size in pixels.
    #[inline]
    pub fn pixel_size(&self) -> Vec2U {
        self.0.size().into()
    }

    /// The monitor dpi-independent position.
    #[inline]
    pub fn pos(&self) -> Vec2I {
        self.0.position().to_logical(self.0.scale_factor()).into()
    }

    /// The monitor position in pixels.
    #[inline]
    pub fn pixel_pos(&self) -> Vec2I {
        self.0.position().into()
    }

    /// The monitor's refresh rate.
    #[inline]
    pub fn refresh_rate_mhz(&self) -> Option<u32> {
        self.0.refresh_rate_millihertz()
    }

    /// The monitor's scale factor.
    #[inline]
    pub fn scale_factor(&self) -> f64 {
        self.0.scale_factor()
    }

    /// The monitor's available video modes.
    #[inline]
    pub fn video_modes(&self) -> impl Iterator<Item = VideoMode> {
        self.0.video_modes().map(VideoMode)
    }
}
