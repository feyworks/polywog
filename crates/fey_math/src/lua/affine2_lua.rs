use crate::lua::impl_temp;
use crate::{Affine2F, Mat2F, RadiansF, Vec2F, vec2};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, Lua, Value};

impl_temp!(Affine2F Affine2Ref Affine2Mut);

pub struct Affine2Module;

impl LuaModule for Affine2Module {
    const PATH: &'static str = "Affine2";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Affine2F>::register(lua, "Affine2", |members| {
            // fields
            members.getter("matrix", |obj| obj.matrix)?;
            members.setter("matrix", |obj, val: Mat2F| obj.matrix = val)?;
            members.getter("translation", |obj| obj.translation)?;
            members.setter("translation", |obj, val: Vec2F| obj.translation = val)?;

            // operators
            members
                .op_mul_ext(|lua, a, b: Temp<Affine2F>| b.read(lua, |_, b| Ok(a.mul_affine2(b))))?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<Affine2F>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("inverse", |obj, _: ()| obj.inverse())?;
            members.method_ext("mul_affine2", |lua, a, b: Temp<Affine2F>| {
                b.read(lua, |_, b| Ok(a.mul_affine2(b)))
            })?;
            members.method("transform_vec2", |mat, vec: Vec2F| mat.transform_vec2(vec))?;
            members.method("transform_pos2", |mat, vec: Vec2F| mat.transform_pos2(vec))?;

            Ok(())
        })?;

        module.set(
            "identity",
            lua.create_function(|_, _: ()| Ok(Affine2F::IDENTITY))?,
        )?;
        module.set("zero", lua.create_function(|_, _: ()| Ok(Affine2F::ZERO))?)?;
        module.set(
            "new",
            lua.create_function(|_, (matrix, translation): (Mat2F, Vec2F)| {
                Ok(Affine2F::new(matrix, translation))
            })?,
        )?;
        module.set(
            "rotation",
            lua.create_function(|_, angle: RadiansF| Ok(Affine2F::rotation(angle)))?,
        )?;
        module.set(
            "scale",
            lua.create_function(|_, (x, y): (Either<Vec2F, f32>, Option<f32>)| {
                Ok(Affine2F::scale(match x {
                    Either::Left(s) => s,
                    Either::Right(x) => vec2(x, y.unwrap_or(x)),
                }))
            })?,
        )?;
        module.set(
            "translation",
            lua.create_function(|_, (x, y): (Either<Vec2F, f32>, Option<f32>)| {
                Ok(Affine2F::translation(match x {
                    Either::Left(t) => t,
                    Either::Right(x) => vec2(x, y.unwrap_or(x)),
                }))
            })?,
        )?;
        module.set(
            "trs",
            lua.create_function(|_, (t, r, s): (Vec2F, RadiansF, Vec2F)| {
                Ok(Affine2F::trs(t, r, s))
            })?,
        )?;

        Ok(Value::Table(module))
    }
}
