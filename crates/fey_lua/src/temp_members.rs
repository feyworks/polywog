use crate::Handle;
use crate::ops::Ops;
use mlua::prelude::{LuaResult, LuaString};
use mlua::{FromLua, FromLuaMulti, Function, IntoLuaMulti, Lua, MultiValue, Table, Value};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct TempMembers<'a, T> {
    lua: &'a Lua,
    marker: PhantomData<T>,
    pub(crate) methods: Table,
    pub(crate) index: HashMap<&'static str, IndexValue>,
    pub(crate) newindex: HashMap<&'static str, Function>,
    pub(crate) ops: Ops,
}

pub(crate) struct IndexValue {
    pub func: Function,
    pub getter: bool,
}

impl<'a, T: Clone + 'static> TempMembers<'a, T> {
    pub(crate) fn new(lua: &'a Lua) -> LuaResult<Self> {
        Ok(Self {
            lua,
            marker: PhantomData,
            methods: lua.create_table()?,
            index: HashMap::default(),
            newindex: HashMap::default(),
            ops: Ops::default(),
        })
    }

    pub fn getter_ext<F, R>(&mut self, name: &'static str, getter: F) -> LuaResult<()>
    where
        F: Fn(&Lua, &T) -> LuaResult<R> + 'static,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self.lua.create_function(move |lua, obj: Handle<T>| {
            obj.read(lua, |lua, obj| getter(lua, obj))
        })?;
        self.index.insert(name, IndexValue { func, getter: true });
        Ok(())
    }

    pub fn getter<F, R>(&mut self, name: &'static str, getter: F) -> LuaResult<()>
    where
        F: Fn(&T) -> R + 'static,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self
            .lua
            .create_function(move |lua, obj: Handle<T>| obj.read(lua, |_, obj| Ok(getter(obj))))?;
        self.index.insert(name, IndexValue { func, getter: true });
        Ok(())
    }

    pub fn setter_ext<V, F>(&mut self, name: &'static str, setter: F) -> LuaResult<()>
    where
        V: FromLua,
        F: Fn(&Lua, &mut T, V) -> LuaResult<()> + 'static,
    {
        assert!(!self.newindex.contains_key(name));
        let setter = self
            .lua
            .create_function(move |lua, (mut obj, val): (Handle<T>, Value)| {
                obj.write(lua, |lua, obj| setter(lua, obj, V::from_lua(val, lua)?))
            })?;
        self.newindex.insert(name, setter);
        Ok(())
    }

    pub fn setter<V, F>(&mut self, name: &'static str, setter: F) -> LuaResult<()>
    where
        V: FromLua,
        F: Fn(&mut T, V) + 'static,
    {
        assert!(!self.newindex.contains_key(name));
        let setter = self
            .lua
            .create_function(move |lua, (mut obj, val): (Handle<T>, Value)| {
                obj.write(lua, |lua, obj| {
                    setter(obj, V::from_lua(val, lua)?);
                    Ok(())
                })
            })?;
        self.newindex.insert(name, setter);
        Ok(())
    }

    pub fn handle_method_ext<F, A, R>(&mut self, name: &'static str, method: F) -> LuaResult<()>
    where
        F: Fn(&Lua, Handle<T>, A) -> LuaResult<R> + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self.lua.create_function(move |lua, mut args: MultiValue| {
            let h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            method(lua, h, A::from_lua_multi(args, lua)?)
        })?;
        self.methods.set(name, func.clone())?;
        self.index.insert(
            name,
            IndexValue {
                func,
                getter: false,
            },
        );
        Ok(())
    }

    pub fn method_ext<F, A, R>(&mut self, name: &'static str, method: F) -> LuaResult<()>
    where
        F: Fn(&Lua, &T, A) -> LuaResult<R> + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self.lua.create_function(move |lua, mut args: MultiValue| {
            let h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            h.read(lua, |lua, obj| {
                method(lua, obj, A::from_lua_multi(args, lua)?)
            })
        })?;
        self.methods.set(name, func.clone())?;
        self.index.insert(
            name,
            IndexValue {
                func,
                getter: false,
            },
        );
        Ok(())
    }

    pub fn method<F, A, R>(&mut self, name: &'static str, method: F) -> LuaResult<()>
    where
        F: Fn(&T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self.lua.create_function(move |lua, mut args: MultiValue| {
            let h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            h.read(lua, |lua, obj| {
                Ok(method(obj, A::from_lua_multi(args, lua)?))
            })
        })?;
        self.methods.set(name, func.clone())?;
        self.index.insert(
            name,
            IndexValue {
                func,
                getter: false,
            },
        );
        Ok(())
    }

    pub fn method_ext_mut<F, A, R>(&mut self, name: &'static str, method: F) -> LuaResult<()>
    where
        F: Fn(&Lua, &mut T, A) -> LuaResult<R> + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self.lua.create_function(move |lua, mut args: MultiValue| {
            let mut h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            h.write(lua, |lua, obj| {
                method(lua, obj, A::from_lua_multi(args, lua)?)
            })
        })?;
        self.methods.set(name, func.clone())?;
        self.index.insert(
            name,
            IndexValue {
                func,
                getter: false,
            },
        );
        Ok(())
    }

    pub fn method_mut<F, A, R>(&mut self, name: &'static str, method: F) -> LuaResult<()>
    where
        F: Fn(&mut T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(!self.index.contains_key(name));
        let func = self.lua.create_function(move |lua, mut args: MultiValue| {
            let mut h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            h.write(lua, |lua, obj| {
                Ok(method(obj, A::from_lua_multi(args, lua)?))
            })
        })?;
        self.methods.set(name, func.clone())?;
        self.index.insert(
            name,
            IndexValue {
                func,
                getter: false,
            },
        );
        Ok(())
    }

    pub fn op_call<F, A, R>(&mut self, op: F) -> LuaResult<()>
    where
        F: Fn(&Lua, &T, A) -> LuaResult<R> + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(self.ops.call.is_none());
        self.ops.call = Some(self.lua.create_function(move |lua, mut args: MultiValue| {
            let h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            h.read(lua, |lua, obj| op(lua, obj, A::from_lua_multi(args, lua)?))
        })?);
        Ok(())
    }

    pub fn op_call_mut<F, A, R>(&mut self, op: F) -> LuaResult<()>
    where
        F: Fn(&Lua, &mut T, A) -> LuaResult<R> + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        assert!(self.ops.call.is_none());
        self.ops.call = Some(self.lua.create_function(move |lua, mut args: MultiValue| {
            let mut h = Handle::<T>::from_lua(args.pop_front().unwrap(), lua)?;
            h.write(lua, |lua, obj| op(lua, obj, A::from_lua_multi(args, lua)?))
        })?);
        Ok(())
    }

    pub fn op_tostring_ext<F>(&mut self, op: F) -> LuaResult<()>
    where
        F: Fn(&Lua, &T) -> LuaResult<LuaString> + 'static,
    {
        assert!(self.ops.tostring.is_none());
        self.ops.tostring =
            Some(self.lua.create_function(move |lua, val: Handle<T>| {
                val.read(lua, |lua, obj| op(lua, obj))
            })?);
        Ok(())
    }

    pub fn op_tostring<F>(&mut self, op: F) -> LuaResult<()>
    where
        F: Fn(&T) -> String + 'static,
    {
        assert!(self.ops.tostring.is_none());
        self.ops.tostring = Some(self.lua.create_function(move |lua, val: Handle<T>| {
            val.read(lua, |lua, obj| lua.create_string(op(obj)))
        })?);
        Ok(())
    }
}

macro_rules! unary_op {
    ($var_name:ident $fn_name:ident $ext_name:ident) => {
        impl<T: Clone + 'static> TempMembers<'_, T> {
            pub fn $ext_name<F, R>(&mut self, op: F) -> LuaResult<()>
            where
                F: Fn(&Lua, &T) -> LuaResult<R> + 'static,
                R: IntoLuaMulti,
            {
                assert!(self.ops.$var_name.is_none());
                self.ops.$var_name =
                    Some(self.lua.create_function(move |lua, val: Handle<T>| {
                        val.read(lua, |lua, obj| op(lua, obj))
                    })?);
                Ok(())
            }

            pub fn $fn_name<F, R>(&mut self, op: F) -> LuaResult<()>
            where
                F: Fn(&T) -> R + 'static,
                R: IntoLuaMulti,
            {
                assert!(self.ops.$var_name.is_none());
                self.ops.$var_name =
                    Some(self.lua.create_function(move |lua, val: Handle<T>| {
                        val.read(lua, |_, obj| Ok(op(obj)))
                    })?);
                Ok(())
            }
        }
    };
}

unary_op!(unm op_unm op_unm_ext);
unary_op!(bnot op_bnot op_bnot_ext);
unary_op!(len op_len op_len_ext);

macro_rules! binary_op {
    ($var_name:ident $fn_name:ident $ext_name:ident) => {
        impl<T: Clone + 'static> TempMembers<'_, T> {
            pub fn $ext_name<F, A, R>(&mut self, op: F) -> LuaResult<()>
            where
                F: Fn(&Lua, &T, A) -> LuaResult<R> + 'static,
                A: FromLua,
                R: IntoLuaMulti,
            {
                assert!(self.ops.$var_name.is_none());
                self.ops.$var_name = Some(self.lua.create_function(
                    move |lua, (a, b): (Handle<T>, Value)| {
                        a.read(lua, |lua, a| op(lua, a, A::from_lua(b, lua)?))
                    },
                )?);
                Ok(())
            }

            pub fn $fn_name<F, A, R>(&mut self, op: F) -> LuaResult<()>
            where
                F: Fn(&T, A) -> R + 'static,
                A: FromLua,
                R: IntoLuaMulti,
            {
                assert!(self.ops.$var_name.is_none());
                self.ops.$var_name = Some(self.lua.create_function(
                    move |lua, (a, b): (Handle<T>, Value)| {
                        a.read(lua, |lua, a| Ok(op(a, A::from_lua(b, lua)?)))
                    },
                )?);
                Ok(())
            }
        }
    };
}

binary_op!(add op_add op_add_ext);
binary_op!(sub op_sub op_sub_ext);
binary_op!(mul op_mul op_mul_ext);
binary_op!(div op_div op_div_ext);
binary_op!(r#mod op_mod op_mod_ext);
binary_op!(pow op_pow op_pow_ext);
binary_op!(idiv op_idiv op_idiv_ext);
binary_op!(band op_band op_band_ext);
binary_op!(bor op_bor op_bor_ext);
binary_op!(bxor op_bxor op_bxor_ext);
binary_op!(shl op_shl op_shl_ext);
binary_op!(shr op_shr op_shr_ext);
binary_op!(eq op_eq op_eq_ext);
binary_op!(lt op_lt op_lt_ext);
binary_op!(le op_le op_le_ext);
binary_op!(concat op_concat op_concat_ext);
