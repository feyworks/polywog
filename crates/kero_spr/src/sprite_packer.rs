use crate::{
    AnimCel, AnimFrame, AnimLayer, AnimTag, AtlasAnim, AtlasCel, AtlasFont, AtlasGlyph,
    AtlasGraphicsMapped, AtlasPatch, AtlasSheet, AtlasSprite, AtlasTile, SpriteAtlas,
};
use fey_ase::{Ase, CelType, Format};
use fey_font::{Font as FeyFont, FontError};
use fey_packer::{Item, Packed, RectPacker};
use fnv::FnvHashMap;
use kero::prelude::*;
use std::ffi::OsStr;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

/// Packs sprites, sheets, fonts, etc. into an atlas.
pub struct SpritePacker<I> {
    images: Vec<ImageData>,
    image_hashes: FnvHashMap<u64, usize>,
    sprites: Vec<PackSprite<I>>,
    sheets: Vec<PackSheet<I>>,
    fonts: Vec<PackFont<I>>,
    patches: Vec<PackPatch<I>>,
    anims: Vec<PackAnim<I>>,
}

impl<I: Hash + Eq> SpritePacker<I> {
    /// Create a new packer.
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            image_hashes: FnvHashMap::default(),
            sprites: Vec::new(),
            sheets: Vec::new(),
            fonts: Vec::new(),
            patches: Vec::new(),
            anims: Vec::new(),
        }
    }

    fn add_image(
        &mut self,
        img: ImageRgba8,
        trim_threshold: Option<u8>,
        offset: Vec2I,
    ) -> Option<PackImage> {
        let trim = match trim_threshold {
            Some(a) => img.get_bounds(|p| p.a > a),
            None => img
                .pixels()
                .iter()
                .any(|p| p.a > 0)
                .then(|| Rect::sized(img.size())),
        }?;
        let offset = -(offset - trim.top_left().to_i32());
        let orig_size = img.size();
        let hash = {
            let mut hasher = DefaultHasher::new();
            img.view_at(trim).hash_grid(&mut hasher);
            hasher.finish()
        };
        let img_data = *self.image_hashes.entry(hash).or_insert_with(|| {
            self.images.push(ImageData { img, trim });
            self.images.len() - 1
        });
        Some(PackImage {
            img_data,
            orig_size,
            offset,
        })
    }

    /// Add a sprite (a single image) to be packed.
    pub fn add_sprite(&mut self, id: I, img: ImageRgba8, trim_threshold: Option<u8>) {
        let img = self.add_image(img, trim_threshold, Vec2::ZERO);
        self.sprites.push(PackSprite { id, img });
    }

    /// Add a sprite (a single image) to be packed from a PNG/QOI file.
    pub fn add_sprite_file(
        &mut self,
        id: I,
        path: impl AsRef<Path>,
        premultiply: bool,
        trim_threshold: Option<u8>,
    ) -> Result<(), ImageError> {
        let mut img = DynImage::load_file(path)?.to_rgba8();
        if premultiply {
            img.premultiply();
        }
        self.add_sprite(id, img, trim_threshold);
        Ok(())
    }

    /// Add a tile sheet to be packed. The sheet will be split up and tiles will be
    /// individually packed in order to fit them in better.
    pub fn add_sheet(
        &mut self,
        id: I,
        img: ImageRgba8,
        tile_size: impl Into<Vec2U>,
        trim_threshold: Option<u8>,
    ) {
        let tile_size = tile_size.into();
        if (img.size() / tile_size) * tile_size != img.size() {
            println!(
                "img size {} is not multiple of tile size {}",
                img.size(),
                tile_size
            );
        }
        let mut tiles = VecGrid::new(img.size() / tile_size);
        for (val, p) in tiles.iter_mut() {
            let sub = ImageRgba8::from_grid(&img.view(
                p.x * tile_size.x,
                p.y * tile_size.y,
                tile_size.x,
                tile_size.y,
            ));
            *val = self.add_image(sub, trim_threshold, Vec2::ZERO);
        }
        self.sheets.push(PackSheet {
            id,
            tile_size,
            tiles,
        });
    }

    /// Add a tile sheet to be packed from a PNG/QOI file. The sheet will be split up and tiles will
    /// be individually packed in order to fit them in better.
    pub fn add_sheet_file(
        &mut self,
        id: I,
        path: impl AsRef<Path>,
        premultiply: bool,
        tile_size: impl Into<Vec2U>,
        trim_threshold: Option<u8>,
    ) -> Result<(), ImageError> {
        let mut img = DynImage::load_file(path)?.to_rgba8();
        if premultiply {
            img.premultiply();
        }
        self.add_sheet(id, img, tile_size, trim_threshold);
        Ok(())
    }

    /// Add a font to be packed. Each glyph will be packed individually.
    pub fn add_font(&mut self, id: I, font: &FeyFont, chars: impl IntoIterator<Item = char>) {
        let chars: Vec<char> = chars.into_iter().collect();

        let glyphs = chars
            .iter()
            .copied()
            .chain(['\0'])
            .map(|chr| {
                let g = font.char_glyph(chr);
                (
                    chr,
                    PackGlyph {
                        img: g
                            .rasterize(|a| Rgba8::splat(a.to_channel::<u8>()))
                            .and_then(|r| {
                                let offset = r.offset + vec2(-g.left_side_bearing(), 0.0);
                                self.add_image(r.image, None, offset.map(f32::round).to_i32())
                            }),
                        adv: g.advance().round() as i32,
                    },
                )
            })
            .collect();

        let kerning: Vec<(char, char, i32)> = chars
            .iter()
            .copied()
            .zip(chars.iter().copied())
            .map(|(left, right)| (left, right, font.char_kerning(left, right) as i32))
            .filter(|(_, _, k)| *k != 0)
            .collect();

        self.fonts.push(PackFont {
            id,
            ascent: font.ascent().round() as i32,
            descent: font.descent().round() as i32,
            line_gap: font.line_gap().round() as i32,
            glyphs,
            kerning,
        });
    }

    /// Add a font to be packed from a TTF/OTF file. Each glyph will be packed individually.
    pub fn add_font_file(
        &mut self,
        id: I,
        path: impl AsRef<Path>,
        size: f32,
        chars: impl IntoIterator<Item = char>,
    ) -> Result<(), FontError> {
        let font = FeyFont::from_file(path, size)?;
        self.add_font(id, &font, chars);
        Ok(())
    }

    /// Add a 9-patch to be packed.
    pub fn add_patch(&mut self, id: I, img: ImageRgba8, inner: impl Into<RectU>) {
        let img = self.add_image(img, None, Vec2::ZERO);
        let inner = inner.into();
        self.patches.push(PackPatch { id, img, inner });
    }

    /// Add a 9-patch to be packed from a PNG/QOI file.
    pub fn add_patch_file(
        &mut self,
        id: I,
        path: impl AsRef<Path>,
        premultiply: bool,
        inner: impl Into<RectU>,
    ) -> Result<(), ImageError> {
        let mut img = DynImage::load_file(path)?.to_rgba8();
        if premultiply {
            img.premultiply();
        }
        self.add_patch(id, img, inner);
        Ok(())
    }

    /// Add an aseprite animation to be packed. The individual cels of the animation
    /// will be packed individually to better fit them into the atlas.
    pub fn add_ase(&mut self, id: I, ase: &Ase) {
        let make_img = |size: Vec2<usize>, data: &[u8]| match ase.format {
            Format::Rgba => ImageRgba8::new_slice(size.to_u32(), data).to_owned(),
            Format::Grayscale => {
                assert_eq!(data.len(), size.x * size.y * 2);
                ImageRgba8::new_mapped(size.to_u32(), |p| {
                    let p = p.to_usize();
                    let i = (p.y * size.x + p.x) * 2;
                    Rgba8::new(data[i], data[i], data[i], data[i + 1])
                })
            }
            Format::Indexed { transparent_index } => {
                assert_eq!(data.len(), size.x * size.y);
                ImageRgba8::new_mapped(size.to_u32(), |p| {
                    let p = p.to_usize();
                    let i = p.y * size.x + p.x;
                    if data[i] == transparent_index {
                        Rgba8::TRANSPARENT
                    } else {
                        ase.palette[data[i] as usize]
                    }
                })
            }
        };

        let mut images = Vec::new();
        let mut img_lookup = FnvHashMap::default();

        let frames = ase
            .frames
            .iter()
            .enumerate()
            .map(|(frame_index, frame)| {
                let cels = frame
                    .cels
                    .iter()
                    .filter(|cel| !ase.layers[cel.layer_index].name.starts_with('_'))
                    .map(|cel| {
                        let index = match &cel.ty {
                            // if it's a linked cel, use the image from another frame on this layer
                            CelType::Link { frame_index } => *img_lookup
                                .get(&(*frame_index as usize, cel.layer_index))
                                .unwrap(),

                            // if it's an image cel, add a new image and store the frame/layer pair
                            CelType::Image { size, data } => {
                                let img_index = images.len();
                                img_lookup.insert((frame_index, cel.layer_index), img_index);
                                let mut img = make_img(size.to_usize(), data.as_slice());
                                let opacity = ase.layers[cel.layer_index].opacity;
                                if opacity < u8::MAX {
                                    for p in img.pixels_mut() {
                                        *p = p.un_mul(opacity);
                                    }
                                }
                                images.push(self.add_image(img, None, -cel.pos.to_i32()).unwrap());
                                img_index
                            }
                        };
                        AnimCel {
                            layer: cel.layer_index,
                            index,
                        }
                    })
                    .collect();
                AnimFrame {
                    duration: (frame.duration as f32) / 1000.0,
                    cels,
                }
            })
            .collect();

        let tags = ase
            .tags
            .iter()
            .map(|t| AnimTag {
                name: t.name.clone(),
                from: t.from as usize,
                to: t.to as usize,
                dir: t.loop_dir.into(),
            })
            .collect();

        let layers = ase
            .layers
            .iter()
            .map(|l| AnimLayer {
                opacity: (l.opacity as f32) / 255.0,
                name: l.name.clone(),
                group: l.group,
                level: l.level,
            })
            .collect();

        self.anims.push(PackAnim {
            id,
            size: ase.size,
            images,
            frames,
            tags,
            layers,
        });
    }

    /// Add an aseprite animation to be packed from a file. The individual cels of the animation
    /// will be packed individually to better fit them into the atlas.
    pub fn add_ase_file(&mut self, id: I, path: impl AsRef<Path>) -> Result<(), GameError> {
        let ase = Ase::from_file(path).map_err(GameError::custom)?;
        self.add_ase(id, &ase);
        Ok(())
    }

    /// Pack all the items into a sprite atlas.
    pub fn pack_graphics(
        &mut self,
        max_size: u32,
        gfx: &Graphics,
    ) -> Result<AtlasGraphicsMapped<I>, GameError> {
        let (img, atlas) = self
            .pack_atlas(max_size)
            .ok_or_else(|| GameError::custom("failed to pack atlas"))?;
        let tex = gfx.create_texture_from_img(&img);
        Ok(atlas.create_graphics(tex).mapped())
    }

    /// Pack all the items into a sprite atlas.
    pub fn pack_atlas(&mut self, max_size: u32) -> Option<(ImageRgba8, SpriteAtlas<I>)> {
        let (size, mut packed) = RectPacker::new()
            .with_max_size(max_size)
            .with_spacing(1)
            .with_padding(2)
            .with_power_of_two()
            .pack(
                self.images
                    .iter()
                    .enumerate()
                    .map(|(i, img)| Item::new(img.trim.size(), i))
                    .collect(),
            )?;
        packed.sort_by_key(|p| p.data);

        let mut image = ImageRgba8::new_vec(size, Rgba8::TRANSPARENT);
        for &Packed { data, pos } in &packed {
            let src = self.images[data].view();
            let mut dst = image.view_mut(pos.x, pos.y, src.width(), src.height());
            dst.draw_copied(&src);
        }

        let img_data = |img: PackImage| {
            let size = self.images[img.img_data].trim.size();
            let pos = packed[img.img_data].pos;
            let rect = rect(pos.x, pos.y, size.x, size.y);
            (img.orig_size, rect, img.offset)
        };

        let sprites: Vec<AtlasSprite<I>> = self
            .sprites
            .drain(..)
            .flat_map(|spr| {
                spr.img.map(|img| {
                    let (size, rect, off) = img_data(img);
                    AtlasSprite {
                        id: spr.id,
                        size,
                        rect,
                        off,
                    }
                })
            })
            .collect();

        let sheets: Vec<AtlasSheet<I>> = self
            .sheets
            .drain(..)
            .map(|sheet| AtlasSheet {
                id: sheet.id,
                tile_size: sheet.tile_size,
                size: sheet.tiles.size(),
                tiles: sheet
                    .tiles
                    .to_store()
                    .into_iter()
                    .map(|img| {
                        img.map(|img| {
                            let (_, rect, off) = img_data(img);
                            AtlasTile { rect, off }
                        })
                    })
                    .collect(),
            })
            .collect();

        let fonts: Vec<AtlasFont<I>> = self
            .fonts
            .drain(..)
            .map(|font| {
                let glyphs = font
                    .glyphs
                    .into_iter()
                    .map(|(chr, g)| {
                        let (size, rect, off) = g
                            .img
                            .map(img_data)
                            .unwrap_or_else(|| (Vec2::ZERO, Rect::ZERO, Vec2::ZERO));
                        AtlasGlyph {
                            chr,
                            adv: g.adv,
                            size,
                            rect,
                            off,
                        }
                    })
                    .collect();
                AtlasFont {
                    id: font.id,
                    ascent: font.ascent,
                    descent: font.descent,
                    line_gap: font.line_gap,
                    glyphs,
                    kerning: font.kerning,
                }
            })
            .collect();

        let patches: Vec<AtlasPatch<I>> = self
            .patches
            .drain(..)
            .flat_map(|p| {
                p.img.map(|img| {
                    let size = self.images[img.img_data].trim.size();
                    let pos = packed[img.img_data].pos;
                    let outer = rect(pos.x, pos.y, size.x, size.y);
                    let inner = p.inner.translate(&pos);
                    AtlasPatch {
                        id: p.id,
                        outer,
                        inner,
                    }
                })
            })
            .collect();

        let anims: Vec<AtlasAnim<I>> = self
            .anims
            .drain(..)
            .map(|anim| {
                let cels = anim
                    .images
                    .into_iter()
                    .map(img_data)
                    .map(|(size, rect, off)| AtlasCel { size, rect, off })
                    .collect();
                AtlasAnim {
                    id: anim.id,
                    size: anim.size.to_u32(),
                    cels,
                    frames: anim.frames,
                    tags: anim.tags,
                    layers: anim.layers,
                }
            })
            .collect();

        Some((
            image,
            SpriteAtlas {
                sprites,
                sheets,
                fonts,
                patches,
                anims,
            },
        ))
    }
}

