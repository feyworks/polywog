use crate::{Grid, GridMut};
use fey_math::{Vec2U, vec2};
use std::iter::FusedIterator;

/// Iterator over all values in a grid, and their positions.
#[derive(Copy, Clone)]
pub struct GridIter<GridRef> {
    grid: GridRef,
    x: u32,
    y: u32,
}

impl<GridRef> GridIter<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef) -> Self {
        Self { grid, x: 0, y: 0 }
    }
}

impl<'a, G: Grid> Iterator for GridIter<&'a G> {
    type Item = (&'a G::Item, Vec2U);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get(self.x, self.y)?;
        let pos = vec2(self.x, self.y);
        self.x += 1;
        if self.x == self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        Some((val, pos))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl<G: Grid> ExactSizeIterator for GridIter<&G> {
    #[inline]
    fn len(&self) -> usize {
        let w = self.grid.width();
        let h = self.grid.height();
        ((h.saturating_sub(self.y + 1)) * w + (w - self.x)) as usize
    }
}

impl<G: Grid> FusedIterator for GridIter<&G> {}

impl<'a, G: GridMut> Iterator for GridIter<&'a mut G> {
    type Item = (&'a mut G::Item, Vec2U);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val: *mut G::Item = self.grid.get_mut(self.x, self.y)?;
        let pos = vec2(self.x, self.y);
        self.x += 1;
        if self.x == self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        // SAFETY: this iterator has a mutable reference to the grid, so as long as it exists, the
        // mutable reference to the value fetched from inside the grid will also be valid.
        Some((unsafe { &mut *val }, pos))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl<G: GridMut> ExactSizeIterator for GridIter<&mut G> {
    #[inline]
    fn len(&self) -> usize {
        let w = self.grid.width();
        let h = self.grid.height();
        ((h.saturating_sub(self.y + 1)) * w + (w - self.x)) as usize
    }
}

impl<G: GridMut> FusedIterator for GridIter<&mut G> {}
