use crate::Sprite;
use fey_ase::LoopDir;
use kero::prelude::*;
use serde::{Deserialize, Serialize};

/// A renderable sprite animation.
///
/// When rendering an anim, you can choose which frame and which layers of
/// the animation to show. This will draw all the *cels* for that frame/layer
/// combo in order, composing a sprite. This allows you to toggle certain
/// layers on/off, being able to do things like swappable equipment, or drawing
/// certain layers on different render phases (eg. a glow phase).
#[derive(Clone, Debug)]
pub struct SpriteAnim {
    pub size: Vec2F,
    pub frames: Vec<AnimFrame>,
    pub sprites: Vec<Sprite>,
    pub tags: Vec<AnimTag>,
    pub layers: Vec<AnimLayer>,
}

impl Default for SpriteAnim {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteAnim {
    /// Create a new sprite animation.
    #[inline]
    pub fn new() -> Self {
        Self {
            size: Vec2::ZERO,
            frames: Vec::new(),
            sprites: Vec::new(),
            tags: Vec::new(),
            layers: Vec::new(),
        }
    }

    #[inline]
    pub fn tag(&self, name: &str) -> Option<&AnimTag> {
        self.tags.iter().find(|t| t.name == name)
    }

    #[inline]
    pub fn tag_at(&self, frame: usize) -> Option<&AnimTag> {
        self.tags.iter().find(|t| frame >= t.from && frame <= t.to)
    }

    #[inline]
    pub fn draw_flipped(
        &self,
        draw: &mut Draw,
        frame_index: usize,
        layers: u64,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        let pos = pos.into();
        let flip = flip.into();
        let f = &self.frames[frame_index % self.frames.len()];
        for cel in &f.cels {
            if (layers & (1 << cel.layer)) != 0 {
                self.sprites[cel.index].draw_flipped(draw, pos, color, mode, flip);
            }
        }
    }

    #[inline]
    pub fn draw_ext(
        &self,
        draw: &mut Draw,
        frame_index: usize,
        pos: impl Into<Vec2F>,
        layers: u64,
        color: Rgba8,
        mode: ColorMode,
    ) {
        let pos = pos.into();
        let f = &self.frames[frame_index % self.frames.len()];
        for cel in &f.cels {
            if (layers & (1 << cel.layer)) != 0 {
                self.sprites[cel.index].draw_ext(draw, pos, color, mode);
            }
        }
    }

    #[inline]
    pub fn draw(&self, draw: &mut Draw, frame_index: usize, pos: impl Into<Vec2F>) {
        self.draw_ext(
            draw,
            frame_index,
            pos,
            u64::MAX,
            Rgba8::WHITE,
            ColorMode::MULT,
        );
    }

    /// Index of the layer with the name.
    #[inline]
    pub fn layer_idx(&self, name: &str) -> Option<usize> {
        self.layers.iter().position(|layer| layer.name == name)
    }

    /// Mask of the layer with the name.
    #[inline]
    pub fn layer_mask(&self, name: &str) -> Option<u64> {
        self.layer_idx(name).map(|idx| 1u64 << (idx as u64))
    }

    /// Combined mask of several layers.
    #[inline]
    pub fn layer_masks<T: AsRef<str>>(&self, names: &[T]) -> u64 {
        let mut mask = 0;
        for name in names {
            if let Some(m) = self.layer_mask(name.as_ref()) {
                mask |= m
            }
        }
        mask
    }
}

/// A frame of the sprite animation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimFrame {
    /// How long, in seconds, the frame lasts.
    pub duration: f32,

    /// Cels that are visible on this frame.
    pub cels: Vec<AnimCel>,
}

/// A cel of an animation frame.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimCel {
    /// The cel's layer.
    pub layer: usize,

    /// The cel's sprite index.
    pub index: usize,
}

/// A tag representing an animatable region of frames.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimTag {
    /// The animation's name.
    pub name: String,

    /// First frame of the animation.
    pub from: usize,

    /// Last frame of the animation.
    pub to: usize,

    /// How to animate the region.
    pub dir: AnimDir,
}

impl AnimTag {
    #[inline]
    pub fn len(&self) -> usize {
        (self.to + 1) - self.from
    }
}

/// How an animation should play.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum AnimDir {
    /// Play from the first frame to the last.
    Forward,

    /// Play from the last frame to the first.
    Reverse,

    /// Alternate between [`Forward`](AnimDir::Forward) and [`Reverse`](AnimDir::Reverse).
    PingPong,

    /// Alternate between [`Reverse`](AnimDir::Reverse) and [`Forward`](AnimDir::Forward).
    PingPongReverse,
}

impl From<LoopDir> for AnimDir {
    #[inline]
    fn from(value: LoopDir) -> Self {
        match value {
            LoopDir::Forward => Self::Forward,
            LoopDir::Reverse => Self::Reverse,
            LoopDir::PingPong => Self::PingPong,
            LoopDir::PingPongReverse => Self::PingPongReverse,
        }
    }
}

/// Information about an animation's layer.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnimLayer {
    pub opacity: f32,
    pub name: String,
    pub group: bool,
    pub level: u16,
}