#[inline]
fn files(dir: impl AsRef<Path>) -> impl Iterator<Item = (PathBuf, String)> {
    std::fs::read_dir(dir).unwrap().flatten().map(|file| {
        let path = file.path();
        let id = path
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string();
        (path, id)
    })
}

impl SpritePacker<String> {
    pub fn add_sprite_files(
        &mut self,
        directory: impl AsRef<Path>,
        premultiply: bool,
        trim_threshold: Option<u8>,
    ) -> Result<(), ImageError> {
        for (file, name) in files(directory) {
            self.add_sprite_file(name, file, premultiply, trim_threshold)?;
        }
        Ok(())
    }

    pub fn add_sheet_files(
        &mut self,
        directory: impl AsRef<Path>,
        premultiply: bool,
        tile_size: impl Into<Vec2U>,
        trim_threshold: Option<u8>,
    ) -> Result<(), ImageError> {
        let tile_size = tile_size.into();
        for (file, name) in files(directory) {
            self.add_sheet_file(name, file, premultiply, tile_size, trim_threshold)?;
        }
        Ok(())
    }

    pub fn add_ase_files(&mut self, directory: impl AsRef<Path>) -> Result<(), GameError> {
        for (file, name) in files(directory) {
            self.add_ase_file(name, file)?;
        }
        Ok(())
    }

