use crate::core::GameBuilder;

/// Begin initialization of your game.
pub fn new_game() -> GameBuilder {
    GameBuilder::new().unwrap()
}
