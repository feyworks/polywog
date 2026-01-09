use mlua::prelude::LuaResult;
use mlua::{Lua, Table};

/// If a table is provided, clear and return it. Otherwise, return a new empty table.
#[inline]
pub fn create_fill(lua: &Lua, fill: Option<Table>) -> LuaResult<Table> {
    match fill {
        Some(fill) => {
            fill.clear()?;
            Ok(fill)
        }
        None => lua.create_table(),
    }
}
