use fey_math::Vec2;
use std::fmt::Debug;
use std::hash::Hash;

/// Wraps the inner coordinate (or coord component) in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wrap<C>(pub C);

/// Wraps the x value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WrapX<C: Coord>(pub C);

/// Wraps the y value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WrapY<C: Coord>(pub C);

/// Clamps the inner coordinate (or coord component) in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Clamp<C>(pub C);

/// Clamps the x value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClampX<C: Coord>(pub C);

/// Clamps the y value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClampY<C: Coord>(pub C);

/// An x or y component of a grid coordinate.
pub trait CoordComponent: Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash {
    /// Convert this coordinate from its normal form to a position on the grid,
    /// where `len` is the grid's size on this coordinate's axis.
    fn to_grid(self, len: u32) -> Option<u32>;
}

macro_rules! impl_comp_unsigned {
    ($($type:ty)*) => {
        $(
        impl CoordComponent for $type {
            #[inline]
            fn to_grid(self, len: u32) -> Option<u32> {
                let x = self as u32;
                (x < len).then_some(x)
            }
        }

        impl CoordComponent for Wrap<$type> {
            #[inline]
            fn to_grid(self, len: u32) -> Option<u32> {
                (self.0 as u32).checked_rem(len)
            }
        }

        impl CoordComponent for Clamp<$type> {
            #[inline]
            fn to_grid(self, len: u32) -> Option<u32> {
                len.checked_sub(1).map(|max| max.min(self.0 as u32))
            }
        }
        )*
    }
}

macro_rules! impl_comp_signed {
    ($($type:ty)*) => {
        $(
        impl CoordComponent for $type {
            #[inline]
            fn to_grid(self, len: u32) -> Option<u32> {
                if self < 0 {
                    return None;
                }
                let x = self as u32;
                (x < len).then_some(x)
            }
        }

        impl CoordComponent for Wrap<$type> {
            #[inline]
            fn to_grid(self, len: u32) -> Option<u32> {
                (self.0 as u32).wrapping_add(len).checked_rem(len)
            }
        }

        impl CoordComponent for Clamp<$type> {
            #[inline]
            fn to_grid(self, len: u32) -> Option<u32> {
                if self.0 < 0 {
                    return Some(0);
                }
                len.checked_sub(1).map(|max| max.min(self.0 as u32))
            }
        }
        )*
    }
}

impl_comp_unsigned!(u8 u16 u32 u64 usize);
impl_comp_signed!(i8 i16 i32 i64 isize);

pub trait Coord {
    type X: CoordComponent;
    type Y: CoordComponent;

    fn x(&self) -> Self::X;
    fn y(&self) -> Self::Y;
}

impl<X: CoordComponent, Y: CoordComponent> Coord for (X, Y) {
    type X = X;
    type Y = Y;

    #[inline]
    fn x(&self) -> Self::X {
        self.0
    }

    #[inline]
    fn y(&self) -> Self::Y {
        self.1
    }
}

impl<X: CoordComponent, Y: CoordComponent> Coord for &(X, Y) {
    type X = X;
    type Y = Y;

    #[inline]
    fn x(&self) -> Self::X {
        self.0
    }

    #[inline]
    fn y(&self) -> Self::Y {
        self.1
    }
}

impl<C: Coord> Coord for Wrap<C>
where
    Wrap<C::X>: CoordComponent,
    Wrap<C::Y>: CoordComponent,
{
    type X = Wrap<C::X>;
    type Y = Wrap<C::Y>;

    #[inline]
    fn x(&self) -> Self::X {
        Wrap(self.0.x())
    }

    #[inline]
    fn y(&self) -> Self::Y {
        Wrap(self.0.y())
    }
}

impl<C: Coord> Coord for WrapX<C>
where
    Wrap<C::X>: CoordComponent,
{
    type X = Wrap<C::X>;
    type Y = C::Y;

    #[inline]
    fn x(&self) -> Self::X {
        Wrap(self.0.x())
    }

    #[inline]
    fn y(&self) -> Self::Y {
        self.0.y()
    }
}

impl<C: Coord> Coord for WrapY<C>
where
    Wrap<C::Y>: CoordComponent,
{
    type X = C::X;
    type Y = Wrap<C::Y>;

    #[inline]
    fn x(&self) -> Self::X {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Y {
        Wrap(self.0.y())
    }
}

impl<C: Coord> Coord for Clamp<C>
where
    Clamp<C::X>: CoordComponent,
    Clamp<C::Y>: CoordComponent,
{
    type X = Clamp<C::X>;
    type Y = Clamp<C::Y>;

    #[inline]
    fn x(&self) -> Self::X {
        Clamp(self.0.x())
    }

    #[inline]
    fn y(&self) -> Self::Y {
        Clamp(self.0.y())
    }
}

impl<C: Coord> Coord for ClampX<C>
where
    Clamp<C::X>: CoordComponent,
{
    type X = Clamp<C::X>;
    type Y = C::Y;

    #[inline]
    fn x(&self) -> Self::X {
        Clamp(self.0.x())
    }

    #[inline]
    fn y(&self) -> Self::Y {
        self.0.y()
    }
}

impl<C: Coord> Coord for ClampY<C>
where
    Clamp<C::Y>: CoordComponent,
{
    type X = C::X;
    type Y = Clamp<C::Y>;

    #[inline]
    fn x(&self) -> Self::X {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Y {
        Clamp(self.0.y())
    }
}

impl<T: CoordComponent> Coord for Vec2<T> {
    type X = T;
    type Y = T;

    #[inline]
    fn x(&self) -> Self::X {
        self.x
    }

    #[inline]
    fn y(&self) -> Self::Y {
        self.y
    }
}
