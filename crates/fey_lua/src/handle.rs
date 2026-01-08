use crate::{Temp, UserDataOf};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, Lua, UserData, Value};
use std::any::type_name;
use std::marker::PhantomData;

pub enum Handle<T: 'static> {
    User(UserDataOf<T>),
    Temp(Temp<T>),
}

impl<T: Clone + 'static> Handle<T> {
    #[inline]
    pub fn read<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &T) -> LuaResult<R>,
    {
        match self {
            Self::User(data) => data.read(lua, f),
            Self::Temp(temp) => temp.read(lua, f),
        }
    }

    #[inline]
    pub fn write<R, F>(&mut self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua, &mut T) -> LuaResult<R>,
    {
        match self {
            Self::User(data) => data.write(lua, f),
            Self::Temp(temp) => temp.write(lua, f),
        }
    }

    #[inline]
    pub fn field<R, F>(&self, lua: &Lua, f: F) -> LuaResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        match self {
            Self::User(data) => data.field(f),
            Self::Temp(temp) => temp.field(lua, f),
        }
    }

    #[inline]
    pub fn new_any_userdata(&self, lua: &Lua) -> LuaResult<UserDataOf<T>> {
        self.read(lua, |lua, val| {
            lua.create_any_userdata(val.clone()).map(|data| UserDataOf {
                data,
                marker: PhantomData,
            })
        })
    }

    #[inline]
    pub fn new_userdata(&self, lua: &Lua) -> LuaResult<UserDataOf<T>>
    where
        T: UserData,
    {
        self.read(lua, |lua, val| {
            lua.create_userdata(val.clone()).map(|data| UserDataOf {
                data,
                marker: PhantomData,
            })
        })
    }

    // #[inline]
    // pub fn boxed(self, lua: &Lua) -> LuaResult<UserDataOf<T>> {
    //     match self {
    //         Self::User(data) => Ok(data),
    //         Self::Temp(temp) => temp.read(lua, |lua, val| {
    //             lua.create_any_userdata(val.clone()).map(|data| UserDataOf {
    //                 data,
    //                 marker: PhantomData,
    //             })
    //         }),
    //     }
    // }
}

impl<T: Copy + 'static> Handle<T> {
    #[inline]
    pub fn get(&self, lua: &Lua) -> LuaResult<T> {
        self.read(lua, |_, val| Ok(*val))
    }
}

impl<T: 'static> FromLua for Handle<T> {
    #[inline]
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::UserData(data) => UserDataOf::try_from_any(data).map(Self::User),
            Value::LightUserData(data) => Some(Self::Temp(Temp::from(data))),
            _ => None,
        }
        .ok_or_else(|| LuaError::runtime(format!("expected argument of type {}", type_name::<T>())))
    }
}
