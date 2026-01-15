use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    kero::new_game()
        .with_title("Controller")
        .with_size(1280, 720)
        .run::<ControllerExample>(())
}

pub struct ControllerExample {
    screen: Screen,
    controller: VirtualController,
    base: Texture,
    stick_base: Texture,
    stick: Texture,
    face_btn: Texture,
    small_btn: Texture,
    bumper: Texture,
    trigger: Texture,
    dpad_x: Texture,
    dpad_y: Texture,
}

impl Game for ControllerExample {
    type Config = ();

    fn new(ctx: &Context, _cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        let screen = Screen::new_frame(ctx, (160, 90), false);
        let controller = VirtualController::basic(ctx);

        let load_img = |bytes: &[u8]| -> Result<Texture, GameError> {
            let mut img = DynImage::load_png_from_memory(bytes)?.to_rgba8();
            img.premultiply();
            Ok(ctx.graphics.create_rgba8_texture(&img))
        };

        let base = load_img(include_bytes!("../assets/base.png"))?;
        let stick_base = load_img(include_bytes!("../assets/stick_base.png"))?;
        let stick = load_img(include_bytes!("../assets/stick.png"))?;
        let face_btn = load_img(include_bytes!("../assets/face_btn.png"))?;
        let small_btn = load_img(include_bytes!("../assets/small_btn.png"))?;
        let bumper = load_img(include_bytes!("../assets/bumper.png"))?;
        let trigger = load_img(include_bytes!("../assets/trigger.png"))?;
        let dpad_x = load_img(include_bytes!("../assets/dpad_x.png"))?;
        let dpad_y = load_img(include_bytes!("../assets/dpad_y.png"))?;

        Ok(Self {
            screen,
            controller,
            base,
            stick_base,
            stick,
            face_btn,
            small_btn,
            bumper,
            trigger,
            dpad_x,
            dpad_y,
        })
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        self.screen.update(ctx);
        Ok(())
    }

    fn render(&mut self, _ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        let bg_col = rgb(0x676b76);
        let base_col = rgb(0x343842);
        let btn_col = rgb(0x434750);
        let press_col = rgb(0x282c34);

        self.screen.set_as_draw_surface(draw, bg_col);

        // left bumper
        let (col, y) = if self.controller.left_bumper.down() {
            (press_col, 4.0)
        } else {
            (btn_col, 3.0)
        };
        draw.texture_at_ext(&self.bumper, (29.0, y), col, ColorMode::MULT);

        // left bumper
        let (col, y) = if self.controller.right_bumper.down() {
            (press_col, 4.0)
        } else {
            (btn_col, 3.0)
        };
        draw.texture_at_flipped(&self.bumper, (99.0, y), col, ColorMode::MULT, (true, false));

        // controller base
        draw.texture_at_ext(&self.base, (17.0, 6.0), base_col, ColorMode::MULT);

        // left stick
        let left_stick = (self.controller.left_stick.value() * 4.0).trunc();
        draw.texture_at_ext(
            &self.stick_base,
            (39.0, 21.0),
            Rgba8::BLACK,
            ColorMode::MULT,
        );
        draw.texture_at_ext(
            &self.stick,
            vec2(37.0, 19.0) + left_stick,
            btn_col,
            ColorMode::MULT,
        );

        // right stick
        let right_stick = (self.controller.right_stick.value() * 4.0).trunc();
        draw.texture_at_ext(
            &self.stick_base,
            (89.0, 39.0),
            Rgba8::BLACK,
            ColorMode::MULT,
        );
        draw.texture_at_ext(
            &self.stick,
            vec2(87.0, 37.0) + right_stick,
            btn_col,
            ColorMode::MULT,
        );

        // east
        draw.texture_at_ext(
            &self.face_btn,
            (118.0, 23.0),
            if self.controller.east.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // south
        draw.texture_at_ext(
            &self.face_btn,
            (110.0, 31.0),
            if self.controller.south.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // west
        draw.texture_at_ext(
            &self.face_btn,
            (102.0, 23.0),
            if self.controller.west.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // north
        draw.texture_at_ext(
            &self.face_btn,
            (110.0, 16.0),
            if self.controller.north.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // select
        draw.texture_at_ext(
            &self.small_btn,
            (65.0, 16.0),
            if self.controller.select.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // start
        draw.texture_at_ext(
            &self.small_btn,
            (88.0, 16.0),
            if self.controller.start.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // left trigger
        let y = 5.0.lerp(8.0, self.controller.left_trigger.value());
        draw.texture_at_ext(
            &self.trigger,
            (7.0, y),
            if y >= 5.5 { press_col } else { btn_col },
            ColorMode::MULT,
        );
        draw.rect((7.0, 13.0, 20.0, 1.0), Rgba8::BLACK);
        draw.rect((7.0, 14.0, 20.0, 3.0), bg_col);

        // right trigger
        let y = 5.0.lerp(8.0, self.controller.right_trigger.value());
        draw.texture_at_flipped(
            &self.trigger,
            (133.0, y),
            if y >= 5.5 { press_col } else { btn_col },
            ColorMode::MULT,
            (true, false),
        );
        draw.rect((133.0, 13.0, 20.0, 1.0), Rgba8::BLACK);
        draw.rect((133.0, 14.0, 20.0, 3.0), bg_col);

        // dpad center
        draw.rect((57.0, 41.0, 7.0, 7.0), btn_col);

        // dpad right
        draw.texture_at_ext(
            &self.dpad_x,
            (61.0, 41.0),
            if self.controller.dpad_right.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // dpad left
        draw.texture_at_flipped(
            &self.dpad_x,
            (51.0, 41.0),
            if self.controller.dpad_left.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
            (true, false),
        );

        // dpad down
        draw.texture_at_ext(
            &self.dpad_y,
            (57.0, 45.0),
            if self.controller.dpad_down.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
        );

        // dpad up
        draw.texture_at_flipped(
            &self.dpad_y,
            (57.0, 35.0),
            if self.controller.dpad_up.down() {
                press_col
            } else {
                btn_col
            },
            ColorMode::MULT,
            (false, true),
        );

        self.screen.draw_to_window(draw, Rgba8::BLACK);
        Ok(())
    }
}
