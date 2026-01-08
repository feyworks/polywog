use crate::{Temp, UserDataOf};
use mlua::prelude::LuaResult;
use mlua::{FromLua, Lua, UserData, UserDataRef, UserDataRefMut, Value};

pub enum HandleRef<T: 'static> {
    User(UserDataRef<T>),
    Temp(Temp<T>),
}

impl<T: 'static> FromLua for HandleRef<T> {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::UserData(data) => data.borrow::<T>().map(Self::User),
            val => Temp::from_lua(val, lua).map(Self::Temp),
        }
    }
}

impl<T: Clone + 'static> HandleRef<T> {
    #[inline]
    pub fn read<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &T) -> LuaResult<R>,
    {
        match self {
            Self::User(data) => f(lua, &data),
            Self::Temp(temp) => temp.read(lua, f),
        }
    }

    #[inline]
    pub fn field<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        match self {
            Self::User(data) => Ok(f(&data)),
            Self::Temp(temp) => temp.field(lua, f),
        }
    }

    #[inline]
    pub fn new_any_userdata(&self, lua: &Lua) -> LuaResult<UserDataOf<T>> {
        self.read(lua, |lua, val| {
            lua.create_any_userdata(val.clone())
                .map(UserDataOf::from_any)
        })
    }

    #[inline]
    pub fn new_userdata(&self, lua: &Lua) -> LuaResult<UserDataOf<T>>
    where
        T: UserData,
    {
        self.read(lua, |lua, val| {
            lua.create_userdata(val.clone()).map(UserDataOf::from_any)
        })
    }
}

impl<T: Copy + 'static> HandleRef<T> {
    #[inline]
    pub fn get(&self, lua: &Lua) -> LuaResult<T> {
        self.read(lua, |_, val| Ok(*val))
    }
}

pub enum HandleMut<T: 'static> {
    User(UserDataRefMut<T>),
    Temp(Temp<T>),
}

impl<T: Clone + 'static> HandleMut<T> {
    #[inline]
    pub fn read<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &T) -> LuaResult<R>,
    {
        match self {
            Self::User(data) => f(lua, &data),
            Self::Temp(temp) => temp.read(lua, f),
        }
    }

    #[inline]
    pub fn write<R, F>(&mut self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &mut T) -> LuaResult<R>,
    {
        match self {
            Self::User(data) => f(lua, data),
            Self::Temp(temp) => temp.write(lua, f),
        }
    }

    #[inline]
    pub fn field<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        match self {
            Self::User(data) => Ok(f(&data)),
            Self::Temp(temp) => temp.field(lua, f),
        }
    }
}

impl<T: Copy + 'static> HandleMut<T> {
    #[inline]
    pub fn get(&self, lua: &Lua) -> LuaResult<T> {
        self.read(lua, |_, val| Ok(*val))
    }
}
