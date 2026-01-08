use crate::{
    Circle, CircleF, CircleOut, DynShapeF, PolygonF, Polygonal, Projection, QuadF, RadiansF, Ray,
    RayF, RayHit, Rect, RectF, RectOut, Shape, TriangleF, Vec2, Vec2F,
};
use fey_lua::{HandleMut, HandleRef, TempMembers, TempTypes, UserDataOf};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, FromLua, IntoLua, Lua, UserDataRef, UserDataRefMut, Value};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum ShapeData {
    Circ(UserDataOf<CircleF>),
    Tri(UserDataOf<TriangleF>),
    Rect(UserDataOf<RectF>),
    Quad(UserDataOf<QuadF>),
    Poly(UserDataOf<PolygonF>),
}

pub enum ShapeRef {
    Circ(HandleRef<CircleF>),
    Tri(HandleRef<TriangleF>),
    Rect(HandleRef<RectF>),
    Quad(HandleRef<QuadF>),
    Poly(UserDataRef<PolygonF>),
}

pub enum ShapeMut {
    Circ(HandleMut<CircleF>),
    Tri(HandleMut<TriangleF>),
    Rect(HandleMut<RectF>),
    Quad(HandleMut<QuadF>),
    Poly(UserDataRefMut<PolygonF>),
}

impl ShapeData {
    #[inline]
    pub fn new_circ(lua: &Lua, circ: CircleF) -> LuaResult<Self> {
        lua.create_any_userdata(circ)
            .map(UserDataOf::from_any)
            .map(Self::Circ)
    }

    #[inline]
    pub fn new_tri(lua: &Lua, tri: TriangleF) -> LuaResult<Self> {
        lua.create_any_userdata(tri)
            .map(UserDataOf::from_any)
            .map(Self::Tri)
    }

    #[inline]
    pub fn new_rect(lua: &Lua, rect: RectF) -> LuaResult<Self> {
        lua.create_any_userdata(rect)
            .map(UserDataOf::from_any)
            .map(Self::Rect)
    }

    #[inline]
    pub fn new_quad(lua: &Lua, quad: QuadF) -> LuaResult<Self> {
        lua.create_any_userdata(quad)
            .map(UserDataOf::from_any)
            .map(Self::Quad)
    }

    #[inline]
    pub fn new_poly(lua: &Lua, poly: PolygonF) -> LuaResult<Self> {
        lua.create_userdata(poly)
            .map(UserDataOf::from_any)
            .map(Self::Quad)
    }

    #[inline]
    pub fn clone_new(&self, lua: &Lua) -> LuaResult<Self> {
        match self {
            Self::Circ(data) => Self::new_circ(lua, data.get().clone()),
            Self::Tri(data) => Self::new_tri(lua, data.get().clone()),
            Self::Rect(data) => Self::new_rect(lua, data.get().clone()),
            Self::Quad(data) => Self::new_quad(lua, data.get().clone()),
            Self::Poly(data) => Self::new_poly(lua, data.get().clone()),
        }
    }

