use super::{MonitorRef, VideoModeRef};
use crate::core::{Context, CursorIcon, DisplayMode};
use crate::lua::LuaModule;
use crate::math::Numeric;
use fey_math::Vec2;
use mlua::prelude::LuaResult;
use mlua::{BorrowedStr, Lua, Value};

pub struct WindowModule;

impl LuaModule for WindowModule {
    const PATH: &'static str = "Window";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set(
            "title",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.title())
            })?,
        )?;
        m.set(
            "set_title",
            lua.create_function(|lua, title: BorrowedStr| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.set_title(title.as_ref()))
            })?,
        )?;
        m.set(
            "title",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.scale_factor())
            })?,
        )?;
        m.set(
            "monitor",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.monitor())
            })?,
        )?;
        m.set(
            "center_on",
            lua.create_function(|lua, monitor: MonitorRef| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.center_on(&monitor))
            })?,
        )?;
        m.set(
            "fullscreen_mode",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(match ctx.window.display_mode() {
                    DisplayMode::FullscreenBorderless(_) => Some("borderless"),
                    DisplayMode::FullscreenExclusive(_) => Some("exclusive"),
                    DisplayMode::Windowed(_) => None,
                })
            })?,
        )?;
        m.set(
            "set_windowed",
            lua.create_function(|lua, monitor: Option<MonitorRef>| {
                let ctx = Context::from_lua(lua);
                ctx.window
                    .set_display_mode(DisplayMode::Windowed(monitor.map(|m| m.clone())));
                Ok(())
            })?,
        )?;
        m.set(
            "set_fullscreen_borderless",
            lua.create_function(|lua, monitor: Option<MonitorRef>| {
                let ctx = Context::from_lua(lua);
                ctx.window
                    .set_display_mode(DisplayMode::FullscreenBorderless(
                        monitor.map(|m| m.clone()),
                    ));
                Ok(())
            })?,
        )?;
        m.set(
            "set_fullscreen_exclusive",
            lua.create_function(|lua, mode: VideoModeRef| {
                let ctx = Context::from_lua(lua);
                ctx.window
                    .set_display_mode(DisplayMode::FullscreenExclusive(mode.clone()));
                Ok(())
            })?,
        )?;
        m.set(
            "has_focus",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.has_focus())
            })?,
        )?;
        m.set(
            "pos",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.pos().map(Vec2::to_f32))
            })?,
        )?;
        m.set(
            "x",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.pos().map(|p| p.x))
            })?,
        )?;
        m.set(
            "y",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.pos().map(|p| p.y))
            })?,
        )?;
        m.set(
            "outer_pos",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.outer_pos().map(Vec2::to_f32))
            })?,
        )?;
        m.set(
            "outer_x",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.outer_pos().map(Vec2::to_f32))
            })?,
        )?;
        m.set(
            "outer_y",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.outer_pos().map(|p| p.y))
            })?,
        )?;
        m.set(
            "set_outer_pos",
            lua.create_function(|lua, (x, y): (i32, i32)| {
                let ctx = Context::from_lua(lua);
                ctx.window.set_outer_pos((x, y));
                Ok(())
            })?,
        )?;
        m.set(
            "size",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.size())
            })?,
        )?;
        m.set(
            "width",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.size().x)
            })?,
        )?;
        m.set(
            "height",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.size().y)
            })?,
        )?;
        m.set(
            "outer_size",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.outer_size())
            })?,
        )?;
        m.set(
            "outer_width",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.outer_size().x)
            })?,
        )?;
        m.set(
            "outer_height",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.outer_size().y)
            })?,
        )?;
        m.set(
            "request_size",
            lua.create_function(|lua, (w, h): (u32, u32)| {
                let ctx = Context::from_lua(lua);
                ctx.window.request_size((w, h));
                Ok(())
            })?,
        )?;
        m.set(
            "resizable",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.resizable())
            })?,
        )?;
        m.set(
            "set_resizable",
            lua.create_function(|lua, resizable: bool| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.set_resizable(resizable))
            })?,
        )?;
        m.set(
            "maximized",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.maximized())
            })?,
        )?;
        m.set(
            "set_maximized",
            lua.create_function(|lua, maximized: bool| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.set_maximized(maximized))
            })?,
        )?;
        m.set(
            "minimized",
            lua.create_function(|lua, _: ()| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.minimized())
            })?,
        )?;
        m.set(
            "set_minimized",
            lua.create_function(|lua, minimized: bool| {
                let ctx = Context::from_lua(lua);
                Ok(ctx.window.set_minimized(minimized))
            })?,
        )?;
        m.set(
            "set_cursor",
            lua.create_function(|lua, cursor: CursorIcon| {
                let ctx = Context::from_lua(lua);
                ctx.window.set_cursor(cursor);
                Ok(())
            })?,
        )?;
        Ok(Value::Table(m))
    }
}
