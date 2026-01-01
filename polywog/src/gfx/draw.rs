use crate::color::{Rgba8, Rgba64F, ToRgba};
use crate::core::Window;
use crate::gfx::buffer_cache::BufferCache;
use crate::gfx::{
    BindingValue, BlendMode, ColorMode, DrawCall, IndexBuffer, RenderData, RenderLayer, RenderPass,
    Sampler, Shader, SubTexture, Surface, Texture, Topology, UniformValue, Vertex, VertexBuffer,
};
use crate::math::{
    Affine2F, Angle, CircleF, LineF, Mat2F, Mat3F, Mat4F, Numeric, PolygonF, QuadF, RadiansF,
    RectF, RectU, TriangleF, Vec2, Vec2F, Vec2U, Vec3F, Vec4F, vec2,
};
use std::collections::HashMap;
use std::mem::{replace, swap};
use wgpu::{
    Color, CommandEncoderDescriptor, Device, IndexFormat, LoadOp, Operations, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};

/// Rendering API.
pub struct Draw {
    cache: DrawCache,
    data: RenderData,
    pass: RenderPass,
    layer: usize,
    matrix: Affine2F,
    matrix_stack: Vec<Affine2F>,
    clip_rect: Option<RectU>,
}

impl Draw {
    pub(crate) fn new(
        device: Device,
        queue: Queue,
        default_shader: Shader,
        default_texture: Texture,
    ) -> Self {
        Self {
            cache: DrawCache {
                device,
                queue,
                default_shader,
                default_texture,
                samplers: HashMap::new(),
                buffer_cache: BufferCache::default(),
                render_layer_vecs: Vec::new(),
                draw_call_vecs: Vec::new(),
                vertices_vecs: Vec::new(),
                indices_vecs: Vec::new(),
                window_size: Vec2U::ZERO,
            },
            data: RenderData::new(),
            pass: RenderPass::new(None, None, Vec::new()),
            layer: 0,
            matrix: Affine2F::IDENTITY,
            matrix_stack: Vec::new(),
            clip_rect: None,
        }
    }

    pub(crate) fn begin_frame(&mut self, window_size: Vec2U) {
        self.cache.window_size = window_size;

        // reset the buffer cache so the buffers can be reused
        self.cache.buffer_cache.reset();

        // reclaim vectors from the render data so they can be reused
        for mut pass in self.data.passes.drain(..) {
            for mut layer in pass.layers.drain(..) {
                layer.calls.clear();
                self.cache.draw_call_vecs.push(layer.calls);

                layer.vertices.clear();
                self.cache.vertices_vecs.push(layer.vertices);

                layer.indices.clear();
                self.cache.indices_vecs.push(layer.indices);
            }
            self.cache.render_layer_vecs.push(pass.layers);
        }

        // clear the data from the previous frame
        self.data.clear();

        self.pass = RenderPass::new(
            None,
            Some(Rgba8::BLACK),
            self.cache.render_layer_vecs.pop().unwrap_or_default(),
        );

        self.pass.ensure_layer(0, &mut self.cache);
        self.layer = 0;
        self.matrix = Affine2F::IDENTITY;
        self.matrix_stack.clear();
        self.clip_rect = None;
    }

