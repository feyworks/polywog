use crate::color::Rgba8;
use crate::gfx::ColorMode;
use crate::math::Vec2F;
use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

/// A vertex passed to the shader when rendering.
///
/// You will only need to use this if you are manually drawing geometry or populating vertex
/// buffers. For the most part, [`Draw`](super::Draw) methods will generate vertices for you.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Zeroable, Pod)]
pub struct Vertex {
    /// The position.
    pub pos: Vec2F,

    /// The texture coordinate.
    pub tex: Vec2F,

    /// The color.
    pub col: Rgba8,

    /// The color mode.
    pub mode: ColorMode,
}

impl Vertex {
    pub(crate) const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 8,
                shader_location: 1,
            },
            VertexAttribute {
                format: VertexFormat::Unorm8x4,
                offset: 16,
                shader_location: 2,
            },
            VertexAttribute {
                format: VertexFormat::Unorm8x4,
                offset: 20,
                shader_location: 3,
            },
        ],
    };

    /// Create a new vertex.
    #[inline]
    pub const fn new(pos: Vec2F, tex: Vec2F, col: Rgba8, mode: ColorMode) -> Self {
        Self {
            pos,
            tex,
            col,
            mode,
        }
    }

    /// Create a simple [`MULT`](ColorMode::MULT) vertex.
    #[inline]
    pub const fn simple(pos: Vec2F, tex: Vec2F) -> Self {
        Self::new(pos, tex, Rgba8::WHITE, ColorMode::MULT)
    }

    /// Create a [`MULT`](ColorMode::MULT) vertex.
    #[inline]
    pub const fn mult(pos: Vec2F, tex: Vec2F, col: Rgba8) -> Self {
        Self::new(pos, tex, col, ColorMode::MULT)
    }

    /// Create a [`WASH`](ColorMode::WASH) vertex.
    #[inline]
    pub const fn wash(pos: Vec2F, tex: Vec2F, col: Rgba8) -> Self {
        Self::new(pos, tex, col, ColorMode::WASH)
    }

    /// Create a [`VETO`](ColorMode::VETO) vertex.
    #[inline]
    pub const fn veto(pos: Vec2F, col: Rgba8) -> Self {
        Self::new(pos, Vec2F::ZERO, col, ColorMode::VETO)
    }

    /// Create a [`MISC`](ColorMode::MISC) vertex.
    #[inline]
    pub const fn misc(pos: Vec2F, tex: Vec2F, col: Rgba8) -> Self {
        Self::new(pos, tex, col, ColorMode::MISC)
    }
}
