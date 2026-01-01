use polywog::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    polywog::new_game()
        .with_title("Minimal")
        .with_size(1280, 720)
        .run::<MinimalExample>()
}

pub struct MinimalExample {}

impl Game for MinimalExample {
    fn new(_ctx: &Context) -> Result<Self, GameError> {
        // initialize your game state here, such as creating graphics resources, etc.
        Ok(Self {})
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        // perform your game logic here
        Ok(())
    }

    fn render(&mut self, _ctx: &Context, _draw: &mut Draw) -> Result<(), GameError> {
        // perform your drawing code here
        Ok(())
    }
}
