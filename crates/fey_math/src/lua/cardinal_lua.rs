use crate::{Cardinal, DegreesF, Direction, Octal, RadiansF, RotationsF, Vec2F};
use fey_lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, IntoLua, Lua, Value};

impl FromLua for Cardinal {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let idx = usize::from_lua(value, lua)?;
        Self::from_index(idx).ok_or_else(|| LuaError::runtime("invalid cardinal direction"))
    }
}

impl IntoLua for Cardinal {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::Integer(self as i64))
    }
}

pub struct CardinalModule;

impl LuaModule for CardinalModule {
    const PATH: &'static str = "Cardinal";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set("EAST", Cardinal::East)?;
        m.set("SOUTH", Cardinal::South)?;
        m.set("WEST", Cardinal::West)?;
        m.set("NORTH", Cardinal::North)?;
        m.set(
            "name",
            lua.create_function(|lua, dir: Cardinal| {
                lua.create_string(match dir {
                    Cardinal::East => "EAST",
                    Cardinal::South => "SOUTH",
                    Cardinal::West => "WEST",
                    Cardinal::North => "NORTH",
                })
            })?,
        )?;
        m.set(
            "to_rads",
            lua.create_function(|_, dir: Cardinal| Ok(RadiansF::from_cardinal(dir)))?,
        )?;
        m.set(
            "to_degs",
            lua.create_function(|_, dir: Cardinal| Ok(DegreesF::from_cardinal(dir).0))?,
        )?;
        m.set(
            "to_rots",
            lua.create_function(|_, dir: Cardinal| Ok(RotationsF::from_cardinal(dir).0))?,
        )?;
        m.set(
            "from_vec2",
            lua.create_function(|_, v: Vec2F| Ok(Cardinal::from_vec2(v)))?,
        )?;
        m.set(
            "sin_cos",
            lua.create_function(|_, dir: Cardinal| Ok(Direction::<f32>::sin_cos(dir)))?,
        )?;
        m.set(
            "norm",
            lua.create_function(|_, (dir, len): (Cardinal, Option<f32>)| {
                let norm = Direction::<f32>::norm(dir);
                Ok(match len {
                    Some(len) => norm * len,
                    None => norm,
                })
            })?,
        )?;
        m.set(
            "rev",
            lua.create_function(|_, dir: Cardinal| Ok(dir.rev()))?,
        )?;
        m.set(
            "from_octal",
            lua.create_function(|_, (oct, bias): (Octal, Cardinal)| {
                Ok(Cardinal::from_octal(oct, bias))
            })?,
        )?;
        m.set("cw", lua.create_function(|_, dir: Cardinal| Ok(dir.cw()))?)?;
        m.set(
            "ccw",
            lua.create_function(|_, dir: Cardinal| Ok(dir.ccw()))?,
        )?;
        m.set(
            "grid_step",
            lua.create_function(|_, dir: Cardinal| {
                let s = dir.grid_step::<i64>();
                Ok((s.x, s.y))
            })?,
        )?;
        Ok(Value::Table(m))
    }
}
