use kero::prelude::*;

use crate::{NATIVE_RES, TILE_SIZE, vine};

const LEVEL_BG_COLOR: Rgba8 = rgb(0x201436);
const OUTLINE_COLOR: Rgba8 = rgb(0x8097fa);
const OUTLINE_COLOR_ALT: Rgba8 = rgb(0x3f4984);

pub struct Level {
    size: Vec2U,
    tiles: VecGrid<bool>,
    texture: Option<Texture>,
    vines: Vec<vine::Vine>, // todo, generic props other than just vines,,,
}

impl Level {
    pub fn make_level() -> Level {
        let size = vec2(NATIVE_RES.x / TILE_SIZE.x, NATIVE_RES.y / TILE_SIZE.y + 1);

        let mut level = Level {
            tiles: VecGrid::new_with(size, || true),
            size,
            texture: None,
            vines: Vec::new(),
        };

        let pad = 2.0;

        let mut rng = Rand::new();

        let safe_bounds = RectF::new(
            pad,
            pad,
            size.x as f32 - pad * 2.0,
            (size.y - 2) as f32 - pad * 2.0,
        );

        let from = vec2(5, 0);
        let mut walker_pos = from;
        while walker_pos.y < level.size.y {
            let mut radius = 1.0;
            if walker_pos.y == from.y {
                radius = 2.0;
            }

            level.draw_rect(
                RectF::new(
                    walker_pos.x as f32 - radius,
                    walker_pos.y as f32 - radius,
                    radius * 2.0,
                    radius * 2.0,
                ),
                false,
            );
            walker_pos.x += rng.range(-2.5..2.5) as u32;
            walker_pos.x = walker_pos.x.clamp(1, level.size.x - 4);
            if rng.chance(0.3) {
                walker_pos.y += 1;
            }
        }

        // draw random blocks
        for _ in 0..(safe_bounds.w as u32 * safe_bounds.h as u32 / 40) {
            let x = rng.range(pad..(size.x as f32 - pad)) as f32;
            let y = rng.range(pad..(size.y as f32 - pad)) as f32;
            let w = rng.range(1.0..10.0);
            let h = rng.range(1.0..10.0);
            let rect = RectF::new(x, y, w, h).clamp_inside(&safe_bounds);
            level.draw_rect(rect, false);
        }

        // on some blocks with empty space below, add hanging vines
        for y in 1..(level.size.y - 1) {
            for x in 0..level.size.x {
                if !level.is_solid_at((x, y)) && level.is_solid_at((x, y - 1)) && rng.chance(0.8) {
                    let vine_length = rng.range(2..=5) * 2;
                    let vine_pos = vec2(
                        x as f32 * TILE_SIZE.x as f32
                            + rng
                                .range((TILE_SIZE.x as f32 * 0.1)..(TILE_SIZE.x as f32 * 0.9))
                                .round(),
                        y as f32 * TILE_SIZE.y as f32,
                    );
                    level
                        .vines
                        .push(vine::Vine::new(vine_pos, vine_length as f32));
                }
            }
        }

        level
    }

    pub fn guy_moved(&mut self, pos: Vec2F, vel: Vec2F) {
        // the guy moves vines and probably other stuff
        for vine in &mut self.vines {
            let offset = (vine.pos + vec2(0.0, vine.length / 2.0)) - pos;
            let dist = 16.0;
            if offset.len() < dist {
                // vine gets pushed
                let influence = (vel.len() / 400.0).clamp(0.0, 1.0) * (1.0 - offset.len() / dist);
                vine.sway_vel = vine.sway_vel.lerp(vel * 0.4, influence);
            }
        }
    }

    pub fn draw_rect(&mut self, rect: RectF, value: bool) {
        let rect = rect
            .clamp_inside(&RectF::sized(self.size.to_f32()))
            .to_u32();
        for y in rect.y..rect.bottom() {
            for x in rect.x..rect.right() {
                if let Some(tile) = self.tiles.get_mut(x, y) {
                    *tile = value;
                }
            }
        }
    }

    pub fn is_solid_at(&self, tile_pos: impl Coord) -> bool {
        self.tiles.get_at(tile_pos).is_none_or(|v| *v)
    }

    pub fn collide_terrain_rect(&self, rect: RectF) -> bool {
        let tile_size = TILE_SIZE.to_f32();

        let min_x = (rect.x / tile_size.x).floor() as i32;
        let min_y = (rect.y / tile_size.y).floor() as i32;
        let max_x = ((rect.x + rect.w) / tile_size.x).ceil() as i32;
        let max_y = ((rect.y + rect.h) / tile_size.y).ceil() as i32;

        // check each tile that could overlap
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !self.is_solid_at((x, y)) {
                    continue;
                }

                let tile_rect = RectF::new(
                    x as f32 * tile_size.x,
                    y as f32 * tile_size.y,
                    tile_size.x,
                    tile_size.y,
                );

                if rect.overlaps(&tile_rect) {
                    return true;
                }
            }
        }

        false
    }

    pub fn update(&mut self, ctx: &Context) {
        for vine in &mut self.vines {
            vine.update(ctx);
        }
    }

    pub fn render_bg(&self, _ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        draw.set_surface(None, LEVEL_BG_COLOR);
        Ok(())
    }

    pub fn render_fg(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        // todo: i wanna generate a whole level as a single texture in cpu
        // for (y, row) in self.tiles.iter().enumerate() {
        //     for (x, &tile) in row.iter().enumerate() {
        //         if tile {
        //             draw.rect(
        //                 RectF::new(
        //                     (x * TILE_SIZE.x as usize) as f32,
        //                     (y * TILE_SIZE.y as usize) as f32,
        //                     TILE_SIZE.x as f32,
        //                     TILE_SIZE.y as f32,
        //                 ),
        //                 Rgba8::BLACK,
        //             );
        //         }
        //     }
        // }

        for vine in &self.vines {
            vine.render(draw)?;
        }
        draw.texture_at(self.get_texture(ctx), Vec2F::ZERO);
        Ok(())
    }

    fn get_texture(&mut self, ctx: &Context) -> Texture {
        if let Some(ref tex) = self.texture {
            return tex.clone();
        }

        let mut image = ImageRgba8::new_mapped(self.size * TILE_SIZE, |p| {
            if self.is_solid_at(p / TILE_SIZE) {
                Rgba8::BLACK
            } else {
                Rgba8::TRANSPARENT
            }
        });

        Self::outline_color(&mut image);

        let tex = ctx.graphics.create_rgba8_texture(&image);
        self.texture = Some(tex.clone());
        tex
    }

    /// If there is a black pixel bordering a transparent pixel, make it gray.
    fn outline_color(img: &mut ImageRgba8) {
        let temp = img.clone();
        let mut rng = Rand::new();

        // for each black pixel
        for (color, p) in temp.iter() {
            if *color != Rgba8::BLACK {
                continue;
            }
            let p = p.to_i32();

            // check if any of its neighbors are transparent
            let has_transparent_neighbor = Vec2I::CARDINAL_DIRS
                .into_iter()
                .filter_map(|dir| temp.get_at(p + dir))
                .any(|c| c.a == 0);

            if has_transparent_neighbor {
                if rng.chance(0.6) {
                    img.set_at(p, OUTLINE_COLOR);
                } else {
                    img.set_at(p, OUTLINE_COLOR_ALT);
                }
            }
        }
    }
}
