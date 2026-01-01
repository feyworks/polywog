use crate::Pixel;
use bytemuck::Zeroable;
use fey_color::{
    Grey8, Grey16, Grey32F, GreyAlpha8, GreyAlpha16, GreyAlpha32F, Rgb8, Rgb16, Rgb32F, Rgba8,
    Rgba16, Rgba32F, ToRgba,
};
use fey_grid::{Grid, GridMut};
use fey_math::{Numeric, Vec2U, vec2};
use std::fmt::Debug;
use std::marker::PhantomData;

/// An image with the specified pixel format and storage.
#[derive(Debug, Clone)]
pub struct Image<Px: Pixel, S = Vec<<Px as Pixel>::Channel>> {
    size: Vec2U,
    store: S,
    marker: PhantomData<Px>,
}

/// An 8-bit greyscale image.
pub type ImageGrey8<S = Vec<u8>> = Image<Grey8, S>;

/// A 16-bit greyscale image.
pub type ImageGrey16<S = Vec<u16>> = Image<Grey16, S>;

/// A 32-bit floating point greyscale image.
pub type ImageGrey32F<S = Vec<f32>> = Image<Grey32F, S>;

/// An image with 8-bit greyscale and alpha channels.
pub type ImageGreyAlpha8<S = Vec<u8>> = Image<GreyAlpha8, S>;

/// An image with 16-bit greyscale and alpha channels.
pub type ImageGreyAlpha16<S = Vec<u16>> = Image<GreyAlpha16, S>;

/// An image with 32-bit floating point greyscale and alpha channels.
pub type ImageGreyAlpha32F<S = Vec<f32>> = Image<GreyAlpha32F, S>;

/// An 8-bit RGB image.
pub type ImageRgb8<S = Vec<u8>> = Image<Rgb8, S>;

/// An 16-bit RGB image.
pub type ImageRgb16<S = Vec<u16>> = Image<Rgb16, S>;

/// An 32-bit floating point RGB image.
pub type ImageRgb32F<S = Vec<f32>> = Image<Rgb32F, S>;

/// An 8-bit RGBA image.
pub type ImageRgba8<S = Vec<u8>> = Image<Rgba8, S>;

/// An 16-bit RGBA image.
pub type ImageRgba16<S = Vec<u16>> = Image<Rgba16, S>;

/// An 32-bit floating point RGBA image.
pub type ImageRgba32F<S = Vec<f32>> = Image<Rgba32F, S>;

impl<Px: Pixel, S> Image<Px, S> {
    #[inline]
    fn store_len(size: Vec2U) -> Option<usize> {
        let size = size.to_usize();
        size.x
            .checked_mul(size.y)
            .and_then(|len| len.checked_mul(Px::NUM_CHANNELS))
    }

    #[inline]
    pub fn from_raw(size: impl Into<Vec2U>, store: S) -> Self
    where
        S: AsRef<[Px::Channel]>,
    {
        let size = size.into();
        assert_eq!(Self::store_len(size), Some(store.as_ref().len()));
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn pixels(&self) -> &[Px]
    where
        S: AsRef<[Px::Channel]>,
    {
        bytemuck::cast_slice(self.store.as_ref())
    }

    #[inline]
    pub fn pixels_mut(&mut self) -> &mut [Px]
    where
        S: AsMut<[Px::Channel]>,
    {
        bytemuck::cast_slice_mut(self.store.as_mut())
    }

    #[inline]
    pub fn channels(&self) -> &[Px::Channel]
    where
        S: AsRef<[Px::Channel]>,
    {
        self.store.as_ref()
    }

    #[inline]
    pub fn channels_mut(&mut self) -> &mut [Px::Channel]
    where
        S: AsMut<[Px::Channel]>,
    {
        self.store.as_mut()
    }

    #[inline]
    pub fn bytes(&self) -> &[u8]
    where
        S: AsRef<[Px::Channel]>,
    {
        bytemuck::cast_slice(self.store.as_ref())
    }

    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8]
    where
        S: AsMut<[Px::Channel]>,
    {
        bytemuck::cast_slice_mut(self.store.as_mut())
    }

    #[inline]
    pub fn to_store(self) -> S {
        self.store
    }

    #[inline]
    pub fn premultiply(&mut self)
    where
        S: AsMut<[Px::Channel]>,
    {
        for p in self.pixels_mut() {
            *p = p.premultiply();
        }
    }
}

