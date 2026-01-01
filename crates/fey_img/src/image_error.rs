use thiserror::Error;

/// An image processing error.
#[derive(Error, Debug)]
pub enum ImageError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    PngDecode(#[from] png::DecodingError),

    #[error("{0}")]
    PngEncode(#[from] png::EncodingError),

    #[error("{0}")]
    Qoi(#[from] qoi::Error),

    #[error("unsupported PNG bit-depth: {0}")]
    UnsupportedBitDepth(usize),
}
