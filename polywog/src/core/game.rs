use crate::core::{Context, GameError};
use crate::gfx::Draw;

/// Represents a game that can be passed to [`new_game()`](crate::new_game).
pub trait Game: 'static {
    /// Optional configuration that can be passed into [new()](Game::new).
    type Config;

    /// Called when the game starts to load any startup assets and create your game state.
    fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized;

    /// Called every update in order to perform game logic.
    fn update(&mut self, ctx: &Context) -> Result<(), GameError>;

    /// Called every frame refresh in order to perform game rendering.
    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError>;
}
