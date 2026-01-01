//! Types for working with colors.

mod channel;
mod conversion_traits;
mod grey;
mod grey_alpha;
mod hsl;
mod hsv;
mod oklab;
mod rgb;
mod rgba;

pub use channel::*;
pub use conversion_traits::*;
pub use grey::*;
pub use grey_alpha::*;
pub use hsl::*;
pub use hsv::*;
pub use oklab::*;
pub use rgb::*;
pub use rgba::*;

/// Create an `Rgba8` color from a packed `RGB` integer.
#[inline]
pub const fn rgb(packed: u32) -> Rgba8 {
    Rgba8::new((packed >> 16) as u8, (packed >> 8) as u8, packed as u8, 255)
}

/// Create an `Rgba8` color from a packed `RGBA` integer.
#[inline]
pub const fn rgba(packed: u32) -> Rgba8 {
    Rgba8::new(
        (packed >> 24) as u8,
        (packed >> 16) as u8,
        (packed >> 8) as u8,
        packed as u8,
    )
}

/// Create an `Rgba8` color from a packed `ABGR` integer.
#[inline]
pub const fn abgr(packed: u32) -> Rgba8 {
    Rgba8::new(
        packed as u8,
        (packed >> 8) as u8,
        (packed >> 16) as u8,
        (packed >> 24) as u8,
    )
}
