mod affine2_lua;
mod affine3_lua;
mod cardinal_lua;
mod circle_lua;
mod line_lua;
mod mat2_lua;
mod mat3_lua;
mod mat4_lua;
mod num_lua;
mod octal_lua;
mod polygon_lua;
mod projection_lua;
mod quad_lua;
mod radians_lua;
mod ray_hit_lua;
mod ray_lua;
mod rect_lua;
mod shape_lua;
mod triangle_lua;
mod vec2_lua;
mod vec3_lua;
mod vec4_lua;

pub use affine2_lua::*;
pub use affine3_lua::*;
pub use cardinal_lua::*;
pub use circle_lua::*;
pub use line_lua::*;
pub use mat2_lua::*;
pub use mat3_lua::*;
pub use mat4_lua::*;
pub use num_lua::*;
pub use octal_lua::*;
pub use polygon_lua::*;
pub use projection_lua::*;
pub use quad_lua::*;
pub use ray_hit_lua::*;
pub use ray_lua::*;
pub use rect_lua::*;
pub use shape_lua::*;
pub use triangle_lua::*;
pub use vec2_lua::*;
pub use vec3_lua::*;
pub use vec4_lua::*;

macro_rules! impl_temp {
    ($t:ident $r:ident $m:ident) => {
        pub type $r = mlua::UserDataRef<$t>;
        pub type $m = mlua::UserDataRefMut<$t>;

        impl mlua::FromLua for $t {
            #[inline]
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                fey_lua::Handle::from_lua(value, lua).and_then(|h| h.get(lua))
            }
        }

        impl mlua::IntoLua for $t {
            #[inline]
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                fey_lua::Temp::<Self>::new(lua, self).map(mlua::Value::from)
            }
        }
    };
}

pub(crate) use impl_temp;

pub type MathModules = (
    Affine2Module,
    Affine3Module,
    CardinalModule,
    CircleModule,
    LineModule,
    Mat2Module,
    Mat3Module,
    Mat4Module,
    NumModule,
    OctalModule,
    PolygonModule,
    ProjectionModule,
    QuadModule,
    RayModule,
    RayHitModule,
    RectModule,
    TriangleModule,
    Vec2Module,
    Vec3Module,
    Vec4Module,
);
