use crate::gfx::{DrawError, IndexBufferUploadError, VertexBufferUploadError};
use crate::guid::GuidParseError;
use crate::img::ImageError;
use std::error::Error;
use winit::error::EventLoopError;

/// A top-level game error.
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("{0}")]
    Custom(Box<dyn Error>),

    #[error("{0}")]
    EventLoop(#[from] EventLoopError),

    #[error("{0}")]
    Gamepad(#[from] gilrs::Error),

    #[error("{0}")]
    IndexBufferUpload(#[from] IndexBufferUploadError),

    #[error("{0}")]
    VertexBufferUpload(#[from] VertexBufferUploadError),

    #[error("{0}")]
    Draw(#[from] DrawError),

    #[error("{0}")]
    ParseGuid(#[from] GuidParseError),

    #[error("{0}")]
    Image(#[from] ImageError),

    #[error("{0}")]
    Io(#[from] std::io::Error),
    //
    // #[error("{0}")]
    // TextureUpload(#[from] TextureUploadError),
}

impl GameError {
    #[inline]
    pub fn custom(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Custom(err.into())
    }
}
