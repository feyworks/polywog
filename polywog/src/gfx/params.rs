use crate::gfx::{BindingValue, Sampler, Texture, UniformValue};
use crate::math::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};
use naga::{ImageClass, ImageDimension, Module, Scalar, ScalarKind, TypeInner, VectorSize};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// A shader's defined parameters.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ParamDefs {
    pub defs: Vec<ParamDef>,
}

impl ParamDefs {
    pub(crate) fn new(module: &Module) -> Self {
        let mut defs = Vec::new();
        for (binding_idx, (_, global)) in module.global_variables.iter().enumerate() {
            // must have a name
            let Some(name) = global.name.clone() else {
                panic!("global has no name");
            };

            // must have resource binding
            let Some(binding) = global.binding.as_ref() else {
                panic!("global variable {name:?} has no resource binding");
            };

            // all user-written bindings are in group 0
            if binding.group != 0 {
                panic!("global variable {name:?} must be in @group(0)");
            }
            if binding.binding != binding_idx as u32 {
                panic!(
                    "global variable {name:?} has @binding({}) but @binding({}) was expected next",
                    binding.binding, binding_idx
                );
            }

            let naga_ty = &module.types[global.ty];
            let ty = match &naga_ty.inner {
                // texture
                TypeInner::Image {
                    dim: ImageDimension::D2,
                    arrayed: false,
                    class:
                        ImageClass::Sampled {
                            kind: ScalarKind::Float,
                            multi: false,
                        },
                } => ParamType::Texture,

                // sampler
                TypeInner::Sampler { comparison: false } => ParamType::Sampler,

                // i32
                TypeInner::Scalar(Scalar {
                    kind: ScalarKind::Sint,
                    width: 4,
                }) => ParamType::Uniform(UniformType::Int),

                // u32
                TypeInner::Scalar(Scalar {
                    kind: ScalarKind::Uint,
                    width: 4,
                }) => ParamType::Uniform(UniformType::Uint),

                // f32
                TypeInner::Scalar(Scalar {
                    kind: ScalarKind::Float,
                    width: 4,
                }) => ParamType::Uniform(UniformType::Float),

                // vectors
                TypeInner::Vector {
                    size,
                    scalar:
                        Scalar {
                            kind: ScalarKind::Float,
                            width: 4,
                        },
                } => ParamType::Uniform(match size {
                    VectorSize::Bi => UniformType::Vec2,
                    VectorSize::Tri => UniformType::Vec3,
                    VectorSize::Quad => UniformType::Vec4,
                }),

                // mat2
                TypeInner::Matrix {
                    columns: VectorSize::Bi,
                    rows: VectorSize::Bi,
                    scalar:
                        Scalar {
                            kind: ScalarKind::Float,
                            width: 4,
                        },
                } => ParamType::Uniform(UniformType::Mat2),

                // mat3
                TypeInner::Matrix {
                    columns: VectorSize::Tri,
                    rows: VectorSize::Tri,
                    scalar:
                        Scalar {
                            kind: ScalarKind::Float,
                            width: 4,
                        },
                } => ParamType::Uniform(UniformType::Mat3),

                // mat4
                TypeInner::Matrix {
                    columns: VectorSize::Quad,
                    rows: VectorSize::Quad,
                    scalar:
                        Scalar {
                            kind: ScalarKind::Float,
                            width: 4,
                        },
                } => ParamType::Uniform(UniformType::Mat4),

                _ => {
                    let naga_name = naga_ty.name.clone().unwrap_or_else(|| "???".to_string());
                    panic!("global variable {name:?} has invalid type {naga_name:?}");
                }
            };

            // the parameter is valid, add it to the list
            defs.push(ParamDef { name, ty });
        }

        Self { defs }
    }

    /// Find a parameter by name.
    #[inline]
    pub fn find(&self, name: &str) -> Option<&ParamDef> {
        self.defs.iter().find(|def| def.name == name)
    }
}

/// A parameter defined in the shader.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    pub ty: ParamType,
}

/// A shader's parameter type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ParamType {
    Texture,
    Sampler,
    Uniform(UniformType),
}

impl ParamType {
    pub(crate) fn default_value(self, default_texture: &Texture) -> BindingValue {
        match self {
            Self::Texture => BindingValue::Texture(default_texture.clone()),
            Self::Sampler => BindingValue::Sampler(Sampler::default()),
            Self::Uniform(ty) => BindingValue::Uniform(ty.default_value()),
        }
    }
}

/// The type of a shader parameter's uniform binding.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UniformType {
    Int,
    Uint,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl UniformType {
    /// The default value for this uniform's type.
    #[inline]
    pub fn default_value(self) -> UniformValue {
        match self {
            Self::Int => UniformValue::Int(0),
            Self::Uint => UniformValue::Uint(0),
            Self::Float => UniformValue::Float(0.0),
            Self::Vec2 => UniformValue::Vec2(Vec2::ZERO),
            Self::Vec3 => UniformValue::Vec3(Vec3::ZERO),
            Self::Vec4 => UniformValue::Vec4(Vec4::ZERO),
            Self::Mat2 => UniformValue::Mat2(Mat2::IDENTITY),
            Self::Mat3 => UniformValue::Mat3(Mat3::IDENTITY),
            Self::Mat4 => UniformValue::Mat4(Mat4::IDENTITY),
        }
    }

    /// The size (in bytes) of this uniform type.
    #[inline]
    pub fn size(self) -> usize {
        match self {
            Self::Int | Self::Uint | Self::Float => 4,
            Self::Vec2 => 8,
            Self::Vec3 => 12,
            Self::Vec4 | Self::Mat2 => 16,
            Self::Mat3 => 36,
            Self::Mat4 => 64,
        }
    }
}
