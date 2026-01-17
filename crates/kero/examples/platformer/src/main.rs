use kero::prelude::*;
mod guy;
use guy::Guy;
mod level;
use level::Level;
mod vine;

const NATIVE_RES: Vec2U = vec2(320, 180);
const UPSCALE: u32 = 4; // should be dynamic...
const TILE_SIZE: Vec2U = vec2(8, 8);

fn main() -> Result<(), GameError> {
    kero::new_game()
        .with_default_logger()
        .with_title("Platformer")
        .with_size(NATIVE_RES.x * UPSCALE, NATIVE_RES.y * UPSCALE)
        .run::<SubTexturesExample>(())
}

pub struct SubTexturesExample {
    level: Level,
    guy: Guy,

    // move me or something
    guy_tex: Texture,
    guy_squash_tex: Texture,
    guy_stretch_tex: Texture,
    guy_smush_tex: Texture,
}

fn spr(ctx: &Context, bytes: &[u8]) -> Result<Texture, GameError> {
    Ok(ctx.graphics.load_png_from_memory(bytes, true)?)
}

impl Game for SubTexturesExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        Ok(Self {
            guy: Guy::new(ctx),
            level: Level::make_level(),
            // needs a better way to load and store these
            guy_tex: spr(ctx, include_bytes!("../assets/jelly_idle_0.png"))?,
            guy_squash_tex: spr(ctx, include_bytes!("../assets/jelly_squash_0.png"))?,
            guy_stretch_tex: spr(ctx, include_bytes!("../assets/jelly_stretch_0.png"))?,
            guy_smush_tex: spr(ctx, include_bytes!("../assets/jelly_smush_0.png"))?,
        })
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        if ctx.keyboard.pressed(Key::U) {
            self.level = Level::make_level();
            self.guy = Guy::new(ctx);
        }

        self.guy.update(ctx, &self.level);
        self.level.guy_moved(self.guy.pos, self.guy.vel);
        self.level.update(ctx);
        Ok(())
    }

    fn render(&mut self, _ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        draw.push_scale_of(UPSCALE as f32);

        self.level.render_bg(_ctx, draw)?;
        self.guy.render(self, draw)?;
        self.level.render_fg(_ctx, draw)?;

        draw.pop_transform()?;
        Ok(())
    }
}
