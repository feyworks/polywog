use crate::core::app_handler::AppHandler;
use crate::core::{Game, GameError};
use crate::math::Vec2U;
use winit::event_loop::EventLoop;

/// A builder for a game.
pub struct GameBuilder {
    pub title: String,
    pub size: Vec2U,

    pub app_organization: String,
    pub app_name: String,

    #[cfg(feature = "lua")]
    pub lua: mlua::Lua,
}

impl GameBuilder {
    pub fn new() -> Result<Self, GameError> {
        let this = Self {
            title: "New Game".to_string(),
            size: (1280, 720).into(),

            app_organization: String::new(),
            app_name: String::new(),

            #[cfg(feature = "lua")]
            lua: {
                let lua = mlua::Lua::new();
                crate::lua::TempTypes::init(&lua)?;
                lua
            },
        };

        #[cfg(feature = "lua")]
        let this = {
            use crate::lua_modules::*;
            this //
                .with_module::<fey_color::ColorModule>()?
                .with_module::<fey_guid::GuidModule>()?
                .with_module::<fey_img::ImageModule>()?
                .with_module::<fey_lua::InstantModule>()?
                .with_modules::<fey_math::MathModules>()?
                .with_module::<fey_rand::RandModule>()?
                .with_module::<AppModule>()?
                .with_module::<ColorModeModule>()?
                .with_module::<DrawModule>()?
                .with_module::<FontModule>()?
                .with_module::<IndexBufferModule>()?
                .with_module::<GamepadModule>()?
                .with_module::<GamepadButtonModule>()?
                .with_module::<GamepadAxisModule>()?
                .with_module::<KeyModule>()?
                .with_module::<KeyboardModule>()?
                .with_module::<MonitorModule>()?
                .with_module::<MouseModule>()?
                .with_module::<SamplerModule>()?
                .with_module::<ScreenModule>()?
                .with_module::<ShaderModule>()?
                .with_module::<SubTextureModule>()?
                .with_module::<SurfaceModule>()?
                .with_module::<TextureModule>()?
                .with_module::<TimeModule>()?
                .with_module::<VertexBufferModule>()?
                .with_module::<VertexModule>()?
                .with_module::<VideoModeModule>()?
                .with_module::<WindowModule>()?
        };

        Ok(this)
    }

    /// Run the game with a default [log](https://crates.io/crates/log) implementation initialized.
    /// If you want to bring your own logger, you can omit this call and initialize it before
    /// calling `run_game()`.
    pub fn with_default_logger(self) -> Self {
        env_logger::init();
        self
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

    /// Set the app information used to determine system directories.
    pub fn with_app_info(self, organization: &str, name: &str) -> Self {
        Self {
            app_organization: organization.to_string(),
            app_name: name.to_string(),
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

    #[cfg(feature = "lua")]
    pub fn run_lua(self) -> Result<(), GameError> {
        use crate::gfx::Draw;
        use crate::core::Context;

        pub struct LuaApp;

        impl Game for LuaApp {
            type Config = ();

            fn new(_ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
            where
                Self: Sized,
            {
                Ok(Self {})
            }

            fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
                Ok(())
            }

            fn render(&mut self, _ctx: &Context, _draw: &mut Draw) -> Result<(), GameError> {
                Ok(())
            }
        }

        self.run::<LuaApp>(())
    }
}
