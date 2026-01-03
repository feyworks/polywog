use crate::color::Rgba8;
use crate::gfx::{Graphics, SubTexture, Texture};
use crate::grid::{Grid, GridMut};
use crate::img::ImageRgba8;
use crate::math::{Numeric, RectF, RectU, Vec2F, Vec2U};
use fey_packer::{Item, Packed, RectPacker};
use std::ops::Deref;
use std::rc::Rc;

pub struct TexturePacker<'a> {
    to_pack: Vec<ToPack<'a>>,
}

struct ToPack<'a> {
    img: PackImage<'a>,
    src_rect: RectU,
    trim_rect: RectU,
}

impl<'a> TexturePacker<'a> {
    pub fn new() -> Self {
        Self {
            to_pack: Vec::new(),
        }
    }

    pub fn add_image(
        &mut self,
        img: impl Into<PackImage<'a>>,
        src_rect: impl Into<Option<RectU>>,
        trim_threshold: impl Into<Option<u8>>,
    ) {
        let img = img.into();
        let src_rect = src_rect.into().unwrap_or(RectU::sized(img.size()));
        let src = img.view_at(src_rect);
        let trim_rect = trim_threshold
            .into()
            .and_then(|a| src.get_bounds(|p| p.a > a))
            .unwrap_or_else(|| RectU::sized(src_rect.size()));
        self.to_pack.push(ToPack {
            img,
            src_rect,
            trim_rect,
        });
    }

    pub fn pack(self, gfx: &Graphics) -> Option<(Texture, Vec<SubTexture>)> {
        self.pack_ext(gfx, gfx.max_texture_size(), 1, 2)
    }

    pub fn pack_ext(
        self,
        gfx: &Graphics,
        max_size: u32,
        spacing: u32,
        padding: u32,
    ) -> Option<(Texture, Vec<SubTexture>)> {
        let padding = Vec2U::splat(padding);

        let items: Vec<Item<usize>> = self
            .to_pack
            .iter()
            .enumerate()
            .map(|(i, item)| Item::new(item.trim_rect.size() + padding, i))
            .collect();

        let (size, mut packed) = RectPacker::new()
            .with_max_size(max_size)
            .with_spacing(spacing)
            .pack(items)?;
        packed.sort_by_key(|i| i.data);

        let mut tex_img = ImageRgba8::new_vec(size, Rgba8::TRANSPARENT);

        let padding = padding.to_f32();
        let sub_info: Vec<(RectF, Vec2F, Vec2F)> = packed
            .into_iter()
            .map(|Packed { data: i, pos }| {
                let ToPack {
                    img,
                    src_rect,
                    trim_rect,
                } = &self.to_pack[i];
                let src = img.view_at(*trim_rect + src_rect.top_left());

                let dst_rect = RectU::pos_size(pos, trim_rect.size());
                let mut dst = tex_img.view_mut_at(dst_rect);
                dst.draw_copied(&src);
                (
                    dst_rect.to_f32().inflate(padding),
                    trim_rect.top_left().to_f32() - padding,
                    src_rect.size().to_f32(),
                )
            })
            .collect();

        let tex = gfx.create_texture_from_img(&tex_img);
        let subs = sub_info
            .into_iter()
            .map(|(rect, offset, size)| SubTexture::new_ext(tex.clone(), rect, offset, size))
            .collect();

        Some((tex, subs))
    }
}

#[derive(Debug, Clone)]
pub enum PackImage<'a> {
    Ref(&'a ImageRgba8),
    Owned(ImageRgba8),
    Shared(Rc<ImageRgba8>),
}

impl Deref for PackImage<'_> {
    type Target = ImageRgba8;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(r) => *r,
            Self::Owned(r) => r,
            Self::Shared(r) => &r,
        }
    }
}

impl From<ImageRgba8> for PackImage<'_> {
    #[inline]
    fn from(value: ImageRgba8) -> Self {
        Self::Owned(value)
    }
}

impl<'a> From<&'a ImageRgba8> for PackImage<'a> {
    #[inline]
    fn from(value: &'a ImageRgba8) -> Self {
        Self::Ref(value)
    }
}

impl<'a> From<&'a mut ImageRgba8> for PackImage<'a> {
    #[inline]
    fn from(value: &'a mut ImageRgba8) -> Self {
        Self::Ref(value)
    }
}

impl From<Rc<ImageRgba8>> for PackImage<'_> {
    #[inline]
    fn from(value: Rc<ImageRgba8>) -> Self {
        Self::Shared(value)
    }
}

impl From<&Rc<ImageRgba8>> for PackImage<'_> {
    #[inline]
    fn from(value: &Rc<ImageRgba8>) -> Self {
        Self::Shared(value.clone())
    }
}
