use crate::core::app_handler::AppHandler;
use crate::core::{Game, GameError};
use crate::math::Vec2U;
use winit::event_loop::EventLoop;

/// A builder for a game.
pub struct GameBuilder {
    pub title: String,
    pub size: Vec2U,

    #[cfg(feature = "lua")]
    pub lua: mlua::Lua,
}

impl GameBuilder {
    pub fn new() -> Result<Self, GameError> {
        let this = Self {
            title: "New Game".to_string(),
            size: (1280, 720).into(),

            #[cfg(feature = "lua")]
            lua: {
                let lua = mlua::Lua::new();
                crate::lua::TempTypes::init(&lua)?;
                lua
            },
        };

        #[cfg(feature = "lua")]
        let this = { this.with_modules::<fey_math::MathModules>()? };

        Ok(this)
    }

    /// Set the title of the game window.
    pub fn with_title(self, title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..self
        }
    }

    /// Set the size of the game window.
    pub fn with_size(self, width: u32, height: u32) -> Self {
        Self {
            size: (width, height).into(),
            ..self
        }
    }

    #[cfg(feature = "lua")]
    pub fn with_module<M: crate::lua::LuaModule>(self) -> Result<Self, GameError> {
        let module = M::load(&self.lua)?;
        self.lua.preload_module(
            M::PATH,
            self.lua
                .create_function(move |_, _: ()| Ok(module.clone()))?,
        )?;
        Ok(self)
    }

    #[cfg(feature = "lua")]
    pub fn with_modules<M: crate::lua::LuaModules>(self) -> Result<Self, GameError> {
        for (name, loader) in M::loaders(&self.lua)? {
            self.lua.preload_module(name, loader)?;
        }
        Ok(self)
    }

    /// Run your game.
    pub fn run<G: Game>(self, cfg: G::Config) -> Result<(), GameError> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut AppHandler::<G>::new(self, cfg))?;
        Ok(())
    }
}
