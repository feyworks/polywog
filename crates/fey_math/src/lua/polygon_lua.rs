use crate::{
    Affine2Ref, CircleRef, PolygonF, QuadF, RayRef, RectF, RectRef, Shape, ShapeRef, TriangleF,
    Vec2F,
};
use fey_lua::{LuaModule, UserDataOf};
use mlua::prelude::LuaResult;
use mlua::{Function, Lua, Table, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value};
use std::ops::Deref;

pub type PolygonObj = UserDataOf<PolygonF>;
pub type PolygonRef = UserDataRef<PolygonF>;
pub type PolygonMut = UserDataRefMut<PolygonF>;

pub struct PolygonModule;

impl LuaModule for PolygonModule {
    const PATH: &'static str = "Polygon";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for PolygonModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, true);
    }
}

impl UserData for PolygonF {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, false);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M, module: bool) {
    if module {
        methods.add_function("new", |_, cap: Option<usize>| {
            Ok(match cap {
                Some(cap) => PolygonF::with_capacity(cap),
                None => PolygonF::new(),
            })
        });
        methods.add_function("from_arr", |_, points: Vec<Vec2F>| {
            Ok(PolygonF::from_vec(points))
        });
        methods.add_function("from_quad", |_, quad: QuadF| Ok(PolygonF::from_quad(quad)));
        methods.add_function("from_rect", |_, rect: RectF| Ok(PolygonF::from_rect(rect)));
        methods.add_function("from_tri", |_, tri: TriangleF| Ok(PolygonF::from_tri(tri)));
    }
    methods.add_function("clone", |_, this: PolygonRef| Ok(this.clone()));
    methods.add_function("approx", |_, (a, b): (PolygonRef, PolygonRef)| {
        Ok(a.points()
            .iter()
            .zip(b.points())
            .all(|(a, b)| a.abs_diff_eq(b)))
    });
    methods.add_function("capacity", |_, this: PolygonRef| Ok(this.capacity()));
    methods.add_function("clear", |_, mut this: PolygonMut| {
        this.clear();
        Ok(())
    });
    methods.add_function("edge", |_, (this, idx): (PolygonRef, usize)| {
        Ok(this.edge(idx))
    });
    methods.add_function("edges", |lua, (this, fill): (PolygonRef, Option<Table>)| {
        let fill = match fill {
            Some(fill) => {
                fill.clear()?;
                fill
            }
            None => lua.create_table()?,
        };
        for edge in this.edges() {
            fill.raw_push(edge)?;
        }
        Ok(fill)
    });
    methods.add_function(
        "insert",
        |_, (mut this, idx, p): (PolygonMut, usize, Vec2F)| {
            this.insert(idx, p);
            Ok(())
        },
    );
    methods.add_function("is_empty", |_, this: PolygonRef| Ok(this.is_empty()));
    methods.add_function("len", |_, this: PolygonRef| Ok(this.len()));
    methods.add_function(
        "points",
        |lua, (this, fill): (PolygonRef, Option<Table>)| {
            let fill = match fill {
                Some(fill) => {
                    fill.clear()?;
                    fill
                }
                None => lua.create_table()?,
            };
            for &p in this.points() {
                fill.raw_push(p)?;
            }
            Ok(fill)
        },
    );
    methods.add_function("pop", |_, mut this: PolygonMut| Ok(this.pop()));
    methods.add_function("push", |_, (mut this, p): (PolygonMut, Vec2F)| {
        this.push(p);
        Ok(())
    });
    methods.add_function("remove", |_, (mut this, i): (PolygonMut, usize)| {
        Ok(if i < this.len() {
            Some(this.remove(i))
        } else {
            None
        })
    });
    methods.add_function("reserve", |_, (mut this, cap): (PolygonMut, usize)| {
        this.reserve(cap);
        Ok(())
    });
    methods.add_function(
        "resize_with",
        |_, (mut this, len, fill): (PolygonMut, usize, Function)| {
            this.resize_with(len, || fill.call::<Vec2F>(()).unwrap());
            Ok(())
        },
    );
    methods.add_function(
        "transform_in_place",
        |_, (mut this, mat): (PolygonMut, Affine2Ref)| {
            let mat = mat.deref();
            this.transform_in_place_by(|p| mat.transform_pos2(p));
            Ok(())
        },
    );
    methods.add_function("translate", |_, (mut this, off): (PolygonMut, Vec2F)| {
        this.translate(off);
        Ok(())
    });
    methods.add_function("centroid", |_, this: PolygonRef| Ok(this.centroid()));
    methods.add_function("contains", |_, (this, p): (PolygonRef, Vec2F)| {
        Ok(this.contains(p))
    });
    methods.add_function("bounds", |_, this: PolygonRef| Ok(this.bounds()));
    methods.add_function(
        "project_onto_axis",
        |_, (this, axis): (PolygonRef, Vec2F)| Ok(this.project_onto_axis(axis)),
    );
    methods.add_function("project_point", |_, (this, p): (PolygonRef, Vec2F)| {
        Ok(this.project_point(p))
    });
    methods.add_function("rayhit", |_, (this, ray): (PolygonRef, RayRef)| {
        Ok(this.rayhit(&ray))
    });
    methods.add_function("raycast", |_, (this, ray): (PolygonRef, RayRef)| {
        Ok(this.raycast(&ray))
    });
    methods.add_function(
        "overlaps_circ",
        |_, (this, circ): (PolygonRef, CircleRef)| Ok(this.overlaps_circ(&circ)),
    );
    methods.add_function("overlaps_rect", |_, (this, rect): (PolygonRef, RectRef)| {
        Ok(this.overlaps_rect(&rect))
    });
    methods.add_function(
        "overlaps_shape",
        |lua, (this, shape): (PolygonRef, ShapeRef)| {
            Ok(match shape {
                ShapeRef::Circ(circ) => circ.field(lua, |circ| this.overlaps_circ(circ)),
                ShapeRef::Tri(tri) => tri.field(lua, |tri| this.overlaps_poly(tri)),
                ShapeRef::Rect(rect) => rect.field(lua, |rect| this.overlaps_rect(rect)),
                ShapeRef::Quad(quad) => quad.field(lua, |quad| this.overlaps_poly(quad)),
                ShapeRef::Poly(poly) => Ok(this.overlaps_poly(poly.deref())),
            })
        },
    );
    methods.add_function(
        "extract_from_circ",
        |_, (this, circ): (PolygonRef, CircleRef)| Ok(this.extract_from_circ(circ.deref())),
    );
    methods.add_function(
        "extract_from_rect",
        |_, (this, rect): (PolygonRef, RectRef)| Ok(this.extract_from_poly(rect.deref())),
    );
    methods.add_function(
        "extract_from_shape",
        |lua, (this, shape): (PolygonRef, ShapeRef)| {
            Ok(match shape {
                ShapeRef::Circ(circ) => circ.field(lua, |circ| this.extract_from_circ(circ)),
                ShapeRef::Tri(tri) => tri.field(lua, |tri| this.extract_from_poly(tri)),
                ShapeRef::Rect(rect) => rect.field(lua, |rect| this.extract_from_poly(rect)),
                ShapeRef::Quad(quad) => quad.field(lua, |quad| this.extract_from_poly(quad)),
                ShapeRef::Poly(poly) => Ok(this.extract_from_poly(poly.deref())),
            })
        },
    );
    methods.add_function("is_convex", |_, this: PolygonRef| Ok(this.is_convex()));
}
