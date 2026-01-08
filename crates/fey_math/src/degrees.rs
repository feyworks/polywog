use crate::{Cardinal, Float, Octal, Radians, Rotations, impl_angle};
use serde::{Deserialize, Serialize};

pub type DegreesF = Degrees<f32>;

/// An angle represented in degrees.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Degrees<T>(pub T);

impl_angle!(Degrees, from_degrees, to_degrees);

/// Create a [`Degrees`].
#[inline]
pub const fn degs<T>(value: T) -> Degrees<T> {
    Degrees(value)
}

impl<T: Float> Degrees<T> {
    /// Convert from radians.
    #[inline]
    pub fn from_radians(value: Radians<T>) -> Self {
        Self(T::to_degrees(value.0))
    }

    /// Convert from degrees.
    #[inline]
    pub fn from_degrees(value: Degrees<T>) -> Self {
        value
    }

    /// Convert from rotations.
    #[inline]
    pub fn from_rotations(value: Rotations<T>) -> Self {
        Self(value.0 * T::NUM_360)
    }

    /// Convert from a cardinal direction.
    #[inline]
    pub fn from_cardinal(value: Cardinal) -> Self {
        Self(match value {
            Cardinal::East => T::ZERO,
            Cardinal::South => T::NUM_90,
            Cardinal::West => T::NUM_180,
            Cardinal::North => -T::NUM_90,
        })
    }

    /// Convert from an octal direction.
    #[inline]
    pub fn from_octal(value: Octal) -> Self {
        Self(match value {
            Octal::East => T::ZERO,
            Octal::SouthEast => T::NUM_45,
            Octal::South => T::NUM_90,
            Octal::SouthWest => T::NUM_135,
            Octal::West => T::NUM_180,
            Octal::NorthWest => -T::NUM_45,
            Octal::North => -T::NUM_90,
            Octal::NorthEast => -T::NUM_135,
        })
    }

    /// Wraps the angle into the range `(-180, 180]`.
    #[inline]
    pub fn wrap(self) -> Self {
        let angle = self.0;
        Self(if angle > -T::NUM_180 && angle <= T::NUM_180 {
            angle
        } else {
            let angle = angle % T::NUM_360;
            if angle <= -T::NUM_180 {
                angle + T::NUM_360
            } else if angle > T::NUM_180 {
                angle - T::NUM_360
            } else {
                angle
            }
        })
    }
}
