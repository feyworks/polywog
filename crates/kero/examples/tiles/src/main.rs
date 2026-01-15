use kero::prelude::*;

const TILE_SIZE: Vec2U = vec2(16, 16);

fn main() -> Result<(), GameError> {
    env_logger::init();
    kero::new_game()
        .with_title("SubTextures")
        .with_size(640, 640)
        .run::<SubTexturesExample>(())
}

pub struct SubTexturesExample {
    tiles: VecGrid<SubTexture>,
}

impl Game for SubTexturesExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        // embed a PNG into the executable and load it as a PNG
        let bytes = include_bytes!("../assets/nethack.png");
        let tex = ctx.graphics.load_png_from_memory(bytes, true)?;

        // split the texture into a grid of subtextures
        let tiles = tex.split_into_tiles(TILE_SIZE);

        Ok(Self { tiles })
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        // draw a background color to the window
        draw.set_surface(None, rgb(0x476c6c));

        // let's scale everything up
        draw.push_scale_of(2.0 * ctx.window.scale_factor());

        // draw each of the subtextures in their original tilemap position
        let orig = TILE_SIZE.to_f32() * 0.5;
        for (sub, pos) in self.tiles.iter() {
            // move into the tile position
            let pos = (pos * TILE_SIZE).to_f32() + orig;
            draw.push_translation(pos);

            // rock back and forth
            draw.push_rotation(degs(ctx.time.wave_ext(-10.0, 10.0, 2.0, pos.x / 160.0)));

            // draw the tile centered on its origin
            draw.subtexture_at(sub, -orig);

            // pop the rotation/translation
            draw.pop_transforms(2)?;
        }
        Ok(())
    }
}
