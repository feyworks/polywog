use crate::{Mat3F, RadiansF, Vec2F, Vec3F};
use fey_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};

pub type Mat3Ref = UserDataRef<Mat3F>;
pub type Mat3Mut = UserDataRefMut<Mat3F>;

impl FromLua for Mat3F {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for Mat3F {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

pub struct Mat3Module;

impl LuaModule for Mat3Module {
    const PATH: &'static str = "Mat3";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Mat3F>::register(lua, "Mat3", |members| {
            // fields
            members.getter("x_axis", |obj| obj.x_axis)?;
            members.setter("x_axis", |obj, val: Vec3F| obj.x_axis = val)?;
            members.getter("y_axis", |obj| obj.y_axis)?;
            members.setter("y_axis", |obj, val: Vec3F| obj.y_axis = val)?;
            members.getter("z_axis", |obj| obj.z_axis)?;
            members.setter("z_axis", |obj, val: Vec3F| obj.z_axis = val)?;

            // operators
            members.op_mul_ext(|lua, a, b: Temp<Mat3F>| b.read(lua, |_, b| Ok(a.mul_mat3(b))))?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<Mat3F>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("determinant", |obj, _: ()| obj.determinant())?;
            members.method("inverse", |obj, _: ()| obj.inverse())?;
            members.method_ext("mul_mat3", |lua, a, b: Temp<Mat3F>| {
                b.read(lua, |_, b| Ok(a.mul_mat3(b)))
            })?;
            members.method("transform_vec2", |mat, vec: Vec2F| mat.transform_vec2(vec))?;
            members.method("transform_pos2", |mat, vec: Vec2F| mat.transform_pos2(vec))?;
            members.method("transform_vec3", |mat, vec: Vec3F| mat.transform_vec3(vec))?;
            members.method("transpose", |obj, _: ()| obj.transpose())?;

            Ok(())
        })?;

        module.set(
            "identity",
            lua.create_function(|_, _: ()| Ok(Mat3F::IDENTITY))?,
        )?;
        module.set("zero", lua.create_function(|_, _: ()| Ok(Mat3F::ZERO))?)?;
        module.set(
            "new",
            lua.create_function(|_, (x_axis, y_axis, z_axis): (Vec3F, Vec3F, Vec3F)| {
                Ok(Mat3F::new(x_axis, y_axis, z_axis))
            })?,
        )?;
        module.set(
            "axis_angle",
            lua.create_function(|_, (axis, angle): (Vec3F, RadiansF)| {
                Ok(Mat3F::axis_angle(axis, angle))
            })?,
        )?;
        module.set(
            "rotation_x",
            lua.create_function(|_, angle: RadiansF| Ok(Mat3F::rotation_x(angle)))?,
        )?;
        module.set(
            "rotation_y",
            lua.create_function(|_, angle: RadiansF| Ok(Mat3F::rotation_y(angle)))?,
        )?;
        module.set(
            "rotation_z",
            lua.create_function(|_, angle: RadiansF| Ok(Mat3F::rotation_z(angle)))?,
        )?;
        module.set(
            "scale",
            lua.create_function(|_, scale: Either<Vec3F, f32>| {
                Ok(Mat3F::scale(match scale {
                    Either::Left(s) => s,
                    Either::Right(s) => Vec3F::splat(s),
                }))
            })?,
        )?;
        module.set(
            "translation",
            lua.create_function(|_, translation: Vec2F| Ok(Mat3F::translation(translation)))?,
        )?;

        Ok(Value::Table(module))
    }
}
