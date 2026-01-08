use crate::{RectF, Vec2F, add_shape_methods, impl_temp, vec2};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

impl_temp!(RectF RectRef RectMut);

pub struct RectModule;

impl LuaModule for RectModule {
    const PATH: &'static str = "Rect";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<RectF>::register(lua, "Rect", |members| {
            // fields
            members.getter("x", |rect| rect.x)?;
            members.setter("x", |rect, val: f32| rect.x = val)?;
            members.getter("y", |rect| rect.y)?;
            members.setter("y", |rect, val: f32| rect.y = val)?;
            members.getter("w", |rect| rect.w)?;
            members.setter("w", |rect, val: f32| rect.w = val)?;
            members.getter("h", |rect| rect.h)?;
            members.setter("h", |rect, val: f32| rect.h = val)?;

            // operators
            members.op_tostring(|r| format!("{r}"))?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<RectF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("left", |rect, _: ()| rect.left())?;
            members.method("right", |rect, _: ()| rect.right())?;
            members.method("top", |rect, _: ()| rect.top())?;
            members.method("bottom", |rect, _: ()| rect.bottom())?;
            members.method("top_left", |rect, _: ()| rect.top_left())?;
            members.method("top_right", |rect, _: ()| rect.top_right())?;
            members.method("bottom_right", |rect, _: ()| rect.bottom_right())?;
            members.method("bottom_left", |rect, _: ()| rect.bottom_left())?;
            members.method("corners", |rect, _: ()| match rect.corners() {
                [tl, tr, br, bl] => (tl, tr, br, bl),
            })?;
            members.method("top_center", |rect, _: ()| rect.top_center())?;
            members.method("bottom_center", |rect, _: ()| rect.bottom_center())?;
            members.method("left_center", |rect, _: ()| rect.left_center())?;
            members.method("right_center", |rect, _: ()| rect.right_center())?;
            members.method("center_x", |rect, _: ()| rect.center_x())?;
            members.method("center_y", |rect, _: ()| rect.center_y())?;
            members.method("center", |rect, _: ()| rect.center())?;
            members.method("min_x", |rect, _: ()| rect.min_x())?;
            members.method("min_y", |rect, _: ()| rect.min_y())?;
            members.method("min_pos", |rect, _: ()| rect.min_pos())?;
            members.method("max_x", |rect, _: ()| rect.max_x())?;
            members.method("max_y", |rect, _: ()| rect.max_y())?;
            members.method("max_pos", |rect, _: ()| rect.max_pos())?;
            members.method("right_edge", |rect, _: ()| rect.right_edge())?;
            members.method("left_edge", |rect, _: ()| rect.left_edge())?;
            members.method("top_edge", |rect, _: ()| rect.top_edge())?;
            members.method("bottom_edge", |rect, _: ()| rect.bottom_edge())?;
            members.method("edges", |rect, _: ()| match rect.edges() {
                [tl, tr, br, bl] => (tl, tr, br, bl),
            })?;
            members.method("area", |rect, _: ()| rect.area())?;
            members.method("perimeter", |rect, _: ()| rect.perimeter())?;
            members.method_ext("contains_rect", |lua, rect, r: Temp<RectF>| {
                r.read(lua, |_, r| Ok(rect.contains_rect(r)))
            })?;
            members.method("inflate", |rect, (w, h): (f32, Option<f32>)| {
                rect.inflate(match h {
                    Some(h) => (w, h),
                    None => (w, w),
                })
            })?;
            members.method_ext("overlap", |lua, rect, r: Temp<RectF>| {
                r.read(lua, |_, r| Ok(rect.overlap(r)))
            })?;
            members.method_ext("conflate", |lua, rect, r: Temp<RectF>| {
                r.read(lua, |_, r| Ok(rect.conflate(r)))
            })?;
            members.method("clamp_inside", |rect, p: Vec2F| {
                Vec2F::new(
                    p.x.clamp(rect.x, rect.right()),
                    p.y.clamp(rect.y, rect.bottom()),
                )
            })?;
            members.method("is_positive", |rect, _: ()| rect.is_positive())?;
            members.method("non_neg", |rect, _: ()| rect.non_neg())?;
            members.method("fitted", |rect, (w, h, frac): (f32, f32, bool)| {
                rect.fitted(vec2(w, h), frac)
            })?;
            members.method_ext("map_pos", |lua, rect, (pos, targ): (Vec2F, Temp<RectF>)| {
                targ.read(lua, |_, targ| Ok(rect.map_pos(pos, targ)))
            })?;
            members.method("translate", |rect, amount: Vec2F| rect.translate(&amount))?;

            // impl Shape
            add_shape_methods(members)?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (x, y, w, h): (f32, f32, Option<f32>, Option<f32>)| {
                Ok(match w.zip(h) {
                    Some((w, h)) => RectF::new(x, y, w, h),
                    None => RectF::new(0.0, 0.0, x, y),
                })
            })?,
        )?;
        module.set_metatable(Some({
            let meta = lua.create_table()?;
            meta.set(
                "__call",
                lua.create_function(
                    |_, (_, x, y, w, h): (Value, f32, f32, Option<f32>, Option<f32>)| {
                        Ok(match w.zip(h) {
                            Some((w, h)) => RectF::new(x, y, w, h),
                            None => RectF::new(0.0, 0.0, x, y),
                        })
                    },
                )?,
            )?;
            meta
        }))?;

        module.set("zero", lua.create_function(|_, _: ()| Ok(RectF::ZERO))?)?;

        Ok(Value::Table(module))
    }
}
