use crate::{Vec4F, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Lua, Value, Variadic};

impl_temp!(Vec4F Vec4Ref Vec4Mut);

pub struct Vec4Module;

impl LuaModule for Vec4Module {
    const PATH: &'static str = "Vec4";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Vec4F>::register(lua, "Vec4", |members| {
            // fields
            members.getter("x", |obj| obj.x)?;
            members.setter("x", |obj, val: f32| obj.x = val)?;
            members.getter("y", |obj| obj.y)?;
            members.setter("y", |obj, val: f32| obj.y = val)?;
            members.getter("z", |obj| obj.z)?;
            members.setter("z", |obj, val: f32| obj.z = val)?;
            members.getter("w", |obj| obj.w)?;
            members.setter("w", |obj, val: f32| obj.w = val)?;

            // methods
            members.method("abs", |obj, _: ()| obj.abs())?;
            members.method("approx", |a, b: Vec4F| a.relative_eq(&b))?;
            members.method("approx_zero", |obj, _: ()| obj.relative_eq(&Vec4F::ZERO))?;
            members.method("ceil", |obj, _: ()| obj.ceil())?;
            members.method("clamp", |obj, (min, max): (Vec4F, Vec4F)| {
                obj.clamp(min, max)
            })?;
            members.method("dir_to", |a, b: Vec4F| a.dir_to(b))?;
            members.method("dist", |a, b: Vec4F| a.dist(b))?;
            members.method("dot", |a, b: Vec4F| a.dot(b))?;
            members.method("floor", |obj, _: ()| obj.floor())?;
            members.method("is_zero", |obj, _: ()| obj.is_zero())?;
            members.method("len", |obj, _: ()| obj.len())?;
            members.method("max", |obj, args: Variadic<Vec4F>| {
                args.into_iter().fold(*obj, |max, arg| max.max(arg))
            })?;
            members.method("min", |obj, args: Variadic<Vec4F>| {
                args.into_iter().fold(*obj, |min, arg| min.min(arg))
            })?;
            members.method("norm", |obj, _: ()| obj.norm())?;
            members.method("reflect", |obj, norm: Vec4F| obj.reflect(norm))?;
            members.method("round", |obj, _: ()| obj.round())?;
            members.method("sign", |obj, _: ()| obj.signum())?;
            members.method("sqr_dist", |a, b: Vec4F| a.sqr_dist(b))?;
            members.method("sqr_len", |obj, _: ()| obj.sqr_len())?;
            members.method("with_x", |obj, val: f32| obj.with_x(val))?;
            members.method("with_y", |obj, val: f32| obj.with_y(val))?;
            members.method("with_z", |obj, val: f32| obj.with_z(val))?;
            members.method("with_w", |obj, val: f32| obj.with_w(val))?;
            members.method("xy", |obj, _: ()| obj.xy())?;
            members.method("xyz", |obj, _: ()| obj.xyz())?;
            members.method("wxzy", |obj, _: ()| obj.wxzy())?;
            members.method("wzxy", |obj, _: ()| obj.wzxy())?;
            members.method("wzyx", |obj, _: ()| obj.wzyx())?;
            members.method("ywzx", |obj, _: ()| obj.ywzx())?;
            members.method("yxwz", |obj, _: ()| obj.yxwz())?;
            members.method("yxzw", |obj, _: ()| obj.yxzw())?;
            members.method("yzwx", |obj, _: ()| obj.yzwx())?;
            members.method("yzxw", |obj, _: ()| obj.yzxw())?;
            members.method("zxyw", |obj, _: ()| obj.zxyw())?;
            members.method("zywx", |obj, _: ()| obj.zywx())?;
            members.method("zyxw", |obj, _: ()| obj.zyxw())?;

            Ok(())
        })?;

        module.set_metatable(Some({
            let meta = lua.create_table()?;
            meta.set(
                "__call",
                lua.create_function(|_, (_, x, y, z, w): (Value, f32, f32, f32, f32)| {
                    Ok(Vec4F::new(x, y, z, w))
                })?,
            )?;
            meta
        }))?;

        macro_rules! constant {
            ($name:literal $var:ident) => {
                module.set($name, lua.create_function(|_, _: ()| Ok(Vec4F::$var))?)?;
            };
        }
        constant!("zero" ZERO);
        constant!("one" ONE);
        constant!("x_axis" X_AXIS);
        constant!("y_axis" Y_AXIS);
        constant!("z_axis" Z_AXIS);
        constant!("w_axis" W_AXIS);

        module.set(
            "new",
            lua.create_function(|_, (x, y, z, w): (f32, f32, f32, f32)| {
                Ok(Vec4F::new(x, y, z, w))
            })?,
        )?;
        module.set(
            "splat",
            lua.create_function(|_, val: f32| Ok(Vec4F::splat(val)))?,
        )?;

        Ok(Value::Table(module))
    }
}
