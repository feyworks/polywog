use crate::{DynImage, Image, ImageError, ImageFormat};
use fey_color::{Channel, Grey, GreyAlpha, Rgb, Rgba, Rgba8, ToRgb, ToRgba, grey, grey_alpha};
use fey_grid::{Grid, GridMut};
use fey_lua::{LuaModule, UserDataOf};
use fey_math::{Numeric, Rect, RectF};
use mlua::prelude::{LuaError, LuaResult};
use mlua::{
    AnyUserData, BorrowedStr, FromLua, IntoLua, Lua, UserData, UserDataMethods, UserDataRef,
    UserDataRefMut, Value,
};
use std::ops::{Deref, DerefMut};

pub type DynImageObj = UserDataOf<DynImage>;
pub type DynImageRef = UserDataRef<DynImage>;
pub type DynImageMut = UserDataRefMut<DynImage>;

pub struct ImageModule;

impl LuaModule for ImageModule {
    const PATH: &'static str = "Image";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(ImageModule).map(Value::UserData)
    }
}

impl UserData for ImageModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, true);
    }
}

impl UserData for DynImage {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods, false);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M, module: bool) {
    if module {
        methods.add_function("new", |_, (w, h, f): (u32, u32, ImageFormat)| {
            let size = (w, h);
            Ok(match f {
                ImageFormat::Grey8 => DynImage::Grey8(Image::new_vec(size, Grey::BLACK)),
                ImageFormat::Grey16 => DynImage::Grey16(Image::new_vec(size, Grey::BLACK)),
                ImageFormat::Grey32F => DynImage::Grey32F(Image::new_vec(size, Grey::BLACK)),
                ImageFormat::GreyAlpha8 => {
                    DynImage::GreyAlpha8(Image::new_vec(size, GreyAlpha::TRANSPARENT))
                }
                ImageFormat::GreyAlpha16 => {
                    DynImage::GreyAlpha16(Image::new_vec(size, GreyAlpha::TRANSPARENT))
                }
                ImageFormat::GreyAlpha32F => {
                    DynImage::GreyAlpha32F(Image::new_vec(size, GreyAlpha::TRANSPARENT))
                }
                ImageFormat::Rgb8 => DynImage::Rgb8(Image::new_vec(size, Rgb::BLACK)),
                ImageFormat::Rgb16 => DynImage::Rgb16(Image::new_vec(size, Rgb::BLACK)),
                ImageFormat::Rgb32F => DynImage::Rgb32F(Image::new_vec(size, Rgb::BLACK)),
                ImageFormat::Rgba8 => DynImage::Rgba8(Image::new_vec(size, Rgba::TRANSPARENT)),
                ImageFormat::Rgba16 => DynImage::Rgba16(Image::new_vec(size, Rgba::TRANSPARENT)),
                ImageFormat::Rgba32F => DynImage::Rgba32F(Image::new_vec(size, Rgba::TRANSPARENT)),
            })
        });
        methods.add_function("clone", |_, this: DynImageRef| Ok(this.clone()));
        methods.add_function("format", |_, this: DynImageRef| Ok(this.format()));
        methods.add_function("width", |_, this: DynImageRef| Ok(this.size().x));
        methods.add_function("height", |_, this: DynImageRef| Ok(this.size().y));
        methods.add_function("size", |_, this: DynImageRef| Ok(this.size().to_f32()));
        methods.add_function("clone", |_, this: DynImageRef| Ok(this.deref().clone()));
        methods.add_function("to_rgba8", |lua, this: AnyUserData| {
            let img = this.borrow::<DynImage>()?;
            match img.deref() {
                DynImage::Rgba8(_) => Ok(this.clone()),
                img => lua.create_userdata(img.clone()),
            }
        });

        //     ---Returns an iterator over each pixel that yields `(color, x, y)`.
        // ---@param self Image
        //     ---@return fun(): integer, integer, integer
        //     ---@nodiscard
        // function methods.pixels(self) end

        //     ---Returns an iterator over each pixel in the region that yields `(color, x, y)`.
        // ---@param self Image
        //     ---@param x integer
        //     ---@param y integer
        //     ---@param w integer
        //     ---@param h integer
        //     ---@return fun(): integer, integer, integer
        //     ---@nodiscard
        // function methods.pixels(self, x, y, w, h) end

        methods.add_function("get_pixel", |_, (this, x, y): (DynImageRef, u32, u32)| {
            Ok(match this.deref() {
                DynImage::Grey8(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Grey16(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Grey32F(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::GreyAlpha8(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::GreyAlpha16(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::GreyAlpha32F(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Rgb8(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Rgb16(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Rgb32F(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Rgba8(img) => img.get(x, y).copied(),
                DynImage::Rgba16(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
                DynImage::Rgba32F(img) => img.get(x, y).copied().map(ToRgba::to_rgba),
            }
            .ok_or_else(|| LuaError::runtime(format!("no pixel at ({x}, {y})"))))
        });

        methods.add_function(
            "set_pixel",
            |_, (mut this, x, y, col): (DynImageMut, u32, u32, Rgba8)| {
                match this.deref_mut() {
                    DynImage::Grey8(img) => _ = img.set(x, y, col.lua_to_grey()),
                    DynImage::Grey16(img) => _ = img.set(x, y, col.lua_to_grey()),
                    DynImage::Grey32F(img) => _ = img.set(x, y, col.lua_to_grey()),
                    DynImage::GreyAlpha8(img) => _ = img.set(x, y, col.lua_to_grey_alpha()),
                    DynImage::GreyAlpha16(img) => _ = img.set(x, y, col.lua_to_grey_alpha()),
                    DynImage::GreyAlpha32F(img) => _ = img.set(x, y, col.lua_to_grey_alpha()),
                    DynImage::Rgb8(img) => _ = img.set(x, y, col.lua_to_rgb()),
                    DynImage::Rgb16(img) => _ = img.set(x, y, col.lua_to_rgb()),
                    DynImage::Rgb32F(img) => _ = img.set(x, y, col.lua_to_rgb()),
                    DynImage::Rgba8(img) => _ = img.set(x, y, col),
                    DynImage::Rgba16(img) => _ = img.set(x, y, col.to_rgba()),
                    DynImage::Rgba32F(img) => _ = img.set(x, y, col.to_rgba()),
                }
                Ok(())
            },
        );

        fn fill(mut img: DynImageMut, x: u32, y: u32, w: u32, h: u32, col: Rgba8) {
            match img.deref_mut() {
                DynImage::Grey8(img) => _ = img.view_mut(x, y, w, h).fill(col.lua_to_grey()),
                DynImage::Grey16(img) => _ = img.view_mut(x, y, w, h).fill(col.lua_to_grey()),
                DynImage::Grey32F(img) => _ = img.view_mut(x, y, w, h).fill(col.lua_to_grey()),
                DynImage::GreyAlpha8(img) => {
                    _ = img.view_mut(x, y, w, h).fill(col.lua_to_grey_alpha())
                }
                DynImage::GreyAlpha16(img) => {
                    _ = img.view_mut(x, y, w, h).fill(col.lua_to_grey_alpha())
                }
                DynImage::GreyAlpha32F(img) => {
                    _ = img.view_mut(x, y, w, h).fill(col.lua_to_grey_alpha())
                }
                DynImage::Rgb8(img) => _ = img.view_mut(x, y, w, h).fill(col.lua_to_rgb()),
                DynImage::Rgb16(img) => _ = img.view_mut(x, y, w, h).fill(col.lua_to_rgb()),
                DynImage::Rgb32F(img) => _ = img.view_mut(x, y, w, h).fill(col.lua_to_rgb()),
                DynImage::Rgba8(img) => _ = img.view_mut(x, y, w, h).fill(col),
                DynImage::Rgba16(img) => _ = img.view_mut(x, y, w, h).fill(col.to_rgba()),
                DynImage::Rgba32F(img) => _ = img.view_mut(x, y, w, h).fill(col.to_rgba()),
            }
        }

        methods.add_function("fill", |_, (this, col): (DynImageMut, Rgba8)| {
            let (w, h) = this.size().into();
            fill(this, 0, 0, w, h, col);
            Ok(())
        });

        methods.add_function(
            "fill_rect",
            |_, (this, rect, col): (DynImageMut, RectF, Rgba8)| {
                let Rect { x, y, w, h } = rect.to_u32();
                fill(this, x, y, w, h, col);
                Ok(())
            },
        );

        methods.add_function(
            "fill_at",
            |_, (this, x, y, w, h, col): (DynImageMut, u32, u32, u32, u32, Rgba8)| {
                fill(this, x, y, w, h, col);
                Ok(())
            },
        );

        fn draw(
            mut dst: DynImageMut,
            src: DynImageRef,
            sx: u32,
            sy: u32,
            w: u32,
            h: u32,
            dx: u32,
            dy: u32,
        ) {
            macro_rules! draw {
                ($($dst:ident = $f:ident),*) => {
                    match dst.deref_mut() {
                        $(
                        DynImage::$dst(dst) => match src.deref() {
                            DynImage::Grey8(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Grey16(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Grey32F(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::GreyAlpha8(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::GreyAlpha16(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::GreyAlpha32F(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Rgb8(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Rgb16(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Rgb32F(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Rgba8(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Rgba16(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                            DynImage::Rgba32F(src) => {
                                _ = dst
                                    .view_mut(dx, dy, w, h)
                                    .draw_mapped(&src.view(sx, sy, w, w), |c| c.$f())
                            }
                        }
                        )*
                    }
                }
            }
            draw!(
                Grey8 = lua_to_grey,
                Grey16 = lua_to_grey,
                Grey32F = lua_to_grey,
                GreyAlpha8 = lua_to_grey_alpha,
                GreyAlpha16 = lua_to_grey_alpha,
                GreyAlpha32F = lua_to_grey_alpha,
                Rgb8 = lua_to_rgb,
                Rgb16 = lua_to_rgb,
                Rgb32F = lua_to_rgb,
                Rgba8 = to_rgba,
                Rgba16 = to_rgba,
                Rgba32F = to_rgba
            );
        }

        methods.add_function(
            "draw",
            |_, (this, src, x, y): (DynImageMut, DynImageRef, u32, u32)| {
                let (w, h) = src.size().into();
                draw(this, src, 0, 0, w, h, x, y);
                Ok(())
            },
        );

        methods.add_function(
            "draw_part",
            |_,
             (this, src, sx, sy, w, h, dx, dy): (
                DynImageMut,
                DynImageRef,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )| {
                draw(this, src, sx, sy, w, h, dx, dy);
                Ok(())
            },
        );

        methods.add_function(
            "sub_image",
            |_, (this, x, y, w, h): (DynImageRef, u32, u32, u32, u32)| {
                Ok(match this.deref() {
                    DynImage::Grey8(img) => {
                        DynImage::Grey8(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Grey16(img) => {
                        DynImage::Grey16(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Grey32F(img) => {
                        DynImage::Grey32F(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::GreyAlpha8(img) => {
                        DynImage::GreyAlpha8(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::GreyAlpha16(img) => {
                        DynImage::GreyAlpha16(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::GreyAlpha32F(img) => {
                        DynImage::GreyAlpha32F(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Rgb8(img) => DynImage::Rgb8(Image::from_grid(&img.view(x, y, w, h))),
                    DynImage::Rgb16(img) => {
                        DynImage::Rgb16(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Rgb32F(img) => {
                        DynImage::Rgb32F(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Rgba8(img) => {
                        DynImage::Rgba8(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Rgba16(img) => {
                        DynImage::Rgba16(Image::from_grid(&img.view(x, y, w, h)))
                    }
                    DynImage::Rgba32F(img) => {
                        DynImage::Rgba32F(Image::from_grid(&img.view(x, y, w, h)))
                    }
                })
            },
        );
    }
}

impl ImageFormat {
    pub fn lua_str(&self) -> &'static str {
        match self {
            Self::Grey8 => "grey8",
            Self::Grey16 => "grey16",
            Self::Grey32F => "grey32f",
            Self::GreyAlpha8 => "grey_alpha8",
            Self::GreyAlpha16 => "grey_alpha16",
            Self::GreyAlpha32F => "grey_alpha32f",
            Self::Rgb8 => "rgb8",
            Self::Rgb16 => "rgb16",
            Self::Rgb32F => "rgb32f",
            Self::Rgba8 => "rgba8",
            Self::Rgba16 => "rgba16",
            Self::Rgba32F => "rgba32f",
        }
    }
}

impl FromLua for ImageFormat {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let s = BorrowedStr::from_lua(value, lua)?;
        Ok(match s.as_ref() {
            "grey8" => Self::Grey8,
            "grey16" => Self::Grey16,
            "grey32f" => Self::Grey32F,
            "grey_alpha8" => Self::GreyAlpha8,
            "grey_alpha16" => Self::GreyAlpha16,
            "grey_alpha32f" => Self::GreyAlpha32F,
            "rgb8" => Self::Rgb8,
            "rgb16" => Self::Rgb16,
            "rgb32f" => Self::Rgb32F,
            "rgba8" => Self::Rgba8,
            "rgba16" => Self::Rgba16,
            "rgba32f" => Self::Rgba32F,
            s => return Err(LuaError::runtime(format!("invalid image format [{s}]"))),
        })
    }
}

impl IntoLua for ImageFormat {
    #[inline]
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        self.lua_str().into_lua(lua)
    }
}

trait LuaTo<T> {
    fn lua_to_grey(self) -> Grey<T>;
    fn lua_to_grey_alpha(self) -> GreyAlpha<T>;
    fn lua_to_rgb(self) -> Rgb<T>;
}

impl<F: Channel, T: Channel> LuaTo<T> for Grey<F> {
    #[inline]
    fn lua_to_grey(self) -> Grey<T> {
        self.to_grey()
    }

    #[inline]
    fn lua_to_grey_alpha(self) -> GreyAlpha<T> {
        self.to_grey_alpha()
    }

    #[inline]
    fn lua_to_rgb(self) -> Rgb<T> {
        self.to_rgb()
    }
}

impl<F: Channel, T: Channel> LuaTo<T> for GreyAlpha<F> {
    #[inline]
    fn lua_to_grey(self) -> Grey<T> {
        grey(self.g.to_channel())
    }

    #[inline]
    fn lua_to_grey_alpha(self) -> GreyAlpha<T> {
        self.to_grey_alpha()
    }

    #[inline]
    fn lua_to_rgb(self) -> Rgb<T> {
        let g = self.g.to_channel();
        Rgb::new(g, g, g)
    }
}

impl<F: Channel, T: Channel> LuaTo<T> for Rgb<F> {
    #[inline]
    fn lua_to_grey(self) -> Grey<T> {
        grey(self.r.to_channel())
    }

    #[inline]
    fn lua_to_grey_alpha(self) -> GreyAlpha<T> {
        grey_alpha(self.g.to_channel(), T::CHANNEL_MAX)
    }

    #[inline]
    fn lua_to_rgb(self) -> Rgb<T> {
        self.to_rgb()
    }
}

impl<F: Channel, T: Channel> LuaTo<T> for Rgba<F> {
    #[inline]
    fn lua_to_grey(self) -> Grey<T> {
        grey(self.r.to_channel())
    }

    #[inline]
    fn lua_to_grey_alpha(self) -> GreyAlpha<T> {
        grey_alpha(self.g.to_channel(), self.a.to_channel())
    }

    #[inline]
    fn lua_to_rgb(self) -> Rgb<T> {
        let Rgba { r, g, b, .. } = self.to_rgba();
        Rgb::new(r, g, b)
    }
}

//
// impl<C: Channel> GreyAlpha<C> {
//     #[inline]
//     fn to_grey<T: Channel>(self) -> Grey<T> {
//         grey::<T>(self.g.to_channel())
//     }
//
//     #[inline]
//     fn to_rgb<T: Channel>(self) -> Rgb<T> {
//         self.to_grey::<T>().to_rgb()
//     }
// }
//
// impl<C: Channel> Rgb<C> {
//     #[inline]
//     fn to_grey<T: Channel>(self) -> Grey<T> {
//         grey::<T>(self.r.to_channel())
//     }
//
//     #[inline]
//     fn to_grey_alpha<T: Channel>(self) -> GreyAlpha<T> {
//         grey_alpha::<T>(self.r.to_channel(), T::CHANNEL_MAX)
//     }
//
//     #[inline]
//     fn to_rgb<T: Channel>(self) -> Rgb<T> {
//         rgb::<T>(
//             self.r.to_channel(),
//             self.g.to_channel(),
//             self.b.to_channel(),
//         )
//     }
// }
//
// impl<C: Channel> Rgba<C> {
//     #[inline]
//     fn to_grey<T: Channel>(self) -> Grey<T> {
//         grey::<T>(self.r.to_channel())
//     }
//
//     #[inline]
//     fn to_grey_alpha<T: Channel>(self) -> GreyAlpha<T> {
//         grey_alpha::<T>(self.r.to_channel(), self.a.to_channel())
//     }
//
//     #[inline]
//     fn to_rgb<T: Channel>(self) -> Rgb<T> {
//         rgb::<T>(
//             self.r.to_channel(),
//             self.g.to_channel(),
//             self.b.to_channel(),
//         )
//     }
// }
