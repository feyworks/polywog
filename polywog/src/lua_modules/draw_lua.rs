use crate::gfx::{BlendMode, ColorMode, Draw, Sampler};
use crate::lua_modules::{FontRef, ShaderRef, SubTextureRef, SurfaceRef, TextureRef};
use fey_color::{Rgba8, rgba};
use fey_lua::LuaModule;
use fey_math::{
    Affine2F, CircleF, LineF, Mat2F, Mat3F, Mat4F, Mat4Ref, Numeric, PolygonRef, QuadF, RadiansF,
    RectF, RectU, TriangleF, Vec2F, Vec3F, Vec4F, circle, line, vec2,
};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, Either, IntoLua, Lua, Number, Table, UserData, UserDataMethods, Value};
use std::ops::Deref;

impl Draw {
    pub fn from_lua(lua: &Lua) -> LuaResult<&mut Draw> {
        // SAFETY: app_data_mut() will panic if the pointer is attempted to be borrowed twice
        let draw = *lua
            .app_data_mut::<*mut Draw>()
            .ok_or_else(|| LuaError::runtime("cannot draw outside of render()"))?
            .deref();
        Ok(unsafe { &mut *draw })
    }
}

pub struct DrawModule;

impl LuaModule for DrawModule {
    const PATH: &'static str = "Draw";

    fn load(lua: &Lua) -> LuaResult<Value> {
        Self.into_lua(lua)
    }
}

impl UserData for DrawModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

impl UserData for Draw {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    #[inline]
    fn num_col(num: Option<Number>) -> Option<Rgba8> {
        num.map(|n| n as u32).map(rgba)
    }
    #[inline]
    fn num_col_or_white(num: Option<Number>) -> Rgba8 {
        num_col(num).unwrap_or(Rgba8::WHITE)
    }

