use crate::{Degrees, Direction, Float, Octal, Radians, Rotations, Signed, Vec2, impl_direction};
use serde::{Deserialize, Serialize};

/// A cardinal (4-way) direction.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cardinal {
    East,
    South,
    West,
    North,
}

impl_direction!(Cardinal, 4, East, South, West, North);

impl<T: Float> Direction<T> for Cardinal {
    #[inline]
    fn to_degrees(self) -> Degrees<T> {
        Degrees::from_cardinal(self)
    }

    #[inline]
    fn to_radians(self) -> Radians<T> {
        Radians::from_cardinal(self)
    }

    #[inline]
    fn to_rotations(self) -> Rotations<T> {
        Rotations::from_cardinal(self)
    }

    #[inline]
    fn from_vec2(v: Vec2<T>) -> Self {
        match (v.x > T::ZERO, v.y > T::ZERO, T::abs(v.x) > T::abs(v.y)) {
            (true, true, true) | (true, false, true) => Self::East,
            (true, true, false) | (false, true, false) => Self::South,
            (false, true, true) | (false, false, true) => Self::West,
            (true, false, false) | (false, false, false) => Self::North,
        }
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
            Self::South => Vec2::SOUTH,
            Self::West => Vec2::WEST,
            Self::North => Vec2::NORTH,
        }
    }

    #[inline]
    fn to_cardinal(self, _bias: Cardinal) -> Cardinal {
        self
    }

    #[inline]
    fn to_octal(self) -> Octal {
        Octal::from_cardinal(self)
    }
}

impl Cardinal {
    /// Reverse the direction.
    #[inline]
    pub const fn rev(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
        }
    }

    /// Given an octal direction, return the cardinal direction that most
    /// closely represents it. This is effectively snapping the angle to 90°.
    ///
    /// The bias provides a direction to be used as a tiebreaker; when an octal
    /// direction is equidistant between two cardinal direction (eg. `NorthEast`
    /// is equally close to `North` and `East`), the direction nearest to the bias
    /// will be chosen.
    #[inline]
    pub fn from_octal(oct: Octal, bias: Self) -> Self {
        match oct {
            Octal::East => Self::East,
            Octal::South => Self::South,
            Octal::West => Self::West,
            Octal::North => Self::North,
            Octal::SouthEast => match bias {
                Self::West => Self::South,
                Self::North => Self::East,
                bias => bias,
            },
            Octal::SouthWest => match bias {
                Self::East => Self::South,
                Self::North => Self::West,
                bias => bias,
            },
            Octal::NorthWest => match bias {
                Self::East => Self::North,
                Self::South => Self::West,
                bias => bias,
            },
            Octal::NorthEast => match bias {
                Self::West => Self::North,
                Self::South => Self::East,
                bias => bias,
            },
        }
    }

    /// Returns the cardinal direction 90° clockwise of this one.
    #[inline]
    pub fn cw(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }

    /// Returns the cardinal direction 90° counter-clockwise of this one.
    #[inline]
    pub fn ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }

    /// Get the direction as an offset vector for a grid.
    #[inline]
    pub const fn grid_step<T: Signed>(self) -> Vec2<T> {
        match self {
            Self::East => Vec2::EAST,
            Self::South => Vec2::SOUTH,
            Self::West => Vec2::WEST,
            Self::North => Vec2::NORTH,
        }
    }
}
