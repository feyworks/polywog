use crate::{Affine2F, Angle, RadiansF, Vec2F};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec2F,
    pub rotation: RadiansF,
    pub scale: Vec2F,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        position: Vec2F::ZERO,
        rotation: RadiansF::ZERO,
        scale: Vec2F::ONE,
    };

    pub fn new(
        position: impl Into<Vec2F>,
        rotation: impl Angle<f32>,
        scale: impl Into<Vec2F>,
    ) -> Self {
        Self {
            position: position.into(),
            rotation: rotation.to_radians(),
            scale: scale.into(),
        }
    }

    #[inline]
    pub fn matrix(&self) -> Affine2F {
        Affine2F::trs(self.position, self.rotation, self.scale)
    }
}
