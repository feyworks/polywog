use crate::ops::Ops;
use crate::{Temp, TempInfo};
use fnv::FnvHashMap;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, LightUserData, Lua, MultiValue, Value};
use std::any::{Any, TypeId};
use std::marker::PhantomData;

pub struct TempTypes {
    pub(crate) version: u16,
    pub(crate) types: Vec<TempTypeInfo>,
    pub(crate) type_indices: FnvHashMap<TypeId, usize>,
}

impl TempTypes {
    pub fn init(lua: &Lua) -> LuaResult<()> {
        lua.set_app_data(Self {
            version: 1,
            types: Vec::new(),
            type_indices: FnvHashMap::default(),
        });
        lua.set_type_metatable::<LightUserData>(Some({
            let meta = lua.create_table()?;
            macro_rules! op {
                ($name:literal $fn:ident) => {
                    meta.set(
                        $name,
                        lua.create_function(|lua, args: MultiValue| {
                            let type_idx =
                                TempInfo::from_lua(args.front().cloned().unwrap(), lua)?.type_idx;
                            let types = lua.app_data_ref::<Self>().unwrap();
                            match types.types.get(type_idx as usize) {
                                Some(ty) => {
                                    if let Some(op) = ty.ops.$fn.clone() {
                                        drop(types);
                                        op.call::<Value>(args)
                                    } else {
                                        Err(LuaError::runtime(format!(
                                            "operator [{}] not found on type [{}]",
                                            $name, ty.type_name
                                        )))
                                    }
                                }
                                None => Err(LuaError::runtime(
                                    "invalid temporary type index extracted from light userdata",
                                )),
                            }
                        })?,
                    )?;
                };
            }
            op!("__index" index);
            op!("__newindex" newindex);
            op!("__call" call);
            op!("__tostring" tostring);
            op!("__add" add);
            op!("__sub" sub);
            op!("__mul" mul);
            op!("__div" div);
            op!("__unm" unm);
            op!("__mod" r#mod);
            op!("__pow" pow);
            op!("__idiv" idiv);
            op!("__band" band);
            op!("__bor" bor);
            op!("__bxor" bxor);
            op!("__bnot" bnot);
            op!("__shl" shl);
            op!("__shr" shr);
            op!("__eq" eq);
            op!("__lt" lt);
            op!("__le" le);
            op!("__concat" concat);
            op!("__len" len);
            meta
        }));
        Ok(())
    }

    pub fn try_get_temp<T: 'static>(&self, data: LightUserData) -> Option<Temp<T>> {
        let info = TempInfo::from(data);
        let ty_idx = info.type_idx as usize;
        if *self.type_indices.get(&TypeId::of::<T>())? != ty_idx {
            return None;
        }
        Some(Temp {
            info,
            marker: PhantomData,
        })
    }

    pub fn clear_frame(&mut self) {
        self.version = self.version.wrapping_add(1);
        for ty in self.types.iter_mut() {
            (ty.clear_fn)(&mut ty.values);
        }
    }
}

pub(crate) struct TempTypeInfo {
    pub values: Box<dyn Any>,
    pub type_name: &'static str,
    pub ops: Ops,
    pub clear_fn: fn(&mut Box<dyn Any>),
}