    pub fn add_font_files(
        &mut self,
        directory: impl AsRef<Path>,
        size: f32,
        chars: impl IntoIterator<Item = char> + Clone,
    ) -> Result<(), FontError> {
        for (file, name) in files(directory) {
            self.add_font_file(name, file, size, chars.clone())?;
        }
        Ok(())
    }

    pub fn add_patch_files(
        &mut self,
        directory: impl AsRef<Path>,
        premultiply: bool,
        inner: impl Into<RectU>,
    ) -> Result<(), ImageError> {
        let inner = inner.into();
        for (file, name) in files(directory) {
            self.add_patch_file(name, file, premultiply, inner)?;
        }
        Ok(())
    }
}

struct ImageData {
    img: ImageRgba8,
    trim: RectU,
}

impl ImageData {
    fn view(&self) -> View<&ImageRgba8> {
        self.img.view_at(self.trim)
    }
}

struct PackImage {
    img_data: usize,
    orig_size: Vec2U,
    offset: Vec2I,
}

struct PackSprite<I> {
    id: I,
    img: Option<PackImage>,
}

struct PackSheet<I> {
    id: I,
    tile_size: Vec2U,
    tiles: VecGrid<Option<PackImage>>,
}

struct PackFont<I> {
    id: I,
    ascent: i32,
    descent: i32,
    line_gap: i32,
    glyphs: FnvHashMap<char, PackGlyph>,
    kerning: Vec<(char, char, i32)>,
}

struct PackGlyph {
    img: Option<PackImage>,
    adv: i32,
}

struct PackPatch<I> {
    id: I,
    img: Option<PackImage>,
    inner: RectU,
}

struct PackAnim<I> {
    id: I,
    size: Vec2<u16>,
    images: Vec<PackImage>,
    frames: Vec<AnimFrame>,
    tags: Vec<AnimTag>,
    layers: Vec<AnimLayer>,
}