    methods.add_function(
        "set_surface",
        |lua, (surf, col): (Option<SurfaceRef>, Option<Rgba8>)| {
            let draw = Draw::from_lua(lua)?;
            draw.set_surface(surf.map(|s| s.clone()), col.unwrap_or(Rgba8::WHITE));
            Ok(())
        },
    );
    methods.add_function("set_layer", |lua, layer: usize| {
        Draw::from_lua(lua)?.set_layer(layer);
        Ok(())
    });
    methods.add_function("set_shader", |lua, shader: Option<ShaderRef>| {
        Draw::from_lua(lua)?.set_shader(shader.map(|s| s.clone()));
        Ok(())
    });
    methods.add_function("set_param_i32", |lua, (name, value): (BorrowedStr, i32)| {
        Draw::from_lua(lua)?.set_param_i32(&name, value);
        Ok(())
    });
    methods.add_function("set_param_u32", |lua, (name, value): (BorrowedStr, u32)| {
        Draw::from_lua(lua)?.set_param_u32(&name, value);
        Ok(())
    });
    methods.add_function("set_param_f32", |lua, (name, value): (BorrowedStr, f32)| {
        Draw::from_lua(lua)?.set_param_f32(&name, value);
        Ok(())
    });
    methods.add_function(
        "set_param_vec2",
        |lua, (name, value): (BorrowedStr, Vec2F)| {
            Draw::from_lua(lua)?.set_param_vec2(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_vec3",
        |lua, (name, value): (BorrowedStr, Vec3F)| {
            Draw::from_lua(lua)?.set_param_vec3(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_vec4",
        |lua, (name, value): (BorrowedStr, Vec4F)| {
            Draw::from_lua(lua)?.set_param_vec4(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_mat2",
        |lua, (name, value): (BorrowedStr, Mat2F)| {
            Draw::from_lua(lua)?.set_param_mat2(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_mat3",
        |lua, (name, value): (BorrowedStr, Mat3F)| {
            Draw::from_lua(lua)?.set_param_mat3(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_mat4",
        |lua, (name, value): (BorrowedStr, Mat4F)| {
            Draw::from_lua(lua)?.set_param_mat4(&name, value);
            Ok(())
        },
    );
    methods.add_function(
        "set_param_texture",
        |lua, (name, value): (BorrowedStr, TextureRef)| {
            Draw::from_lua(lua)?.set_param_texture(&name, value.clone());
            Ok(())
        },
    );
    methods.add_function(
        "set_param_sampler",
        |lua, (name, value): (BorrowedStr, Sampler)| {
            Draw::from_lua(lua)?.set_param_sampler(&name, value.clone());
            Ok(())
        },
    );
    methods.add_function("set_view_matrix", |lua, value: Mat4Ref| {
        Draw::from_lua(lua)?.set_view_matrix(&value);
        Ok(())
    });
    methods.add_function("main_sampler", |lua, _: ()| {
        Ok(Draw::from_lua(lua)?.main_sampler())
    });
    methods.add_function("set_main_sampler", |lua, value: Sampler| {
        Draw::from_lua(lua)?.set_main_sampler(value);
        Ok(())
    });
    methods.add_function("blend_mode", |lua, _: ()| {
        Ok(Draw::from_lua(lua)?.blend_mode())
    });
    methods.add_function("set_blend_mode", |lua, value: BlendMode| {
        Draw::from_lua(lua)?.set_blend_mode(value);
        Ok(())
    });
    methods.add_function("clip_rect", |lua, _: ()| {
        Ok(Draw::from_lua(lua)?.clip_rect().copied())
    });
    methods.add_function("set_clip_rect", |lua, value: Option<RectU>| {
        Draw::from_lua(lua)?.set_clip_rect(value);
        Ok(())
    });
    methods.add_function("transform", |lua, _: ()| {
        Ok(*Draw::from_lua(lua)?.transform())
    });
    methods.add_function("push_transform", |lua, value: Affine2F| {
        Draw::from_lua(lua)?.push_transform(value);
        Ok(())
    });
    methods.add_function("push_new_transform", |lua, value: Affine2F| {
        Draw::from_lua(lua)?.push_new_transform(value);
        Ok(())
    });
    methods.add_function("set_transform", |lua, value: Affine2F| {
        Draw::from_lua(lua)?.set_transform(value);
        Ok(())
    });
    methods.add_function("push_translation", |lua, value: Vec2F| {
        Draw::from_lua(lua)?.push_translation(value);
        Ok(())
    });
    methods.add_function("push_rotation", |lua, value: RadiansF| {
        Draw::from_lua(lua)?.push_rotation(value);
        Ok(())
    });
    methods.add_function("push_scale", |lua, value: Either<Vec2F, f32>| {
        let draw = Draw::from_lua(lua)?;
        match value {
            Either::Left(s) => draw.push_scale(s),
            Either::Right(s) => draw.push_scale_of(s),
        }
        Ok(())
    });
    methods.add_function(
        "push_trs",
        |lua, (pos, rot, scale): (Vec2F, RadiansF, Either<Vec2F, f32>)| {
            let scale = match scale {
                Either::Left(s) => s,
                Either::Right(s) => vec2(s, s),
            };
            Draw::from_lua(lua)?.push_trs(pos, rot, scale);
            Ok(())
        },
    );
    methods.add_function("pop_transform", |lua, _: ()| {
        Draw::from_lua(lua)?
            .pop_transform()
            .map_err(LuaError::external)
    });
    methods.add_function("pop_transforms", |lua, count: usize| {
        Draw::from_lua(lua)?
            .pop_transforms(count)
            .map_err(LuaError::external)
    });
    methods.add_function(
        "texture_quad",
        |lua,
         (tex, quad, col, mode, fx, fy): (
            TextureRef,
            QuadF,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let tex = tex.deref();
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.textured_quad_ext(tex, quad, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.textured_quad_flipped(tex, quad, col, mode, (fx, fy));
                }
            }
            Ok(())
        },
    );
    methods.add_function(
        "texture_at",
        |lua,
         (tex, pos, col, mode, fx, fy): (
            TextureRef,
            Vec2F,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let tex = tex.deref();
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.texture_at_ext(tex, pos, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.texture_at_flipped(tex, pos, col, mode, vec2(fx, fy));
                }
            }
            Ok(())
        },
    );
    methods.add_function("point", |lua, (pos, col): (Vec2F, Rgba8)| {
        Draw::from_lua(lua)?.point(pos, col);
        Ok(())
    });
    methods.add_function("points", |lua, (points, col): (Table, Rgba8)| {
        Draw::from_lua(lua)?.points(
            points.sequence_values::<Vec2F>().filter_map(|p| p.ok()),
            col,
        );
        Ok(())
    });
    methods.add_function(
        "line",
        |lua,
         (a, b, c, d, e): (
            Either<f32, Vec2F>, // x1 | from
            Either<f32, Vec2F>, // y1 | to
            Number,             // x2 | color
            Option<f32>,        // y2
            Option<Rgba8>,      // color
        )| {
            let (line, col) = match a {
                Either::Left(a) => (
                    line(vec2(a, b.unwrap_left()), vec2(c as f32, d.unwrap())),
                    e.unwrap(),
                ),
                Either::Right(a) => (line(a, b.unwrap_right()), rgba(c as u32)),
            };
            Draw::from_lua(lua)?.line(line, col);
            Ok(())
        },
    );
    methods.add_function("line_obj", |lua, (line, col): (LineF, Option<Rgba8>)| {
        Draw::from_lua(lua)?.line(line, col.unwrap_or(Rgba8::WHITE));
        Ok(())
    });
    methods.add_function(
        "lines",
        |lua, (points, col, loops): (Table, Rgba8, bool)| {
            Draw::from_lua(lua)?.lines(
                points.sequence_values::<Vec2F>().filter_map(|p| p.ok()),
                col,
                loops,
            );
            Ok(())
        },
    );
    methods.add_function(
        "triangle",
        |lua, (a, b, c, col): (Vec2F, Vec2F, Vec2F, Rgba8)| {
            Draw::from_lua(lua)?.triangle((a, b, c), col);
            Ok(())
        },
    );
    methods.add_function("triangle_obj", |lua, (tri, col): (TriangleF, Rgba8)| {
        Draw::from_lua(lua)?.triangle(tri, col);
        Ok(())
    });
    methods.add_function(
        "triangle_outline",
        |lua, (a, b, c, col): (Vec2F, Vec2F, Vec2F, Rgba8)| {
            Draw::from_lua(lua)?.triangle_outline((a, b, c), col);
            Ok(())
        },
    );
    methods.add_function(
        "triangle_obj_outline",
        |lua, (tri, col): (TriangleF, Rgba8)| {
            Draw::from_lua(lua)?.triangle_outline(tri, col);
            Ok(())
        },
    );
    methods.add_function(
        "quad",
        |lua, (a, b, c, d, col): (Vec2F, Vec2F, Vec2F, Vec2F, Rgba8)| {
            Draw::from_lua(lua)?.quad((a, b, c, d), col);
            Ok(())
        },
    );
    methods.add_function("quad_obj", |lua, (quad, col): (QuadF, Rgba8)| {
        Draw::from_lua(lua)?.quad(quad, col);
        Ok(())
    });
    methods.add_function(
        "quad_outline",
        |lua, (a, b, c, d, col): (Vec2F, Vec2F, Vec2F, Vec2F, Rgba8)| {
            Draw::from_lua(lua)?.quad_outline((a, b, c, d), col);
            Ok(())
        },
    );
    methods.add_function("quad_obj_outline", |lua, (quad, col): (QuadF, Rgba8)| {
        Draw::from_lua(lua)?.quad_outline(quad, col);
        Ok(())
    });
    methods.add_function(
        "rect",
        |lua, (x, y, w, h, col): (f32, f32, f32, f32, Rgba8)| {
            Draw::from_lua(lua)?.rect((x, y, w, h), col);
            Ok(())
        },
    );
    methods.add_function("rect_obj", |lua, (rect, col): (RectF, Rgba8)| {
        Draw::from_lua(lua)?.rect(rect, col);
        Ok(())
    });
    methods.add_function(
        "rect_outline",
        |lua, (x, y, w, h, col): (f32, f32, f32, f32, Rgba8)| {
            Draw::from_lua(lua)?.rect_outline((x, y, w, h), col);
            Ok(())
        },
    );
    methods.add_function("rect_obj_outline", |lua, (rect, col): (RectF, Rgba8)| {
        Draw::from_lua(lua)?.rect_outline(rect, col);
        Ok(())
    });
    methods.add_function("polygon", |lua, (poly, col): (PolygonRef, Rgba8)| {
        Draw::from_lua(lua)?.polygon(&poly, col);
        Ok(())
    });
    methods.add_function(
        "polygon_outline",
        |lua, (poly, col): (PolygonRef, Rgba8)| {
            Draw::from_lua(lua)?.polygon_outline(&poly, col);
            Ok(())
        },
    );
    methods.add_function(
        "circle",
        |lua,
         (
            a, // x      | center
            b, // y      | radius
            c, // radius | color
            d, // color  | segs
            e, // segs
        ): (Either<f32, Vec2F>, f32, Number, Option<Number>, Option<u32>)| {
            let (circ, col, segs) = match a {
                Either::Left(a) => (circle(vec2(a, b), c as f32), rgba(d.unwrap() as u32), e),
                Either::Right(a) => (circle(a, b), rgba(c as u32), d.map(|d| d as u32)),
            };
            Draw::from_lua(lua)?.circle(circ, col, segs);
            Ok(())
        },
    );
    methods.add_function(
        "circle_obj",
        |lua, (circ, col, segs): (CircleF, Rgba8, Option<u32>)| {
            Draw::from_lua(lua)?.circle(circ, col, segs);
            Ok(())
        },
    );
    methods.add_function(
        "circle_outline",
        |lua,
         (
            a, // x      | center
            b, // y      | radius
            c, // radius | color
            d, // color  | segs
            e, // segs
        ): (Either<f32, Vec2F>, f32, Number, Option<Number>, Option<u32>)| {
            let (circ, col, segs) = match a {
                Either::Left(a) => (circle(vec2(a, b), c as f32), rgba(d.unwrap() as u32), e),
                Either::Right(a) => (circle(a, b), rgba(c as u32), d.map(|d| d as u32)),
            };
            Draw::from_lua(lua)?.circle_outline(circ, col, segs);
            Ok(())
        },
    );
    methods.add_function(
        "circle_obj_outline",
        |lua, (circ, col, segs): (CircleF, Rgba8, Option<u32>)| {
            Draw::from_lua(lua)?.circle_outline(circ, col, segs);
            Ok(())
        },
    );
    methods.add_function(
        "subtextured_quad",
        |lua,
         (sub, quad, col, mode, fx, fy): (
            SubTextureRef,
            QuadF,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.subtextured_quad_ext(sub.deref(), quad, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.subtextured_quad_flipped(sub.deref(), quad, col, mode, (fx, fy));
                }
            }
            Ok(())
        },
    );
    methods.add_function(
        "subtexture_at",
        |lua,
         (sub, pos, col, mode, fx, fy): (
            SubTextureRef,
            Vec2F,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.subtexture_at_ext(sub.deref(), pos, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.subtexture_at_flipped(sub.deref(), pos, col, mode, (fx, fy));
                }
            }
            Ok(())
        },
    );
    methods.add_function(
        "text",
        |lua,
         (txt, a, b, c, d, e): (
            BorrowedStr,                    // text
            Either<f32, Vec2F>,             // x     | pos
            Either<f32, FontRef>,           // y     | font
            Either<FontRef, Option<Rgba8>>, // font  | color
            Option<Number>,                 // color | size
            Option<f32>,                    // size
        )| {
            let (pos, font, col, size) = match a {
                Either::Left(a) => (vec2(a, b.unwrap_left()), c.unwrap_left(), num_col(d), e),
                Either::Right(a) => (a, b.unwrap_right(), c.unwrap_right(), d.map(|d| d as f32)),
            };
            let col = col.unwrap_or(Rgba8::WHITE);
            Draw::from_lua(lua)?.text(txt.as_ref(), pos, font.deref(), col, size);
            Ok(())
        },
    );

    // ---Draw a custom set of vertices & indices.
    // ---@param texture Texture?
    // ---@param topology Topology
    // ---@param vertices Vertex[]
    // ---@param indices integer[]
    // function Draw.custom(texture, topology, vertices, indices) end
    //
    // ---Draw the provided vertex & index buffers.
    // ---@param texture Texture?
    // ---@param topology Topology
    // ---@param vertices VertexBuffer
    // ---@param indices IndexBuffer
    // function Draw.buffers(texture, topology, vertices, indices) end
}
