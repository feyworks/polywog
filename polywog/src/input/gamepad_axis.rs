use gilrs::Axis;
use strum::{EnumCount, FromRepr, VariantArray};

/// A gamepad axis.
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, EnumCount, FromRepr, VariantArray,
)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    DPadX,
    DPadY,
}

impl TryFrom<Axis> for GamepadAxis {
    type Error = ();

    #[inline]
    fn try_from(value: Axis) -> Result<Self, Self::Error> {
        Ok(match value {
            Axis::LeftStickX => Self::LeftX,
            Axis::LeftStickY => Self::LeftY,
            Axis::RightStickX => Self::RightX,
            Axis::RightStickY => Self::RightY,
            Axis::DPadX => Self::DPadX,
            Axis::DPadY => Self::DPadY,
            _ => return Err(()),
        })
    }
}