    pub(crate) fn end_frame(
        &mut self,
        frame: u64,
        surface: &wgpu::Surface<'static>,
        window: &Window,
    ) {
        // if the current render pass has anything in it, finish and submit it
        let mut pass = replace(&mut self.pass, RenderPass::new(None, None, Vec::new()));
        if pass.finish(&mut self.cache) {
            self.data.passes.push(pass);
        }

        // get the window surface
        let window_surface = surface
            .get_current_texture()
            .expect("failed to acquire surface texture");

        // create the command encoder
        let mut encoder = self
            .cache
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        // if there are no user-submitted render passes, clear the window black
        if self.data.passes.is_empty() {
            _ = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &window_surface
                        .texture
                        .create_view(&TextureViewDescriptor::default()),
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // perform the rest of our render passes
        for pass in &self.data.passes {
            let surface_tex = if let Some(surface) = pass.surface.as_ref() {
                surface.texture().0.texture.clone()
            } else {
                window_surface.texture.clone()
            };
            let surface_format = surface_tex.format();
            let load = if let Some(clear_color) = pass.clear_color {
                let Rgba64F { r, g, b, a } = clear_color.to_rgba();
                LoadOp::Clear(Color { r, g, b, a })
            } else {
                LoadOp::Load
            };
            let mut wgpu_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_tex.create_view(&TextureViewDescriptor::default()),
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // render all the layers in depth-order
            for layer in pass.layers.iter() {
                // perform all the draw calls
                for call in layer.calls.iter() {
                    // set the render pipeline
                    wgpu_pass.set_pipeline(&call.shader.request_pipeline(
                        &self.cache.device,
                        call.topology,
                        surface_format,
                        call.blend_mode,
                    ));

                    if let Some(RectU { x, y, w, h }) = call.clip_rect {
                        wgpu_pass.set_scissor_rect(x, y, w, h);
                    } else {
                        let size = surface_tex.size();
                        wgpu_pass.set_scissor_rect(0, 0, size.width, size.height);
                    };

                    // set the shader bindings
                    wgpu_pass.set_bind_group(
                        0,
                        &call.shader.request_bind_group(
                            &self.cache.device,
                            &self.cache.queue,
                            &call.bindings,
                            &mut self.cache.samplers,
                            frame,
                        ),
                        &[],
                    );

                    // assign the vertex and index buffers
                    wgpu_pass.set_vertex_buffer(
                        0,
                        call.vertices
                            .buffer()
                            .slice(..call.vertices.size_in_bytes().to_u64()),
                    );
                    wgpu_pass.set_index_buffer(
                        call.indices
                            .buffer()
                            .slice(..call.indices.size_in_bytes().to_u64()),
                        IndexFormat::Uint32,
                    );

                    // perform the draw call
                    wgpu_pass.draw_indexed(0..call.indices.count().to_u32(), 0, 0..1);
                }
            }
        }

        self.cache.queue.submit([encoder.finish()]);
        window.0.pre_present_notify();
        window_surface.present();
        window.0.request_redraw();
    }

    /// Set the target surface and optionally clear it with a single color. If `None` is passed
    /// as the surface, the window will be drawn to. If `None` is passed as the clear color, then
    /// the surface will not be cleared, drawing will instead be appended to its current pixels.
    #[inline]
    pub fn set_surface(
        &mut self,
        surface: impl Into<Option<Surface>>,
        clear_color: impl Into<Option<Rgba8>>,
    ) {
        let surface = surface.into();
        let clear_color = clear_color.into();
        let mut prev = replace(
            &mut self.pass,
            RenderPass::new(
                surface,
                clear_color,
                self.cache.render_layer_vecs.pop().unwrap_or_default(),
            ),
        );
        self.pass.ensure_layer(self.layer, &mut self.cache);
        if prev.finish(&mut self.cache) {
            self.data.passes.push(prev);
        }
    }

    /// Set the target layer. For the most part you will be rendering to the default layer `0`,
    /// but in rare cases you may want to use layers to improve render batching.
    #[inline]
    pub fn set_layer(&mut self, layer: usize) {
        if self.layer == layer {
            return;
        }
        self.layer = layer;
        self.pass.ensure_layer(layer, &mut self.cache);
    }

    /// Set the shader future drawing methods will use. If the shader is already in use, nothing
    /// will happen. If not, the shader will switch and all the new shader's parameters will be
    /// initialized with their default values.
    #[inline]
    pub fn set_shader(&mut self, shader: impl Into<Option<Shader>>) {
        let shader = shader
            .into()
            .unwrap_or_else(|| self.cache.default_shader.clone());
        self.pass
            .layer(self.layer)
            .set_shader(&shader, &mut self.cache);
    }

    /// Set an `i32` parameter.
    #[inline]
    pub fn set_param_i32(&mut self, name: &str, value: i32) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Int(value)),
            &mut self.cache,
        );
    }

    /// Set a `u32` parameter.
    #[inline]
    pub fn set_param_u32(&mut self, name: &str, value: u32) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Uint(value)),
            &mut self.cache,
        );
    }

    /// Set an `f32` parameter.
    #[inline]
    pub fn set_param_f32(&mut self, name: &str, value: f32) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Float(value)),
            &mut self.cache,
        );
    }

    /// Set a `vec2f` parameter.
    #[inline]
    pub fn set_param_vec2(&mut self, name: &str, value: Vec2F) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Vec2(value)),
            &mut self.cache,
        );
    }

    /// Set a `vec3f` parameter.
    #[inline]
    pub fn set_param_vec3(&mut self, name: &str, value: Vec3F) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Vec3(value)),
            &mut self.cache,
        );
    }

    /// Set a `vec4f` parameter.
    #[inline]
    pub fn set_param_vec4(&mut self, name: &str, value: Vec4F) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Vec4(value)),
            &mut self.cache,
        );
    }

    /// Set a `mat2f` parameter.
    #[inline]
    pub fn set_param_mat2(&mut self, name: &str, value: Mat2F) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Mat2(value)),
            &mut self.cache,
        );
    }

    /// Set a `mat3f` parameter.
    #[inline]
    pub fn set_param_mat3(&mut self, name: &str, value: Mat3F) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Mat3(value)),
            &mut self.cache,
        );
    }

    /// Set a `mat4f` parameter.
    #[inline]
    pub fn set_param_mat4(&mut self, name: &str, value: Mat4F) {
        self.pass.layer(self.layer).set_param(
            name,
            BindingValue::Uniform(UniformValue::Mat4(value)),
            &mut self.cache,
        );
    }

    /// Set a `texture_2d<f32>` parameter.
    #[inline]
    pub fn set_param_texture(&mut self, name: &str, value: Texture) {
        self.pass
            .layer(self.layer)
            .set_param(name, BindingValue::Texture(value), &mut self.cache);
    }

    /// Set a `sampler` parameter.
    #[inline]
    pub fn set_param_sampler(&mut self, name: &str, value: Sampler) {
        self.pass
            .layer(self.layer)
            .set_param(name, BindingValue::Sampler(value), &mut self.cache);
    }

    /// Set the view matrix.
    #[inline]
    pub fn set_view_matrix(&mut self, value: &Mat4F) {
        self.pass
            .layer(self.layer)
            .set_view_matrix(value, &mut self.cache);
    }

    /// Set the main sampler.
    #[inline]
    pub fn set_main_sampler(&mut self, value: Sampler) {
        self.pass
            .layer(self.layer)
            .set_main_sampler(value, &mut self.cache);
    }

    /// The current blend mode.
    #[inline]
    pub fn blend_mode(&mut self) -> BlendMode {
        self.pass.layer(self.layer).blend_mode
    }

    /// Set the blend mode.
    #[inline]
    pub fn set_blend_mode(&mut self, value: BlendMode) {
        self.pass
            .layer(self.layer)
            .set_blend_mode(value, &mut self.cache);
    }

    /// The current clipping rectangle.
    #[inline]
    pub fn clip_rect(&self) -> Option<&RectU> {
        self.clip_rect.as_ref()
    }

    /// Set the clipping rectangle.
    #[inline]
    pub fn set_clip_rect(&mut self, value: impl Into<Option<RectU>>) {
        self.clip_rect = value.into();
        self.pass
            .layer(self.layer)
            .set_scissor_rect(self.clip_rect, &mut self.cache);
    }

    /// The current transform.
    #[inline]
    pub fn transform(&self) -> &Affine2F {
        &self.matrix
    }

    /// Push the transform to the top of the stack, optionally concatenating it.
    #[inline]
    pub fn push_transform(&mut self, matrix: Affine2F) {
        self.matrix_stack.push(self.matrix);
        self.matrix = self.matrix * matrix;
    }

    #[inline]
    pub fn set_transform(&mut self, matrix: Affine2F) {
        self.matrix = matrix;
    }

    #[inline]
    pub fn push_new_transform(&mut self, matrix: Affine2F) {
        self.matrix_stack.push(self.matrix);
        self.matrix = matrix;
    }

    #[inline]
    pub fn push_translation(&mut self, amount: impl Into<Vec2F>) {
        self.push_transform(Affine2F::translation(amount));
    }

    #[inline]
    pub fn push_rotation(&mut self, amount: impl Angle<f32>) {
        self.push_transform(Affine2F::rotation(amount));
    }

    #[inline]
    pub fn push_scale(&mut self, scale: impl Into<Vec2F>) {
        self.push_transform(Affine2F::scale(scale));
    }

    #[inline]
    pub fn push_trs(
        &mut self,
        translation: impl Into<Vec2F>,
        rotation: impl Angle<f32>,
        scale: impl Into<Vec2F>,
    ) {
        self.push_transform(Affine2F::trs(translation.into(), rotation, scale.into()));
    }

    #[inline]
    pub fn push_scale_of(&mut self, scale: f32) {
        self.push_transform(Affine2F::scale_of(scale));
    }

    /// Pop a transform off the top of the stack.
    #[inline]
    pub fn pop_transform(&mut self) -> Result<(), DrawError> {
        self.matrix = self
            .matrix_stack
            .pop()
            .ok_or_else(|| DrawError::NoTransformToPop)?;
        Ok(())
    }

    #[inline]
    pub fn pop_transforms(&mut self, count: usize) -> Result<(), DrawError> {
        for _ in 0..count {
            self.pop_transform()?;
        }
        Ok(())
    }

    #[inline]
    fn point_mode(&mut self) -> (&mut Vec<Vertex>, &mut Vec<u32>, &Affine2F) {
        let layer = self.pass.layer(self.layer);
        layer.set_topology(Topology::Points, &mut self.cache);
        (&mut layer.vertices, &mut layer.indices, &self.matrix)
    }

    #[inline]
    fn line_mode(&mut self) -> (&mut Vec<Vertex>, &mut Vec<u32>, &Affine2F) {
        let layer = self.pass.layer(self.layer);
        layer.set_topology(Topology::Lines, &mut self.cache);
        (&mut layer.vertices, &mut layer.indices, &self.matrix)
    }

    #[inline]
    fn tri_mode(&mut self) -> (&mut Vec<Vertex>, &mut Vec<u32>, &Affine2F) {
        let layer = self.pass.layer(self.layer);
        layer.set_topology(Topology::Triangles, &mut self.cache);
        (&mut layer.vertices, &mut layer.indices, &self.matrix)
    }

    #[inline]
    fn tex_mode(&mut self, texture: &Texture) -> (&mut Vec<Vertex>, &mut Vec<u32>, &Affine2F) {
        let layer = self.pass.layer(self.layer);
        layer.set_tex_mode(texture, &mut self.cache);
        (&mut layer.vertices, &mut layer.indices, &self.matrix)
    }

    /// Draw a quad filled with a texture.
    #[inline]
    pub fn textured_quad_flipped(
        &mut self,
        texture: impl AsRef<Texture>,
        quad: impl Into<QuadF>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        let (verts, inds, mat) = self.tex_mode(texture.as_ref());
        let [a, b, c, d] = quad.into().0.map(|p| mat.transform_pos2(p));
        let i = verts.len() as u32;
        let [mut aa, mut bb, mut cc, mut dd] = RectF::sized(Vec2F::ONE).corners();
        let flip = flip.into();
        if flip.x {
            swap(&mut aa.x, &mut bb.x);
            swap(&mut cc.x, &mut dd.x);
        }
        if flip.y {
            swap(&mut aa.y, &mut dd.y);
            swap(&mut bb.y, &mut cc.y);
        }
        verts.extend_from_slice(&[
            Vertex::new(a, aa, color, mode),
            Vertex::new(b, bb, color, mode),
            Vertex::new(c, cc, color, mode),
            Vertex::new(d, dd, color, mode),
        ]);
        inds.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
    }

    /// Draw a quad filled with a texture.
    #[inline]
    pub fn textured_quad_ext(
        &mut self,
        texture: impl AsRef<Texture>,
        quad: impl Into<QuadF>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        let (verts, inds, mat) = self.tex_mode(texture.as_ref());
        let [a, b, c, d] = quad.into().0.map(|p| mat.transform_pos2(p));
        let i = verts.len() as u32;
        verts.extend_from_slice(&[
            Vertex::new(a, vec2(0.0, 0.0), color, mode),
            Vertex::new(b, vec2(1.0, 0.0), color, mode),
            Vertex::new(c, vec2(1.0, 1.0), color, mode),
            Vertex::new(d, vec2(0.0, 1.0), color, mode),
        ]);
        inds.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
    }

    /// Draw a quad filled with a texture.
    #[inline]
    pub fn textured_quad(&mut self, texture: impl AsRef<Texture>, quad: impl Into<QuadF>) {
        self.textured_quad_ext(texture, quad, Rgba8::WHITE, ColorMode::MULT);
    }

    /// Draw a texture with the top-left at the provided position.
    #[inline]
    pub fn texture_at_flipped(
        &mut self,
        texture: impl AsRef<Texture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        let rect = RectF::pos_size(pos.into(), texture.as_ref().size().to_f32());
        self.textured_quad_flipped(texture, rect, color, mode, flip);
    }

    /// Draw a texture with the top-left at the provided position.
    #[inline]
    pub fn texture_at_ext(
        &mut self,
        texture: impl AsRef<Texture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        let rect = RectF::pos_size(pos.into(), texture.as_ref().size().to_f32());
        self.textured_quad_ext(texture, rect, color, mode);
    }

    /// Draw a texture with the top-left at the provided position.
    #[inline]
    pub fn texture_at(&mut self, texture: impl AsRef<Texture>, pos: impl Into<Vec2F>) {
        self.texture_at_ext(texture, pos, Rgba8::WHITE, ColorMode::MULT);
    }

    /// Draw a single point.
    #[inline]
    pub fn point(&mut self, pos: Vec2F, color: Rgba8) {
        let (verts, inds, mat) = self.point_mode();
        let i = verts.len() as u32;
        verts.push(Vertex::veto(mat.transform_pos2(pos), color));
        inds.push(i);
    }

    /// Draw a set of points.
    #[inline]
    pub fn points(&mut self, points: impl Iterator<Item = Vec2F>, color: Rgba8) {
        let (verts, inds, mat) = self.point_mode();
        let mut i = verts.len() as u32;
        for p in points {
            verts.push(Vertex::veto(mat.transform_pos2(p), color));
            inds.push(i);
            i += 1;
        }
    }

    /// Draw a line.
    #[inline]
    pub fn line(&mut self, line: impl Into<LineF>, color: Rgba8) {
        let (verts, inds, mat) = self.line_mode();
        let i = verts.len() as u32;
        verts.extend_from_slice(
            &line
                .into()
                .points()
                .map(|p| Vertex::veto(mat.transform_pos2(p), color)),
        );
        inds.extend_from_slice(&[i, i + 1]);
    }

    /// Draw lines connecting the series of points into a chain, optionally looping to the start.
    #[inline]
    pub fn lines(&mut self, points: impl IntoIterator<Item = Vec2F>, color: Rgba8, loops: bool) {
        let (verts, inds, mat) = self.line_mode();
        let start = verts.len() as u32;
        verts.extend(
            points
                .into_iter()
                .map(|p| Vertex::veto(mat.transform_pos2(p), color)),
        );
        let end = verts.len() as u32;
        if end - start > 1 {
            for i in start..(end - 1) {
                inds.extend_from_slice(&[i, i + 1]);
            }
            if loops {
                inds.extend_from_slice(&[end - 1, start]);
            }
        }
    }

    /// Draw a filled triangle.
    #[inline]
    pub fn triangle(&mut self, tri: impl Into<TriangleF>, color: Rgba8) {
        let (verts, inds, mat) = self.tri_mode();
        let i = verts.len() as u32;
        verts.extend_from_slice(
            &tri.into()
                .0
                .map(|p| Vertex::veto(mat.transform_pos2(p), color)),
        );
        inds.extend_from_slice(&[i, i + 1, i + 2]);
    }

    // #[inline]
    // fn triangles(&mut self, points: impl IntoIterator<Item = Vec2F>, color: Rgba8) {
    //     let (verts, inds, mat) = self.tri_mode();
    //
    //     // vertices
    //     let start = verts.len() as u32;
    //     verts.extend(
    //         points
    //             .into_iter()
    //             .map(|p| Vertex::veto(mat.transform_pos2(p), color)),
    //     );
    //     let end = verts.len() as u32;
    //
    //     // indices
    //     if end - start > 2 {
    //         for i in start..(end - 2) {
    //             inds.extend_from_slice(&[start, i + 1, i + 2]);
    //         }
    //     }
    // }

    /// Draw a triangle outline.
    #[inline]
    pub fn triangle_outline(&mut self, tri: impl Into<TriangleF>, color: Rgba8) {
        self.lines(tri.into().0, color, true);
    }

    /// Draw a filled quad.
    #[inline]
    pub fn quad(&mut self, quad: impl Into<QuadF>, color: Rgba8) {
        let (verts, inds, mat) = self.tri_mode();
        let i = verts.len() as u32;
        verts.extend_from_slice(
            &quad
                .into()
                .0
                .map(|p| Vertex::veto(mat.transform_pos2(p), color)),
        );
        inds.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
    }

    /// Draw a quad outline.
    #[inline]
    pub fn quad_outline(&mut self, quad: impl Into<QuadF>, color: Rgba8) {
        self.lines(quad.into().0, color, true);
    }

    /// Draw a filled rectangle.
    #[inline]
    pub fn rect(&mut self, rect: impl Into<RectF>, color: Rgba8) {
        self.quad(rect.into(), color);
    }

    /// Draw a rectangle outline.
    #[inline]
    pub fn rect_outline(&mut self, rect: impl Into<RectF>, color: Rgba8) {
        self.quad_outline(rect.into(), color);
    }

    /// Draw a filled polygon.
    #[inline]
    pub fn polygon(&mut self, poly: &PolygonF, color: Rgba8) {
        let (verts, inds, mat) = self.tri_mode();
        let start = verts.len() as u32;
        verts.extend(
            poly.points()
                .iter()
                .map(|p| Vertex::veto(mat.transform_pos2(*p), color)),
        );
        let end = verts.len() as u32;
        for i in start..(end - 2) {
            inds.extend_from_slice(&[start, i + 1, i + 2]);
        }
    }

    /// Draw a polygon outline.
    #[inline]
    pub fn polygon_outline(&mut self, poly: &PolygonF, color: Rgba8) {
        self.lines(poly.points().iter().copied(), color, true);
    }

    #[inline]
    fn fan(&mut self, points: impl IntoIterator<Item = Vec2F>, color: Rgba8, loops: bool) {
        let (verts, inds, mat) = self.tri_mode();
        let start = verts.len() as u32;
        verts.extend(
            points
                .into_iter()
                .map(|p| Vertex::veto(mat.transform_pos2(p), color)),
        );
        let end = verts.len() as u32;

        if end > start + 2 {
            for i in (start + 1)..(end - 1) {
                inds.push(start);
                inds.push(i);
                inds.push(i + 1);
            }
            if loops {
                inds.push(start);
                inds.push(end - 1);
                inds.push(start + 1);
            }
        }
    }

    /// Draw a filled circle using the provided number of segments. If `None`, then
    /// [`suggest_seg_count_f`](crate::math::Circle::suggest_seg_count) will be used.
    #[inline]
    pub fn circle(&mut self, circ: impl Into<CircleF>, color: Rgba8, seg_count: Option<u32>) {
        let circ = circ.into();
        let seg_count = seg_count
            .map(u32::to_f32)
            .unwrap_or_else(|| circ.suggest_seg_count_f(|p| self.matrix.transform_pos2(p)));
        self.fan(
            Some(circ.center)
                .into_iter()
                .chain(circ.iter_hull_points_n(seg_count, RadiansF::ZERO)),
            color,
            true,
        );
    }

    /// Draw a circle outline using the provided number of segments. If `None`, then
    /// [`suggest_seg_count_f`](crate::math::Circle::suggest_seg_count) will be used.
    #[inline]
    pub fn circle_outline(
        &mut self,
        circ: impl Into<CircleF>,
        color: Rgba8,
        seg_count: Option<u32>,
    ) {
        let circ = circ.into();
        let seg_count = match seg_count.map(u32::to_f32) {
            Some(n) => (n / 2.0).floor() * 2.0,
            None => circ.suggest_seg_count_f(|p| self.matrix.transform_pos2(p)),
        };
        self.lines(
            circ.iter_hull_points_n(seg_count, RadiansF::ZERO),
            color,
            true,
        );
    }

    /// Draw a subtexture.
    #[inline]
    pub fn subtexture_ext(
        &mut self,
        sub: impl AsRef<SubTexture>,
        dst: impl Into<QuadF>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        let sub = sub.as_ref();
        let (verts, inds, mat) = self.tex_mode(&sub.texture);
        let points = dst.into().0;
        let i = verts.len() as u32;
        verts.extend_from_slice(&std::array::from_fn::<_, 4, _>(|i| {
            let pos = mat.transform_pos2(points[i] + sub.offset);
            Vertex::new(pos, sub.coords[i], color, mode)
        }));
        inds.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
    }

    /// Draw a subtexture.
    #[inline]
    pub fn subtexture(&mut self, sub: impl AsRef<SubTexture>, dst: impl Into<QuadF>) {
        self.subtexture_ext(sub, dst, Rgba8::WHITE, ColorMode::MULT);
    }

    /// Draw a subtexture at the provided position.
    #[inline]
    pub fn subtexture_at_ext(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        let dst = RectF::pos_size(pos.into(), sub.as_ref().rect.size());
        self.subtexture_ext(sub, dst, color, mode);
    }

    /// Draw a subtexture at the provided position.
    #[inline]
    pub fn subtexture_at(&mut self, sub: impl AsRef<SubTexture>, pos: impl Into<Vec2F>) {
        self.subtexture_at_ext(sub, pos, Rgba8::WHITE, ColorMode::MULT);
    }

    /// Draw a custom set of vertices/indices.
    #[inline]
    pub fn custom(
        &mut self,
        texture: impl AsRef<Texture>,
        topology: Topology,
        vertices: impl IntoIterator<Item = Vertex>,
        indices: impl IntoIterator<Item = u32>,
    ) {
        let (verts, inds, mat) = match topology {
            Topology::Triangles => self.tex_mode(texture.as_ref()),
            Topology::Lines => self.line_mode(),
            Topology::Points => self.point_mode(),
        };
        let len = verts.len() as u32;
        for mut v in vertices {
            v.pos = mat.transform_pos2(v.pos);
            verts.push(v);
        }
        inds.extend(indices.into_iter().map(|i| len + i));
    }

    /// Draw the provided vertex/index buffers.
    #[inline]
    pub fn buffers(
        &mut self,
        texture: Option<&Texture>,
        topology: Topology,
        vertices: &VertexBuffer,
        indices: &IndexBuffer,
    ) {
        let layer = self.pass.layer(self.layer);
        let texture = texture
            .unwrap_or_else(|| &self.cache.default_texture)
            .clone();
        layer.submit_buffers(
            texture,
            topology,
            vertices.clone(),
            indices.clone(),
            &mut self.cache,
        );
    }
}

pub(crate) struct DrawCache {
    pub device: Device,
    pub queue: Queue,
    pub default_shader: Shader,
    pub default_texture: Texture,
    pub samplers: HashMap<Sampler, wgpu::Sampler>,
    pub buffer_cache: BufferCache,
    pub render_layer_vecs: Vec<Vec<RenderLayer>>,
    pub draw_call_vecs: Vec<Vec<DrawCall>>,
    pub vertices_vecs: Vec<Vec<Vertex>>,
    pub indices_vecs: Vec<Vec<u32>>,
    pub window_size: Vec2U,
}

/// A drawing error.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DrawError {
    #[error("no transform to pop")]
    NoTransformToPop,
}
