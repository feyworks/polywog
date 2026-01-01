use serde::{Deserialize, Serialize};
use wgpu::PrimitiveTopology;

/// Format of your rendering primitives.
///
/// This determines what format the shader is expecting your index buffers to be, and the
/// rendering mode used by the shader pipeline.
///
/// You will only use this when calling [`Draw::custom()`](super::Draw::custom) or
/// [`Draw::buffers()`](super::Draw::buffers).
#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Topology {
    /// Indices are a list of vertex triplets representing triangles with clockwise winding.
    #[default]
    Triangles,

    /// Indices are a list of vertex pairs representing the start and end point of lines.
    Lines,

    /// Indices are a list of individual vertex points.
    Points,
}

impl Into<PrimitiveTopology> for Topology {
    #[inline]
    fn into(self) -> PrimitiveTopology {
        match self {
            Self::Points => PrimitiveTopology::PointList,
            Self::Lines => PrimitiveTopology::LineList,
            Self::Triangles => PrimitiveTopology::TriangleList,
        }
    }
}
