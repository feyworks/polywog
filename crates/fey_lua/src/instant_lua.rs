use crate::{LuaModule, UserDataOf};
use mlua::prelude::LuaResult;
use mlua::{Lua, UserData, UserDataMethods, UserDataRef, Value};
use std::time::Instant;

pub type InstantObj = UserDataOf<LuaInstant>;
pub type InstantRef = UserDataRef<LuaInstant>;

pub struct InstantModule;

pub struct LuaInstant(pub Instant);

impl LuaModule for InstantModule {
    const PATH: &'static str = "Instant";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for InstantModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("now", |_, _: ()| Ok(LuaInstant(Instant::now())));
        add_methods(methods);
    }
}

impl UserData for LuaInstant {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("elapsed_secs", |_, this: InstantRef| {
        Ok(this.0.elapsed().as_secs_f64())
    });
    methods.add_function("elapsed_millis", |_, this: InstantRef| {
        Ok(this.0.elapsed().as_millis() as u64)
    });
    methods.add_function("elapsed_micros", |_, this: InstantRef| {
        Ok(this.0.elapsed().as_micros() as u64)
    });
    methods.add_function("elapsed_nanos", |_, this: InstantRef| {
        Ok(this.0.elapsed().as_nanos() as u64)
    });
}
