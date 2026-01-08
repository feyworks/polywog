use mlua::prelude::LuaResult;
use mlua::{Function, Lua, Value};

pub trait LuaModule {
    const PATH: &'static str;

    fn load(lua: &Lua) -> LuaResult<Value>;
}

pub trait LuaModules {
    fn loaders(lua: &Lua) -> LuaResult<Vec<(&'static str, Function)>>;
}

macro_rules! tuple {
    ($($type:ident),*) => {
        impl<$($type,)*> LuaModules for ($($type,)*)
        where
            $($type: LuaModule,)*
        {
            fn loaders(lua: &Lua) -> LuaResult<Vec<(&'static str, Function)>> {
                Ok(vec![
                    $((
                        $type::PATH,
                        {
                            let module = $type::load(lua)?;
                            lua.create_function(move |_, _: ()| Ok(module.clone()))?
                        }
                    ),)*
                ])
            }
        }
    };
}

tuple!(T0);
tuple!(T0, T1);
tuple!(T0, T1, T2);
tuple!(T0, T1, T2, T3);
tuple!(T0, T1, T2, T3, T4);
tuple!(T0, T1, T2, T3, T4, T5);
tuple!(T0, T1, T2, T3, T4, T5, T6);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
);
tuple!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
tuple!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
tuple!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
tuple!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
tuple!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
