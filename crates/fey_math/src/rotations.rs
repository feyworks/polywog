use crate::{Cardinal, Degrees, Float, Octal, Radians, impl_angle};
use serde::{Deserialize, Serialize};

pub type RotationsF = Rotations<f32>;

/// An angle represented in rotations.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Rotations<T>(pub T);

impl_angle!(Rotations, from_rotations, to_rotations);

/// Create a [`Rotations`].
#[inline]
pub const fn rots<T>(value: T) -> Rotations<T> {
    Rotations(value)
}

impl<T: Float> Rotations<T> {
    /// A 360° turn.
    pub const ONE: Self = Self(T::ONE);

    /// A 180° turn.
    pub const HALF: Self = Self(T::HALF);

    /// A 90° turn.
    pub const QUARTER: Self = Self(T::QUARTER);

    /// Convert from radians.
    #[inline]
    pub fn from_radians(value: Radians<T>) -> Self {
        Self(value.0 / T::TAU)
    }

    /// Convert from degrees.
    #[inline]
    pub fn from_degrees(value: Degrees<T>) -> Self {
        Self(value.0 / T::NUM_360)
    }

    /// Convert from rotations.
    #[inline]
    pub fn from_rotations(value: Rotations<T>) -> Self {
        value
    }

    /// Convert from a cardinal direction.
    #[inline]
    pub fn from_cardinal(value: Cardinal) -> Self {
        Self(match value {
            Cardinal::East => T::ZERO,
            Cardinal::South => T::QUARTER,
            Cardinal::West => T::HALF,
            Cardinal::North => -T::QUARTER,
        })
    }

    /// Convert from an octal direction.
    #[inline]
    pub fn from_octal(value: Octal) -> Self {
        Self(match value {
            Octal::East => T::ZERO,
            Octal::SouthEast => T::EIGHTH,
            Octal::South => T::QUARTER,
            Octal::SouthWest => T::QUARTER + T::EIGHTH,
            Octal::West => T::HALF,
            Octal::NorthWest => -T::EIGHTH,
            Octal::North => -T::QUARTER,
            Octal::NorthEast => -(T::QUARTER + T::EIGHTH),
        })
    }

    /// Wraps the angle into the range `(-0.5,0.5]`.
    #[inline]
    pub fn wrap(self) -> Self {
        let angle = self.0;
        Self(if angle > -T::HALF && angle <= T::HALF {
            angle
        } else {
            let angle = angle % T::ONE;
            if angle <= -T::HALF {
                angle + T::ONE
            } else if angle > T::HALF {
                angle - T::ONE
            } else {
                angle
            }
        })
    }
}