    #[inline]
    pub fn ptr(&self) -> *const c_void {
        match self {
            Self::Circ(data) => data.ptr(),
            Self::Tri(data) => data.ptr(),
            Self::Rect(data) => data.ptr(),
            Self::Quad(data) => data.ptr(),
            Self::Poly(data) => data.ptr(),
        }
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Circ(a), Self::Circ(b)) => a.ptr_eq(b),
            (Self::Tri(a), Self::Tri(b)) => a.ptr_eq(b),
            (Self::Rect(a), Self::Rect(b)) => a.ptr_eq(b),
            (Self::Quad(a), Self::Quad(b)) => a.ptr_eq(b),
            (Self::Poly(a), Self::Poly(b)) => a.ptr_eq(b),
            _ => false,
        }
    }

    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        match other {
            Self::Circ(sh) => self.overlaps_circ(sh.get().deref()),
            Self::Tri(sh) => self.overlaps_poly(sh.get().deref()),
            Self::Rect(sh) => self.overlaps_rect(sh.get().deref()),
            Self::Quad(sh) => self.overlaps_poly(sh.get().deref()),
            Self::Poly(sh) => self.overlaps_poly(sh.get().deref()),
        }
    }

    #[inline]
    pub fn extract_from(&self, other: &Self) -> Option<Vec2F> {
        match other {
            Self::Circ(sh) => self.extract_from_circ(sh.get().deref()),
            Self::Tri(sh) => self.extract_from_poly(sh.get().deref()),
            Self::Rect(sh) => self.extract_from_poly(sh.get().deref()),
            Self::Quad(sh) => self.extract_from_poly(sh.get().deref()),
            Self::Poly(sh) => self.extract_from_poly(sh.get().deref()),
        }
    }

    pub fn transform_into(
        &self,
        into: &mut DynShapeF,
        circle_out: CircleOut<f32>,
        rect_out: RectOut,
        f: impl FnMut(Vec2F) -> Vec2F,
    ) {
        match &self {
            Self::Circ(sh) => match circle_out {
                CircleOut::Circle => {
                    *into = DynShapeF::Circle(sh.get().transform_by_retain(f));
                }
                CircleOut::SegCount { count, angle } => {
                    if let DynShapeF::Polygon(into) = into {
                        sh.get().transform_by_into_n(count, angle, into, f);
                    } else {
                        *into = DynShapeF::Polygon(sh.get().transform_by_n(count, angle, f));
                    }
                }
                CircleOut::SegLen { len, angle } => {
                    if let DynShapeF::Polygon(into) = into {
                        sh.get().transform_by_into(len, angle, into, f);
                    } else {
                        *into = DynShapeF::Polygon(sh.get().transform_by(len, angle, f));
                    }
                }
            },
            Self::Tri(sh) => {
                if let DynShapeF::Triangle(into) = into {
                    *into = sh.get().transform_by(f);
                } else {
                    *into = DynShapeF::Triangle(sh.get().transform_by(f));
                }
            }
            Self::Rect(sh) => match rect_out {
                RectOut::Rect => {
                    if let DynShapeF::Rect(into) = into {
                        *into = sh.get().transform_by_retain(f);
                    } else {
                        *into = DynShapeF::Rect(sh.get().transform_by_retain(f));
                    }
                }
                RectOut::Quad => {
                    if let DynShapeF::Quad(into) = into {
                        *into = sh.get().transform_by(f);
                    } else {
                        *into = DynShapeF::Quad(sh.get().transform_by(f));
                    }
                }
            },
            Self::Quad(sh) => {
                if let DynShapeF::Quad(into) = into {
                    *into = sh.get().transform_by(f);
                } else {
                    *into = DynShapeF::Quad(sh.get().transform_by(f));
                }
            }
            Self::Poly(sh) => {
                if let DynShapeF::Polygon(into) = into {
                    sh.get().transform_by_into(into, f);
                } else {
                    *into = DynShapeF::Polygon(sh.get().transform_by(f));
                }
            }
        }
    }

    pub fn translate_into(&self, into: &mut DynShapeF, amount: Vec2F) {
        match &self {
            Self::Circ(sh) => {
                if let DynShapeF::Circle(into) = into {
                    *into = *sh.get() + amount;
                } else {
                    *into = DynShapeF::Circle(*sh.get() + amount);
                }
            }
            Self::Tri(sh) => {
                if let DynShapeF::Triangle(into) = into {
                    *into = sh.get().transform_by(|p| p + amount);
                } else {
                    *into = DynShapeF::Triangle(sh.get().transform_by(|p| p + amount));
                }
            }
            Self::Rect(sh) => {
                if let DynShapeF::Rect(into) = into {
                    *into = *sh.get() + amount;
                } else {
                    *into = DynShapeF::Rect(*sh.get() + amount);
                }
            }
            Self::Quad(sh) => {
                if let DynShapeF::Quad(into) = into {
                    *into = sh.get().transform_by(|p| p + amount);
                } else {
                    *into = DynShapeF::Quad(sh.get().transform_by(|p| p + amount));
                }
            }
            Self::Poly(sh) => {
                if let DynShapeF::Polygon(into) = into {
                    sh.get().transform_by_into(into, |p| p + amount);
                } else {
                    *into = DynShapeF::Polygon(sh.get().transform_by(|p| p + amount));
                }
            }
        }
    }
}

impl FromLua for ShapeData {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Ok(match value {
            Value::UserData(data) => {
                if data.is::<CircleF>() {
                    Self::Circ(UserDataOf::from_any(data))
                } else if data.is::<TriangleF>() {
                    Self::Tri(UserDataOf::from_any(data))
                } else if data.is::<RectF>() {
                    Self::Rect(UserDataOf::from_any(data))
                } else if data.is::<QuadF>() {
                    Self::Quad(UserDataOf::from_any(data))
                } else if data.is::<PolygonF>() {
                    Self::Poly(UserDataOf::from_any(data))
                } else {
                    return Err(LuaError::runtime("userdata is not a shape type"));
                }
            }
            Value::LightUserData(data) => {
                let types = lua.app_data_ref::<TempTypes>().unwrap();
                if let Some(temp) = types.try_get_temp::<CircleF>(data) {
                    temp.into_userdata(lua).map(Self::Circ)?
                } else if let Some(temp) = types.try_get_temp::<TriangleF>(data) {
                    temp.into_userdata(lua).map(Self::Tri)?
                } else if let Some(temp) = types.try_get_temp::<RectF>(data) {
                    temp.into_userdata(lua).map(Self::Rect)?
                } else if let Some(temp) = types.try_get_temp::<QuadF>(data) {
                    temp.into_userdata(lua).map(Self::Quad)?
                } else {
                    return Err(LuaError::runtime("light userdata is not a shape type"));
                }
            }
            _ => return Err(LuaError::runtime("value is not a userdata or temp type")),
        })
    }
}

