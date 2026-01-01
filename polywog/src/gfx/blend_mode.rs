use serde::{Deserialize, Serialize};
use strum::{EnumCount, FromRepr, VariantArray};
use wgpu::{BlendComponent, BlendFactor, BlendOperation, BlendState};

/// Different blend mode types.
#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    EnumCount,
    FromRepr,
    VariantArray,
)]
pub enum BlendMode {
    /// Pixels are alpha-composited into the backdrop.
    #[default]
    Normal,

    /// Pixels are additively blended into the backdrop.
    Add,

    /// Pixels are subtractively blended from the backdrop.
    Subtract,

    /// Pixels are multiplicatively blended into the backdrop.
    Multiply,
}

impl Into<BlendState> for BlendMode {
    fn into(self) -> BlendState {
        match self {
            Self::Normal => BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
            },
            Self::Add => BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
            },
            Self::Subtract => BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::ReverseSubtract,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
            },
            Self::Multiply => BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::Dst,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::Dst,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
            },
        }
    }
}
