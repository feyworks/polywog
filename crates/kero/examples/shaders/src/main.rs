use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();
    kero::new_game()
        .with_title("Shaders")
        .with_size(960, 640)
        .run::<ShadersExample>(())
}

pub struct ShadersExample {
    invert_shader: Shader,
    perlin: Texture,
    screenshot: Texture,
}

impl Game for ShadersExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        // load the shader file
        let invert_shader = ctx
            .graphics
            .create_shader(include_str!("../assets/invert_shader.wgsl"));

        // load a perlin noise texture
        let perlin = ctx
            .graphics
            .load_png_from_memory(include_bytes!("../assets/perlin.png"), true)?;

        // load an image to apply the shader to
        let screenshot = ctx
            .graphics
            .load_png_from_memory(include_bytes!("../assets/screenshot.png"), true)?;

        Ok(Self {
            invert_shader,
            perlin,
            screenshot,
        })
    }

    fn update(&mut self, _ctx: &Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        // after we set a shader, we can set its parameters
        draw.set_shader(self.invert_shader.clone());
        draw.set_param_texture("perlin_texture", self.perlin.clone());
        draw.set_param_sampler("perlin_sampler", Sampler::linear(AddressMode::Repeat));
        draw.set_param_vec2("scroll", vec2(0.1, 0.05) * ctx.time.since_startup());

        let window_rect = RectF::sized(ctx.window.size().to_f32());
        draw.textured_quad(&self.screenshot, window_rect);

        Ok(())
    }
}
