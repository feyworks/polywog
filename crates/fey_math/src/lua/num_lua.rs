use crate::{Approach, Interp};
use fey_lua::LuaModule;
use mlua::prelude::LuaResult;
use mlua::{Integer, Lua, Number, Value, Variadic};

pub struct NumModule;

impl LuaModule for NumModule {
    const PATH: &'static str = "Num";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set("PI", std::f64::consts::PI)?;
        m.set("PI_OVER_2", std::f64::consts::FRAC_PI_2)?;
        m.set("PI_OVER_4", std::f64::consts::FRAC_PI_4)?;
        m.set("TAU", std::f64::consts::TAU)?;
        m.set("SQRT_2", std::f64::consts::SQRT_2)?;
        m.set("MAX_INT", Integer::MAX)?;
        m.set("MIN_INT", Integer::MIN)?;
        m.set("MAX_NUM", Number::MAX)?;
        m.set("MIN_NUM", Number::MIN)?;
        m.set("abs", lua.create_function(|_, x: f64| Ok(x.abs()))?)?;
        m.set(
            "diff",
            lua.create_function(|_, (x, y): (f64, f64)| Ok((x - y).abs()))?,
        )?;
        m.set("acos", lua.create_function(|_, x: f64| Ok(x.acos()))?)?;
        m.set("acosh", lua.create_function(|_, x: f64| Ok(x.acosh()))?)?;
        m.set(
            "approach",
            lua.create_function(|_, (from, to, amount): (f64, f64, f64)| {
                Ok(from.approach(to, amount))
            })?,
        )?;
        m.set("asin", lua.create_function(|_, x: f64| Ok(x.asin()))?)?;
        m.set("asinh", lua.create_function(|_, x: f64| Ok(x.asinh()))?)?;
        m.set("atan", lua.create_function(|_, x: f64| Ok(x.atan()))?)?;
        m.set(
            "atan2",
            lua.create_function(|_, (y, x): (f64, f64)| Ok(y.atan2(x)))?,
        )?;
        m.set("atanh", lua.create_function(|_, x: f64| Ok(x.atanh()))?)?;
        m.set(
            "catmull_rom",
            lua.create_function(|_, (a, b, c, d, t): (f64, f64, f64, f64, f64)| {
                Ok(f64::catmull_rom(a, b, c, d, t))
            })?,
        )?;
        m.set("cbrt", lua.create_function(|_, x: f64| Ok(x.cbrt()))?)?;
        m.set("ceil", lua.create_function(|_, x: f64| Ok(x.ceil()))?)?;
        m.set(
            "clamp",
            lua.create_function(|_, (x, min, max): (f64, f64, f64)| Ok(f64::clamp(x, min, max)))?,
        )?;
        m.set("cos", lua.create_function(|_, x: f64| Ok(x.cos()))?)?;
        m.set("cosh", lua.create_function(|_, x: f64| Ok(x.cosh()))?)?;
        m.set(
            "cubic_bezier",
            lua.create_function(|_, (a, b, c, d, t): (f64, f64, f64, f64, f64)| {
                Ok(f64::cubic_bezier(a, b, c, d, t))
            })?,
        )?;
        m.set("exp", lua.create_function(|_, x: f64| Ok(x.exp()))?)?;
        m.set("exp2", lua.create_function(|_, x: f64| Ok(x.exp2()))?)?;
        m.set("floor", lua.create_function(|_, x: f64| Ok(x.floor()))?)?;
        m.set("fract", lua.create_function(|_, x: f64| Ok(x.fract()))?)?;
        m.set(
            "hermite",
            lua.create_function(|_, (a, b, c, d, t): (f64, f64, f64, f64, f64)| {
                Ok(f64::hermite(a, b, c, d, t))
            })?,
        )?;
        m.set(
            "inv_lerp",
            lua.create_function(|_, (x, a, b): (f64, f64, f64)| Ok((x - a) / (b - a)))?,
        )?;
        m.set(
            "is_finite",
            lua.create_function(|_, x: f64| Ok(x.is_finite()))?,
        )?;
        m.set(
            "is_infinite",
            lua.create_function(|_, x: f64| Ok(x.is_infinite()))?,
        )?;
        m.set("is_nan", lua.create_function(|_, x: f64| Ok(x.is_nan()))?)?;
        m.set(
            "lerp",
            lua.create_function(|_, (a, b, t): (f64, f64, f64)| Ok(f64::lerp(a, b, t)))?,
        )?;
        m.set("ln", lua.create_function(|_, x: f64| Ok(x.ln()))?)?;
        m.set(
            "log",
            lua.create_function(|_, (x, base): (f64, f64)| Ok(x.log(base)))?,
        )?;
        m.set(
            "remap",
            lua.create_function(|_, (x, a, b, c, d): (f64, f64, f64, f64, f64)| {
                Ok(f64::lerp(c, d, (x - a) / (b - a)))
            })?,
        )?;
        m.set(
            "max",
            lua.create_function(|_, (x, args): (f64, Variadic<f64>)| {
                let mut max = x;
                for x in args {
                    max = max.max(x);
                }
                Ok(max)
            })?,
        )?;
        m.set(
            "min",
            lua.create_function(|_, (x, args): (f64, Variadic<f64>)| {
                let mut min = x;
                for x in args {
                    min = min.min(x);
                }
                Ok(min)
            })?,
        )?;
        m.set(
            "pow",
            lua.create_function(|_, (x, n): (f64, f64)| Ok(f64::powf(x, n)))?,
        )?;
        m.set(
            "quad_bezier",
            lua.create_function(|_, (a, b, c, t): (f64, f64, f64, f64)| {
                Ok(f64::quad_bezier(a, b, c, t))
            })?,
        )?;
        m.set("round", lua.create_function(|_, x: f64| Ok(x.round()))?)?;
        m.set(
            "sign",
            lua.create_function(|_, x: f64| {
                Ok(if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    -1.0
                } else {
                    0.0
                })
            })?,
        )?;
        m.set("sin", lua.create_function(|_, x: f64| Ok(x.sin()))?)?;
        m.set("sin_cos", lua.create_function(|_, x: f64| Ok(x.sin_cos()))?)?;
        m.set("sinh", lua.create_function(|_, x: f64| Ok(x.sinh()))?)?;
        m.set(
            "smoothstep",
            lua.create_function(|_, (a, b, t): (f64, f64, f64)| Ok(f64::smooth_step(a, b, t)))?,
        )?;
        m.set("sqrt", lua.create_function(|_, x: f64| Ok(x.sqrt()))?)?;
        m.set("tan", lua.create_function(|_, x: f64| Ok(x.tan()))?)?;
        m.set("tanh", lua.create_function(|_, x: f64| Ok(x.tanh()))?)?;
        m.set(
            "to_degrees",
            lua.create_function(|_, x: f64| Ok(x.to_degrees()))?,
        )?;
        m.set(
            "to_radians",
            lua.create_function(|_, x: f64| Ok(x.to_radians()))?,
        )?;
        m.set("trunc", lua.create_function(|_, x: f64| Ok(x.trunc()))?)?;
        Ok(Value::Table(m))
    }
}
