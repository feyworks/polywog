use crate::{LineF, RayF, Vec2F};
use fey_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};

pub type LineRef = UserDataRef<LineF>;
pub type LineMut = UserDataRefMut<LineF>;

impl FromLua for LineF {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for LineF {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

pub struct LineModule;

impl LuaModule for LineModule {
    const PATH: &'static str = "Line";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<LineF>::register(lua, "Line", |members| {
            // fields
            members.getter("a", |line| line.start)?;
            members.setter("a", |line, val: Vec2F| line.start = val)?;
            members.getter("b", |line| line.end)?;
            members.setter("b", |line, val: Vec2F| line.end = val)?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<LineF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("points", |line, _: ()| (line.start, line.end))?;
            members.method("rev", |line, _: ()| line.rev())?;
            members.method("vector", |line, _: ()| line.vector())?;
            members.method("sqr_len", |line, _: ()| line.sqr_len())?;
            members.method("bounds", |line, _: ()| line.bounds())?;
            members.method("center", |line, _: ()| line.center())?;
            members.method("len", |line, _: ()| line.len())?;
            members.method("norm", |line, _: ()| line.norm())?;
            members.method("left_norm", |line, _: ()| line.left_norm())?;
            members.method("right_norm", |line, _: ()| line.right_norm())?;
            members.method("project_onto_axis", |line, axis: Vec2F| {
                line.project_onto_axis(axis)
            })?;
            members.method("project_point", |line, p: Vec2F| line.project_point(p))?;
            members.method_ext("rayhit", |lua, line, ray: Temp<RayF>| {
                ray.read(lua, |_, ray| Ok(line.rayhit(ray)))
            })?;
            members.method_ext("raycast", |lua, line, ray: Temp<RayF>| {
                ray.read(lua, |_, ray| Ok(line.raycast(ray)))
            })?;

            Ok(())
        })?;

        module.set("zero", lua.create_function(|_, _: ()| Ok(LineF::ZERO))?)?;
        module.set(
            "new",
            lua.create_function(|_, (a, b): (Vec2F, Vec2F)| Ok(LineF::new(a, b)))?,
        )?;

        Ok(Value::Table(module))
    }
}
