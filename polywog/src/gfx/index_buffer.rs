use bytemuck::cast_slice;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wgpu::{BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};

/// Handle to an index buffer.
///
/// This handle can be cloned and passed around freely to give objects access to the buffer.
///
/// Index buffers are created from [`Graphics`](super::Graphics).
#[derive(Clone)]
pub struct IndexBuffer(Arc<Inner>);

impl Debug for IndexBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IndexBuffer").finish_non_exhaustive()
    }
}

impl PartialEq for IndexBuffer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for IndexBuffer {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0))
    }
}

#[derive(Debug)]
struct Inner {
    buffer: wgpu::Buffer,
    queue: Queue,
    count: AtomicUsize,
}

impl IndexBuffer {
    pub(crate) fn new(device: &Device, queue: Queue, capacity: usize) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (capacity * size_of::<u32>()) as BufferAddress,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self(Arc::new(Inner {
            buffer,
            queue,
            count: AtomicUsize::new(0),
        }))
    }

    /// Replace the buffer's indices. Buffers will not grow in size so this must be less than or
    /// equal to the buffer's [`capacity`](Self::capacity).
    #[inline]
    pub fn upload(&self, indices: &[u32]) -> Result<(), IndexBufferUploadError> {
        assert!(indices.len() <= self.capacity());
        if indices.len() > self.capacity() {
            return Err(IndexBufferUploadError::InsufficientSpace {
                expected: self.capacity(),
                got: indices.len(),
            });
        }
        self.0.count.store(indices.len(), Ordering::Relaxed);
        self.0
            .queue
            .write_buffer(&self.0.buffer, 0, cast_slice(indices));
        Ok(())
    }

    #[inline]
    pub(crate) fn buffer(&self) -> &wgpu::Buffer {
        &self.0.buffer
    }

    /// Maximum amount of indices the buffer can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity_in_bytes() / size_of::<u32>()
    }

    /// Maximum amount of bytes the buffer can hold.
    #[inline]
    pub fn capacity_in_bytes(&self) -> usize {
        self.0.buffer.size() as usize
    }

    /// How many indices are in the buffer.
    #[inline]
    pub fn count(&self) -> usize {
        self.0.count.load(Ordering::Relaxed)
    }

    /// How many bytes are in the buffer.
    #[inline]
    pub fn size_in_bytes(&self) -> usize {
        self.0.count.load(Ordering::Relaxed) * size_of::<u32>()
    }
}

/// An error uploading indices to a buffer.
#[derive(Debug, thiserror::Error)]
pub enum IndexBufferUploadError {
    #[error("attempted to upload {got} indices to buffer with a capacity of {expected}")]
    InsufficientSpace { expected: usize, got: usize },
}
