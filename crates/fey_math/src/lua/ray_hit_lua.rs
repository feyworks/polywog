use crate::{RayHitF, Vec2F, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

impl_temp!(RayHitF RayHitRef RayHitMut);

pub struct RayHitModule;

impl LuaModule for RayHitModule {
    const PATH: &'static str = "RayHit";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<RayHitF>::register(lua, "RayHit", |members| {
            // fields
            members.getter("normal", |hit| hit.normal)?;
            members.setter("normal", |hit, val: Vec2F| hit.normal = val)?;
            members.getter("distance", |hit| hit.distance)?;
            members.setter("distance", |hit, val: f32| hit.distance = val)?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<RayHitF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (norm, dist): (Vec2F, f32)| Ok(RayHitF::new(norm, dist)))?,
        )?;

        Ok(Value::Table(module))
    }
}
