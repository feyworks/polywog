use crate::{Cardinal, DegreesF, Direction, Octal, RadiansF, RotationsF, Vec2F};
use fey_lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, IntoLua, Lua, Value};

impl FromLua for Octal {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let idx = usize::from_lua(value, lua)?;
        Self::from_index(idx).ok_or_else(|| LuaError::runtime("invalid octal direction"))
    }
}

impl IntoLua for Octal {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::Integer(self as i64))
    }
}

pub struct OctalModule;

impl LuaModule for OctalModule {
    const PATH: &'static str = "Octal";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set("EAST", Octal::East)?;
        m.set("SOUTH_EAST", Octal::SouthEast)?;
        m.set("SOUTH", Octal::South)?;
        m.set("SOUTH_WEST", Octal::SouthWest)?;
        m.set("WEST", Octal::West)?;
        m.set("NORTH_WEST", Octal::NorthWest)?;
        m.set("NORTH", Octal::North)?;
        m.set("NORTH_EAST", Octal::NorthEast)?;
        m.set(
            "name",
            lua.create_function(|lua, dir: Octal| {
                lua.create_string(match dir {
                    Octal::East => "EAST",
                    Octal::SouthEast => "SOUTH_EAST",
                    Octal::South => "SOUTH",
                    Octal::SouthWest => "SOUTH_WEST",
                    Octal::West => "WEST",
                    Octal::NorthWest => "NORTH_WEST",
                    Octal::North => "NORTH",
                    Octal::NorthEast => "NORTH_EAST",
                })
            })?,
        )?;
        m.set(
            "to_rads",
            lua.create_function(|_, dir: Octal| Ok(RadiansF::from_octal(dir)))?,
        )?;
        m.set(
            "to_degs",
            lua.create_function(|_, dir: Octal| Ok(DegreesF::from_octal(dir).0))?,
        )?;
        m.set(
            "to_rots",
            lua.create_function(|_, dir: Octal| Ok(RotationsF::from_octal(dir).0))?,
        )?;
        m.set(
            "from_vec2",
            lua.create_function(|_, v: Vec2F| Ok(Octal::from_vec2(v)))?,
        )?;
        m.set(
            "sin_cos",
            lua.create_function(|_, dir: Octal| Ok(Direction::<f32>::sin_cos(dir)))?,
        )?;
        m.set(
            "norm",
            lua.create_function(|_, (dir, len): (Octal, Option<f32>)| {
                let norm = Direction::<f32>::norm(dir);
                Ok(match len {
                    Some(len) => norm * len,
                    None => norm,
                })
            })?,
        )?;
        m.set("rev", lua.create_function(|_, dir: Octal| Ok(dir.rev()))?)?;
        m.set(
            "from_cardinal",
            lua.create_function(|_, dir: Cardinal| Ok(Octal::from_cardinal(dir)))?,
        )?;
        m.set("cw", lua.create_function(|_, dir: Octal| Ok(dir.cw()))?)?;
        m.set("ccw", lua.create_function(|_, dir: Octal| Ok(dir.ccw()))?)?;
        m.set(
            "grid_step",
            lua.create_function(|_, dir: Octal| {
                let s = dir.grid_step::<i64>();
                Ok((s.x, s.y))
            })?,
        )?;
        Ok(Value::Table(m))
    }
}
