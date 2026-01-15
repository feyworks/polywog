use kero::prelude::*;

const VINE_COLOR: Rgba8 = rgb(0x6553ad);

pub struct Vine {
    pub pos: Vec2F,
    pub length: f32,
    pub sway_pos: Vec2F,
    pub sway_vel: Vec2F,
}

impl Vine {
    pub fn new(pos: Vec2F, length: f32) -> Self {
        Self {
            pos,
            length,
            sway_pos: Vec2F::ZERO,
            sway_vel: vec2(1.0, 0.0),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        self.sway_pos += self.sway_vel * ctx.time.delta();
        self.sway_vel += vec2(-self.sway_pos.x * 100.0, 0.0) * ctx.time.delta();
        self.sway_vel = self
            .sway_vel
            .smooth_lerp(Vec2F::ZERO, 4.0, ctx.time.delta());
    }

    pub fn render(&self, draw: &mut Draw) -> Result<(), GameError> {
        let segments = self.length.ceil() as u32;
        for i in 0..segments {
            let segment_pos = self.pos + vec2(0.0, i as f32);
            let wiggle = (i % 2) as f32;
            let sway_offset = self.sway_pos.x * (i as f32 / segments as f32); //.powf(2.0);
            draw.rect(
                RectF::new(
                    (segment_pos.x + wiggle + sway_offset).round(),
                    segment_pos.y,
                    1.0,
                    1.0,
                ),
                VINE_COLOR,
            );
        }
        Ok(())
    }
}