impl IntoLua for ShapeData {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::UserData(match self {
            Self::Circ(data) => data.into(),
            Self::Tri(data) => data.into(),
            Self::Rect(data) => data.into(),
            Self::Quad(data) => data.into(),
            Self::Poly(data) => data.into(),
        }))
    }
}

impl FromLua for ShapeRef {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::UserData(data) => {
                if data.is::<CircleF>() {
                    data.borrow::<CircleF>()
                        .map(HandleRef::User)
                        .map(Self::Circ)
                } else if data.is::<TriangleF>() {
                    data.borrow::<TriangleF>()
                        .map(HandleRef::User)
                        .map(Self::Tri)
                } else if data.is::<RectF>() {
                    data.borrow::<RectF>().map(HandleRef::User).map(Self::Rect)
                } else if data.is::<QuadF>() {
                    data.borrow::<QuadF>().map(HandleRef::User).map(Self::Quad)
                } else if data.is::<PolygonF>() {
                    data.borrow::<PolygonF>().map(Self::Poly)
                } else {
                    Err(LuaError::runtime("userdata is not a shape type"))
                }
            }
            Value::LightUserData(data) => {
                let types = lua.app_data_ref::<TempTypes>().unwrap();
                Ok(if let Some(temp) = types.try_get_temp::<CircleF>(data) {
                    Self::Circ(HandleRef::Temp(temp))
                } else if let Some(temp) = types.try_get_temp::<TriangleF>(data) {
                    Self::Tri(HandleRef::Temp(temp))
                } else if let Some(temp) = types.try_get_temp::<RectF>(data) {
                    Self::Rect(HandleRef::Temp(temp))
                } else if let Some(temp) = types.try_get_temp::<QuadF>(data) {
                    Self::Quad(HandleRef::Temp(temp))
                } else {
                    return Err(LuaError::runtime("light userdata is not a shape type"));
                })
            }
            _ => Err(LuaError::runtime("value is not a userdata or temp type")),
        }
    }
}

impl FromLua for ShapeMut {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::UserData(data) => {
                if data.is::<CircleF>() {
                    data.borrow_mut::<CircleF>()
                        .map(HandleMut::User)
                        .map(Self::Circ)
                } else if data.is::<TriangleF>() {
                    data.borrow_mut::<TriangleF>()
                        .map(HandleMut::User)
                        .map(Self::Tri)
                } else if data.is::<RectF>() {
                    data.borrow_mut::<RectF>()
                        .map(HandleMut::User)
                        .map(Self::Rect)
                } else if data.is::<QuadF>() {
                    data.borrow_mut::<QuadF>()
                        .map(HandleMut::User)
                        .map(Self::Quad)
                } else if data.is::<PolygonF>() {
                    data.borrow_mut::<PolygonF>().map(Self::Poly)
                } else {
                    Err(LuaError::runtime("userdata is not a shape type"))
                }
            }
            Value::LightUserData(data) => {
                let types = lua.app_data_ref::<TempTypes>().unwrap();
                Ok(if let Some(temp) = types.try_get_temp::<CircleF>(data) {
                    Self::Circ(HandleMut::Temp(temp))
                } else if let Some(temp) = types.try_get_temp::<TriangleF>(data) {
                    Self::Tri(HandleMut::Temp(temp))
                } else if let Some(temp) = types.try_get_temp::<RectF>(data) {
                    Self::Rect(HandleMut::Temp(temp))
                } else if let Some(temp) = types.try_get_temp::<QuadF>(data) {
                    Self::Quad(HandleMut::Temp(temp))
                } else {
                    return Err(LuaError::runtime("light userdata is not a shape type"));
                })
            }
            _ => Err(LuaError::runtime("value is not a userdata or temp type")),
        }
    }
}

