use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();
    kero::new_game()
        .with_title("Text")
        .with_size(1280, 720)
        .run::<TextExample>(())
}

pub struct TextExample {
    noto: Font,
    _noto_texture: Texture,
    virtue: Font,
    _virtue_texture: Texture,
}

impl Game for TextExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        // load a smooth font
        let (noto, _noto_texture) = Font::from_ttf_bytes(
            &ctx.graphics,
            include_bytes!("../assets/NotoSans-Regular.ttf"),
            32.0,
            false,
            BASIC_LATIN.chars(),
        )?
        .ok_or_else(|| GameError::custom("failed to load font"))?;

        // load a pixelated font
        let (virtue, _virtue_texture) = Font::from_ttf_bytes(
            &ctx.graphics,
            include_bytes!("../assets/virtue.ttf"),
            16.0,
            true,
            BASIC_LATIN.chars(),
        )?
        .ok_or_else(|| GameError::custom("failed to load font"))?;

        Ok(Self {
            noto,
            _noto_texture,
            virtue,
            _virtue_texture,
        })
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        draw.push_scale_of(ctx.window.inv_scale_factor());

        // draw smooth text
        draw.text(
            "Thinking meat! You're asking me to believe in thinking meat!",
            vec2(75.0, 100.0),
            &self.noto,
            Rgba8::WHITE,
            None,
        );

        // draw pixelated text
        draw.text(
            "Thinking meat! You're asking me to believe in thinking meat!",
            vec2(75.0, 150.0),
            &self.virtue,
            Rgba8::WHITE,
            48.0,
        );

        draw.pop_transform()?;

        Ok(())
    }
}
