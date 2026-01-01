use crate::core::Monitor;
use crate::math::Vec2U;
use std::fmt::{Debug, Formatter};
use winit::monitor::VideoModeHandle;

/// Handle to a monitor's video mode.
///
/// This handle can be cloned and passed around freely to give objects access to the mode.
///
/// Obtained from a [`Monitor`](super::Monitor).
#[derive(Clone, PartialEq)]
pub struct VideoMode(pub VideoModeHandle);

impl Debug for VideoMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("VideoMode").finish_non_exhaustive()
    }
}

impl VideoMode {
    /// DPI-independent size of the mode.
    #[inline]
    pub fn size(&self) -> Vec2U {
        self.0
            .size()
            .to_logical(self.0.monitor().scale_factor())
            .into()
    }

    /// Pixel size of the mode.
    #[inline]
    pub fn pixel_size(&self) -> Vec2U {
        self.0.size().into()
    }

    /// The mode's monitor.
    #[inline]
    pub fn monitor(&self) -> Monitor {
        Monitor(self.0.monitor())
    }

    /// The mode's refresh rate.
    #[inline]
    pub fn refresh_rate(&self) -> u32 {
        self.0.refresh_rate_millihertz()
    }

    /// The mode's bit-depth.
    #[inline]
    pub fn bit_depth(&self) -> u16 {
        self.0.bit_depth()
    }
}
