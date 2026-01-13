use super::Time;
use crate::core::Window;
use crate::gfx::Graphics;
use crate::input::{Gamepads, Keyboard, Mouse};
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

/// Context with the core systems.
#[derive(Clone)]
pub struct Context(pub(crate) Rc<ContextData>);

/// Context data.
#[derive(Clone)]
pub struct ContextData {
    pub window: Window,
    pub time: Time,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub gamepads: Gamepads,
    pub graphics: Graphics,

    #[cfg(feature = "lua")]
    pub lua: mlua::WeakLua,

    #[cfg(feature = "lua")]
    pub reload_lua: Cell<bool>,

    pub(crate) quit_requested: Cell<bool>,
}

impl Deref for Context {
    type Target = ContextData;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Debug for Context {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Context").finish_non_exhaustive()
    }
}

impl Context {
    #[cfg(feature = "lua")]
    pub fn from_lua(lua: &mlua::Lua) -> mlua::AppDataRef<'_, Self> {
        lua.app_data_ref::<Self>().unwrap()
    }

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

    #[cfg(feature = "lua")]
    pub fn reload_lua(&self) {
        self.reload_lua.set(true);
    }

    #[cfg(feature = "lua")]
    pub fn reload_lua_requested(&self) -> bool {
        self.reload_lua.get()
    }
}
