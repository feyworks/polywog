use serde::{Deserialize, Serialize};
use strum::FromRepr;

/// A sampler type to be used by shaders.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub struct Sampler {
    /// Horizontal address mode.
    pub address_x: AddressMode,

    /// Vertical address mode.
    pub address_y: AddressMode,

    /// Filter mode when texture is scaling down.
    pub min_filter: FilterMode,

    /// Filter mode when texture is scaling up.
    pub mag_filter: FilterMode,
}

impl Sampler {
    /// All possible sampler variants.
    #[inline]
    pub fn all() -> Vec<Self> {
        let mut samplers = Vec::new();
        for ax in AddressMode::VARIANTS {
            for ay in AddressMode::VARIANTS {
                for min in FilterMode::VARIANTS {
                    for mag in FilterMode::VARIANTS {
                        samplers.push(Self::new(ax, ay, min, mag));
                    }
                }
            }
        }
        samplers
    }

    /// Create a new sampler.
    #[inline]
    pub const fn new(
        address_x: AddressMode,
        address_y: AddressMode,
        min_filter: FilterMode,
        mag_filter: FilterMode,
    ) -> Self {
        Self {
            address_x,
            address_y,
            min_filter,
            mag_filter,
        }
    }

    /// Create a uniform x/y min/mag sampler.
    #[inline]
    pub const fn with(address: AddressMode, filter: FilterMode) -> Self {
        Self::new(address, address, filter, filter)
    }

    /// Create a clamped sampler with the selected filter mode.
    #[inline]
    pub const fn clamp(filter: FilterMode) -> Self {
        Self::with(AddressMode::Clamp, filter)
    }

    /// Create a repeating sampler with the selected filter mode.
    #[inline]
    pub const fn repeat(filter: FilterMode) -> Self {
        Self::with(AddressMode::Repeat, filter)
    }

    /// Create a nearest sampler with the selected address mode.
    #[inline]
    pub const fn nearest(address: AddressMode) -> Self {
        Self::with(address, FilterMode::Nearest)
    }

    /// Create a linear sampler with the selected address mode.
    #[inline]
    pub const fn linear(address: AddressMode) -> Self {
        Self::with(address, FilterMode::Linear)
    }
}

/// How edges should be handled in texture addressing.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum AddressMode {
    #[default]
    Clamp,
    Repeat,
    MirrorRepeat,
}

impl Into<wgpu::AddressMode> for AddressMode {
    #[inline]
    fn into(self) -> wgpu::AddressMode {
        match self {
            Self::Clamp => wgpu::AddressMode::ClampToEdge,
            Self::Repeat => wgpu::AddressMode::Repeat,
            Self::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

impl AddressMode {
    pub const VARIANTS: [Self; 3] = [Self::Clamp, Self::Repeat, Self::MirrorRepeat];
}

/// Texel mixing mode when sampling between texels.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    FromRepr,
)]
pub enum FilterMode {
    #[default]
    Nearest,
    Linear,
}

impl Into<wgpu::FilterMode> for FilterMode {
    #[inline]
    fn into(self) -> wgpu::FilterMode {
        match self {
            Self::Nearest => wgpu::FilterMode::Nearest,
            Self::Linear => wgpu::FilterMode::Linear,
        }
    }
}

impl FilterMode {
    pub const VARIANTS: [Self; 2] = [Self::Nearest, Self::Linear];
}
