use crate::{LineF, QuadF, RectF, Vec2F, add_shape_methods, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

impl_temp!(QuadF QuadRef QuadMut);

pub struct QuadModule;

impl LuaModule for QuadModule {
    const PATH: &'static str = "Quad";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<QuadF>::register(lua, "Quad", |members| {
            // fields
            members.getter("a", |quad| quad.0[0])?;
            members.setter("a", |quad, val: Vec2F| quad.0[0] = val)?;
            members.getter("b", |quad| quad.0[1])?;
            members.setter("b", |quad, val: Vec2F| quad.0[1] = val)?;
            members.getter("c", |quad| quad.0[2])?;
            members.setter("c", |quad, val: Vec2F| quad.0[2] = val)?;
            members.getter("d", |quad| quad.0[3])?;
            members.setter("d", |quad, val: Vec2F| quad.0[3] = val)?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<QuadF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("points", |quad, _: ()| {
                let [a, b, c, d] = quad.0;
                (a, b, c, d)
            })?;
            members.method("vectors", |quad, _: ()| {
                (quad.ab(), quad.bc(), quad.cd(), quad.da())
            })?;
            members.method("ab", |quad, _: ()| quad.ab())?;
            members.method("bc", |quad, _: ()| quad.bc())?;
            members.method("cd", |quad, _: ()| quad.cd())?;
            members.method("da", |quad, _: ()| quad.da())?;
            members.method("edges", |quad, _: ()| {
                let [a, b, c, d] = quad.edges();
                (a, b, c, d)
            })?;
            members.method("edge_ab", |quad, _: ()| quad.edge_ab())?;
            members.method("edge_bc", |quad, _: ()| quad.edge_bc())?;
            members.method("edge_cd", |quad, _: ()| quad.edge_cd())?;
            members.method("edge_da", |quad, _: ()| quad.edge_da())?;
            members.method("norms", |q, _: ()| {
                (q.norm_ab(), q.norm_bc(), q.norm_cd(), q.norm_da())
            })?;
            members.method("norm_ab", |quad, _: ()| quad.norm_ab())?;
            members.method("norm_bc", |quad, _: ()| quad.norm_bc())?;
            members.method("norm_cd", |quad, _: ()| quad.norm_cd())?;
            members.method("norm_da", |quad, _: ()| quad.norm_da())?;

            // impl Shape
            add_shape_methods(members)?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (a, b, c, d): (Vec2F, Vec2F, Vec2F, Vec2F)| {
                Ok(QuadF::new(a, b, c, d))
            })?,
        )?;
        module.set(
            "from_rect",
            lua.create_function(|_, rect: RectF| Ok(QuadF::from_rect(rect)))?,
        )?;
        module.set(
            "line",
            lua.create_function(|_, (a, b, a_w, b_w): (Vec2F, Vec2F, f32, Option<f32>)| {
                Ok(QuadF::stroke(LineF::new(a, b), a_w, b_w.unwrap_or(a_w)))
            })?,
        )?;

        Ok(Value::Table(module))
    }
}
