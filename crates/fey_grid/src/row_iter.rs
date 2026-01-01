use crate::{Grid, GridMut};
use std::fmt::{Debug, Formatter};
use std::iter::FusedIterator;

/// Iterator over the values in a row.
#[derive(Clone)]
pub struct RowIter<GridRef> {
    grid: GridRef,
    x: u32,
    y: u32,
    r: u32,
}

impl<GridRef> RowIter<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, y: u32, r: u32) -> Self {
        Self { grid, x: 0, y, r }
    }
}

impl<G: Grid<Item: Debug>> Debug for RowIter<&G> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RowIter(")?;
        let mut list = f.debug_list();
        for x in self.x..self.r {
            list.entry(self.grid.get(x, self.y).unwrap());
        }
        list.finish()?;
        f.write_str(")")
    }
}

// ---------- IMMUTABLE ITERATOR ----------

impl<'a, G: Grid> Iterator for RowIter<&'a G> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let x = self.x;
            self.x += 1;
            self.grid.get(x, self.y)
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
            self.grid.get(self.r - 1, self.y)
        } else {
            None
        }
    }
}

impl<G: Grid> DoubleEndedIterator for RowIter<&G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            self.r -= 1;
            self.grid.get(self.r, self.y)
        } else {
            None
        }
    }
}

impl<G: Grid> ExactSizeIterator for RowIter<&G> {
    #[inline]
    fn len(&self) -> usize {
        (self.r - self.x) as usize
    }
}

impl<G: Grid> FusedIterator for RowIter<&G> {}

// ---------- MUTABLE ITERATOR ----------

impl<'a, G: GridMut> Iterator for RowIter<&'a mut G> {
    type Item = &'a mut G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let item: *mut G::Item = self.grid.get_mut(self.x, self.y).unwrap();
            self.x += 1;
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
        if self.x < self.r {
            self.grid.get_mut(self.r - 1, self.y)
        } else {
            None
        }
    }
}

impl<G: GridMut> DoubleEndedIterator for RowIter<&mut G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            self.r -= 1;
            let item: *mut G::Item = self.grid.get_mut(self.r, self.y).unwrap();
            Some(unsafe { &mut *item })
        } else {
            None
        }
    }
}

impl<G: GridMut> ExactSizeIterator for RowIter<&mut G> {
    #[inline]
    fn len(&self) -> usize {
        (self.r - self.x) as usize
    }
}

impl<G: GridMut> FusedIterator for RowIter<&mut G> {}

impl<A: Grid, B: Grid> PartialEq<RowIter<&B>> for RowIter<&A>
where
    A::Item: PartialEq<B::Item>,
{
    #[inline]
    fn eq(&self, other: &RowIter<&B>) -> bool {
        self.len() == other.len() && self.clone().zip(other.clone()).all(|(a, b)| a == b)
    }
}
