use crate::SpriteSheet;
use fey_lua::{LuaModule, UserDataOf};
use kero::prelude::*;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{Lua, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value};

pub struct SpriteSheetModule;

pub type SpriteSheetObj = UserDataOf<SpriteSheet>;
pub type SpriteSheetRef = UserDataRef<SpriteSheet>;
pub type SpriteSheetMut = UserDataRefMut<SpriteSheet>;

impl LuaModule for SpriteSheetModule {
    const PATH: &'static str = "SpriteSheet";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SpriteSheetModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

impl UserData for SpriteSheet {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("cols", |_, this: SpriteSheetRef| Ok(this.tiles.width()));
    methods.add_function("rows", |_, this: SpriteSheetRef| Ok(this.tiles.height()));
    methods.add_function("tile_w", |_, this: SpriteSheetRef| Ok(this.tile_size.x));
    methods.add_function("tile_h", |_, this: SpriteSheetRef| Ok(this.tile_size.y));
    methods.add_function("tile_size", |_, this: SpriteSheetRef| Ok(this.tile_size));
    methods.add_function(
        "draw_tile",
        |lua,
         (this, tx, ty, pos, col, mode, fx, fy): (
            SpriteSheetRef,
            u32,
            u32,
            Vec2F,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let tile = this.tile((tx, ty)).ok_or_else(|| {
                LuaError::runtime(format!("tile coordinate ({tx}, {ty}) out of bounds"))
            })?;
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.subtexture_at_ext(&tile.sub, pos, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.subtexture_at_flipped(&tile.sub, pos, col, mode, (fx, fy));
                }
            }
            Ok(())
        },
    );
}
