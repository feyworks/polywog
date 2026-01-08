use crate::ProjectionF;
use fey_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};

pub type ProjectionRef = UserDataRef<ProjectionF>;
pub type ProjectionMut = UserDataRefMut<ProjectionF>;

impl FromLua for ProjectionF {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for ProjectionF {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

pub struct ProjectionModule;

impl LuaModule for ProjectionModule {
    const PATH: &'static str = "Projection";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<ProjectionF>::register(lua, "Projection", |members| {
            // fields
            members.getter("min", |proj| proj.min)?;
            members.setter("min", |proj, val: f32| proj.min = val)?;
            members.getter("max", |proj| proj.max)?;
            members.setter("max", |proj, val: f32| proj.max = val)?;

            // methods
            members.method("approx", |a, b: ProjectionF| a.relative_eq(&b))?;
            members.method("len", |proj, _: ()| proj.len())?;
            members.method("overlap", |a, b: ProjectionF| a.overlap(b))?;
            members.method("overlaps", |a, b: ProjectionF| a.overlaps(b))?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (min, max): (f32, f32)| Ok(ProjectionF { min, max }))?,
        )?;

        Ok(Value::Table(module))
    }
}
