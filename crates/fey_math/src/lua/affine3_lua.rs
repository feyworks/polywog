use crate::lua::impl_temp;
use crate::{Affine3F, Mat3F, RadiansF, Vec2F, Vec3F};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, Lua, Value};

impl_temp!(Affine3F Affine3Ref Affine3Mut);

pub struct Affine3Module;

impl LuaModule for Affine3Module {
    const PATH: &'static str = "Affine3";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Affine3F>::register(lua, "Affine3", |members| {
            // fields
            members.getter("matrix", |obj| obj.matrix)?;
            members.setter("matrix", |obj, val: Mat3F| obj.matrix = val)?;
            members.getter("translation", |obj| obj.translation)?;
            members.setter("translation", |obj, val: Vec3F| obj.translation = val)?;

            // operators
            members
                .op_mul_ext(|lua, a, b: Temp<Affine3F>| b.read(lua, |_, b| Ok(a.mul_affine3(b))))?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<Affine3F>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("inverse", |obj, _: ()| obj.inverse())?;
            members.method_ext("mul_affine3", |lua, a, b: Temp<Affine3F>| {
                b.read(lua, |_, b| Ok(a.mul_affine3(b)))
            })?;
            members.method("transform_vec2", |mat, vec: Vec2F| mat.transform_vec2(vec))?;
            members.method("transform_pos2", |mat, vec: Vec2F| mat.transform_pos2(vec))?;
            members.method("transform_vec3", |mat, vec: Vec3F| mat.transform_vec3(vec))?;
            members.method("transform_pos3", |mat, vec: Vec3F| mat.transform_pos3(vec))?;

            Ok(())
        })?;

        module.set(
            "identity",
            lua.create_function(|_, _: ()| Ok(Affine3F::IDENTITY))?,
        )?;
        module.set("zero", lua.create_function(|_, _: ()| Ok(Affine3F::ZERO))?)?;
        module.set(
            "new",
            lua.create_function(|_, (matrix, translation): (Mat3F, Vec3F)| {
                Ok(Affine3F::new(matrix, translation))
            })?,
        )?;
        module.set(
            "rotation_x",
            lua.create_function(|_, angle: RadiansF| Ok(Affine3F::rotation_x(angle)))?,
        )?;
        module.set(
            "rotation_y",
            lua.create_function(|_, angle: RadiansF| Ok(Affine3F::rotation_y(angle)))?,
        )?;
        module.set(
            "rotation_z",
            lua.create_function(|_, angle: RadiansF| Ok(Affine3F::rotation_z(angle)))?,
        )?;
        module.set(
            "scale",
            lua.create_function(|_, scale: Either<Vec3F, f32>| {
                Ok(Affine3F::scale(match scale {
                    Either::Left(s) => s,
                    Either::Right(s) => Vec3F::splat(s),
                }))
            })?,
        )?;
        module.set(
            "translation",
            lua.create_function(|_, translation: Vec3F| Ok(Affine3F::translation(translation)))?,
        )?;

        Ok(Value::Table(module))
    }
}
