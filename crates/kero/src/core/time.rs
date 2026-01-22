use crate::math::Float;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Handle to the game's core timer.
///
/// This handle can be cloned and passed around freely to give objects access to the time.
///
/// Obtained from [`Context`](super::Context).
#[derive(Clone)]
pub struct Time(pub(crate) Rc<TimeState>);

impl Debug for Time {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Time").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TimeState {
    pub target_fps: Cell<Option<f64>>,
    pub max_frame_skip: Cell<u32>,
    pub fps: Cell<u32>,
    pub delta: Cell<f32>,
    pub unfixed_delta: Cell<f32>,
    pub since_startup: Cell<f32>,
    pub frame: Cell<u64>,
}

impl Default for TimeState {
    #[inline]
    fn default() -> Self {
        Self {
            target_fps: Cell::new(Some(60.0)),
            max_frame_skip: Cell::new(0),
            fps: Cell::new(60),
            delta: Cell::new(1.0 / 60.0),
            unfixed_delta: Cell::new(0.0),
            since_startup: Cell::new(0.0),
            frame: Cell::new(0),
        }
    }
}

impl Time {
    pub(crate) fn new() -> Self {
        Self(Rc::new(TimeState::default()))
    }

    #[inline]
    pub fn target_fps(&self) -> Option<f64> {
        self.0.target_fps.get()
    }

    #[inline]
    pub fn set_target_fps(&self, fps: Option<f64>) {
        self.0.target_fps.set(fps);
    }

    #[inline]
    pub fn max_frame_skip(&self) -> u32 {
        self.0.max_frame_skip.get()
    }

    #[inline]
    pub fn set_max_frame_skip(&self, max: u32) {
        self.0.max_frame_skip.set(max);
    }

    /// FPS the app is running at.
    #[inline]
    pub fn fps(&self) -> u32 {
        self.0.fps.get()
    }

    /// Duration since the last frame, in seconds.
    #[inline]
    pub fn delta(&self) -> f32 {
        self.0.delta.get()
    }

    /// Unfixed duration since last frame, in seconds.
    #[inline]
    pub fn unfixed_delta(&self) -> f32 {
        self.0.unfixed_delta.get()
    }

    /// Total time since app startup, in seconds.
    #[inline]
    pub fn since_startup(&self) -> f32 {
        self.0.since_startup.get()
    }

    /// The current frame number.
    #[inline]
    pub fn frame(&self) -> u64 {
        self.0.frame.get()
    }

    #[inline]
    pub fn flicker(&self, on_time: f32, off_time: f32) -> bool {
        (self.since_startup() % (on_time + off_time)) < on_time
    }

    #[inline]
    pub fn wave_ext(&self, from: f32, to: f32, duration: f32, offset_percent: f32) -> f32 {
        let range = (to - from) * 0.5;
        from + range
            + (((self.since_startup() + duration * offset_percent) / duration) * f32::TAU).sin()
                * range
    }

    #[inline]
    pub fn wave(&self, from: f32, to: f32, duration: f32) -> f32 {
        self.wave_ext(from, to, duration, 0.0)
    }
}
