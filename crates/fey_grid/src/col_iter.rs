use crate::{Grid, GridMut};
use std::iter::FusedIterator;

/// Iterator over the values in a column.
#[derive(Clone)]
pub struct ColIter<GridRef> {
    grid: GridRef,
    x: u32,
    y: u32,
    b: u32,
}

impl<GridRef> ColIter<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, x: u32, b: u32) -> Self {
        Self { grid, x, y: 0, b }
    }
}

// ---------- IMMUTABLE ITERATOR ----------

impl<'a, G: Grid> Iterator for ColIter<&'a G> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            let y = self.y;
            self.y += 1;
            self.grid.get(self.x, y)
        } else {
            None
        }
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

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.y < self.b {
            self.grid.get(self.x, self.b - 1)
        } else {
            None
        }
    }
}

impl<G: Grid> DoubleEndedIterator for ColIter<&G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            self.b -= 1;
            self.grid.get(self.x, self.b)
        } else {
            None
        }
    }
}

impl<G: Grid> ExactSizeIterator for ColIter<&G> {
    #[inline]
    fn len(&self) -> usize {
        (self.b - self.y) as usize
    }
}

impl<G: Grid> FusedIterator for ColIter<&G> {}

// ---------- MUTABLE ITERATOR ----------

impl<'a, G: GridMut> Iterator for ColIter<&'a mut G> {
    type Item = &'a mut G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            let item: *mut G::Item = self.grid.get_mut(self.x, self.y).unwrap();
            self.y += 1;
            Some(unsafe { &mut *item })
        } else {
            None
        }
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

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.y < self.b {
            self.grid.get_mut(self.x, self.b - 1)
        } else {
            None
        }
    }
}

impl<G: GridMut> DoubleEndedIterator for ColIter<&mut G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            self.b -= 1;
            let item: *mut G::Item = self.grid.get_mut(self.x, self.b).unwrap();
            Some(unsafe { &mut *item })
        } else {
            None
        }
    }
}

impl<G: GridMut> ExactSizeIterator for ColIter<&mut G> {
    #[inline]
    fn len(&self) -> usize {
        (self.b - self.y) as usize
    }
}

impl<G: GridMut> FusedIterator for ColIter<&mut G> {}
