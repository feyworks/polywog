use crate::{
    ArrGrid, Col, ColsIter, Coord, CoordComponent, GridBuf, GridIter, GridMut, Row, RowsIter,
    VecGrid, View,
};
use fey_math::{RectU, Vec2U, rect, vec2};
use std::fmt::{Debug, Write};
use std::hash::{Hash, Hasher};

/// A type representing an immutable 2D array.
pub trait Grid {
    /// The type of item this grid contains.
    type Item;

    /// The root grid type. [Views](View) use this to store a reference to the root
    /// grid so they can read and modify it.
    type Root: Grid<Item = Self::Item>;

    /// The root grid for this one. If this grid is the root, this returns `self`.
    fn root(&self) -> &Self::Root;

    /// This grid's x-offset from the root grid. For the root grid this is `0`.
    fn root_x(&self) -> u32;

    /// This grid's y-offset from the root grid. For the root grid this is `0`.
    fn root_y(&self) -> u32;

    /// Width of the grid (how many columns it has).
    fn width(&self) -> u32;

    /// Height of the grid (how many rows it has).
    fn height(&self) -> u32;

    /// Size of the grid as a `(width, height)` pair.
    #[inline]
    fn size(&self) -> Vec2U {
        vec2(self.width(), self.height())
    }

    /// Surface area of the grid, equal to `width * height`.
    #[inline]
    fn area(&self) -> u32 {
        self.width() * self.height()
    }

    /// Returns a reference to the value stored at `(x, y)` in the grid, or `None` if
    /// the provided coordinate is out of bounds.
    fn get(&self, x: u32, y: u32) -> Option<&Self::Item>;

    /// Returns a reference to the value stored at `(x, y)` in the grid, skipping
    /// any bounds checks.
    unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Self::Item;

    /// Returns a reference to the value stored at the provided coordinate in the grid,
    /// or `None` if the provided coordinate is out of bounds.
    #[inline]
    fn get_at(&self, coord: impl Coord) -> Option<&Self::Item> {
        self.get(
            coord.x().to_grid(self.width())?,
            coord.y().to_grid(self.height())?,
        )
    }

    /// Returns a reference to the value stored at the provided coordinate in the grid,
    /// skipping any bounds checks.
    #[inline]
    unsafe fn get_unchecked_at(&self, coord: impl Coord) -> &Self::Item {
        unsafe {
            self.get_unchecked(
                coord.x().to_grid(self.width()).unwrap_unchecked(),
                coord.y().to_grid(self.height()).unwrap_unchecked(),
            )
        }
    }

    /// Returns row `y` of the grid as a slice if it is able to do so. Algorithms that work
    /// on large portions of the grid may use this to look for performance gain. For example,
    /// [`Row::draw_copied`] uses this internally to call `copy_from_slice` when possible,
    /// which can be faster than manually copying elements one-by-one.
    fn row_slice(&self, y: u32) -> Option<&[Self::Item]>;

    /// Returns row `y` of the grid as a slice if it is able to do so. This variation
    /// can take signed integers, or y-values in a [`Wrap`](super::Wrap) or
    /// [`Clamp`](super::Clamp).
    #[inline]
    fn row_slice_at(&self, y: impl CoordComponent) -> Option<&[Self::Item]> {
        self.row_slice(y.to_grid(self.height())?)
    }

    /// Returns true if both grids are the same size.
    #[inline]
    fn same_size<G2: Grid>(&self, other: &G2) -> bool {
        self.width() == other.width() && self.height() == other.height()
    }

