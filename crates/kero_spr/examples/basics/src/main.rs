use kero::prelude::*;
use kero_spr::{Sprite, SpriteAnim, SpriteFont, SpritePacker, SpritePatch, SpriteSheet};

fn main() -> Result<(), GameError> {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;

    kero::new_game()
        .with_default_logger()
        .with_title("Basics")
        .with_size(1280, 720)
        .run::<BasicsExample>(())
}

pub struct BasicsExample {
    screen: Screen,
    portrait: Sprite,
    tiles: SpriteSheet,
    textbox: SpritePatch,
    virtue: SpriteFont,
    player: Player,
}

impl Game for BasicsExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        let screen = Screen::new_frame(ctx, (320, 180), false);

        let mut packer = SpritePacker::new();
        packer.add_ase_file("player", "assets/player.aseprite")?;
        packer.add_sprite_file("portrait", "assets/portrait.png", true, Some(0))?;
        packer.add_sheet_file("tiles", "assets/tiles.png", true, (16, 16), Some(0))?;
        packer.add_patch_file("textbox", "assets/textbox.png", true, (8, 8, 16, 16))?;
        packer.add_font_file("virtue", "assets/virtue.ttf", 16.0, BASIC_LATIN.chars())?;

        let mut atlas = packer.pack_graphics(4096, &ctx.graphics)?;
        let player = atlas.anims.remove("player").unwrap();
        let portrait = atlas.sprites.remove("portrait").unwrap();
        let tiles = atlas.sheets.remove("tiles").unwrap();
        let textbox = atlas.patches.remove("textbox").unwrap();
        let virtue = atlas.fonts.remove("virtue").unwrap();

        Ok(Self {
            screen,
            portrait,
            tiles,
            textbox,
            virtue,
            player: Player {
                anim: player,
                frame_idx: 0,
                frame_timer: 0.0,
                frame: 0,
                pos: vec2(216.0, 40.0),
            },
        })
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        self.screen.update(ctx);
        self.player.update(ctx);
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        self.screen.set_as_draw_surface(draw, rgb(0x286d38));

        self.portrait.draw(draw, (0.0, 111.0));

        for (spr, tile) in self.tiles.tiles.iter() {
            if let Some(spr) = spr {
                spr.draw(
                    draw,
                    tile.to_f32() * self.tiles.tile_size + vec2(48.0, 16.0),
                );
            }
        }

        let rect = rect(80.0, 120.0, 232.0, 56.0);
        let wave = ctx.time.wave(-3.0, 3.0, 2.0).round();
        self.textbox.draw(draw, rect.inflate((wave, -wave)));

        self.virtue.draw(
            draw,
            "Patches can be used to draw things like",
            (96.0, 145.0),
            Rgba8::WHITE,
        );
        self.virtue.draw(
            draw,
            "textboxes and menu containers!",
            (96.0, 157.0),
            Rgba8::WHITE,
        );

        self.player.render(draw);

        self.screen.draw_to_window(draw, Rgba8::BLACK);
        Ok(())
    }
}

struct Player {
    anim: SpriteAnim,
    frame_idx: usize,
    frame_timer: f32,
    frame: usize,
    pos: Vec2F,
}

impl Player {
    fn update(&mut self, ctx: &Context) {
        let tag_name = match (ctx.time.since_startup() / 2.0) as u64 % 4 {
            0 => "walk_s",
            1 => "walk_w",
            2 => "walk_n",
            _ => "walk_e",
        };
        let tag = self.anim.tag(tag_name).unwrap();

        self.frame = tag.from + self.frame_idx;

        if let Some(frame) = self.anim.frames.get(self.frame) {
            self.frame_timer += ctx.time.delta();

            if self.frame_timer >= frame.duration {
                self.frame_timer -= frame.duration;
                self.frame_idx = (self.frame_idx + 1) % 4;
            }
        }
    }

    fn render(&mut self, draw: &mut Draw) {
        self.anim.draw(draw, self.frame, self.pos);
    }
}