impl<Px: Pixel> Image<Px, Vec<Px::Channel>> {
    #[inline]
    pub fn new_vec_with<F: FnMut() -> Px>(size: impl Into<Vec2U>, mut fill: F) -> Self {
        let size = size.into();
        let len = Self::store_len(size).expect("image capacity overflow");
        let mut store = Vec::with_capacity(len);
        for _ in 0..(size.x * size.y) {
            store.extend_from_slice(fill().as_slice());
        }
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn new_vec(size: impl Into<Vec2U>, fill: Px) -> Self {
        let size = size.into();
        let len = Self::store_len(size).expect("image capacity overflow");
        let mut store = Vec::with_capacity(len);
        let fill = fill.as_slice();
        for _ in 0..(size.x * size.y) {
            store.extend_from_slice(fill);
        }
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn new_mapped<F: FnMut(Vec2U) -> Px>(size: impl Into<Vec2U>, mut fill: F) -> Self {
        let size = size.into();
        let len = Self::store_len(size).expect("image capacity overflow");
        let mut store = Vec::with_capacity(len);
        for y in 0..size.y {
            for x in 0..size.x {
                store.extend_from_slice(fill(vec2(x, y)).as_slice());
            }
        }
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn map<Px2: Pixel, F: FnMut(Px) -> Px2>(&self, map: F) -> Image<Px2, Vec<Px2::Channel>> {
        let mut pixels = self.pixels().iter().copied().map(map);
        Image::<Px2, Vec<Px2::Channel>>::new_vec_with(self.size, || pixels.next().unwrap())
    }

    #[inline]
    pub fn to_rgba8(&self) -> ImageRgba8
    where
        Px: ToRgba<u8>,
    {
        self.map(Px::to_rgba)
    }

    #[inline]
    pub fn from_grid<G: Grid<Item = Px>>(grid: &G) -> Self {
        let len = Self::store_len(grid.size()).unwrap();
        let mut vec = Vec::with_capacity(len);
        for row in grid.rows() {
            if let Some(row) = row.as_slice() {
                vec.extend_from_slice(bytemuck::cast_slice(row));
            } else {
                for p in row.iter().map(Px::as_slice) {
                    vec.extend_from_slice(p);
                }
            }
        }
        Self::from_raw(grid.size(), vec)
    }
}

impl<Px: Pixel, const N: usize> Image<Px, [Px::Channel; N]> {
    #[inline]
    pub fn new_arr_with<F: FnMut() -> Px>(size: impl Into<Vec2U>, fill: F) -> Self {
        let mut img = Self::from_raw(size, std::array::from_fn(|_| Zeroable::zeroed()));
        img.pixels_mut().fill_with(fill);
        img
    }

    #[inline]
    pub fn new_arr(size: impl Into<Vec2U>, fill: Px) -> Self {
        let mut img = Self::from_raw(size, std::array::from_fn(|_| Zeroable::zeroed()));
        img.pixels_mut().fill(fill);
        img
    }
}

impl<'a, Px: Pixel> Image<Px, &'a [Px::Channel]> {
    #[inline]
    pub fn new_slice(size: impl Into<Vec2U>, slice: &'a [Px::Channel]) -> Self {
        Self::from_raw(size, slice)
    }

    #[inline]
    pub fn to_owned(&self) -> Image<Px, Vec<Px::Channel>> {
        Image::<Px, Vec<Px::Channel>>::from_grid(self)
    }
}

impl<'a, Px: Pixel> Image<Px, &'a mut [Px::Channel]> {
    #[inline]
    pub fn new_mut_slice(size: impl Into<Vec2U>, slice: &'a mut [Px::Channel]) -> Self {
        Self::from_raw(size, slice)
    }

    #[inline]
    pub fn to_owned(&self) -> Image<Px, Vec<Px::Channel>> {
        Image::<Px, Vec<Px::Channel>>::from_grid(self)
    }
}

impl<Px: Pixel, S: AsRef<[Px::Channel]>> Grid for Image<Px, S> {
    type Item = Px;
    type Root = Self;

    #[inline]
    fn root(&self) -> &Self::Root {
        self
    }

    #[inline]
    fn root_x(&self) -> u32 {
        0
    }

    #[inline]
    fn root_y(&self) -> u32 {
        0
    }

    #[inline]
    fn width(&self) -> u32 {
        self.size.x
    }

    #[inline]
    fn height(&self) -> u32 {
        self.size.y
    }

    #[inline]
    fn size(&self) -> Vec2U {
        self.size
    }

    #[inline]
    fn get(&self, x: u32, y: u32) -> Option<&Self::Item> {
        self.pixels()
            .get(y.checked_mul(self.size.x)?.checked_add(x)? as usize)
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Self::Item {
        unsafe {
            self.pixels()
                .get_unchecked(y.unchecked_mul(self.size.x).unchecked_add(x) as usize)
        }
    }

    #[inline]
    fn row_slice(&self, y: u32) -> Option<&[Self::Item]> {
        let i = y.checked_mul(self.size.x)?;
        self.pixels().get({
            let i = i as usize;
            let len = self.size.x as usize;
            i..(i + len)
        })
    }
}

impl<Px: Pixel, S: AsRef<[Px::Channel]> + AsMut<[Px::Channel]>> GridMut for Image<Px, S> {
    type RootMut = Self;

    #[inline]
    fn root_mut(&mut self) -> &mut Self::RootMut {
        self
    }

    #[inline]
    fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Item> {
        let i = y.checked_mul(self.size.x)?.checked_add(x)?;
        self.pixels_mut().get_mut(i as usize)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: u32, y: u32) -> &mut Self::Item {
        unsafe {
            let i = y.unchecked_mul(self.size.x).unchecked_add(x);
            self.pixels_mut().get_unchecked_mut(i as usize)
        }
    }

    #[inline]
    fn row_slice_mut(&mut self, y: u32) -> Option<&mut [Self::Item]> {
        let i = y.checked_mul(self.size.x)? as usize;
        let w = self.size.x as usize;
        self.pixels_mut().get_mut(i..(i + w))
    }
}
