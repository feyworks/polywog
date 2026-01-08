use crate::{Vec2F, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, Lua, Value, Variadic};

impl_temp!(Vec2F Vec2Ref Vec2Mut);

pub struct Vec2Module;

impl LuaModule for Vec2Module {
    const PATH: &'static str = "Vec2";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Vec2F>::register(lua, "Vec2", |members| {
            // fields
            members.getter("x", |obj| obj.x)?;
            members.setter("x", |obj, val: f32| obj.x = val)?;
            members.getter("y", |obj| obj.y)?;
            members.setter("y", |obj, val: f32| obj.y = val)?;

            // operators
            members.op_tostring(|obj| format!("{obj}"))?;
            members.op_eq(|a, b: Vec2F| a == &b)?;
            members.op_add(|a, b: Vec2F| a + b)?;
            members.op_sub(|a, b: Vec2F| a - b)?;
            members.op_mul(|a, b: Either<Vec2F, f32>| match b {
                Either::Left(b) => a * b,
                Either::Right(b) => a * b,
            })?;
            members.op_div(|a, b: Either<Vec2F, f32>| match b {
                Either::Left(b) => a / b,
                Either::Right(b) => a / b,
            })?;
            members.op_unm(|obj| -*obj)?;

            // methods
            members.method_mut("set", |obj, (x, y): (f32, f32)| {
                obj.x = x;
                obj.y = y;
            })?;
            members.method("abs", |obj, _: ()| obj.abs())?;
            members.method("approx", |a, b: Vec2F| a.relative_eq(&b))?;
            members.method("approx_zero", |obj, _: ()| obj.relative_eq(&Vec2F::ZERO))?;
            members.method("ceil", |obj, _: ()| obj.ceil())?;
            members.method("clamp", |obj, (min, max): (Vec2F, Vec2F)| {
                obj.clamp(min, max)
            })?;
            members.method("cross", |a, b: Vec2F| a.cross(b))?;
            members.method("dir_to", |a, b: Vec2F| a.dir_to(b))?;
            members.method("dist", |a, b: Vec2F| a.dist(b))?;
            members.method("dot", |a, b: Vec2F| a.dot(b))?;
            members.method("floor", |obj, _: ()| obj.floor())?;
            members.method("is_zero", |obj, _: ()| obj.is_zero())?;
            members.method("len", |obj, _: ()| obj.len())?;
            members.method("max", |obj, args: Variadic<Vec2F>| {
                args.into_iter().fold(*obj, |max, arg| max.max(arg))
            })?;
            members.method("min", |obj, args: Variadic<Vec2F>| {
                args.into_iter().fold(*obj, |min, arg| min.min(arg))
            })?;
            members.method("norm", |obj, _: ()| obj.norm())?;
            members.method("reflect", |obj, norm: Vec2F| obj.reflect(norm))?;
            members.method("round", |obj, _: ()| obj.round())?;
            members.method("sign", |obj, _: ()| obj.signum())?;
            members.method("sqr_dist", |a, b: Vec2F| a.sqr_dist(b))?;
            members.method("sqr_len", |obj, _: ()| obj.sqr_len())?;
            members.method("trunc", |obj, _: ()| obj.trunc())?;
            members.method("turn_left", |obj, _: ()| obj.turn_left())?;
            members.method("turn_right", |obj, _: ()| obj.turn_right())?;
            members.method("with_len", |obj, new_len: f32| obj.norm_safe() * new_len)?;
            members.method("with_x", |obj, val: f32| obj.with_x(val))?;
            members.method("with_y", |obj, val: f32| obj.with_y(val))?;
            members.method("with_z", |obj, val: f32| obj.with_z(val))?;
            members.method("yx", |obj, _: ()| obj.yx())?;

            Ok(())
        })?;

        module.set_metatable(Some({
            let meta = lua.create_table()?;
            meta.set(
                "__call",
                lua.create_function(|_, (_, x, y): (Value, f32, f32)| Ok(Vec2F::new(x, y)))?,
            )?;
            meta
        }))?;

        macro_rules! constant {
            ($name:literal $var:ident) => {
                module.set($name, lua.create_function(|_, _: ()| Ok(Vec2F::$var))?)?;
            };
        }
        constant!("zero" ZERO);
        constant!("one" ONE);
        constant!("x_axis" X_AXIS);
        constant!("y_axis" Y_AXIS);
        constant!("right" RIGHT);
        constant!("left" LEFT);
        constant!("down" DOWN);
        constant!("up" UP);
        constant!("east" EAST);
        constant!("south_east" SOUTH_EAST);
        constant!("south" SOUTH);
        constant!("south_west" SOUTH_WEST);
        constant!("west" WEST);
        constant!("north_west" NORTH_WEST);
        constant!("north" NORTH);
        constant!("north_east" NORTH_EAST);

        module.set(
            "new",
            lua.create_function(|_, (x, y): (f32, f32)| Ok(Vec2F::new(x, y)))?,
        )?;
        module.set(
            "splat",
            lua.create_function(|_, val: f32| Ok(Vec2F::splat(val)))?,
        )?;
        module.set(
            "barycentric",
            lua.create_function(|_, (a, b, c, ab, bc): (Vec2F, Vec2F, Vec2F, f32, f32)| {
                Ok(Vec2F::barycentric(a, b, c, ab, bc))
            })?,
        )?;

        // module.set(
        //     "persistent",
        //     lua.create_function(|lua, (x, y): (f32, f32)| {
        //         lua.create_any_userdata(Self::new(x, y))
        //     })?,
        // )?;

        Ok(Value::Table(module))
    }
}
