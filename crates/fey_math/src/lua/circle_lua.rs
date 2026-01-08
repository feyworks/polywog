use crate::{CircleF, Vec2F, add_shape_methods};
use fey_lua::{Handle, LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};

pub type CircleRef = UserDataRef<CircleF>;
pub type CircleMut = UserDataRefMut<CircleF>;

impl FromLua for CircleF {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Handle::<Self>::from_lua(value, lua).and_then(|h| h.get(lua))
    }
}

impl IntoLua for CircleF {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        Temp::<Self>::new(lua, self).map(Value::from)
    }
}

pub struct CircleModule;

impl LuaModule for CircleModule {
    const PATH: &'static str = "Circle";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<CircleF>::register(lua, "Circle", |members| {
            // fields
            members.getter("center", |obj| obj.center)?;
            members.setter("center", |obj, val: Vec2F| obj.center = val)?;
            members.getter("radius", |obj| obj.radius)?;
            members.setter("radius", |obj, val: f32| obj.radius = val)?;

            // methods
            members.method_ext("approx", |lua, a, b: Temp<CircleF>| {
                b.read(lua, |_, b| Ok(a.relative_eq(b)))
            })?;
            members.method("area", |obj, _: ()| obj.area())?;
            members.method("circumference", |obj, _: ()| obj.circumference())?;
            members.method_ext("contains_circ", |lua, a, b: Temp<CircleF>| {
                b.read(lua, |_, b| Ok(a.contains_circ(b)))
            })?;
            members.method("diameter", |obj, _: ()| obj.diameter())?;

            // impl Shape
            add_shape_methods(members)?;

            Ok(())
        })?;

        module.set(
            "new",
            lua.create_function(|_, (center, radius): (Either<Vec2F, f32>, Option<f32>)| {
                Ok(match center {
                    Either::Left(center) => CircleF::new(center, radius.unwrap()),
                    Either::Right(radius) => CircleF::with_radius(radius),
                })
            })?,
        )?;

        Ok(Value::Table(module))
    }
}
