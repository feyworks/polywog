use crate::{Grid, GridMut};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Sub-section of a larger grid.
#[repr(C)]
#[derive(Clone)]
pub struct View<GridRef> {
    grid: GridRef,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl<GridRef> View<GridRef> {
    pub(crate) fn new(grid: GridRef, x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { grid, x, y, w, h }
    }
}

impl<'a, G> Deref for View<&'a mut G> {
    type Target = View<&'a G>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<View<&'a mut G>> for View<&'a G> {
    #[inline]
    fn from(View { grid, x, y, w, h }: View<&'a mut G>) -> Self {
        Self { grid, x, y, w, h }
    }
}

impl<'a, G> From<&'a View<&'a G>> for View<&'a G> {
    #[inline]
    fn from(View { grid, x, y, w, h }: &'a View<&'a G>) -> Self {
        Self::new(grid, *x, *y, *w, *h)
    }
}

impl<'a, G> From<&'a View<&'a mut G>> for View<&'a G> {
    #[inline]
    fn from(View { grid, x, y, w, h }: &'a View<&'a mut G>) -> Self {
        Self::new(grid, *x, *y, *w, *h)
    }
}

impl<G: Grid> Grid for View<&G> {
    type Item = G::Item;
    type Root = G;

    #[inline]
    fn root(&self) -> &Self::Root {
        self.grid
    }

    #[inline]
    fn root_x(&self) -> u32 {
        self.x
    }

    #[inline]
    fn root_y(&self) -> u32 {
        self.y
    }

    #[inline]
    fn width(&self) -> u32 {
        self.w
    }

    #[inline]
    fn height(&self) -> u32 {
        self.h
    }

    #[inline]
    fn get(&self, x: u32, y: u32) -> Option<&Self::Item> {
        if x < self.w && y < self.h {
            self.root().get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Self::Item {
        unsafe { self.root().get_unchecked(self.x + x, self.y + y) }
    }

    #[inline]
    fn row_slice(&self, y: u32) -> Option<&[Self::Item]> {
        if y < self.h {
            self.grid.row_slice(self.y + y).and_then(|s| {
                s.get({
                    let x = self.x as usize;
                    let w = self.w as usize;
                    x..(x + w)
                })
            })
        } else {
            None
        }
    }
}

impl<G: Grid> Grid for View<&mut G> {
    type Item = G::Item;
    type Root = G;

    #[inline]
    fn root(&self) -> &Self::Root {
        self.grid
    }

    #[inline]
    fn root_x(&self) -> u32 {
        self.x
    }

    #[inline]
    fn root_y(&self) -> u32 {
        self.y
    }

    #[inline]
    fn width(&self) -> u32 {
        self.w
    }

    #[inline]
    fn height(&self) -> u32 {
        self.h
    }

    #[inline]
    fn get(&self, x: u32, y: u32) -> Option<&Self::Item> {
        if x < self.w && y < self.h {
            self.grid.get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Self::Item {
        unsafe { self.grid.get_unchecked(self.x + x, self.y + y) }
    }

    #[inline]
    fn row_slice(&self, y: u32) -> Option<&[Self::Item]> {
        if y < self.h {
            self.grid.row_slice(self.y + y).and_then(|s| {
                s.get({
                    let x = self.x as usize;
                    let w = self.w as usize;
                    x..(x + w)
                })
            })
        } else {
            None
        }
    }
}

impl<G: GridMut> GridMut for View<&mut G> {
    type RootMut = G;

    #[inline]
    fn root_mut(&mut self) -> &mut Self::RootMut {
        self.grid
    }

    #[inline]
    fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Item> {
        if x < self.w && y < self.h {
            self.grid.get_mut(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: u32, y: u32) -> &mut Self::Item {
        unsafe { self.grid.get_unchecked_mut(self.x + x, self.y + y) }
    }

    #[inline]
    fn row_slice_mut(&mut self, y: u32) -> Option<&mut [Self::Item]> {
        if y < self.h {
            self.grid.row_slice_mut(self.y + y).and_then(|s| {
                s.get_mut({
                    let x = self.x as usize;
                    let w = self.w as usize;
                    x..(x + w)
                })
            })
        } else {
            None
        }
    }
}

impl<A: Grid, B: Grid> PartialEq<View<&B>> for View<&A>
where
    A::Item: PartialEq<B::Item>,
{
    #[inline]
    fn eq(&self, other: &View<&B>) -> bool {
        self.eq_grid(other)
    }
}

impl<G: Grid> Debug for View<&G>
where
    G::Item: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}
