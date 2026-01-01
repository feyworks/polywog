use crate::{Col, ColsIter, Coord, CoordComponent, Grid, GridIter, Row, RowsIter, View};
use fey_math::RectU;

/// A type representing a mutable 2D array.
pub trait GridMut: Grid {
    /// The root grid type. [Views](View) use this to store a reference to the root
    /// grid so they can read and modify it.
    type RootMut: GridMut<Item = Self::Item>;

    /// The root grid for this one. If this grid is the root, this returns `self`.
    fn root_mut(&mut self) -> &mut Self::RootMut;

    /// Returns a mutable reference to the value stored at `(x, y)` in the grid,
    /// or `None` if the provided coordinate is out of bounds.
    fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Item>;

    /// Returns a mujtable reference to the value stored at `(x, y)` in the grid,
    /// skipping any bounds checks.
    ///
    /// For a safe alternative, see [`get_mut`](Self::get_mut).
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked_mut(&mut self, x: u32, y: u32) -> &mut Self::Item;

    /// Returns a mutable reference to the value stored at the provided coordinate in
    /// the grid, or `None` if the provided coordinate is out of bounds.
    #[inline]
    fn get_mut_at(&mut self, coord: impl Coord) -> Option<&mut Self::Item> {
        self.get_mut(
            coord.x().to_grid(self.width())?,
            coord.y().to_grid(self.height())?,
        )
    }

    /// Returns a mutable reference to the value stored at the provided coordinate
    /// in the grid, skipping any bounds checks.
    #[inline]
    unsafe fn get_unchecked_mut_at(&mut self, coord: impl Coord) -> &mut Self::Item {
        unsafe {
            self.get_unchecked_mut(
                coord.x().to_grid(self.width()).unwrap_unchecked(),
                coord.y().to_grid(self.height()).unwrap_unchecked(),
            )
        }
    }

    /// Returns row `y` of the grid as a mutable slice if it is able to do so. Algorithms that
    /// work on large portions of the grid may use this to look for performance gain. For
    /// example, [`Row::draw_copied`] uses this internally to call std's [`copy_from_slice`] when
    /// possible, which can be faster than manually copying elements one-by-one.
    ///
    /// [`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
    fn row_slice_mut(&mut self, y: u32) -> Option<&mut [Self::Item]>;

    /// Returns row `y` of the grid as a mutable slice if it is able to do so. This variation
    /// can take signed integers, or y-values in a [`Wrap`](super::Wrap) or
    /// [`Clamp`](super::Clamp).
    #[inline]
    fn row_slice_mut_at(&mut self, y: impl CoordComponent) -> Option<&mut [Self::Item]> {
        self.row_slice_mut(y.to_grid(self.height())?)
    }

    /// Replace the value stored at `(x, y)` in the grid. If the provided coordinate was
    /// out of bounds, `None` is returned, otherwise the replaced value is returned.
    #[inline]
    fn set(&mut self, x: u32, y: u32, value: Self::Item) -> Option<Self::Item> {
        self.get_mut(x, y)
            .map(|curr| std::mem::replace(curr, value))
    }

    #[inline]
    fn set_at(&mut self, coord: impl Coord, value: Self::Item) -> Option<Self::Item> {
        self.set(
            coord.x().to_grid(self.width())?,
            coord.y().to_grid(self.height())?,
            value,
        )
    }

