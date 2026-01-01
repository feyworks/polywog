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
pub struct Time(Rc<TimeState>);

impl Debug for Time {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Time").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TimeState {
    pub fps: Cell<u32>,
    pub delta: Cell<f32>,
    pub since_startup: Cell<f32>,
    pub frame: Cell<u64>,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            fps: Cell::new(60),
            delta: Cell::new(1.0 / 60.0),
            since_startup: Cell::new(0.0),
            frame: Cell::new(0),
        }
    }
}

impl Time {
    pub(crate) fn new() -> Self {
        Self(Rc::new(TimeState::default()))
    }

    pub(crate) fn set_state(&self, state: TimeState) {
        self.0.fps.set(state.fps.get());
        self.0.delta.set(state.delta.get());
        self.0.since_startup.set(state.since_startup.get());
        self.0.frame.set(state.frame.get());
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
