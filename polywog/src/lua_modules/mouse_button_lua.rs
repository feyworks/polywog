use crate::input::MouseButton;
use crate::lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{Lua, Value};

pub struct MouseButtonModule;

impl LuaModule for MouseButtonModule {
    const PATH: &'static str = "MouseButton";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set("LEFT", MouseButton::Left as usize)?;
        m.set("RIGHT", MouseButton::Right as usize)?;
        m.set("MIDDLE", MouseButton::Middle as usize)?;
        Ok(Value::Table(m))
    }
}
