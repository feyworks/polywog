use bytemuck::{Pod, Zeroable};
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Per-fragment mode controlling how the shader blends colors.
///
/// The effect of each mode is applied in the shader's `apply_mode()` method:
///
/// ```wgsl
/// fn apply_mode(pixel: vec4f, color: vec4f, mode: vec4f) -> vec4f {
///     return
///         (mode.x * pixel * color) +   // mult
///         (mode.y * pixel.a * color) + // wash
///         (mode.z * color);            // veto
/// }
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Zeroable, Pod)]
pub struct ColorMode {
    pub mult: u8,
    pub wash: u8,
    pub veto: u8,
    pub misc: u8,
}

impl ColorMode {
    /// The pixel's value is multiplied by the vertex color.
    pub const MULT: Self = Self::new(255, 0, 0, 0);

    /// The pixel's alpha value is multiplied by the vertex color. This means when using this mode,
    /// the sprite will be completely silhouetted in the provided vertex color.
    pub const WASH: Self = Self::new(0, 255, 0, 0);

    /// The pixel is ignored and overwritten by the vertex color completely. This is used to draw
    /// colored geometry when we want to ignore the texture coords completey.
    pub const VETO: Self = Self::new(0, 0, 255, 0);

    /// Currently unused.
    pub const MISC: Self = Self::new(0, 0, 0, 255);

    /// Create a new custom color mode (you usually won't use this).
    #[inline]
    pub const fn new(mult: u8, wash: u8, veto: u8, misc: u8) -> Self {
        Self {
            mult,
            wash,
            veto,
            misc,
        }
    }
}

impl Default for ColorMode {
    #[inline]
    fn default() -> Self {
        Self::MULT
    }
}

impl BitOr for ColorMode {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(
            self.mult.bitor(rhs.mult),
            self.wash.bitor(rhs.wash),
            self.veto.bitor(rhs.veto),
            self.misc.bitor(rhs.misc),
        )
    }
}

impl BitAnd for ColorMode {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(
            self.mult.bitand(rhs.mult),
            self.wash.bitand(rhs.wash),
            self.veto.bitand(rhs.veto),
            self.misc.bitand(rhs.misc),
        )
    }
}

impl BitXor for ColorMode {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(
            self.mult.bitxor(rhs.mult),
            self.wash.bitxor(rhs.wash),
            self.veto.bitxor(rhs.veto),
            self.misc.bitxor(rhs.misc),
        )
    }
}

impl Not for ColorMode {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self::new(!self.mult, !self.wash, !self.veto, !self.misc)
    }
}
