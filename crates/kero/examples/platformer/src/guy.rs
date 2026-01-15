use kero::prelude::*;

use crate::{SubTexturesExample, TILE_SIZE};

const SPRITE_ORIG: Vec2F = vec2(6.0, 8.0);
const GRAVITY: Vec2F = vec2(0.0, 800.0);
const COLLISION_RECT: RectF = RectF::new(-4.0, -5.0, 8.0, 6.0);
const JUMP_HEIGHT: f32 = TILE_SIZE.y as f32 * 4.2;
const MOVE_SPEED: f32 = 800.0;
const MAX_X_SPEED: f32 = 80.0;

pub struct Guy {
    pub pos: Vec2F,
    pub vel: Vec2F,
    pub flip_x: bool,
    pub remainder: Vec2F,
    pub squash_timer: f32,
    pub is_grounded: bool,
    pub is_face_smushed: bool,
    pub x_axis: VirtualAxis,
    pub jump_btn: VirtualButton,
}

impl Guy {
    pub fn new(ctx: &Context) -> Self {
        let src = VirtualSource::last_active(ctx);
        let left_btn = VirtualButton::new(&src, Key::A, GamepadButton::DPadLeft);
        let right_btn = VirtualButton::new(&src, Key::D, GamepadButton::DPadRight);
        let x_axis = VirtualAxis::new(&src, None, left_btn, right_btn);
        let jump_btn = VirtualButton::new(&src, Key::Space, GamepadButton::South);
        Self {
            pos: vec2(5.0, 1.0) * TILE_SIZE.to_f32(),
            vel: Vec2F::ZERO,
            flip_x: false,
            remainder: Vec2F::ZERO,
            squash_timer: 0.0,
            is_grounded: false,
            is_face_smushed: false,
            x_axis,
            jump_btn,
        }
    }

    pub fn update(&mut self, ctx: &Context, level: &crate::Level) {
        let input = vec2(self.x_axis.value(), 0.0);
        if self.jump_btn.pressed() {
            if self.is_grounded {
                self.vel.y = -GRAVITY.y * (2.0 * JUMP_HEIGHT / GRAVITY.y).sqrt();
                self.squash_timer = 0.0;
            }
            // else if self.is_face_smushed {
            //     self.vel.y = -GRAVITY.y * (2.0 * JUMP_HEIGHT / GRAVITY.y).sqrt();
            //     self.vel.x = if self.flip_x {
            //         MAX_X_SPEED
            //     } else {
            //         -MAX_X_SPEED
            //     };
            //     self.is_face_smushed = false;
            // }
        }

        // update velocity
        self.vel += (input * MOVE_SPEED + GRAVITY) * ctx.time.delta();
        if input.x == 0.0 {
            self.vel.x *= 0.8;
        }
        self.vel.x = self.vel.x.clamp(-MAX_X_SPEED, MAX_X_SPEED);

        // actually move
        let (collided_x, collided_y) = self.move_by(level, self.vel * ctx.time.delta());
        if collided_x {
            self.vel.x = 0.0;
        }
        if collided_y {
            self.vel.y = 0.0;
        }
        if self.vel.x != 0.0 {
            self.flip_x = self.vel.x < 0.0;
        }
        self.squash_timer = (self.squash_timer - ctx.time.delta()).max(0.0);

        let is_now_grounded = self.collide_at(level, self.pos + vec2(0.0, 1.0));
        if is_now_grounded {
            if self.is_grounded == false {
                self.squash_timer = 0.15;
            }
        }

        // self.is_face_smushed = input.x != 0.0
        //     && self.collide_at(
        //         level,
        //         self.pos + vec2(if self.flip_x { -1.0 } else { 1.0 }, 0.0),
        //     );

        if self.is_face_smushed && self.vel.y > 0.0 {
            self.vel.y *= 0.8;
        }

        self.is_grounded = is_now_grounded;
    }

    pub fn render(&self, game: &SubTexturesExample, draw: &mut Draw) -> Result<(), GameError> {
        draw.push_translation(self.pos.round());
        draw.push_scale(vec2(if self.flip_x { -1.0 } else { 1.0 }, 1.0));
        draw.texture_at(
            match (
                self.squash_timer > 0.0,
                !self.is_grounded,
                self.is_face_smushed,
            ) {
                // maybe this is the wrong control structure here actually lol
                // (_, _, true) => &game.guy_smush_tex,
                (true, _, _) => &game.guy_squash_tex,
                (_, true, _) => &game.guy_stretch_tex,
                _ => &game.guy_tex,
            },
            -SPRITE_ORIG,
        );
        draw.pop_transforms(2)?;
        Ok(())
    }

    fn collide_at(&self, level: &crate::Level, test_pos: Vec2F) -> bool {
        level.collide_terrain_rect(COLLISION_RECT + test_pos)
    }

    pub fn move_by(&mut self, level: &crate::Level, amount: Vec2F) -> (bool, bool) {
        let collided_x = self.move_x(level, amount.x);
        let collided_y = self.move_y(level, amount.y);
        (collided_x, collided_y)
    }

    pub fn move_x(&mut self, level: &crate::Level, amount: f32) -> bool {
        self.move_t(level, amount, true)
    }

    pub fn move_y(&mut self, level: &crate::Level, amount: f32) -> bool {
        self.move_t(level, amount, false)
    }

    fn move_t(&mut self, level: &crate::Level, amount: f32, horizontal: bool) -> bool {
        let remainder = if horizontal {
            &mut self.remainder.x
        } else {
            &mut self.remainder.y
        };

        *remainder += amount;
        let mut move_amount = remainder.round() as i32;

        if move_amount != 0 {
            *remainder -= move_amount as f32;
            let sign = move_amount.signum();

            while move_amount != 0 {
                let offset = if horizontal {
                    vec2(sign as f32, 0.0)
                } else {
                    vec2(0.0, sign as f32)
                };

                if !self.collide_at(level, self.pos + offset) {
                    if horizontal {
                        self.pos.x += sign as f32;
                    } else {
                        self.pos.y += sign as f32;
                    }
                    move_amount -= sign;
                } else {
                    return true;
                }
            }
        }

        false
    }
}
