use crate::{ColIter, Grid, GridMut};
use std::ops::Deref;

/// A single column of a grid.
#[repr(C)]
#[derive(Clone)]
pub struct Col<GridRef> {
    grid: GridRef,
    x: u32,
}

impl<GridRef> Col<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, x: u32) -> Self {
        Self { grid, x }
    }

    /// The column's index, or x-position.
    #[inline]
    pub fn index(&self) -> u32 {
        self.x
    }
}

impl<'a, G> Deref for Col<&'a mut G> {
    type Target = Col<&'a G>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<Col<&'a mut G>> for Col<&'a G> {
    #[inline]
    fn from(Col { grid, x }: Col<&'a mut G>) -> Self {
        Self { grid, x }
    }
}

impl<'a, G> From<&'a Col<&'a G>> for Col<&'a G> {
    #[inline]
    fn from(Col { grid, x }: &'a Col<&'a G>) -> Self {
        Self { grid, x: *x }
    }
}

impl<'a, G> From<&'a Col<&'a mut G>> for Col<&'a G> {
    #[inline]
    fn from(Col { grid, x }: &'a Col<&'a mut G>) -> Self {
        Self { grid, x: *x }
    }
}

impl<'a, G: Grid> Col<&'a G> {
    /// Length of the column (equal to the grid's height).
    #[inline]
    pub fn len(&self) -> u32 {
        self.grid.height()
    }

    /// Returns `true` if the column has no height.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a reference to the value in row `y` of this column.
    #[inline]
    pub fn get(&self, y: u32) -> Option<&G::Item> {
        self.grid.get(self.x, y)
    }

    /// Get a reference to the value in row `y` of this column, without bounds checking.
    ///
    /// For a safe alternative, see [`get`](Self::get).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds `y` value is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn get_unchecked(&self, y: u32) -> &G::Item {
        unsafe { self.grid.get_unchecked(self.x, y) }
    }

    /// Iterate over all values in the column.
    #[inline]
    pub fn iter(&self) -> ColIter<&'a G> {
        ColIter::new(self.grid, self.x, self.len())
    }
}

impl<'a, G: GridMut> Col<&'a mut G> {
    /// Get a mutable reference to the value in row `y` of this column.
    #[inline]
    pub fn get_mut(&mut self, y: u32) -> Option<&mut G::Item> {
        self.grid.get_mut(self.x, y)
    }

    /// Get a mutable reference to the value in row `y` of this column, without
    /// bounds checking.
    ///
    /// For a safe alternative, see [`get_mut`](Self::get_mut).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds `y` value is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked_mut(&mut self, y: u32) -> &mut G::Item {
        unsafe { self.grid.get_unchecked_mut(self.x, y) }
    }

    /// Fill the entire column with values provided by a function.
    #[inline]
    pub fn fill_with<F: FnMut() -> G::Item>(&mut self, mut f: F) {
        for val in self.iter_mut() {
            *val = f();
        }
    }

    /// Fill the entire column with the provided value.
    #[inline]
    pub fn fill(&mut self, value: G::Item)
    where
        G::Item: Clone,
    {
        if self.is_empty() {
            return;
        }
        for y in 1..self.len() {
            *self.get_mut(y).unwrap() = value.clone();
        }
        *self.get_mut(0).unwrap() = value;
    }

    /// Clone all values from the provided column to this one.
    #[inline]
    pub fn clone_from<G2>(&mut self, col: impl Into<Col<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Clone,
    {
        let col = col.into();
        assert_eq!(self.len(), col.len());
        for (dst, src) in self.iter_mut().zip(col) {
            *dst = src.clone();
        }
    }

    /// Copy all values from the provided column to this one.
    #[inline]
    pub fn copy_from<G2>(&mut self, col: impl Into<Col<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Copy,
    {
        let col = col.into();
        assert_eq!(self.len(), col.len());
        for (dst, src) in self.iter_mut().zip(col) {
            *dst = *src;
        }
    }

    /// Mutably iterate over all values in the column.
    #[inline]
    pub fn iter_mut(&mut self) -> ColIter<&mut G> {
        ColIter::new(self.grid, self.x, self.len())
    }
}

impl<'a, G: Grid> IntoIterator for Col<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = ColIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColIter::new(self.grid, self.x, self.grid.height())
    }
}

impl<'a, G: GridMut> IntoIterator for Col<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = ColIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let h = self.grid.height();
        ColIter::new(self.grid, self.x, h)
    }
}

impl<'a, G: Grid> IntoIterator for &'a Col<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = ColIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColIter::new(self.grid, self.x, self.grid.height())
    }
}

impl<'a, G: Grid> IntoIterator for &'a Col<&'a mut G> {
    type Item = &'a G::Item;
    type IntoIter = ColIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColIter::new(self.grid, self.x, self.grid.height())
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut Col<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = ColIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let h = self.grid.height();
        ColIter::new(self.grid, self.x, h)
    }
}
