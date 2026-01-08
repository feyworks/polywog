use crate::{Handle, TempMembers, TempTypeInfo, TempTypes, UserDataOf};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{
    BorrowedStr, FromLua, IntoLua, LightUserData, Lua, MultiValue, Table, UserDataMethods, Value,
};
use std::any::{TypeId, type_name};
use std::ffi::c_void;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct Temp<T: 'static> {
    pub(crate) info: TempInfo,
    pub(crate) marker: PhantomData<T>,
}

impl<T: Clone + 'static> Temp<T> {
    /// Registers a handle type and assign fields and methods to it. This returns a table
    /// with all the methods in it.
    pub fn register<F>(lua: &Lua, type_name: &'static str, f: F) -> LuaResult<Table>
    where
        F: FnOnce(&mut TempMembers<T>) -> LuaResult<()>,
    {
        let mut members = TempMembers::new(lua)?;

        // creates a clone of the value
        members.method_ext("clone", |lua, this: &T, _: ()| Temp::new(lua, this.clone()))?;

        // creates a clone of the value that persists beyond the frame as a userdata
        // if the value is already boxed, this just returns the same value back
        members.handle_method_ext("box", |lua, this, _: ()| match this {
            Handle::User(data) => Ok(data.data.clone()),
            Handle::Temp(temp) => temp.read(lua, |lua, temp| lua.create_any_userdata(temp.clone())),
        })?;

        // creates a boxed clone of this value whether it is boxed or not.
        members.method_ext("box_clone", |lua, this, _: ()| {
            lua.create_any_userdata(this.clone())
        })?;

        f(&mut members)?;
        let TempMembers {
            methods,
            index,
            newindex,
            mut ops,
            ..
        } = members;

        let index = lua.create_function(move |lua, mut args: MultiValue| {
            let key = BorrowedStr::from_lua(args.pop_back().unwrap(), lua)?;
            match index.get(key.as_ref()) {
                Some(val) => {
                    if val.getter {
                        val.func.call::<Value>(args)
                    } else {
                        Ok(Value::Function(val.func.clone()))
                    }
                }
                None => Err(LuaError::runtime(format!(
                    "tried to get nonexistent field [{key}] on type [{type_name}]",
                ))),
            }
        })?;

        let newindex = lua.create_function(move |lua, mut args: MultiValue| {
            let obj = args.pop_front().unwrap();
            let key = BorrowedStr::from_lua(args.pop_front().unwrap(), lua)?;
            args.push_front(obj);
            match newindex.get(key.as_ref()) {
                Some(setter) => setter.call::<Value>(args),
                None => Err(LuaError::runtime(format!(
                    "tried to set nonexistent field [{key}] on type [{type_name}]",
                ))),
            }
        })?;

        ops.index = Some(index);
        ops.newindex = Some(newindex);

        // add the type to the temp types
        let mut types = lua.app_data_mut::<TempTypes>().unwrap();
        let type_idx = types.types.len();
        assert!(!types.type_indices.contains_key(&TypeId::of::<T>()));
        types.type_indices.insert(TypeId::of::<T>(), type_idx);
        types.types.push(TempTypeInfo {
            values: Box::new(Vec::<T>::new()),
            type_name,
            ops: ops.clone(),
            clear_fn: |values| {
                let values = values.downcast_mut::<Vec<T>>().unwrap();
                values.clear();
            },
        });

        lua.register_userdata_type::<T>(|reg| {
            macro_rules! op {
                ($name:literal $var:ident) => {
                    if let Some($var) = ops.$var {
                        reg.add_meta_function($name, move |_, args: MultiValue| {
                            $var.call::<Value>(args)
                        });
                    }
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
        })?;

        Ok(methods)
    }

    #[inline]
    pub fn type_idx(&self) -> u16 {
        self.info.type_idx
    }

    #[inline]
    pub fn version(&self) -> u16 {
        self.info.version
    }

    #[inline]
    pub fn value_idx(&self) -> u32 {
        self.info.value_idx
    }

    #[inline]
    pub fn new(lua: &Lua, value: T) -> LuaResult<Self> {
        let mut types = lua.app_data_mut::<TempTypes>().unwrap();
        let type_idx = *types.type_indices.get(&TypeId::of::<T>()).unwrap();
        let ty = &mut types.types[type_idx];
        let values = ty.values.downcast_mut::<Vec<T>>().unwrap();
        let value_idx = values.len();
        values.push(value);
        Ok(Self {
            info: TempInfo {
                type_idx: type_idx as u16,
                version: types.version,
                value_idx: value_idx as u32,
            },
            marker: PhantomData,
        })
    }

    #[inline]
    pub fn set(&self, lua: &Lua, val: T) -> LuaResult<()> {
        self.write(lua, |_, dst| {
            *dst = val;
            Ok(())
        })
    }

    #[inline]
    pub fn read<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &T) -> LuaResult<R>,
    {
        let types = lua.app_data_ref::<TempTypes>().unwrap();
        let ty = &types.types[self.info.type_idx as usize];
        if types.version != self.info.version {
            return Err(LuaError::runtime(format!(
                "attempt to index a temporary [{}] from another frame",
                ty.type_name
            )));
        }
        let values = ty.values.downcast_ref::<Vec<T>>().unwrap();
        f(lua, &values[self.info.value_idx as usize])
    }

    #[inline]
    pub fn field<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.read(lua, |_, dst| Ok(f(dst)))
    }

    #[inline]
    pub fn write<F, R>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &mut T) -> LuaResult<R>,
    {
        let mut types = lua.app_data_mut::<TempTypes>().unwrap();
        let version = types.version;
        let ty = &mut types.types[self.info.type_idx as usize];
        if version != self.info.version {
            return Err(LuaError::runtime(format!(
                "attempt to index a temporary [{}] from another frame",
                ty.type_name
            )));
        }
        let values = ty.values.downcast_mut::<Vec<T>>().unwrap();
        f(lua, &mut values[self.info.value_idx as usize])
    }

    #[inline]
    pub fn into_userdata(self, lua: &Lua) -> LuaResult<UserDataOf<T>> {
        self.read(lua, |lua, val| {
            lua.create_any_userdata(val.clone()).map(|data| UserDataOf {
                data,
                marker: PhantomData,
            })
        })
    }
}

