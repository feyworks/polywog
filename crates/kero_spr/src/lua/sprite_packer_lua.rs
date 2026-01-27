use crate::SpritePacker;
use fey_lua::{LuaModule, UserDataOf};
use kero::prelude::*;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{
    BorrowedStr, Either, Lua, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value,
};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct SpritePackerModule;

pub type SpritePackerData = UserDataOf<SpritePacker<String>>;
pub type SpritePackerRef = UserDataRef<SpritePacker<String>>;
pub type SpritePackerMut = UserDataRefMut<SpritePacker<String>>;

impl LuaModule for SpritePackerModule {
    const PATH: &'static str = "SpritePacker";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SpritePackerModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, _: ()| Ok(SpritePacker::<String>::new()));
        add_methods(methods);
    }
}

impl UserData for SpritePacker<String> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

// fn load_img_file(path: impl AsRef<Path>, premult: bool) -> LuaResult<ImageRgba8> {
//     let path = path.as_ref();
//     let mut img = DynImage::load_file(path)
//         .map(DynImage::to_rgba8)
//         .map_err(LuaError::external)?;
//     if premult {
//         img.premultiply();
//     }
//     Ok(img)
// }

// fn load_img(img: Either<DynImageRef, BorrowedStr>) -> LuaResult<ImageRgba8> {
//     match img {
//         Either::Left(img) => Ok(img.clone().to_rgba8()),
//         Either::Right(path) => {
//             let path = Path::new(path.as_ref());
//             load_img_file(&path)
//         }
//     }
// }

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function(
        "add_sprite",
        |_,
         (mut this, id, file, premult, thresh): (
            SpritePackerMut,
            String,
            BorrowedStr,
            bool,
            Option<u8>,
        )| {
            this.add_sprite_file(id, file.as_ref(), premult, thresh)
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_sprites_in",
        |_, (mut this, dir, premult, thresh): (SpritePackerMut, BorrowedStr, bool, Option<u8>)| {
            this.add_sprite_files(dir.as_ref(), premult, thresh)
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_sheet",
        |_,
         (mut this, id, file, premult, tw, th, thresh): (
            SpritePackerMut,
            String,
            BorrowedStr,
            bool,
            u32,
            u32,
            Option<u8>,
        )| {
            this.add_sheet_file(id, file.as_ref(), premult, vec2(tw, th), thresh)
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_sheets_in",
        |_,
         (mut this, dir, premult, tw, th, thresh): (
            SpritePackerMut,
            BorrowedStr,
            bool,
            u32,
            u32,
            Option<u8>,
        )| {
            this.add_sheet_files(dir.as_ref(), premult, (tw, th), thresh)
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_font",
        |_,
         (mut this, id, file, size, chars): (
            SpritePackerMut,
            String,
            BorrowedStr,
            f32,
            Option<Vec<char>>,
        )| {
            let chars = chars.unwrap_or_else(|| BASIC_LATIN.chars().collect());
            this.add_font_file(id, file.as_ref(), size, chars)
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_fonts_in",
        |_,
         (mut this, dir, size, chars): (
             SpritePackerMut,
             BorrowedStr,
             f32,
             Option<Vec<char>>,
         )| {
            let chars = chars.unwrap_or_else(|| BASIC_LATIN.chars().collect());
            this.add_font_files(dir.as_ref(), size, chars)
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_patch",
        |_,
         (mut this, id, file, premult, inner): (
            SpritePackerMut,
            String,
            BorrowedStr,
            bool,
            RectF,
        )| {
            this.add_patch_file(id, file.as_ref(), premult, inner.to_u32())
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_patches_in",
        |_, (mut this, dir, premult, inner): (SpritePackerMut, BorrowedStr, bool, RectF)| {
            this.add_patch_files(dir.as_ref(), premult, inner.to_u32())
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_ase",
        |_, (mut this, id, file): (SpritePackerMut, String, BorrowedStr)| {
            this.add_ase_file(id, file.as_ref())
                .map_err(LuaError::external)
        },
    );
    methods.add_function(
        "add_ases_in",
        |_, (mut this, dir): (SpritePackerMut, BorrowedStr)| {
            this.add_ase_files(dir.as_ref()).map_err(LuaError::external)
        },
    );
    methods.add_function(
        "pack",
        |lua, (mut this, max_size): (SpritePackerMut, u32)| {
            let ctx = Context::from_lua(lua);
            let atlas = this
                .pack_graphics(max_size, &ctx.graphics)
                .map_err(LuaError::external)?;
            let t = lua.create_table()?;
            t.set("texture", atlas.texture)?;
            t.set("sprites", atlas.sprites)?;
            t.set("sheets", atlas.sheets)?;
            t.set("fonts", atlas.fonts)?;
            t.set("patches", atlas.patches)?;
            t.set("anims", atlas.anims)?;
            Ok(Some(t))
        },
    );
}
