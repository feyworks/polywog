use crate::{Vec3F, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, Lua, Value, Variadic};

impl_temp!(Vec3F Vec3Ref Vec3Mut);

pub struct Vec3Module;

impl LuaModule for Vec3Module {
    const PATH: &'static str = "Vec3";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Vec3F>::register(lua, "Vec3", |members| {
            // fields
            members.getter("x", |obj| obj.x)?;
            members.setter("x", |obj, val: f32| obj.x = val)?;
            members.getter("y", |obj| obj.y)?;
            members.setter("y", |obj, val: f32| obj.y = val)?;
            members.getter("z", |obj| obj.z)?;
            members.setter("z", |obj, val: f32| obj.z = val)?;

            // operators
            members.op_tostring(|obj| format!("{obj}"))?;
            members.op_eq(|a, b: Vec3F| a == &b)?;
            members.op_add(|a, b: Vec3F| a + b)?;
            members.op_sub(|a, b: Vec3F| a - b)?;
            members.op_mul(|a, b: Either<Vec3F, f32>| match b {
                Either::Left(b) => a * b,
                Either::Right(b) => a * b,
            })?;
            members.op_div(|a, b: Either<Vec3F, f32>| match b {
                Either::Left(b) => a / b,
                Either::Right(b) => a / b,
            })?;
            members.op_unm(|obj| -*obj)?;

            // methods
            members.method("abs", |obj, _: ()| obj.abs())?;
            members.method("approx", |a, b: Vec3F| a.relative_eq(&b))?;
            members.method("approx_zero", |obj, _: ()| obj.relative_eq(&Vec3F::ZERO))?;
            members.method("ceil", |obj, _: ()| obj.ceil())?;
            members.method("clamp", |obj, (min, max): (Vec3F, Vec3F)| {
                obj.clamp(min, max)
            })?;
            members.method("cross", |a, b: Vec3F| a.cross(b))?;
            members.method("dir_to", |a, b: Vec3F| a.dir_to(b))?;
            members.method("dist", |a, b: Vec3F| a.dist(b))?;
            members.method("dot", |a, b: Vec3F| a.dot(b))?;
            members.method("floor", |obj, _: ()| obj.floor())?;
            members.method("is_zero", |obj, _: ()| obj.is_zero())?;
            members.method("len", |obj, _: ()| obj.len())?;
            members.method("max", |obj, args: Variadic<Vec3F>| {
                args.into_iter().fold(*obj, |max, arg| max.max(arg))
            })?;
            members.method("min", |obj, args: Variadic<Vec3F>| {
                args.into_iter().fold(*obj, |min, arg| min.min(arg))
            })?;
            members.method("norm", |obj, _: ()| obj.norm())?;
            members.method("reflect", |obj, norm: Vec3F| obj.reflect(norm))?;
            members.method("round", |obj, _: ()| obj.round())?;
            members.method("sign", |obj, _: ()| obj.signum())?;
            members.method("sqr_dist", |a, b: Vec3F| a.sqr_dist(b))?;
            members.method("sqr_len", |obj, _: ()| obj.sqr_len())?;
            members.method("trunc", |obj, _: ()| obj.trunc())?;
            members.method("with_len", |obj, new_len: f32| obj.norm_safe() * new_len)?;
            members.method("with_x", |obj, val: f32| obj.with_x(val))?;
            members.method("with_y", |obj, val: f32| obj.with_y(val))?;
            members.method("with_z", |obj, val: f32| obj.with_z(val))?;
            members.method("with_w", |obj, val: f32| obj.with_w(val))?;
            members.method("xy", |obj, _: ()| obj.xy())?;
            members.method("xzy", |obj, _: ()| obj.xzy())?;
            members.method("yxz", |obj, _: ()| obj.yxz())?;
            members.method("yzx", |obj, _: ()| obj.yzx())?;
            members.method("zxy", |obj, _: ()| obj.zxy())?;
            members.method("zyx", |obj, _: ()| obj.zyx())?;

            Ok(())
        })?;

        module.set_metatable(Some({
            let meta = lua.create_table()?;
            meta.set(
                "__call",
                lua.create_function(|_, (_, x, y, z): (Value, f32, f32, f32)| {
                    Ok(Vec3F::new(x, y, z))
                })?,
            )?;
            meta
        }))?;

        macro_rules! constant {
            ($name:literal $var:ident) => {
                module.set($name, lua.create_function(|_, _: ()| Ok(Vec3F::$var))?)?;
            };
        }
        constant!("zero" ZERO);
        constant!("one" ONE);
        constant!("x_axis" X_AXIS);
        constant!("y_axis" Y_AXIS);
        constant!("z_axis" Z_AXIS);
        constant!("right" RIGHT);
        constant!("left" LEFT);
        constant!("down" DOWN);
        constant!("up" UP);
        constant!("forward" FORWARD);
        constant!("backward" BACKWARD);

        module.set(
            "new",
            lua.create_function(|_, (x, y, z): (f32, f32, f32)| Ok(Vec3F::new(x, y, z)))?,
        )?;
        module.set(
            "splat",
            lua.create_function(|_, val: f32| Ok(Vec3F::splat(val)))?,
        )?;
        module.set(
            "barycentric",
            lua.create_function(|_, (a, b, c, ab, bc): (Vec3F, Vec3F, Vec3F, f32, f32)| {
                Ok(Vec3F::barycentric(a, b, c, ab, bc))
            })?,
        )?;

        Ok(Value::Table(module))
    }
}