    /// Replace the value stored at `(x, y)` in the grid, without bounds checking, and
    /// return the replaced value.
    ///
    /// For a safe alternative, see [`set`](Self::set).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    unsafe fn set_unchecked(&mut self, x: u32, y: u32, value: Self::Item) -> Self::Item {
        std::mem::replace(unsafe { self.get_unchecked_mut(x, y) }, value)
    }

    #[inline]
    unsafe fn set_unchecked_at(&mut self, coord: impl Coord, value: Self::Item) -> Self::Item {
        unsafe {
            self.set_unchecked(
                coord.x().to_grid(self.width()).unwrap_unchecked(),
                coord.y().to_grid(self.height()).unwrap_unchecked(),
                value,
            )
        }
    }

    /// Get a mutable [`View`] into this grid, or `None` if the provided region is
    /// out of bounds.
    #[inline]
    fn try_view_mut(&mut self, x: u32, y: u32, w: u32, h: u32) -> Option<View<&mut Self::RootMut>> {
        if x.checked_add(w)? <= self.width() && y.checked_add(h)? <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(View::new(self.root_mut(), x, y, w, h))
        } else {
            None
        }
    }

    /// Get a mutable [`View`] into this grid. Panicks if the provided region is out
    /// of bounds.
    #[inline]
    fn view_mut(&mut self, x: u32, y: u32, w: u32, h: u32) -> View<&mut Self::RootMut> {
        self.try_view_mut(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn view_mut_at(&mut self, region: impl Into<RectU>) -> View<&mut Self::RootMut> {
        let RectU { x, y, w, h } = region.into();
        self.view_mut(x, y, w, h)
    }

    /// Mutably iterate over all values in the grid, with their positions.
    #[inline]
    fn iter_mut(&mut self) -> GridIter<&mut Self>
    where
        Self: Sized,
    {
        GridIter::new(self)
    }

    /// Mutably iterate over all columns in the grid.
    #[inline]
    fn cols_mut(&mut self) -> ColsIter<&mut Self>
    where
        Self: Sized,
    {
        ColsIter::new(self, self.width())
    }

    /// Return the column `x`, or `None` if `x` is out of bounds.
    #[inline]
    fn try_col_mut(&mut self, x: u32) -> Option<Col<&mut Self>> {
        (x < self.width()).then(|| Col::new(self, x))
    }

    /// Return the column `x`. Panics if `x` is out of bounds.
    #[inline]
    fn col_mut(&mut self, x: u32) -> Col<&mut Self> {
        self.try_col_mut(x).expect("column index out of bounds")
    }

    /// Mutably iterate over the rows of the grid.
    #[inline]
    fn rows_mut(&mut self) -> RowsIter<&mut Self>
    where
        Self: Sized,
    {
        RowsIter::new(self, self.height())
    }

    // Return the row `y`, or `None` if `y` is out of bounds.
    #[inline]
    fn try_row_mut(&mut self, y: u32) -> Option<Row<&mut Self>> {
        (y < self.height()).then(|| Row::new(self, y))
    }

    /// Return the row `y`. Panics if `y` is out of bounds.
    #[inline]
    fn row_mut(&mut self, y: u32) -> Row<&mut Self> {
        self.try_row_mut(y).expect("row index out of bounds")
    }

    /// Fill the entire grid with values provided by a function.
    #[inline]
    fn fill_with<F: FnMut() -> Self::Item>(&mut self, mut f: F)
    where
        Self: Sized,
    {
        for mut row in self.rows_mut() {
            row.fill_with(&mut f);
        }
    }

    /// Fill the entire grid with the provided value.
    #[inline]
    fn fill(&mut self, value: Self::Item)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let mut rows = self.rows_mut();
        if let Some(mut row) = rows.next() {
            for mut row in rows {
                row.fill(value.clone());
            }
            row.fill(value);
        }
    }

    /// Clone all values from a source grid into this one. Panics if the grids
    /// are not the same size.
    #[inline]
    fn draw_cloned<G2>(&mut self, grid: &G2)
    where
        G2: Grid<Item = Self::Item>,
        G2::Item: Clone,
        Self: Sized,
    {
        assert_eq!(self.width(), grid.width());
        assert_eq!(self.height(), grid.height());
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.draw_cloned(src);
        }
    }

    /// Copy all values from a source grid into this one. Panics if the grids
    /// are not the same size.
    #[inline]
    fn draw_copied<G2>(&mut self, grid: &G2)
    where
        G2: Grid<Item = Self::Item>,
        G2::Item: Copy,
        Self: Sized,
    {
        assert_eq!(self.width(), grid.width());
        assert_eq!(self.height(), grid.height());
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.draw_copied(src);
        }
    }

    #[inline]
    fn draw_mapped<G2, M>(&mut self, grid: &G2, mut map_fn: M)
    where
        G2: Grid,
        Self: Sized,
        M: FnMut(&G2::Item) -> Self::Item,
    {
        assert_eq!(self.width(), grid.width());
        assert_eq!(self.height(), grid.height());
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.draw_mapped(src, &mut map_fn);
        }
    }
}

impl<T, const W: usize, const H: usize> GridMut for [[T; W]; H] {
    type RootMut = Self;

    #[inline]
    fn root_mut(&mut self) -> &mut Self::RootMut {
        self
    }

    #[inline]
    fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Item> {
        let x = x as usize;
        let y = y as usize;
        (x < W && y < H).then(|| &mut self[y][x])
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: u32, y: u32) -> &mut Self::Item {
        unsafe {
            self.as_mut_slice()
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(x as usize)
        }
    }

    #[inline]
    fn row_slice_mut(&mut self, y: u32) -> Option<&mut [Self::Item]> {
        let y = y as usize;
        (y < H).then(|| self[y].as_mut_slice())
    }
}
