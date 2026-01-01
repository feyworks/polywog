use crate::gfx::{IndexBuffer, Vertex, VertexBuffer};
use std::collections::HashMap;
use wgpu::{Device, Queue};

#[derive(Debug, Default)]
pub struct BufferCache {
    pub vertices: HashMap<usize, Vec<VertexBuffer>>,
    pub indices: HashMap<usize, Vec<IndexBuffer>>,
    used_vertices: Vec<(usize, VertexBuffer)>,
    used_indices: Vec<(usize, IndexBuffer)>,
}

impl BufferCache {
    pub fn reset(&mut self) {
        for (size, buffer) in self.used_vertices.drain(..) {
            self.vertices.get_mut(&size).unwrap().push(buffer);
        }
        for (size, buffer) in self.used_indices.drain(..) {
            self.indices.get_mut(&size).unwrap().push(buffer);
        }
    }

    pub fn request(
        &mut self,
        device: &Device,
        queue: &Queue,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> (VertexBuffer, IndexBuffer) {
        let (vb_size, vb) = {
            let size = vertices.len().next_power_of_two();
            let cache = self.vertices.entry(size).or_insert_with(Vec::new);
            let buffer = cache
                .pop()
                .unwrap_or_else(|| VertexBuffer::new(device, queue.clone(), size));
            buffer
                .upload(vertices)
                .expect("cache gave invalid vertex buffer size");
            (size, buffer)
        };

        let (ib_size, ib) = {
            let size = indices.len().next_power_of_two();
            let cache = self.indices.entry(size).or_insert_with(Vec::new);
            let buffer = cache
                .pop()
                .unwrap_or_else(|| IndexBuffer::new(device, queue.clone(), size));
            buffer
                .upload(indices)
                .expect("cache gave invalid index buffer size");
            (size, buffer)
        };

        self.used_vertices.push((vb_size, vb.clone()));
        self.used_indices.push((ib_size, ib.clone()));

        (vb, ib)
    }
}
