use crate::core::app_handler::AppHandler;
use crate::core::{Game, GameError};
use crate::math::Vec2U;
use winit::event_loop::EventLoop;

/// A builder for a game.
pub struct GameBuilder {
    pub title: String,
    pub size: Vec2U,
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            title: "New Game".to_string(),
            size: (1280, 720).into(),
        }
    }
}

impl GameBuilder {
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

    /// Run your game.
    pub fn run<G: Game>(self) -> Result<(), GameError> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut AppHandler::new::<G>(self))?;
        Ok(())
    }
}
