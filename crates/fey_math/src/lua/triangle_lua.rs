use crate::{TriangleF, Vec2F, add_shape_methods, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

impl_temp!(TriangleF TriangleRef TriangleMut);

pub struct TriangleModule;

impl LuaModule for TriangleModule {
    const PATH: &'static str = "Triangle";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<TriangleF>::register(lua, "Triangle", |members| {
            // fields
            members.getter("a", |tri| tri.0[0])?;
            members.setter("a", |tri, val: Vec2F| tri.0[0] = val)?;
            members.getter("b", |tri| tri.0[1])?;
            members.setter("b", |tri, val: Vec2F| tri.0[1] = val)?;
            members.getter("c", |tri| tri.0[2])?;
            members.setter("c", |tri, val: Vec2F| tri.0[2] = val)?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<TriangleF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("points", |tri, _: ()| {
                let [a, b, c] = tri.0;
                (a, b, c)
            })?;
            members.method("vectors", |tri, _: ()| (tri.ab(), tri.bc(), tri.ca()))?;
            members.method("ab", |tri, _: ()| tri.ab())?;
            members.method("bc", |tri, _: ()| tri.bc())?;
            members.method("ca", |tri, _: ()| tri.ca())?;
            members.method("edges", |tri, _: ()| {
                let [a, b, c] = tri.edges();
                (a, b, c)
            })?;
            members.method("edge_ab", |tri, _: ()| tri.edge_ab())?;
            members.method("edge_bc", |tri, _: ()| tri.edge_bc())?;
            members.method("edge_ca", |tri, _: ()| tri.edge_ca())?;
            members.method("norms", |q, _: ()| (q.norm_ab(), q.norm_bc(), q.norm_ca()))?;
            members.method("norm_ab", |tri, _: ()| tri.norm_ab())?;
            members.method("norm_bc", |tri, _: ()| tri.norm_bc())?;
            members.method("norm_ca", |tri, _: ()| tri.norm_ca())?;

            // impl Shape
            add_shape_methods(members)?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (a, b, c): (Vec2F, Vec2F, Vec2F)| Ok(TriangleF::new(a, b, c)))?,
        )?;

        Ok(Value::Table(module))
    }
}