impl FromLua for CircleOut<f32> {
    #[inline]
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        Ok(match value {
            Value::String(s) if s.to_str()?.as_ref() == "circ" => Self::Circle,
            Value::Table(t) => {
                let angle = t.get::<RadiansF>("angle")?;
                if let Some(count) = t.get::<Option<f32>>("count")? {
                    Self::SegCount { count, angle }
                } else {
                    let len = t.get::<f32>("len")?;
                    Self::SegLen { len, angle }
                }
            }
            _ => {
                return Err(LuaError::runtime("value is not a CircleOut"));
            }
        })
    }
}

impl FromLua for RectOut {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        match s.as_ref() {
            "rect" => Ok(Self::Rect),
            "quad" => Ok(Self::Quad),
            _ => Err(LuaError::runtime("expected \"rect\" or \"quad\"")),
        }
    }
}

pub fn add_shape_methods<T>(members: &mut TempMembers<T>) -> LuaResult<()>
where
    T: Shape<f32> + Clone + 'static,
{
    members.method("centroid", |this, _: ()| this.centroid())?;
    members.method("contains", |this, p: Vec2F| this.contains(p))?;
    members.method("bounds", |this, _: ()| this.bounds())?;
    members.method("project_onto_axis", |this, axis: Vec2F| {
        this.project_onto_axis(axis)
    })?;
    members.method("project_point", |this, p: Vec2F| this.project_point(p))?;
    members.method("rayhit", |this, ray: RayF| this.rayhit(&ray))?;
    members.method("raycast", |this, ray: RayF| this.raycast(&ray))?;
    members.method("overlaps_circ", |this, circ: CircleF| {
        this.overlaps_circ(&circ)
    })?;
    members.method("overlaps_rect", |this, rect: RectF| {
        this.overlaps_rect(&rect)
    })?;
    members.method_ext("overlaps_shape", |lua, this, shape: ShapeRef| match shape {
        ShapeRef::Circ(circ) => circ.field(lua, |circ| this.overlaps_circ(circ)),
        ShapeRef::Tri(tri) => tri.field(lua, |tri| this.overlaps_poly(tri)),
        ShapeRef::Rect(rect) => rect.field(lua, |rect| this.overlaps_rect(rect)),
        ShapeRef::Quad(quad) => quad.field(lua, |quad| this.overlaps_poly(quad)),
        ShapeRef::Poly(poly) => Ok(this.overlaps_poly(poly.deref())),
    })?;
    members.method("extract_from_circ", |this, circ: CircleF| {
        this.extract_from_circ(&circ)
    })?;
    members.method("extract_from_rect", |this, rect: RectF| {
        this.extract_from_poly(&rect)
    })?;
    members.method_ext(
        "extract_from_shape",
        |lua, this, shape: ShapeRef| match shape {
            ShapeRef::Circ(circ) => circ.field(lua, |circ| this.extract_from_circ(circ)),
            ShapeRef::Tri(tri) => tri.field(lua, |tri| this.extract_from_poly(tri)),
            ShapeRef::Rect(rect) => rect.field(lua, |rect| this.extract_from_poly(rect)),
            ShapeRef::Quad(quad) => quad.field(lua, |quad| this.extract_from_poly(quad)),
            ShapeRef::Poly(poly) => Ok(this.extract_from_poly(poly.deref())),
        },
    )?;
    members.method("is_convex", |this, _: ()| this.is_convex())?;
    Ok(())
}

// impl ShapeRef {
//     pub fn box_clone(&self) -> ShapeData {
//         match self {
//             Self::Circ(circ) => circ.
//         }
//     }
// }

impl Shape<f32> for ShapeData {
    #[inline]
    fn centroid(&self) -> Vec2<f32> {
        match self {
            Self::Circ(this) => this.get().centroid(),
            Self::Tri(this) => this.get().centroid(),
            Self::Rect(this) => this.get().centroid(),
            Self::Quad(this) => this.get().centroid(),
            Self::Poly(this) => this.get().centroid(),
        }
    }

    #[inline]
    fn contains(&self, p: Vec2<f32>) -> bool {
        match self {
            Self::Circ(this) => this.get().contains(p),
            Self::Tri(this) => this.get().contains(p),
            Self::Rect(this) => this.get().contains(p),
            Self::Quad(this) => this.get().contains(p),
            Self::Poly(this) => this.get().contains(p),
        }
    }

    #[inline]
    fn bounds(&self) -> Rect<f32> {
        match self {
            Self::Circ(this) => this.get().bounds(),
            Self::Tri(this) => this.get().bounds(),
            Self::Rect(this) => this.get().bounds(),
            Self::Quad(this) => this.get().bounds(),
            Self::Poly(this) => this.get().bounds(),
        }
    }

