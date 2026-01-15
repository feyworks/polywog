use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    kero::new_game()
        .with_title("Smooth Moves")
        .with_size(1280, 720)
        .run::<SmoothMovesExample>(())
}

pub struct SmoothMovesExample {
    small_pos: Vec2F,
    big_pos: Vec2F,
    big_vel: Vec2F,
}

impl Game for SmoothMovesExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        let mouse = ctx.mouse.pos();
        Ok(Self {
            small_pos: mouse,
            big_pos: mouse,
            big_vel: Vec2F::ZERO,
        })
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        let mouse = ctx.mouse.pos();

        self.small_pos = self.small_pos.smooth_lerp(mouse, 2.0, ctx.time.delta());
        self.big_pos
            .smooth_damp(&mut self.big_vel, mouse, 1.0, 640.0, ctx.time.delta());

        Ok(())
    }

    fn render(&mut self, _ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        draw.circle((self.big_pos, 32.0), Rgba8::BLUE, None);
        draw.circle((self.small_pos, 16.0), Rgba8::RED, None);
        Ok(())
    }
}
