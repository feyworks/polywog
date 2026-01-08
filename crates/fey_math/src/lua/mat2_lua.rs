use crate::{Mat2F, RadiansF, Vec2F};
use fey_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};

pub type Mat2Ref = UserDataRef<Mat2F>;
pub type Mat2Mut = UserDataRefMut<Mat2F>;

impl FromLua for Mat2F {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for Mat2F {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

pub struct Mat2Module;

impl LuaModule for Mat2Module {
    const PATH: &'static str = "Mat2";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Mat2F>::register(lua, "Mat2", |members| {
            // fields
            members.getter("x_axis", |obj| obj.x_axis)?;
            members.setter("x_axis", |obj, val: Vec2F| obj.x_axis = val)?;
            members.getter("y_axis", |obj| obj.y_axis)?;
            members.setter("y_axis", |obj, val: Vec2F| obj.y_axis = val)?;

            // operators
            members.op_mul_ext(|lua, a, b: Temp<Mat2F>| b.read(lua, |_, b| Ok(a.mul_mat2(b))))?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<Mat2F>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("determinant", |obj, _: ()| obj.determinant())?;
            members.method("inverse", |obj, _: ()| obj.inverse())?;
            members.method_ext("mul_mat2", |lua, a, b: Temp<Mat2F>| {
                b.read(lua, |_, b| Ok(a.mul_mat2(b)))
            })?;
            members.method("transform_vec2", |mat, vec: Vec2F| mat.transform_vec2(vec))?;
            members.method("transpose", |obj, _: ()| obj.transpose())?;

            Ok(())
        })?;

        module.set(
            "identity",
            lua.create_function(|_, _: ()| Ok(Mat2F::IDENTITY))?,
        )?;
        module.set("zero", lua.create_function(|_, _: ()| Ok(Mat2F::ZERO))?)?;
        module.set(
            "new",
            lua.create_function(|_, (x_axis, y_axis): (Vec2F, Vec2F)| {
                Ok(Mat2F::new(x_axis, y_axis))
            })?,
        )?;
        module.set(
            "rotation",
            lua.create_function(|_, angle: RadiansF| Ok(Mat2F::rotation(angle)))?,
        )?;
        module.set(
            "scale",
            lua.create_function(|_, scale: Either<Vec2F, f32>| {
                Ok(Mat2F::scale(match scale {
                    Either::Left(s) => s,
                    Either::Right(s) => Vec2F::splat(s),
                }))
            })?,
        )?;
        module.set(
            "scale_rotation",
            lua.create_function(|_, (scale, angle): (Either<Vec2F, f32>, RadiansF)| {
                Ok(Mat2F::scale_rotation(
                    match scale {
                        Either::Left(s) => s,
                        Either::Right(s) => Vec2F::splat(s),
                    },
                    angle,
                ))
            })?,
        )?;

        Ok(Value::Table(module))
    }
}
