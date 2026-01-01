use crate::{
    Cardinal, Degrees, Direction, Float, Radians, Rotations, Signed, Vec2, impl_direction, vec2,
};
use serde::{Deserialize, Serialize};

/// An octal (8-way) direction.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Octal {
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    North,
    NorthEast,
}

impl_direction!(
    Octal, 8, East, SouthEast, South, SouthWest, West, NorthWest, North, NorthEast
);

impl<T: Float> Direction<T> for Octal {
    #[inline]
    fn to_degrees(self) -> Degrees<T> {
        Degrees::from_octal(self)
    }

    #[inline]
    fn to_radians(self) -> Radians<T> {
        Radians::from_octal(self)
    }

    #[inline]
    fn to_rotations(self) -> Rotations<T> {
        Rotations::from_octal(self)
    }

    #[inline]
    fn from_vec2(v: Vec2<T>) -> Self {
        let mut min_dist = T::MAX;
        let mut min_i = 0;
        for i in 0..8 {
            let dist = v.sqr_dist(Vec2::OCTAL_DIRS[i]);
            if dist < min_dist {
                min_dist = dist;
                min_i = i;
            }
        }
        Self::VARIANTS[min_i]
    }

    #[inline]
    fn sin_cos(self) -> (T, T) {
        let norm = self.norm();
        (norm.y, norm.x)
    }

    #[inline]
    fn norm(self) -> Vec2<T> {
        match self {
            Self::East => Vec2::EAST,
            Self::SouthEast => Vec2::SOUTH_EAST,
            Self::South => Vec2::SOUTH,
            Self::SouthWest => Vec2::SOUTH_WEST,
            Self::West => Vec2::WEST,
            Self::NorthWest => Vec2::NORTH_WEST,
            Self::North => Vec2::NORTH,
            Self::NorthEast => Vec2::NORTH_EAST,
        }
    }

    #[inline]
    fn to_cardinal(self, bias: Cardinal) -> Cardinal {
        Cardinal::from_octal(self, bias)
    }

    #[inline]
    fn to_octal(self) -> Octal {
        self
    }
}

impl Octal {
    /// Convert from a cardinal direction.
    #[inline]
    pub fn from_cardinal(dir: Cardinal) -> Self {
        match dir {
            Cardinal::East => Self::East,
            Cardinal::South => Self::South,
            Cardinal::West => Self::West,
            Cardinal::North => Self::North,
        }
    }

    // /// Given an angle, return the octal direction that most closely represents
    // /// it. This is effectively snapping the angle to 45°.
    // #[inline]
    // pub fn from_degrees<T: Float>(deg: T) -> Self {
    //     let max_diff = T::NUM_45 * T::HALF;
    //     let near = |a: T| T::abs(deg - a) < max_diff;
    //     if near(T::ZERO) {
    //         Self::East
    //     } else if near(T::NUM_45) {
    //         Self::SouthEast
    //     } else if near(T::NUM_90) {
    //         Self::South
    //     } else if near(T::NUM_135) {
    //         Self::SouthWest
    //     } else if near(T::NUM_180) {
    //         Self::West
    //     } else if near(-T::NUM_135) {
    //         Self::NorthWest
    //     } else if near(-T::NUM_90) {
    //         Self::North
    //     } else {
    //         Self::NorthEast
    //     }
    // }

    /// Reverse the direction.
    #[inline]
    pub const fn rev(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
        }
    }

    /// Returns the octal direction 45° clockwise of this one.
    #[inline]
    pub fn cw(self) -> Self {
        match self {
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
        }
    }

    /// Returns the octal direction 45° counter-clockwise of this one.
    #[inline]
    pub fn ccw(self) -> Self {
        match self {
            Self::East => Self::NorthEast,
            Self::SouthEast => Self::East,
            Self::South => Self::SouthEast,
            Self::SouthWest => Self::South,
            Self::West => Self::SouthWest,
            Self::NorthWest => Self::West,
            Self::North => Self::NorthWest,
            Self::NorthEast => Self::North,
        }
    }

    /// Get the direction as an offset vector for a grid.
    #[inline]
    pub const fn grid_step<T: Signed>(self) -> Vec2<T> {
        match self {
            Self::East => Vec2::EAST,
            Self::SouthEast => vec2(T::ONE, T::ONE),
            Self::South => Vec2::SOUTH,
            Self::SouthWest => vec2(T::NEG_ONE, T::ONE),
            Self::West => Vec2::WEST,
            Self::NorthWest => vec2(T::NEG_ONE, T::NEG_ONE),
            Self::North => Vec2::NORTH,
            Self::NorthEast => vec2(T::ONE, T::NEG_ONE),
        }
    }
}

impl From<Cardinal> for Octal {
    #[inline]
    fn from(value: Cardinal) -> Self {
        Self::from_cardinal(value)
    }
}
