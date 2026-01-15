use kero::prelude::*;
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    kero::new_game()
        .with_title("Texture Packer")
        .with_size(1280, 720)
        .run::<TexturePackerExample>(())
}

pub struct TexturePackerExample {
    tex: Texture,
    subs: HashMap<String, SubTexture>,
}

impl Game for TexturePackerExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

        let mut packer = TexturePacker::new();

        // add a bunch of individual image files
        for path in path
            .join("monsters")
            .read_dir()?
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "png"))
        {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let img = DynImage::load_png_from_file(path)?.to_rgba8();
            packer.add_image(name, img, None, 0);
        }

        // load a tilesheet and add all its tiles as individual images
        let items = DynImage::load_png_from_file(path.join("items.png"))?.to_rgba8();
        let tile_size = Vec2U::splat(16);
        let grid_size = items.size() / tile_size;
        for y in 0..grid_size.y {
            for x in 0..grid_size.x {
                let name = format!("item_{x}_{y}");
                let rect = RectU::pos_size(vec2(x, y) * tile_size, tile_size);
                packer.add_image(name, &items, rect, 0);
            }
        }

        let (tex, subs) = packer.pack(&ctx.graphics).unwrap();

        Ok(Self { tex, subs })
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, _ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        draw.push_scale_of(2.0);

        let tex_size = RectF::sized(self.tex.size().to_f32());

        draw.rect(tex_size, rgb(0x476c6c));

        for sub in self.subs.values() {
            draw.rect_outline(sub.rect, Rgba8::WHITE);
        }

        for sub in self.subs.values() {
            draw.subtexture_at(sub, sub.rect.top_left() - sub.offset);
        }

        Ok(())
    }
}