    #[inline]
    fn project_onto_axis(&self, axis: Vec2<f32>) -> Projection<f32> {
        match self {
            Self::Circ(this) => this.get().project_onto_axis(axis),
            Self::Tri(this) => this.get().project_onto_axis(axis),
            Self::Rect(this) => this.get().project_onto_axis(axis),
            Self::Quad(this) => this.get().project_onto_axis(axis),
            Self::Poly(this) => this.get().project_onto_axis(axis),
        }
    }

    #[inline]
    fn project_point(&self, p: Vec2<f32>) -> Vec2<f32> {
        match self {
            Self::Circ(this) => this.get().project_point(p),
            Self::Tri(this) => this.get().project_point(p),
            Self::Rect(this) => this.get().project_point(p),
            Self::Quad(this) => this.get().project_point(p),
            Self::Poly(this) => this.get().project_point(p),
        }
    }

    #[inline]
    fn rayhit(&self, ray: &Ray<f32>) -> bool {
        match self {
            Self::Circ(this) => this.get().rayhit(ray),
            Self::Tri(this) => this.get().rayhit(ray),
            Self::Rect(this) => this.get().rayhit(ray),
            Self::Quad(this) => this.get().rayhit(ray),
            Self::Poly(this) => this.get().rayhit(ray),
        }
    }

    #[inline]
    fn raycast(&self, ray: &Ray<f32>) -> Option<RayHit<f32>> {
        match self {
            Self::Circ(this) => this.get().raycast(ray),
            Self::Tri(this) => this.get().raycast(ray),
            Self::Rect(this) => this.get().raycast(ray),
            Self::Quad(this) => this.get().raycast(ray),
            Self::Poly(this) => this.get().raycast(ray),
        }
    }

    #[inline]
    fn overlaps_rect(&self, rect: &Rect<f32>) -> bool {
        match self {
            Self::Circ(this) => this.get().overlaps_rect(rect),
            Self::Tri(this) => this.get().overlaps_rect(rect),
            Self::Rect(this) => this.get().overlaps_rect(rect),
            Self::Quad(this) => this.get().overlaps_rect(rect),
            Self::Poly(this) => this.get().overlaps_rect(rect),
        }
    }

    #[inline]
    fn overlaps_circ(&self, circ: &Circle<f32>) -> bool {
        match self {
            Self::Circ(this) => this.get().overlaps_circ(circ),
            Self::Tri(this) => this.get().overlaps_circ(circ),
            Self::Rect(this) => this.get().overlaps_circ(circ),
            Self::Quad(this) => this.get().overlaps_circ(circ),
            Self::Poly(this) => this.get().overlaps_circ(circ),
        }
    }

    #[inline]
    fn overlaps_poly<P: Polygonal<f32>>(&self, poly: &P) -> bool {
        match self {
            Self::Circ(this) => this.get().overlaps_poly(poly),
            Self::Tri(this) => this.get().overlaps_poly(poly),
            Self::Rect(this) => this.get().overlaps_poly(poly),
            Self::Quad(this) => this.get().overlaps_poly(poly),
            Self::Poly(this) => this.get().overlaps_poly(poly),
        }
    }

    #[inline]
    fn extract_from_circ(&self, circ: &Circle<f32>) -> Option<Vec2<f32>> {
        match self {
            Self::Circ(this) => this.get().extract_from_circ(circ),
            Self::Tri(this) => this.get().extract_from_circ(circ),
            Self::Rect(this) => this.get().extract_from_circ(circ),
            Self::Quad(this) => this.get().extract_from_circ(circ),
            Self::Poly(this) => this.get().extract_from_circ(circ),
        }
    }

    #[inline]
    fn extract_from_poly<P: Polygonal<f32>>(&self, poly: &P) -> Option<Vec2<f32>> {
        match self {
            Self::Circ(this) => this.get().extract_from_poly(poly),
            Self::Tri(this) => this.get().extract_from_poly(poly),
            Self::Rect(this) => this.get().extract_from_poly(poly),
            Self::Quad(this) => this.get().extract_from_poly(poly),
            Self::Poly(this) => this.get().extract_from_poly(poly),
        }
    }

    #[inline]
    fn is_convex(&self) -> bool {
        match self {
            Self::Circ(this) => this.get().is_convex(),
            Self::Tri(this) => this.get().is_convex(),
            Self::Rect(this) => this.get().is_convex(),
            Self::Quad(this) => this.get().is_convex(),
            Self::Poly(this) => this.get().is_convex(),
        }
    }
}
