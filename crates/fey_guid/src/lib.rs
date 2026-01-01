//! A 128-bit globally unique identifier.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

/// A 128-bit globally unique identifier.
///
/// This can be used to generate unique IDs on the fly, and serialize to the form
/// `"a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"`.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Guid(Uuid);

impl Guid {
    /// An ID equal to `"00000000-0000-0000-0000-000000000000"`.
    pub const ZERO: Self = Self(Uuid::nil());

    /// Generate a new random ID.
    #[inline]
    pub fn new() -> Self {
        Self::from_bytes(rand::random())
    }

    /// Generate a new ID using the provided RNG.
    #[inline]
    pub fn from_rng<R: Rng>(rng: &mut R) -> Self {
        Self(Uuid::from_bytes(rng.random()))
    }

    /// Losslessly convert a 128-bit unsigned integer to an ID.
    #[inline]
    pub const fn from_u128(val: u128) -> Self {
        Self(Uuid::from_u128(val))
    }

    /// Losslessly convert 16 bytes to an ID.
    #[inline]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(Uuid::from_bytes(bytes))
    }

    /// The ID as an array of bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    /// The ID as a 128-bit unsigned integer.
    #[inline]
    pub fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    /// Parse an ID string of the form `"a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"`.
    #[inline]
    pub fn parse_str(s: &str) -> Result<Self, GuidParseError> {
        Uuid::parse_str(s).map(Self).map_err(GuidParseError)
    }

    /// Encode the ID into a string of the form `"a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"`.
    #[inline]
    pub fn encode_str<'a>(&self, buf: &'a mut [u8; 36]) -> &'a str {
        self.0.as_hyphenated().encode_lower(buf)
    }
}

impl Debug for Guid {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.0.as_hyphenated(), f)
    }
}

impl Display for Guid {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.0.as_hyphenated(), f)
    }
}

/// An error parsing a `Guid` string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GuidParseError(uuid::Error);

impl std::error::Error for GuidParseError {}

impl Display for GuidParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
