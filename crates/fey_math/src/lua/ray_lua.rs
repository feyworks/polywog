use crate::{RayF, Vec2F, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

impl_temp!(RayF RayRef RayMut);

pub struct RayModule;

impl LuaModule for RayModule {
    const PATH: &'static str = "Ray";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<RayF>::register(lua, "Ray", |members| {
            // fields
            members.getter("origin", |ray| ray.origin)?;
            members.setter("origin", |ray, val: Vec2F| ray.origin = val)?;
            members.getter("direction", |ray| ray.direction)?;
            members.setter("direction", |ray, val: Vec2F| ray.direction = val)?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<RayF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("point", |ray, dist: f32| ray.point(dist))?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (orig, dir): (Vec2F, Vec2F)| Ok(RayF::new(orig, dir)))?,
        )?;

        Ok(Value::Table(module))
    }
}
