use crate::gfx::{ParamType, Sampler, Shader, Texture, UniformType};
use crate::math::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};
use arrayvec::ArrayVec;
use bytemuck::bytes_of;
use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Bindings {
    pub values: ArrayVec<BindingValue, 16>,
}

impl Bindings {
    #[inline]
    pub fn new(shader: &Shader, default_texture: &Texture) -> Self {
        Self {
            values: shader
                .param_defs()
                .defs
                .iter()
                .map(|p| p.ty.default_value(default_texture))
                .collect(),
        }
    }

    #[inline]
    pub fn reset(&mut self, shader: &Shader, default_texture: &Texture) {
        self.values.clear();
        self.values.extend(
            shader
                .param_defs()
                .defs
                .iter()
                .map(|p| p.ty.default_value(default_texture)),
        )
    }

    #[inline]
    pub fn set(&mut self, shader: &Shader, name: &str, value: BindingValue) {
        let Some((idx, def)) = shader
            .param_defs()
            .defs
            .iter()
            .enumerate()
            .find(|(_, def)| def.name == name)
        else {
            panic!("param {name:?} not found");
        };

        let value_ty = value.param_ty();
        if def.ty != value_ty {
            panic!(
                "cannot set param {name:?} of type {:?} to a value of type {value_ty:?}",
                def.ty
            );
        }

        self.values[idx] = value;
    }

    #[inline]
    pub(crate) fn cache_key(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(self.values.len());
        for val in &self.values {
            std::mem::discriminant(val).hash(&mut hasher);
            match val {
                BindingValue::Texture(t) => Arc::as_ptr(&t.0).hash(&mut hasher),
                BindingValue::Sampler(s) => s.hash(&mut hasher),
                _ => {}
            }
        }
        hasher.finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BindingValue {
    Texture(Texture),
    Sampler(Sampler),
    Uniform(UniformValue),
}

impl BindingValue {
    #[inline]
    pub fn param_ty(&self) -> ParamType {
        match self {
            Self::Texture(_) => ParamType::Texture,
            Self::Sampler(_) => ParamType::Sampler,
            Self::Uniform(uniform) => ParamType::Uniform(uniform.uniform_ty()),
        }
    }
}

/// The value assigned to a uniform parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UniformValue {
    Int(i32),
    Uint(u32),
    Float(f32),
    Vec2(Vec2<f32>),
    Vec3(Vec3<f32>),
    Vec4(Vec4<f32>),
    Mat2(Mat2<f32>),
    Mat3(Mat3<f32>),
    Mat4(Mat4<f32>),
}

impl UniformValue {
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::Int(val) => bytes_of(val),
            Self::Uint(val) => bytes_of(val),
            Self::Float(val) => bytes_of(val),
            Self::Vec2(val) => bytes_of(val),
            Self::Vec3(val) => bytes_of(val),
            Self::Vec4(val) => bytes_of(val),
            Self::Mat2(val) => bytes_of(val),
            Self::Mat3(val) => bytes_of(val),
            Self::Mat4(val) => bytes_of(val),
        }
    }

    #[inline]
    pub fn uniform_ty(&self) -> UniformType {
        match self {
            Self::Int(_) => UniformType::Int,
            Self::Uint(_) => UniformType::Uint,
            Self::Float(_) => UniformType::Float,
            Self::Vec2(_) => UniformType::Vec2,
            Self::Vec3(_) => UniformType::Vec3,
            Self::Vec4(_) => UniformType::Vec4,
            Self::Mat2(_) => UniformType::Mat2,
            Self::Mat3(_) => UniformType::Mat3,
            Self::Mat4(_) => UniformType::Mat4,
        }
    }
}
