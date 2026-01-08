use crate::{Mat4F, RadiansF, Vec2F, Vec3F, Vec4F};
use fey_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};

pub type Mat4Ref = UserDataRef<Mat4F>;
pub type Mat4Mut = UserDataRefMut<Mat4F>;

impl FromLua for Mat4F {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for Mat4F {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

pub struct Mat4Module;

impl LuaModule for Mat4Module {
    const PATH: &'static str = "Mat4";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Mat4F>::register(lua, "Mat4", |members| {
            // fields
            members.getter("x_axis", |obj| obj.x_axis)?;
            members.setter("x_axis", |obj, val: Vec4F| obj.x_axis = val)?;
            members.getter("y_axis", |obj| obj.y_axis)?;
            members.setter("y_axis", |obj, val: Vec4F| obj.y_axis = val)?;
            members.getter("z_axis", |obj| obj.z_axis)?;
            members.setter("z_axis", |obj, val: Vec4F| obj.z_axis = val)?;
            members.getter("w_axis", |obj| obj.w_axis)?;
            members.setter("w_axis", |obj, val: Vec4F| obj.w_axis = val)?;

            // operators
            members.op_mul_ext(|lua, a, b: Temp<Mat4F>| b.read(lua, |_, b| Ok(a.mul_mat4(b))))?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<Mat4F>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("determinant", |obj, _: ()| obj.determinant())?;
            members.method("inverse", |obj, _: ()| obj.inverse())?;
            members.method_ext("mul_mat4", |lua, a, b: Temp<Mat4F>| {
                b.read(lua, |_, b| Ok(a.mul_mat4(b)))
            })?;
            members.method("transform_vec2", |mat, vec: Vec2F| mat.transform_vec2(vec))?;
            members.method("transform_pos2", |mat, vec: Vec2F| mat.transform_pos2(vec))?;
            members.method("transform_vec3", |mat, vec: Vec3F| mat.transform_vec3(vec))?;
            members.method("transform_pos3", |mat, vec: Vec3F| mat.transform_pos3(vec))?;
            members.method("transform_vec4", |mat, vec: Vec4F| mat.transform_vec4(vec))?;
            members.method("transpose", |obj, _: ()| obj.transpose())?;

            Ok(())
        })?;

        module.set(
            "identity",
            lua.create_function(|_, _: ()| Ok(Mat4F::IDENTITY))?,
        )?;
        module.set("zero", lua.create_function(|_, _: ()| Ok(Mat4F::ZERO))?)?;
        module.set(
            "new",
            lua.create_function(
                |_, (x_axis, y_axis, z_axis, w_axis): (Vec4F, Vec4F, Vec4F, Vec4F)| {
                    Ok(Mat4F::new(x_axis, y_axis, z_axis, w_axis))
                },
            )?,
        )?;
        module.set(
            "axis_angle",
            lua.create_function(|_, (axis, angle): (Vec3F, RadiansF)| {
                Ok(Mat4F::axis_angle(axis, angle))
            })?,
        )?;
        module.set(
            "rotation_x",
            lua.create_function(|_, angle: RadiansF| Ok(Mat4F::rotation_x(angle)))?,
        )?;
        module.set(
            "rotation_y",
            lua.create_function(|_, angle: RadiansF| Ok(Mat4F::rotation_y(angle)))?,
        )?;
        module.set(
            "rotation_z",
            lua.create_function(|_, angle: RadiansF| Ok(Mat4F::rotation_z(angle)))?,
        )?;
        module.set(
            "scale",
            lua.create_function(|_, scale: Either<Vec3F, f32>| {
                Ok(Mat4F::scale(match scale {
                    Either::Left(s) => s,
                    Either::Right(s) => Vec3F::splat(s),
                }))
            })?,
        )?;
        module.set(
            "translation",
            lua.create_function(|_, translation: Either<Vec3F, Vec3F>| {
                Ok(Mat4F::translation(match translation {
                    Either::Left(t) => t,
                    Either::Right(t) => t.with_z(0.0),
                }))
            })?,
        )?;
        module.set(
            "ortho",
            lua.create_function(
                |_, (left, right, bottom, top, z_near, z_far): (f32, f32, f32, f32, f32, f32)| {
                    Ok(Mat4F::ortho(left, right, bottom, top, z_near, z_far))
                },
            )?,
        )?;
        module.set(
            "ortho_size",
            lua.create_function(|_, (w, h): (f32, f32)| Ok(Mat4F::ortho_size((w, h))))?,
        )?;

        Ok(Value::Table(module))
    }
}
