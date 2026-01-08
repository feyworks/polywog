use crate::RadiansF;
use mlua::{FromLua, IntoLua, Lua, Value};

impl FromLua for RadiansF {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        f32::from_lua(value, lua).map(Self)
    }
}

impl IntoLua for RadiansF {
    #[inline]
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        self.0.into_lua(lua)
    }
}
