use crate::core::Context;
use crate::input::{Gamepad, GamepadAxis, GamepadButton, GamepadStatus};
use crate::lua::LuaModule;
use fey_lua::{UserDataOf, create_fill};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, IntoLua, Lua, Table, UserData, UserDataMethods, UserDataRef, Value};

pub type GamepadObj = UserDataOf<Gamepad>;
pub type GamepadRef = UserDataRef<Gamepad>;

pub struct GamepadModule;

impl LuaModule for GamepadModule {
    const PATH: &'static str = "Gamepad";

    #[inline]
    fn load(lua: &Lua) -> LuaResult<Value> {
        Self.into_lua(lua)
    }
}

impl UserData for GamepadModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("count", |lua, _: ()| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.gamepads.count())
        });
        methods.add_function("all", |lua, fill: Option<Table>| {
            let ctx = Context::from_lua(lua);
            let fill = create_fill(lua, fill)?;
            if ctx.gamepads.count() > 0 {
                for pad in ctx.gamepads.all_lua() {
                    fill.raw_push(pad)?;
                }
                Ok(Some(fill))
            } else {
                Ok(None)
            }
        });
        methods.add_function("newly_connected", |lua, fill: Option<Table>| {
            let ctx = Context::from_lua(lua);
            let fill = create_fill(lua, fill)?;
            if ctx.gamepads.count() > 0 {
                for pad in ctx.gamepads.newly_connected_lua() {
                    fill.raw_push(pad)?;
                }
                Ok(Some(fill))
            } else {
                Ok(None)
            }
        });
        methods.add_function("last_active", |lua, _: ()| {
            let ctx = Context::from_lua(lua);
            Ok(ctx.gamepads.last_active_lua())
        });
        add_methods(methods);
    }
}

impl UserData for Gamepad {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("name", |lua, this: GamepadRef| this.name().into_lua(lua));
    methods.add_function("was_connected", |_, this: GamepadRef| {
        Ok(this.was_connected())
    });
    methods.add_function("charging_status", |lua, this: GamepadRef| {
        Ok(match this.charging_status() {
            GamepadStatus::Unknown => ("unknown".into_lua(lua)?, None),
            GamepadStatus::Wired => ("wired".into_lua(lua)?, None),
            GamepadStatus::Draining(p) => ("draining".into_lua(lua)?, Some(p)),
            GamepadStatus::Charging(p) => ("charging".into_lua(lua)?, Some(p)),
            GamepadStatus::Charged => ("charged".into_lua(lua)?, None),
        })
    });
    methods.add_function("down", |_, (this, btn): (GamepadRef, GamepadButton)| {
        Ok(this.down(btn))
    });
    methods.add_function("pressed", |_, (this, btn): (GamepadRef, GamepadButton)| {
        Ok(this.pressed(btn))
    });
    methods.add_function("released", |_, (this, btn): (GamepadRef, GamepadButton)| {
        Ok(this.released(btn))
    });
    methods.add_function("repeated", |_, (this, btn): (GamepadRef, GamepadButton)| {
        Ok(this.repeated(btn))
    });
    methods.add_function(
        "btn_changed",
        |_, (this, btn): (GamepadRef, GamepadButton)| Ok(this.btn_changed(btn)),
    );
    methods.add_function("value", |_, (this, btn): (GamepadRef, GamepadButton)| {
        Ok(this.value(btn))
    });
    methods.add_function("axis", |_, (this, axis): (GamepadRef, GamepadAxis)| {
        Ok(this.axis(axis))
    });
    methods.add_function(
        "axis_changed",
        |_, (this, axis): (GamepadRef, GamepadAxis)| Ok(this.axis_changed(axis)),
    );
}

pub struct GamepadButtonModule;

impl LuaModule for GamepadButtonModule {
    const PATH: &'static str = "GamepadButton";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.raw_set("SOUTH", GamepadButton::South)?;
        m.raw_set("EAST", GamepadButton::East)?;
        m.raw_set("WEST", GamepadButton::West)?;
        m.raw_set("NORTH", GamepadButton::North)?;
        m.raw_set("SELECT", GamepadButton::Select)?;
        m.raw_set("MENU", GamepadButton::Menu)?;
        m.raw_set("START", GamepadButton::Start)?;
        m.raw_set("LEFT_THUMB", GamepadButton::LeftThumb)?;
        m.raw_set("RIGHT_THUMB", GamepadButton::RightThumb)?;
        m.raw_set("LEFT_BUMPER", GamepadButton::LeftBumper)?;
        m.raw_set("RIGHT_BUMPER", GamepadButton::RightBumper)?;
        m.raw_set("LEFT_TRIGGER", GamepadButton::LeftTrigger)?;
        m.raw_set("RIGHT_TRIGGER", GamepadButton::RightTrigger)?;
        m.raw_set("DPAD_UP", GamepadButton::DPadUp)?;
        m.raw_set("DPAD_DOWN", GamepadButton::DPadDown)?;
        m.raw_set("DPAD_LEFT", GamepadButton::DPadLeft)?;
        m.raw_set("DPAD_RIGHT", GamepadButton::DPadRight)?;
        Ok(Value::Table(m))
    }
}

pub struct GamepadAxisModule;

impl LuaModule for GamepadAxisModule {
    const PATH: &'static str = "GamepadAxis";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.raw_set("LEFT_X", GamepadAxis::LeftX)?;
        m.raw_set("LEFT_Y", GamepadAxis::LeftY)?;
        m.raw_set("RIGHT_X", GamepadAxis::RightX)?;
        m.raw_set("RIGHT_Y", GamepadAxis::RightY)?;
        m.raw_set("DPAD_X", GamepadAxis::DPadX)?;
        m.raw_set("DPAD_Y", GamepadAxis::DPadY)?;
        Ok(Value::Table(m))
    }
}

impl FromLua for GamepadButton {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Self::from_repr(usize::from_lua(value, lua)?)
            .ok_or_else(|| LuaError::runtime("invalid gamepad button"))
    }
}

impl IntoLua for GamepadButton {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::Integer(self as _))
    }
}

impl FromLua for GamepadAxis {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        Self::from_repr(usize::from_lua(value, lua)?)
            .ok_or_else(|| LuaError::runtime("invalid gamepad axis"))
    }
}

impl IntoLua for GamepadAxis {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::Integer(self as _))
    }
}
