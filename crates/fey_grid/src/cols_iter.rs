use crate::{Col, Grid, GridMut};
use std::iter::FusedIterator;

/// Iterator over the columns of a grid.
#[derive(Clone)]
pub struct ColsIter<GridRef> {
    grid: GridRef,
    x: u32,
    r: u32,
}

impl<GridRef> ColsIter<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, r: u32) -> Self {
        Self { grid, x: 0, r }
    }
}

impl<'a, G: Grid> Iterator for ColsIter<&'a G> {
    type Item = Col<&'a G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let col = Col::new(self.grid, self.x);
            self.x += 1;
            Some(col)
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
        if self.x < self.r {
            Some(Col::new(self.grid, self.r - 1))
        } else {
            None
        }
    }
}

impl<G: Grid> DoubleEndedIterator for ColsIter<&G> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            self.r -= 1;
            Some(Col::new(self.grid, self.r))
        } else {
            None
        }
    }
}

impl<G: Grid> ExactSizeIterator for ColsIter<&G> {
    #[inline]
    fn len(&self) -> usize {
        (self.r - self.x) as usize
    }
}

impl<G: Grid> FusedIterator for ColsIter<&G> {}

impl<'a, G: GridMut> Iterator for ColsIter<&'a mut G> {
    type Item = Col<&'a mut G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let grid: *mut G = self.grid;
            let row = Col::new(unsafe { &mut *grid }, self.x);
            self.x += 1;
            Some(row)
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
        if self.x < self.r {
            let grid: *mut G = self.grid;
            Some(Col::new(unsafe { &mut *grid }, self.r - 1))
        } else {
            None
        }
    }
}

impl<G: GridMut> DoubleEndedIterator for ColsIter<&mut G> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            self.r -= 1;
            let grid: *mut G = self.grid;
            Some(Col::new(unsafe { &mut *grid }, self.r))
        } else {
            None
        }
    }
}

impl<G: GridMut> ExactSizeIterator for ColsIter<&mut G> {
    #[inline]
    fn len(&self) -> usize {
        (self.r - self.x) as usize
    }
}

impl<G: GridMut> FusedIterator for ColsIter<&mut G> {}
