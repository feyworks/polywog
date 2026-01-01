use crate::core::Window;
use crate::gfx::Graphics;
use crate::input::{Gamepads, Keyboard, Mouse};
use std::cell::Cell;
use std::fmt::{Debug, Formatter};

use super::Time;

/// Context with the core systems.
#[derive(Clone)]
pub struct Context {
    pub window: Window,
    pub time: Time,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub gamepads: Gamepads,
    pub graphics: Graphics,
    pub(crate) quit_requested: Cell<bool>,
}

impl Debug for Context {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Context").finish_non_exhaustive()
    }
}

impl Context {
    #[inline]
    pub fn dt(&self) -> f32 {
        self.time.delta()
    }

    #[inline]
    pub fn quit(&self) {
        self.quit_requested.set(true);
    }

    #[inline]
    pub fn quit_requested(&self) -> bool {
        self.quit_requested.get()
    }
}
