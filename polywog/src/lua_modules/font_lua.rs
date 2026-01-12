use crate::core::Context;
use crate::gfx::{Font, Glyph, SubTexture};
use crate::lua::LuaModule;
use crate::misc::BASIC_LATIN;
use fey_lua::UserDataOf;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{BorrowedStr, Lua, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value};

pub type FontData = UserDataOf<Font>;
pub type FontRef = UserDataRef<Font>;
pub type FontMut = UserDataRefMut<Font>;

pub struct FontModule;

impl LuaModule for FontModule {
    const PATH: &'static str = "Font";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for FontModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (size, pixelated): (f32, bool)| {
            Ok(Font::new(size, pixelated))
        });
        methods.add_function(
            "from_ttf_file",
            |lua, (path, size, pixelated, chars): (BorrowedStr, f32, bool, Option<BorrowedStr>)| {
                let chars = chars
                    .map(|chrs| chrs.to_string())
                    .unwrap_or_else(|| BASIC_LATIN.chars().collect());
                let ctx = Context::from_lua(lua);
                Font::from_ttf_file(&ctx.graphics, path.as_ref(), size, pixelated, chars.chars())
                    .map_err(LuaError::external)?
                    .ok_or_else(|| LuaError::runtime("failed to pack font"))
                    .map(|(font, _)| font)
            },
        );
        add_methods(methods);
    }
}

impl UserData for Font {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    fn get_char(s: BorrowedStr) -> LuaResult<char> {
        s.chars()
            .next()
            .ok_or_else(|| LuaError::runtime(format!("invalid char {s:?}")))
    }

    methods.add_function("size", |_, this: FontRef| Ok(this.size()));
    methods.add_function("pixelated", |_, this: FontRef| Ok(this.pixelated()));
    methods.add_function(
        "set_glyph",
        |_, (mut this, chr, sub, adv): (FontMut, BorrowedStr, Option<SubTexture>, f32)| {
            this.set_glyph(get_char(chr)?, Glyph { sub, adv });
            Ok(())
        },
    );
    methods.add_function(
        "set_kerning",
        |_, (mut this, left, right, kern): (FontMut, BorrowedStr, BorrowedStr, f32)| {
            let left = get_char(left)?;
            let right = get_char(right)?;
            this.set_kerning(left, right, kern);
            Ok(())
        },
    );
    methods.add_function(
        "kerning",
        |_, (this, left, right): (FontRef, BorrowedStr, BorrowedStr)| {
            let left = get_char(left)?;
            let right = get_char(right)?;
            Ok(this.kerning(left, right).unwrap_or(0.0))
        },
    );
}
