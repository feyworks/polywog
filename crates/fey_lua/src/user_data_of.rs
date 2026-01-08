use mlua::prelude::{LuaError, LuaResult};
use mlua::{AnyUserData, FromLua, IntoLua, Lua, UserData, UserDataRef, UserDataRefMut, Value};
use std::any::type_name;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct UserDataOf<T> {
    pub(crate) data: AnyUserData,
    pub(crate) marker: PhantomData<T>,
}

impl<T: 'static> UserDataOf<T> {
    #[inline]
    pub fn new(lua: &Lua, value: T) -> Self
    where
        T: UserData,
    {
        lua.create_userdata(value)
            .map(|data| Self {
                data,
                marker: PhantomData,
            })
            .unwrap()
    }

    #[inline]
    pub fn try_from_any(data: AnyUserData) -> Option<Self> {
        data.is::<T>().then(|| Self {
            data,
            marker: PhantomData,
        })
    }

    #[inline]
    pub fn try_from_any_ref(data: &AnyUserData) -> Option<Self> {
        data.is::<T>().then(|| Self {
            data: data.clone(),
            marker: PhantomData,
        })
    }

    #[inline]
    pub fn from_any(data: AnyUserData) -> Self {
        Self::try_from_any(data).unwrap()
    }

    #[inline]
    pub fn from_any_ref(data: &AnyUserData) -> Self {
        Self::try_from_any_ref(data).unwrap()
    }

    #[inline]
    pub fn ptr(&self) -> *const c_void {
        self.data.to_pointer()
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.data.to_pointer() == other.data.to_pointer()
    }

    #[inline]
    pub fn get(&self) -> UserDataRef<T> {
        self.data.borrow().unwrap()
    }

    #[inline]
    pub fn get_mut(&self) -> UserDataRefMut<T> {
        self.data.borrow_mut().unwrap()
    }

    #[inline]
    pub fn get_clone(&self) -> T
    where
        T: Clone,
    {
        self.get().deref().clone()
    }

    #[inline]
    pub fn read<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &T) -> LuaResult<R>,
    {
        self.data.borrow::<T>().map(|data| f(lua, &data))?
    }

    #[inline]
    pub fn write<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &mut T) -> LuaResult<R>,
    {
        self.data
            .borrow_mut::<T>()
            .map(|mut data| f(lua, &mut data))?
    }

    #[inline]
    pub fn set(&self, value: T) -> LuaResult<()> {
        *self.data.borrow_mut::<T>()? = value;
        Ok(())
    }

    #[inline]
    pub fn field<R, F>(&self, f: F) -> LuaResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.data.borrow::<T>().map(|data| f(&data))
    }
}

impl<T: 'static> Clone for UserDataOf<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: 'static> FromLua for UserDataOf<T> {
    #[inline]
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        let Value::UserData(data) = value else {
            return Err(LuaError::runtime("expected userdata value"));
        };
        Self::try_from_any(data).ok_or_else(|| {
            LuaError::runtime(format!(
                "userdata is not expected type [{}]",
                type_name::<T>()
            ))
        })
    }
}

impl<T: 'static> IntoLua for UserDataOf<T> {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::UserData(self.data))
    }
}

impl<T: 'static> Into<AnyUserData> for UserDataOf<T> {
    #[inline]
    fn into(self) -> AnyUserData {
        self.data
    }
}
