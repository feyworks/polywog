use crate::core::Context;
use crate::input::Key;
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

pub struct KeyboardModule;

impl LuaModule for KeyboardModule {
    const PATH: &'static str = "Keyboard";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set(
            "down",
            lua.create_function(|lua, key: Key| Ok(Context::from_lua(lua).keyboard.down(key)))?,
        )?;
        m.set(
            "pressed",
            lua.create_function(|lua, key: Key| Ok(Context::from_lua(lua).keyboard.pressed(key)))?,
        )?;
        m.set(
            "released",
            lua.create_function(|lua, key: Key| Ok(Context::from_lua(lua).keyboard.released(key)))?,
        )?;
        m.set(
            "repeated",
            lua.create_function(|lua, key: Key| Ok(Context::from_lua(lua).keyboard.repeated(key)))?,
        )?;
        m.set(
            "pressed_or_repeated",
            lua.create_function(|lua, key: Key| {
                Ok(Context::from_lua(lua).keyboard.pressed_or_repeated(key))
            })?,
        )?;
        m.set(
            "text_input",
            lua.create_function(|lua, _: ()| {
                lua.create_string(Context::from_lua(lua).keyboard.text_input())
            })?,
        )?;
        m.set(
            "ctrl",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).keyboard.ctrl()))?,
        )?;
        m.set(
            "shift",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).keyboard.shift()))?,
        )?;
        m.set(
            "alt",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).keyboard.alt()))?,
        )?;
        m.set(
            "cmd",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).keyboard.cmd()))?,
        )?;
        m.set(
            "ctrl_or_cmd",
            lua.create_function(|lua, _: ()| Ok(Context::from_lua(lua).keyboard.ctrl_or_cmd()))?,
        )?;

        Ok(Value::Table(m))
    }
}
