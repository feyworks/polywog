use crate::{Grid, GridMut, RowIter};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// A single row of a grid.
#[repr(C)]
#[derive(Clone)]
pub struct Row<GridRef> {
    grid: GridRef,
    y: u32,
}

impl<G: Grid<Item: Debug>> Debug for Row<&G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for x in 0..self.grid.width() {
            list.entry(self.grid.get(x, self.y).unwrap());
        }
        list.finish()
    }
}

impl<GridRef> Row<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, y: u32) -> Self {
        Self { grid, y }
    }

    /// The row's index, or y-position.
    #[inline]
    pub fn index(&self) -> u32 {
        self.y
    }
}

impl<'a, G> Deref for Row<&'a mut G> {
    type Target = Row<&'a G>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<Row<&'a mut G>> for Row<&'a G> {
    #[inline]
    fn from(Row { grid, y }: Row<&'a mut G>) -> Self {
        Self { grid, y }
    }
}

impl<'a, G> From<&'a Row<&'a G>> for Row<&'a G> {
    #[inline]
    fn from(Row { grid, y }: &'a Row<&'a G>) -> Self {
        Self { grid, y: *y }
    }
}

impl<'a, G> From<&'a Row<&'a mut G>> for Row<&'a G> {
    #[inline]
    fn from(Row { grid, y }: &'a Row<&'a mut G>) -> Self {
        Self { grid, y: *y }
    }
}

impl<'a, G: Grid> Row<&'a G> {
    /// Length of the row (equal to the grid's width).
    #[inline]
    pub fn len(&self) -> u32 {
        self.grid.width()
    }

    /// Returns `true` if the row has no width.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a reference to the value in column `x` of this row.
    #[inline]
    pub fn get(&self, x: u32) -> Option<&G::Item> {
        self.grid.get(x, self.y)
    }

    /// Get a reference to the value in column `x` of this row, without bounds checking.
    ///
    /// For a safe alternative, see [`get`](Self::get).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds `x` value is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn get_unchecked(&self, x: u32) -> &G::Item {
        unsafe { self.grid.get_unchecked(x, self.y) }
    }

    /// Returns this row as an immutable slice if able to do so.
    #[inline]
    pub fn as_slice(&self) -> Option<&[G::Item]> {
        self.grid.row_slice(self.y)
    }

    /// Iterate over all values in the row.
    #[inline]
    pub fn iter(&self) -> RowIter<&'a G> {
        RowIter::new(self.grid, self.y, self.len())
    }
}

impl<'a, G: GridMut> Row<&'a mut G> {
    /// Get a mutable reference to the value in column `x` of this row.
    #[inline]
    pub fn get_mut(&mut self, x: u32) -> Option<&mut G::Item> {
        self.grid.get_mut(x, self.y)
    }

    /// Get a mutable reference to the value in column `x` of this row, without
    /// bounds checking.
    ///
    /// For a safe alternative, see [`get`](Self::get_mut).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds `x` value is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked_mut(&mut self, x: u32) -> &mut G::Item {
        unsafe { self.grid.get_unchecked_mut(x, self.y) }
    }

    /// Returns this row as a mutable slice if able to do so.
    #[inline]
    pub fn as_mut_slice(&mut self) -> Option<&mut [G::Item]> {
        self.grid.row_slice_mut(self.y)
    }

    /// Fill the entire row with values provided by a function.
    #[inline]
    pub fn fill_with<F: FnMut() -> G::Item>(&mut self, mut f: F) {
        if let Some(slice) = self.as_mut_slice() {
            slice.fill_with(f);
        } else {
            for val in self.iter_mut() {
                *val = f();
            }
        }
    }

    /// Fill the entire row with the provided value.
    #[inline]
    pub fn fill(&mut self, value: G::Item)
    where
        G::Item: Clone,
    {
        if self.is_empty() {
            return;
        }
        if let Some(slice) = self.as_mut_slice() {
            slice.fill(value);
        } else {
            for x in 1..self.len() {
                *self.get_mut(x).unwrap() = value.clone();
            }
            *self.get_mut(0).unwrap() = value;
        }
    }

    /// Clone all values from the provided row to this one.
    #[inline]
    pub fn draw_cloned<G2>(&mut self, row: impl Into<Row<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Clone,
    {
        let row = row.into();
        assert_eq!(self.len(), row.len());
        match (self.as_mut_slice(), row.as_slice()) {
            (Some(dst), Some(src)) => {
                dst.clone_from_slice(src);
            }
            (Some(dst), None) => {
                for (dst, src) in dst.iter_mut().zip(row) {
                    *dst = src.clone();
                }
            }
            (None, Some(src)) => {
                for (dst, src) in self.iter_mut().zip(src) {
                    *dst = src.clone();
                }
            }
            (None, None) => {
                for (dst, src) in self.iter_mut().zip(row) {
                    *dst = src.clone();
                }
            }
        }
    }

    /// Copy all values from the provided row to this one.
    #[inline]
    pub fn draw_copied<G2>(&mut self, row: impl Into<Row<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Copy,
    {
        let row = row.into();
        assert_eq!(self.len(), row.len());
        match (self.as_mut_slice(), row.as_slice()) {
            (Some(dst), Some(src)) => {
                dst.copy_from_slice(src);
            }
            (Some(dst), None) => {
                for (dst, src) in dst.iter_mut().zip(row) {
                    *dst = *src;
                }
            }
            (None, Some(src)) => {
                for (dst, src) in self.iter_mut().zip(src) {
                    *dst = *src;
                }
            }
            (None, None) => {
                for (dst, src) in self.iter_mut().zip(row) {
                    *dst = *src;
                }
            }
        }
    }

    /// Copy all values from the provided row to this one, mapping them with the provided function.
    #[inline]
    pub fn draw_mapped<G2, M>(&mut self, row: impl Into<Row<&'a G2>>, mut map_fn: M)
    where
        G2: Grid + 'a,
        M: FnMut(&G2::Item) -> G::Item,
    {
        for (dst, src) in self.iter_mut().zip(row.into().iter()) {
            *dst = map_fn(src);
        }
    }

    /// Mutably iterate over all values in the row.
    #[inline]
    pub fn iter_mut(&mut self) -> RowIter<&mut G> {
        RowIter::new(self.grid, self.y, self.len())
    }
}

impl<'a, G: Grid> IntoIterator for Row<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter::new(self.grid, self.y, self.grid.width())
    }
}

impl<'a, G: GridMut> IntoIterator for Row<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = RowIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let w = self.grid.width();
        RowIter::new(self.grid, self.y, w)
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter::new(self.grid, self.y, self.grid.width())
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a mut G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter::new(self.grid, self.y, self.grid.width())
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut Row<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = RowIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let w = self.grid.width();
        RowIter::new(self.grid, self.y, w)
    }
}

impl<A: Grid, B: Grid> PartialEq<Row<&B>> for Row<&A>
where
    A::Item: PartialEq<B::Item>,
{
    #[inline]
    fn eq(&self, other: &Row<&B>) -> bool {
        match (self.as_slice(), other.as_slice()) {
            (Some(a), Some(b)) => a == b,
            _ => self.iter() == other.iter(),
        }
    }
}
