use crate::core::GameBuilder;

/// Set forth!
///
/// Here is a minimal example to run your game:
///
/// ```rust
/// use polywog::core::{Context, Game, GameError};
/// use polywog::gfx::Draw;
///
/// fn main() -> Result<(), GameError> {
///     polywog::new_game()
///         .with_title("Minimal")
///         .with_size(1280, 720)
///         .run::<MinimalExample>()
/// }
///
/// pub struct MinimalExample;
///
/// impl Game for MinimalExample {
///     fn new(_ctx: &Context) -> Result<Self, GameError> {
///         Ok(Self)
///     }
///
///     fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
///         Ok(())
///     }
///
///     fn render(&mut self, _ctx: &Context, _draw: &mut Draw) -> Result<(), GameError> {
///         Ok(())
///     }
/// }
/// ```
///
/// See the [`Game`](crate::core::Game) trait for more info.
pub fn new_game() -> GameBuilder {
    GameBuilder::default()
}
