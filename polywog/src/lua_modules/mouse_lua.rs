use crate::core::Context;
use crate::input::MouseButton;
use crate::lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, Integer, IntoLua, Lua, Result, Value};

pub struct MouseModule;

impl LuaModule for MouseModule {
    const PATH: &'static str = "Mouse";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;

        m.set(
            "pos",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).mouse.pos()))?,
        )?;
        m.set(
            "x",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).mouse.pos().x))?,
        )?;
        m.set(
            "y",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).mouse.pos().y))?,
        )?;
        m.set(
            "scroll",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).mouse.scroll_delta().y))?,
        )?;
        m.set(
            "down",
            lua.create_function(|lua, btn: MouseButton| {
                Ok(Context::from_lua(lua).mouse.down(btn))
            })?,
        )?;
        m.set(
            "pressed",
            lua.create_function(|lua, btn: MouseButton| {
                Ok(Context::from_lua(lua).mouse.pressed(btn))
            })?,
        )?;
        m.set(
            "released",
            lua.create_function(|lua, btn: MouseButton| {
                Ok(Context::from_lua(lua).mouse.released(btn))
            })?,
        )?;

        Ok(Value::Table(m))
    }
}

impl FromLua for MouseButton {
    #[inline]
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::Integer(btn) => Self::from_repr(btn as usize)
                .ok_or_else(|| LuaError::runtime(format!("invalid button [{btn}]"))),
            value => Err(LuaError::runtime(format!("invalid button [{value:?}]"))),
        }
    }
}

impl IntoLua for MouseButton {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Integer(self as Integer))
    }
}
