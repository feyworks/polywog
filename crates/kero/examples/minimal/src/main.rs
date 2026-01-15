use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    kero::new_game()
        .with_title("Minimal")
        .with_size(1280, 720)
        .run::<MinimalExample>(())
}

pub struct MinimalExample {}

impl Game for MinimalExample {
    type Config = ();

    fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        // initialize your game state here, such as creating graphics resources, etc.
        Ok(Self {})
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        // perform your game logic here
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        // perform your drawing code here
        Ok(())
    }
}
