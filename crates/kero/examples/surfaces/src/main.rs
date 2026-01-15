use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();
    kero::new_game()
        .with_title("Surfaces")
        .with_size(1280, 720)
        .run::<SurfacesExample>(())
}

pub struct SurfacesExample {
    pattern: Surface,
}

impl Game for SurfacesExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        // create a surface that we can render our pattern to
        let pattern = ctx.graphics.create_rgba8_surface((80, 80));
        Ok(Self { pattern })
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        let tile_size = self.pattern.size().to_f32();

        // set our pattern surface as the target for drawing
        draw.set_surface(self.pattern.clone(), Rgba8::TRANSPARENT);

        // draw an animated circle to the pattern surface
        let max_radius = tile_size.x.min(tile_size.y) * 0.5;
        let circ = Circle::new(
            tile_size * 0.5,
            ctx.time.wave(max_radius * 0.5, max_radius, 2.0),
        );
        draw.circle(circ, rgb(0xff3377), None);

        // set the window as the target for drawing
        draw.set_surface(None, rgb(0x202020));

        // draw the pattern repeated over the window
        let tiles = ctx.window.size() / self.pattern.size();
        for y in 0..tiles.y {
            for x in 0..tiles.x {
                let pos = tile_size * vec2(x, y).to_f32();
                draw.texture_at(&self.pattern, pos);
            }
        }

        Ok(())
    }
}
