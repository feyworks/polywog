use gilrs::PowerInfo;
use serde::{Deserialize, Serialize};

/// Charging status of a gamepad.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GamepadStatus {
    /// Status is not known.
    Unknown,

    /// Gamepad is wired.
    Wired,

    /// Gamepad is draining (contains the battery charge %).
    Draining(u8),

    /// Gamepad is charging (contains the battery charge %).
    Charging(u8),

    /// Gamepad is charged.
    Charged,
}

impl From<PowerInfo> for GamepadStatus {
    #[inline]
    fn from(value: PowerInfo) -> Self {
        match value {
            PowerInfo::Unknown => Self::Unknown,
            PowerInfo::Wired => Self::Wired,
            PowerInfo::Discharging(p) => Self::Draining(p),
            PowerInfo::Charging(p) => Self::Charging(p),
            PowerInfo::Charged => Self::Charged,
        }
    }
}
