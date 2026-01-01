use crate::{Cardinal, Degrees, Float, Octal, Rotations, impl_angle};
use serde::{Deserialize, Serialize};

pub type RadiansF = Radians<f32>;

/// An angle represented in radians.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Radians<T>(pub T);

impl_angle!(Radians, from_radians, to_radians);

/// Create a [`Radians`].
#[inline]
pub const fn rads<T>(value: T) -> Radians<T> {
    Radians(value)
}

impl<T: Float> Radians<T> {
    /// Convert from radians.
    #[inline]
    pub fn from_radians(value: Radians<T>) -> Self {
        value
    }

    /// Convert from degrees.
    #[inline]
    pub fn from_degrees(value: Degrees<T>) -> Self {
        Self(T::to_radians(value.0))
    }

    /// Convert from rotations.
    #[inline]
    pub fn from_rotations(value: Rotations<T>) -> Self {
        Self(value.0 * T::TAU)
    }

    /// Convert from a cardinal direction.
    #[inline]
    pub fn from_cardinal(value: Cardinal) -> Self {
        Self(match value {
            Cardinal::East => T::ZERO,
            Cardinal::South => T::PI_OVER_2,
            Cardinal::West => T::PI,
            Cardinal::North => -T::PI_OVER_2,
        })
    }

    /// Convert from an octal direction.
    #[inline]
    pub fn from_octal(value: Octal) -> Self {
        Self(match value {
            Octal::East => T::ZERO,
            Octal::SouthEast => T::PI_OVER_4,
            Octal::South => T::PI_OVER_2,
            Octal::SouthWest => T::PI_OVER_2 + T::PI_OVER_4,
            Octal::West => T::PI,
            Octal::NorthWest => -T::PI_OVER_4,
            Octal::North => -T::PI_OVER_4,
            Octal::NorthEast => -(T::PI_OVER_2 + T::PI_OVER_4),
        })
    }

    /// Wraps the angle into the range `(-PI, PI]`.
    #[inline]
    pub fn wrap(self) -> Self {
        let angle = self.0;
        Self(if angle > -T::PI && angle <= T::PI {
            angle
        } else {
            let angle = angle % T::TAU;
            if angle <= -T::PI {
                angle + T::TAU
            } else if angle > T::PI {
                angle - T::TAU
            } else {
                angle
            }
        })
    }
}
