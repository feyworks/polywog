use super::Time;
use crate::core::Window;
use crate::gfx::Graphics;
use crate::input::{Gamepads, Keyboard, Mouse};
use directories::ProjectDirs;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

/// Handle to the game's core systems.
///
/// This handle can be cloned and passed around freely to give objects access to the context.
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

    pub quit_requested: Cell<bool>,
    pub dirs: ProjectDirs,
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

    /// Returns the path to the game's cache directory.
    ///
    /// |Platform | Example                                              |
    /// | ------- | ---------------------------------------------------- |
    /// | Linux   | /home/alice/.cache/appname                           |
    /// | macOS   | /Users/Alice/Library/Caches/Org-Name.App-Name        |
    /// | Windows | C:\Users\Alice\AppData\Local\Org Name\App Name\cache |
    #[inline]
    pub fn cache_dir(&self) -> &Path {
        self.dirs.cache_dir()
    }

    /// Returns the path to the game's config directory.
    ///
    /// |Platform | Example                                                    |
    /// | ------- | ---------------------------------------------------------- |
    /// | Linux   | /home/alice/.config/appname                                |
    /// | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
    /// | Windows | C:\Users\Alice\AppData\Roaming\Org Name\App Name\config    |
    #[inline]
    pub fn config_dir(&self) -> &Path {
        self.dirs.config_dir()
    }

    /// Returns the path to the game's local config directory.
    ///
    /// |Platform | Example                                                    |
    /// | ------- | ---------------------------------------------------------- |
    /// | Linux   | /home/alice/.config/appname                                |
    /// | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
    /// | Windows | C:\Users\Alice\AppData\Local\Org Name\App Name\config      |
    #[inline]
    pub fn config_local_dir(&self) -> &Path {
        self.dirs.config_local_dir()
    }

    /// Returns the path to the game's data directory.
    ///
    /// |Platform | Example                                                    |
    /// | ------- | ---------------------------------------------------------- |
    /// | Linux   | /home/alice/.local/share/appname                           |
    /// | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
    /// | Windows | C:\Users\Alice\AppData\Roaming\Org Name\App Name\data      |
    #[inline]
    pub fn data_dir(&self) -> &Path {
        self.dirs.data_dir()
    }

    /// Returns the path to the game's local data directory.
    ///
    /// |Platform | Example                                                    |
    /// | ------- | ---------------------------------------------------------- |
    /// | Linux   | /home/alice/.local/share/appname                           |
    /// | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
    /// | Windows | C:\Users\Alice\AppData\Local\Org Name\App Name\data        |
    #[inline]
    pub fn data_local_dir(&self) -> &Path {
        self.dirs.data_local_dir()
    }

    /// Returns the path to the game's preference directory.
    ///
    /// |Platform | Example                                                 |
    /// | ------- | ------------------------------------------------------- |
    /// | Linux   | /home/alice/.config/appname                             |
    /// | macOS   | /Users/Alice/Library/Preferences/Org-Name.App-Name      |
    /// | Windows | C:\Users\Alice\AppData\Roaming\Org Name\App Name\config |
    #[inline]
    pub fn preferences_dir(&self) -> &Path {
        self.dirs.preference_dir()
    }
}