    /// Get an immutable [`View`] into this grid, or `None` if the provided region is
    /// out of bounds.
    #[inline]
    fn try_view(&self, x: u32, y: u32, w: u32, h: u32) -> Option<View<&Self::Root>> {
        if x.checked_add(w)? <= self.width() && y.checked_add(h)? <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(View::new(self.root(), x, y, w, h))
        } else {
            None
        }
    }

    #[inline]
    fn try_view_at(&self, region: impl Into<RectU>) -> Option<View<&Self::Root>> {
        let RectU { x, y, w, h } = region.into();
        self.try_view(x, y, w, h)
    }

    /// Get an immutable [`View`] into this grid. Panicks if the provided region is out
    /// of bounds.
    #[inline]
    fn view(&self, x: u32, y: u32, w: u32, h: u32) -> View<&Self::Root> {
        self.try_view(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn view_at(&self, region: impl Into<RectU>) -> View<&Self::Root> {
        let RectU { x, y, w, h } = region.into();
        self.view(x, y, w, h)
    }

    /// Create a [`GridBuf`] using the provided storage and clone this entire
    /// grid into it. The resulting grid will be the same size as this one.
    #[inline]
    fn to_grid_buf<S>(&self, store: S) -> GridBuf<Self::Item, S>
    where
        S: AsRef<[Self::Item]> + AsMut<[Self::Item]>,
        Self::Item: Clone,
        Self: Sized,
    {
        let mut buf = GridBuf::with_store(self.size(), store);
        buf.draw_cloned(self);
        buf
    }

    /// Create a stack-allocated [`GridBuf`], using an `N`-sized array for storage,
    /// and clone this entire grid into it. Panics if `N` is not exactly the area
    /// of the grid (`width * height`).
    #[inline]
    fn to_arr_grid<const N: usize>(&self) -> ArrGrid<Self::Item, N>
    where
        Self::Item: Default + Clone,
        Self: Sized,
    {
        assert_eq!(self.width() * self.height(), N as u32);
        let mut arr = std::array::from_fn(|_| Self::Item::default());
        for (dst, src) in arr.chunks_exact_mut(self.width() as usize).zip(self.rows()) {
            if let Some(src) = src.as_slice() {
                dst.clone_from_slice(src);
            } else {
                for (dst, src) in dst.iter_mut().zip(&src) {
                    *dst = src.clone();
                }
            }
        }
        GridBuf::with_store(self.size(), arr)
    }

    /// Create a stack-allocated [`GridBuf`], using a [`Vec`] for storage, and
    /// clone this entire grid into it.
    fn to_vec_grid(&self) -> VecGrid<Self::Item>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        let mut vec = Vec::with_capacity(self.area() as usize);
        for row in self.rows() {
            if let Some(row) = row.as_slice() {
                vec.extend_from_slice(row);
            } else {
                vec.extend(row.iter().cloned())
            }
        }
        GridBuf::with_store(self.size(), vec)
    }

    /// Iterate over all values in the grid, with their positions.
    #[inline]
    fn iter(&self) -> GridIter<&Self>
    where
        Self: Sized,
    {
        GridIter::new(self)
    }

    /// Iterate over all columns in the grid.
    #[inline]
    fn cols(&self) -> ColsIter<&Self>
    where
        Self: Sized,
    {
        ColsIter::new(self, self.width())
    }

    /// Return the column `x`, or `None` if `x` is out of bounds.
    #[inline]
    fn try_col(&self, x: u32) -> Option<Col<&Self>> {
        (x < self.width()).then(|| Col::new(self, x))
    }

    /// Return the column `x`. Panics if `x` is out of bounds.
    #[inline]
    fn col(&self, x: u32) -> Col<&Self> {
        self.try_col(x).expect("column index out of bounds")
    }

    /// Iterate over the rows of the grid.
    #[inline]
    fn rows(&self) -> RowsIter<&Self>
    where
        Self: Sized,
    {
        RowsIter::new(self, self.height())
    }

    /// Return the row `y`, or `None` if `y` is out of bounds.
    #[inline]
    fn try_row(&self, y: u32) -> Option<Row<&Self>> {
        (y < self.height()).then(|| Row::new(self, y))
    }

    /// Return the row `y`. Panics if `y` is out of bounds.
    #[inline]
    fn row(&self, y: u32) -> Row<&Self> {
        self.try_row(y).expect("row index out of bounds")
    }

    /// Return the bounds of the grid containing all elements match the predicate.
    #[inline]
    fn get_bounds<F: FnMut(&Self::Item) -> bool>(&self, mut cond: F) -> Option<RectU>
    where
        Self: Sized,
    {
        let mut min = Vec2U::MAX;
        let mut max = Vec2U::MIN;
        for (val, p) in self.iter() {
            if cond(val) {
                min = min.min(p);
                max = max.max(p);
            }
        }
        (min.x <= max.x && min.y <= max.y)
            .then(|| rect(min.x, min.y, (max.x - min.x) + 1, (max.y - min.y) + 1))
    }

    #[inline]
    fn eq_grid<'a, H: Grid>(&'a self, other: &'a H) -> bool
    where
        Self::Item: PartialEq<H::Item>,
        Self: Sized,
    {
        self.same_size(other)
            && self.rows().zip(other.rows()).all(|(a, b)| {
                if let (Some(a), Some(b)) = (a.as_slice(), b.as_slice()) {
                    a == b
                } else {
                    a.iter().zip(b.iter()).all(|(a, b)| a == b)
                }
            })
    }

    #[inline]
    fn debug_fmt<W: Write>(&self, mut f: W) -> std::fmt::Result
    where
        Self::Item: Debug,
    {
        let mut s = String::new();
        let mut len = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let val = self.get(x, y).unwrap();
                s.clear();
                write!(s, "{:?}", val)?;
                len = len.max(s.len());
            }
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                let val = self.get(x, y).unwrap();
                s.clear();
                write!(s, "{:?}", val)?;
                while s.len() < len {
                    s.push(' ');
                }
                write!(f, "[{}]", s)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }

    fn hash_grid<H: Hasher>(&self, hasher: &mut H)
    where
        Self: Sized,
        Self::Item: Hash,
    {
        self.width().hash(hasher);
        self.height().hash(hasher);
        for row in self.rows() {
            if let Some(row) = row.as_slice() {
                row.hash(hasher);
            } else {
                for val in row {
                    val.hash(hasher);
                }
            }
        }
    }
}

impl<T, const W: usize, const H: usize> Grid for [[T; W]; H] {
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
        W as u32
    }

    #[inline]
    fn height(&self) -> u32 {
        H as u32
    }

    #[inline]
    fn get(&self, x: u32, y: u32) -> Option<&Self::Item> {
        let x = x as usize;
        let y = y as usize;
        (x < W && y < H).then(|| &self[y][x])
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Self::Item {
        unsafe {
            self.as_slice()
                .get_unchecked(y as usize)
                .get_unchecked(x as usize)
        }
    }

    #[inline]
    fn row_slice(&self, y: u32) -> Option<&[Self::Item]> {
        let y = y as usize;
        (y < H).then(|| self[y].as_slice())
    }
}
