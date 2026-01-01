use crate::{Coord, CoordComponent, Grid, GridIter, GridMut};
use fey_math::Vec2U;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// A grid implementation for different storage types.
pub struct GridBuf<T, S = Vec<T>> {
    pub(crate) size: Vec2U,
    pub(crate) store: S,
    pub(crate) marker: PhantomData<T>,
}

/// A grid implementation using a `Vec` for storage.
pub type VecGrid<T> = GridBuf<T, Vec<T>>;

/// A grid implementation using an array for storage.
pub type ArrGrid<T, const N: usize> = GridBuf<T, [T; N]>;

/// A grid implementation using a slice for storage.
pub type SliceGrid<'a, T> = GridBuf<T, &'a [T]>;

/// A grid implementation using a mutable slice for storage.
pub type MutSliceGrid<'a, T> = GridBuf<T, &'a mut [T]>;

impl<T, S> GridBuf<T, S> {
    /// Create a new grid using the provided storage. Panics if the length
    /// of `store` is not equal to `width * height`.
    #[inline]
    pub fn with_store(size: impl Into<Vec2U>, store: S) -> Self
    where
        S: AsRef<[T]>,
    {
        let size = size.into();
        assert_eq!(
            size.x.checked_mul(size.y),
            Some(store.as_ref().len() as u32)
        );
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    /// Get the contents of the grid as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T]
    where
        S: AsRef<[T]>,
    {
        self.store.as_ref()
    }

    /// Get the contents of the grid as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T]
    where
        S: AsMut<[T]>,
    {
        self.store.as_mut()
    }

    /// Drop the grid and return its storage.
    #[inline]
    pub fn to_store(self) -> S {
        self.store
    }
}

impl<T> VecGrid<T> {
    /// Create a new `VecGrid` filled with values from the provided function.
    #[inline]
    pub fn new_from(size: impl Into<Vec2U>, mut fill: impl FnMut(Vec2U) -> T) -> Self {
        let size = size.into();
        let len = size.x.checked_mul(size.y).expect("grid capacity overflow");
        let mut store = Vec::new();
        store.reserve(len as usize);
        for y in 0..size.y {
            for x in 0..size.x {
                store.push(fill(Vec2U::new(x, y)));
            }
        }
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    /// Create a new `VecGrid` filled with values from the provided function.
    #[inline]
    pub fn new_with<F: FnMut() -> T>(size: impl Into<Vec2U>, fill: F) -> Self {
        let size = size.into();
        let len = size.x.checked_mul(size.y).expect("grid capacity overflow");
        let mut store = Vec::new();
        store.resize_with(len as usize, fill);
        Self {
            size,
            store,
            marker: PhantomData,
        }
    }

    /// Create a new `VecGrid` fill with default values.
    #[inline]
    pub fn new(size: impl Into<Vec2U>) -> Self
    where
        T: Default,
    {
        Self::new_with(size, T::default)
    }

    /// Create a new `VecGrid` of size `(0, 0)`.
    #[inline]
    pub fn new_empty() -> Self {
        Self {
            size: Vec2U::ZERO,
            store: Vec::new(),
            marker: PhantomData,
        }
    }
}

impl<'a, T> SliceGrid<'a, T> {
    /// Create a new `SliceGrid` from the provided slice. Panics if the length of
    /// the slice is not exactly `width * height`.
    #[inline]
    pub fn new_slice(size: impl Into<Vec2U>, slice: &'a [T]) -> Self {
        Self::with_store(size, slice)
    }
}

impl<'a, T> MutSliceGrid<'a, T> {
    /// Create a new `MutSliceGrid` from the provided slice. Panics if the length of
    /// the slice is not exactly `width * height`.
    #[inline]
    pub fn new_mut_slice(size: impl Into<Vec2U>, slice: &'a mut [T]) -> Self {
        Self::with_store(size, slice)
    }
}

impl<T, const N: usize> ArrGrid<T, N> {
    /// Create a new `ArrGrid` filled with values from the provided function.
    #[inline]
    pub fn new_arr_with<F: FnMut() -> T>(size: impl Into<Vec2U>, mut fill: F) -> Self {
        Self::with_store(size, std::array::from_fn(|_| fill()))
    }

    /// Create a new `ArrGrid` fill with default values.
    #[inline]
    pub fn new_arr(size: impl Into<Vec2U>) -> Self
    where
        T: Default,
    {
        Self::new_arr_with(size, T::default)
    }
}

impl<T, S: AsRef<[T]>> Grid for GridBuf<T, S> {
    type Item = T;
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
        y.checked_mul(self.size.x)
            .and_then(|y| y.checked_add(x))
            .and_then(|i| self.as_slice().get(i as usize))
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Self::Item {
        unsafe {
            let i = y.unchecked_mul(self.size.x).unchecked_add(x);
            self.as_slice().get_unchecked(i as usize)
        }
    }

    #[inline]
    fn row_slice(&self, y: u32) -> Option<&[Self::Item]> {
        y.checked_mul(self.size.x).and_then(|i| {
            self.as_slice().get({
                let i = i as usize;
                let len = self.size.x as usize;
                i..(i + len)
            })
        })
    }
}

impl<T, S: AsRef<[T]> + AsMut<[T]>> GridMut for GridBuf<T, S> {
    type RootMut = Self;

    #[inline]
    fn root_mut(&mut self) -> &mut Self::RootMut {
        self
    }

    #[inline]
    fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Item> {
        y.checked_mul(self.size.x)
            .and_then(|y| y.checked_add(x))
            .and_then(|i| self.as_mut_slice().get_mut(i as usize))
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: u32, y: u32) -> &mut Self::Item {
        unsafe {
            let i = y.unchecked_mul(self.size.x).unchecked_add(x);
            self.as_mut_slice().get_unchecked_mut(i as usize)
        }
    }

    #[inline]
    fn row_slice_mut(&mut self, y: u32) -> Option<&mut [Self::Item]> {
        let w = self.size.x;
        y.checked_mul(w).and_then(|i| {
            self.as_mut_slice().get_mut({
                let i = i as usize;
                let w = w as usize;
                i..(i + w)
            })
        })
    }
}

impl<'a, T, S: AsRef<[T]>> IntoIterator for &'a GridBuf<T, S> {
    type Item = (&'a T, Vec2U);
    type IntoIter = GridIter<&'a GridBuf<T, S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, S: AsRef<[T]> + AsMut<[T]>> IntoIterator for &'a mut GridBuf<T, S> {
    type Item = (&'a mut T, Vec2U);
    type IntoIter = GridIter<&'a mut GridBuf<T, S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, S: Clone> Clone for GridBuf<T, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            store: self.store.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Debug, S: AsRef<[T]>> Debug for GridBuf<T, S> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}

impl<C: Coord, T, S: AsRef<[T]>> Index<C> for GridBuf<T, S> {
    type Output = T;

    #[inline]
    fn index(&self, index: C) -> &Self::Output {
        let (w, h) = self.size.into();
        self.get(
            index.x().to_grid(w).expect("invalid x-coordinate"),
            index.y().to_grid(h).expect("invalid y-coordinate"),
        )
        .expect("coordinate out of bounds")
    }
}

impl<C: Coord, T, S: AsRef<[T]> + AsMut<[T]>> IndexMut<C> for GridBuf<T, S> {
    #[inline]
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        let (w, h) = self.size.into();
        self.get_mut(
            index.x().to_grid(w).expect("invalid x-coordinate"),
            index.y().to_grid(h).expect("invalid y-coordinate"),
        )
        .expect("coordinate out of bounds")
    }
}
