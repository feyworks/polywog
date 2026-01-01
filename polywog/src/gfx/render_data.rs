use crate::color::Rgba8;
use crate::gfx::draw::DrawCache;
use crate::gfx::{
    BindingValue, Bindings, BlendMode, IndexBuffer, Sampler, Shader, Surface, Texture, Topology,
    UniformValue, Vertex, VertexBuffer,
};
use crate::math::{Mat4, Numeric, Rect, Vec2};

#[derive(Debug)]
pub struct RenderData {
    pub passes: Vec<RenderPass>,
}

impl RenderData {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.passes.clear();
    }
}

#[derive(Debug)]
pub struct RenderPass {
    pub surface: Option<Surface>,
    pub clear_color: Option<Rgba8>,
    pub layers: Vec<RenderLayer>,
}

impl RenderPass {
    pub fn new(
        surface: Option<Surface>,
        clear_color: Option<Rgba8>,
        layers_vec: Vec<RenderLayer>,
    ) -> Self {
        Self {
            surface,
            clear_color,
            layers: layers_vec,
        }
    }

    pub fn layer(&mut self, index: usize) -> &mut RenderLayer {
        self.layers.get_mut(index).unwrap()
    }

    pub fn ensure_layer(&mut self, layer: usize, cache: &mut DrawCache) {
        let size = self
            .surface
            .as_ref()
            .map(|s| s.size())
            .unwrap_or(cache.window_size);
        while self.layers.len() <= layer {
            self.layers.push(RenderLayer::new(cache, size.to_f32()));
        }
    }

    pub fn finish(&mut self, cache: &mut DrawCache) -> bool {
        let mut should_submit = self.clear_color.is_some();
        for layer in self.layers.iter_mut() {
            layer.flush(cache);
            should_submit |= layer.calls.len() > 0;
        }
        should_submit
    }
}

#[derive(Debug)]
pub struct RenderLayer {
    pub calls: Vec<DrawCall>,
    pub shader: Shader,
    pub bindings: Bindings,
    pub blend_mode: BlendMode,
    pub scissor_rect: Option<Rect<u32>>,
    pub topology: Topology,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub view_matrix: Mat4<f32>,
    pub main_texture: Texture,
    pub main_sampler: Sampler,
    pub ortho: Mat4<f32>,
}

impl RenderLayer {
    fn new(cache: &mut DrawCache, size: Vec2<f32>) -> Self {
        Self {
            calls: cache.draw_call_vecs.pop().unwrap_or_default(),
            shader: cache.default_shader.clone(),
            bindings: Bindings::new(&cache.default_shader, &cache.default_texture),
            blend_mode: BlendMode::Normal,
            scissor_rect: None,
            topology: Topology::Triangles,
            vertices: cache.vertices_vecs.pop().unwrap_or_default(),
            indices: cache.indices_vecs.pop().unwrap_or_default(),
            view_matrix: Mat4::IDENTITY,
            main_texture: cache.default_texture.clone(),
            main_sampler: Sampler::default(),
            ortho: Mat4::ortho(0.0, size.x, size.y, 0.0, 0.0, 1.0),
        }
    }

    fn flush(&mut self, cache: &mut DrawCache) {
        if self.vertices.is_empty() {
            return;
        }

        // update the vertex/index buffers
        let (vertices, indices) =
            cache
                .buffer_cache
                .request(&cache.device, &cache.queue, &self.vertices, &self.indices);
        self.vertices.clear();
        self.indices.clear();

        // update the binding values
        self.bindings.set(
            &self.shader,
            "view_matrix",
            BindingValue::Uniform(UniformValue::Mat4(self.ortho * self.view_matrix)),
        );
        self.bindings.set(
            &self.shader,
            "main_texture",
            BindingValue::Texture(self.main_texture.clone()),
        );
        self.bindings.set(
            &self.shader,
            "main_sampler",
            BindingValue::Sampler(self.main_sampler),
        );

        // submit the draw call
        self.calls.push(DrawCall {
            shader: self.shader.clone(),
            bindings: self.bindings.clone(),
            blend_mode: self.blend_mode,
            clip_rect: self.scissor_rect,
            vertices,
            indices,
            topology: self.topology,
        });
    }

    pub fn set_shader(&mut self, shader: &Shader, cache: &mut DrawCache) {
        if &self.shader == shader {
            return;
        }
        self.flush(cache);
        self.shader = shader.clone();
        self.bindings.reset(&self.shader, &cache.default_texture);
    }

    pub fn set_param(&mut self, name: &str, value: BindingValue, cache: &mut DrawCache) {
        self.flush(cache);
        self.bindings.set(&self.shader, name, value);
    }

    pub fn set_view_matrix(&mut self, matrix: &Mat4<f32>, cache: &mut DrawCache) {
        if !self.view_matrix.abs_diff_eq(matrix) {
            self.flush(cache);
            self.view_matrix = *matrix;
        }
    }

    // pub fn set_main_texture(&mut self, texture: &TextureHandle, graphics: &mut Graphics) {
    //     if &self.main_texture != texture {
    //         self.flush(graphics);
    //         self.main_texture = texture.clone();
    //     }
    // }

    pub fn set_main_sampler(&mut self, sampler: Sampler, cache: &mut DrawCache) {
        if self.main_sampler != sampler {
            self.flush(cache);
            self.main_sampler = sampler;
        }
    }

    pub fn set_blend_mode(&mut self, blend_mode: BlendMode, cache: &mut DrawCache) {
        if self.blend_mode != blend_mode {
            self.flush(cache);
            self.blend_mode = blend_mode;
        }
    }

    pub fn set_scissor_rect(&mut self, rect: Option<Rect<u32>>, cache: &mut DrawCache) {
        if self.scissor_rect != rect {
            self.flush(cache);
            self.scissor_rect = rect;
        }
    }

    pub fn set_topology(&mut self, topology: Topology, cache: &mut DrawCache) {
        if self.topology != topology {
            self.flush(cache);
            self.topology = topology;
        }
    }

    pub fn set_tex_mode(&mut self, texture: &Texture, cache: &mut DrawCache) {
        if self.topology != Topology::Triangles || &self.main_texture != texture {
            self.flush(cache);
            self.topology = Topology::Triangles;
            self.main_texture = texture.clone();
        }
    }

    pub fn submit_buffers(
        &mut self,
        texture: Texture,
        topology: Topology,
        vertices: VertexBuffer,
        indices: IndexBuffer,
        cache: &mut DrawCache,
    ) {
        self.flush(cache);
        self.main_texture = texture;
        self.calls.push(DrawCall {
            shader: self.shader.clone(),
            bindings: self.bindings.clone(),
            blend_mode: self.blend_mode,
            clip_rect: self.scissor_rect,
            vertices,
            indices,
            topology,
        });
    }
}

#[derive(Debug, Clone)]
pub struct DrawCall {
    pub shader: Shader,
    pub bindings: Bindings,
    pub blend_mode: BlendMode,
    pub clip_rect: Option<Rect<u32>>,
    pub vertices: VertexBuffer,
    pub indices: IndexBuffer,
    pub topology: Topology,
}