impl<T: Copy + 'static> Temp<T> {
    #[inline]
    pub fn get(&self, lua: &Lua) -> LuaResult<T> {
        self.read(lua, |_, val| Ok(*val))
    }
}

impl<T: 'static> From<LightUserData> for Temp<T> {
    #[inline]
    fn from(value: LightUserData) -> Self {
        Self {
            info: unsafe { TempPtr { ptr: value.0 }.info },
            marker: PhantomData,
        }
    }
}

impl<T: 'static> From<Temp<T>> for LightUserData {
    #[inline]
    fn from(Temp { info, .. }: Temp<T>) -> Self {
        Self(unsafe { TempPtr { info }.ptr })
    }
}

impl<T: 'static> From<Temp<T>> for Value {
    #[inline]
    fn from(value: Temp<T>) -> Self {
        Self::LightUserData(LightUserData::from(value))
    }
}

impl<T: 'static> FromLua for Temp<T> {
    #[inline]
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        if let Value::LightUserData(data) = value {
            Ok(Self::from(data))
        } else {
            Err(LuaError::runtime(format!(
                "value is not type [{}]",
                type_name::<T>()
            )))
        }
    }
}

impl<T: 'static> IntoLua for Temp<T> {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::LightUserData(LightUserData(unsafe {
            TempPtr { info: self.info }.ptr
        })))
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union TempPtr {
    ptr: *mut c_void,
    info: TempInfo,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub(crate) struct TempInfo {
    pub type_idx: u16,
    pub version: u16,
    pub value_idx: u32,
}

impl FromLua for TempInfo {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        LightUserData::from_lua(value, lua).map(Self::from)
    }
}

impl From<LightUserData> for TempInfo {
    #[inline]
    fn from(LightUserData(ptr): LightUserData) -> Self {
        unsafe { TempPtr { ptr }.info }
    }
}

impl From<TempInfo> for LightUserData {
    #[inline]
    fn from(info: TempInfo) -> Self {
        Self(unsafe { TempPtr { info }.ptr })
    }
}
