use crate::gfx::{
    BindingValue, Bindings, BlendMode, ParamDefs, ParamType, Sampler, Texture, Topology, Vertex,
};
use naga::valid::{Capabilities, ValidationFlags, Validator};
use naga::{FunctionResult, Scalar, ScalarKind, ShaderStage, TypeInner, VectorSize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, Buffer, BufferBinding, BufferBindingType, BufferSize,
    BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState, FrontFace,
    MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, Queue,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureSampleType, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

/// Handle to a compiled shader.
///
/// This handle can be cloned and passed around freely to give objects access to the shader.
///
/// Shaders are created from [`Graphics`](super::Graphics).
#[derive(Clone)]
pub struct Shader(Arc<Inner>);

impl Debug for Shader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Shader").finish_non_exhaustive()
    }
}

impl PartialEq for Shader {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for Shader {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0))
    }
}

#[derive(Debug)]
struct Inner {
    shader: ShaderModule,
    param_defs: ParamDefs,
    bind_group_layout: BindGroupLayout,
    bind_group_cache: RwLock<BindGroupCache>,
    pipeline_cache: RwLock<PipelineCache>,
}

impl Shader {
    /// The maximum amount of bindings allowed in a shader.
    pub const MAX_BINDINGS: usize = 16;

    pub(crate) fn new(device: &Device, source: &str) -> Self {
        // get the shared footer code for the shader, but re-position the
        // bindings in @group(0) so they trail after the user-defined ones
        let footer = {
            let mut next = 0;
            while source.contains(&format!("@binding({next})")) {
                next += 1;
            }
            include_str!("shader_footer.wgsl")
                .replace("$0", &format!("{}", next))
                .replace("$1", &format!("{}", next + 1))
                .replace("$2", &format!("{}", next + 2))
        };
        let source = format!("{source}\n{footer}");

        // parse the module so we can validate it
        let module = match naga::front::wgsl::parse_str(&source) {
            Ok(module) => module,
            Err(err) => {
                err.emit_to_stderr(&source);
                std::process::exit(0);
            }
        };

        // validate the module
        if let Err(err) =
            Validator::new(ValidationFlags::default(), Capabilities::default()).validate(&module)
        {
            err.emit_to_stderr(&source);
            std::process::exit(0);
        };

        // make sure it has a valid @vertex entry point
        {
            let Some(main) = module
                .entry_points
                .iter()
                .find(|e| e.stage == ShaderStage::Vertex)
            else {
                panic!("shader has no @vertex entry point");
            };
            let Some(name) = main.function.name.as_ref() else {
                panic!("@vertex entry point has no name");
            };
            let args = &main.function.arguments;
            if args.len() != 1
                || args[0].binding.is_some()
                || module.types[args[0].ty].name != Some("Vertex".to_string())
            {
                panic!("invalid arguments to @vertex entry point {name:?}, expected `Vertex`");
            }
            let Some(ret) = main.function.result.as_ref() else {
                panic!("@vertex entry point {name:?} has no return value, expected `-> Fragment`");
            };
            if ret.binding.is_some() || module.types[ret.ty].name != Some("Fragment".to_string()) {
                panic!(
                    "@vertex entry point {name:?} has invalid return value, expected `-> Fragment`"
                );
            }
        }

        // make sure it has a valid @fragment entry point
        {
            let Some(main) = module
                .entry_points
                .iter()
                .find(|e| e.stage == ShaderStage::Fragment)
            else {
                panic!("shader has no @fragment entry point");
            };
            let Some(name) = main.function.name.as_ref() else {
                panic!("@fragment entry point has no name");
            };
            let args = &main.function.arguments;
            if args.len() != 1
                || args[0].binding.is_some()
                || module.types[args[0].ty].name != Some("Fragment".to_string())
            {
                panic!("invalid arguments to @fragment entry point {name:?}, expected `Fragment`");
            }
            let good = if let Some(FunctionResult {
                ty,
                binding: Some(naga::Binding::Location { location: 0, .. }),
            }) = main.function.result.as_ref()
            {
                matches!(
                    &module.types[*ty].inner,
                    TypeInner::Vector {
                        size: VectorSize::Quad,
                        scalar: Scalar {
                            kind: ScalarKind::Float,
                            width: 4,
                        },
                    }
                )
            } else {
                false
            };
            if !good {
                panic!(
                    "@fragment entry point {name:?} has invalid return value, expected `-> @location(0) vec4f`"
                );
            }
        }

        // get the user-made parameter definitions (@group(0))
        let param_defs = ParamDefs::new(&module);

        // cap bindings
        if param_defs.defs.len() > Self::MAX_BINDINGS {
            panic!(
                "shader has {} bindings which exceeds the maximum of {}",
                param_defs.defs.len(),
                Self::MAX_BINDINGS
            );
        }

        // create the bind group layout for this shader
        let bind_group_layout = {
            let entries: Vec<BindGroupLayoutEntry> = param_defs
                .defs
                .iter()
                .enumerate()
                .map(|(binding, def)| BindGroupLayoutEntry {
                    binding: binding as u32,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: match def.ty {
                        ParamType::Texture => wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        ParamType::Sampler => {
                            wgpu::BindingType::Sampler(SamplerBindingType::Filtering)
                        }
                        ParamType::Uniform(ty) => wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(ty.size() as u64),
                        },
                    },
                    count: None,
                })
                .collect();
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            })
        };

        // compile the shader module
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(source.into()),
        });

        // create the pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        Self(Arc::new(Inner {
            shader,
            param_defs,
            bind_group_layout,
            bind_group_cache: RwLock::default(),
            pipeline_cache: RwLock::new(PipelineCache::new(pipeline_layout)),
        }))
    }

    pub(crate) fn request_pipeline(
        &self,
        device: &Device,
        topology: Topology,
        format: wgpu::TextureFormat,
        blend_mode: BlendMode,
    ) -> RenderPipeline {
        self.0
            .pipeline_cache
            .write()
            .unwrap()
            .request(device, &self.0.shader, topology, format, blend_mode)
            .clone()
    }

    pub(crate) fn request_bind_group(
        &self,
        device: &Device,
        queue: &Queue,
        bindings: &Bindings,
        samplers: &mut HashMap<Sampler, wgpu::Sampler>,
        frame: u64,
    ) -> BindGroup {
        self.0
            .bind_group_cache
            .write()
            .unwrap()
            .request(
                device,
                queue,
                bindings,
                samplers,
                &self.0.bind_group_layout,
                frame,
            )
            .clone()
    }

    /// All parameters defined on the shader.
    #[inline]
    pub fn param_defs(&self) -> &ParamDefs {
        &self.0.param_defs
    }
}

#[derive(Debug)]
struct PipelineCache {
    layout: PipelineLayout,
    cache: HashMap<PipelineKey, RenderPipeline>,
}

impl PipelineCache {
    fn new(layout: PipelineLayout) -> Self {
        Self {
            layout,
            cache: HashMap::new(),
        }
    }

    pub fn request(
        &mut self,
        device: &Device,
        shader: &ShaderModule,
        topology: Topology,
        format: wgpu::TextureFormat,
        blend_mode: BlendMode,
    ) -> &RenderPipeline {
        self.cache
            .entry(PipelineKey {
                topology,
                format,
                blend_mode,
            })
            .or_insert_with(|| {
                device.create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&self.layout),
                    vertex: VertexState {
                        module: shader,
                        entry_point: None,
                        compilation_options: Default::default(),
                        buffers: &[Vertex::LAYOUT],
                    },
                    primitive: PrimitiveState {
                        topology: topology.into(),
                        strip_index_format: None,
                        front_face: FrontFace::Cw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(FragmentState {
                        module: shader,
                        entry_point: None,
                        compilation_options: Default::default(),
                        targets: &[Some(ColorTargetState {
                            format,
                            blend: Some(blend_mode.into()),
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                    cache: None,
                })
            })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PipelineKey {
    topology: Topology,
    format: wgpu::TextureFormat,
    blend_mode: BlendMode,
}

#[derive(Debug, Default)]
struct BindGroupCache {
    cache: HashMap<u64, GroupCache>,
    used: Vec<(u64, CachedGroup)>,
    frame: u64,
}

impl BindGroupCache {
    /// Return all the used bind groups back to the cache
    fn reset(&mut self) {
        // return all the used bind groups to their respective caches
        for (key, group) in self.used.drain(..) {
            self.cache.get_mut(&key).unwrap().groups.push(group);
        }

        // only keep bind group caches that have textures that are still referenced
        // elsewhere. if the group cache has the last reference to a texture, it means
        // that group will never be summoned again, so we can free it up
        self.cache.retain(|_, group| {
            group
                .textures
                .iter()
                .all(|tex| Arc::strong_count(&tex.0) > 1)
        });
    }

    pub fn request(
        &mut self,
        device: &Device,
        queue: &Queue,
        bindings: &Bindings,
        samplers: &mut HashMap<Sampler, wgpu::Sampler>,
        layout: &BindGroupLayout,
        frame: u64,
    ) -> &BindGroup {
        // reset the cache every new frame
        if self.frame != frame {
            self.frame = frame;
            self.reset();
        }

        // get the cache key for this texture/sampler set
        let key = bindings.cache_key();

        // get the bind group cache for this key
        let cache = self.cache.entry(key).or_insert_with(|| {
            let mut textures = Vec::new();
            for val in bindings.values.iter() {
                if let BindingValue::Texture(tex) = val {
                    textures.push(tex.clone());
                }
            }
            GroupCache {
                textures,
                groups: Vec::new(),
            }
        });

        // get the next available cached group, or create one if one isn't available
        let group = if let Some(group) = cache.groups.pop() {
            // if we get an existing group, update its bindings
            let mut next_buf = 0;
            for val in &bindings.values {
                if let BindingValue::Uniform(uniform) = val {
                    queue.write_buffer(&group.buffers[next_buf], 0, uniform.bytes());
                    next_buf += 1;
                }
            }
            group
        } else {
            // create views into all our textures
            let mut texture_views = Vec::new();
            for texture in &cache.textures {
                texture_views.push(
                    texture
                        .0
                        .texture
                        .create_view(&TextureViewDescriptor::default()),
                );
            }

            // create our buffers and samplers
            let mut buffers = Vec::new();
            for val in bindings.values.iter() {
                match val {
                    BindingValue::Sampler(sampler) => {
                        if let Entry::Vacant(entry) = samplers.entry(*sampler) {
                            entry.insert(device.create_sampler(&SamplerDescriptor {
                                label: None,
                                address_mode_u: sampler.address_x.into(),
                                address_mode_v: sampler.address_y.into(),
                                address_mode_w: Default::default(),
                                mag_filter: sampler.mag_filter.into(),
                                min_filter: sampler.min_filter.into(),
                                mipmap_filter: Default::default(),
                                lod_min_clamp: 0.0,
                                lod_max_clamp: 0.0,
                                compare: None,
                                anisotropy_clamp: 1,
                                border_color: None,
                            }));
                        }
                    }
                    BindingValue::Uniform(uniform) => {
                        buffers.push(device.create_buffer_init(&BufferInitDescriptor {
                            label: None,
                            contents: uniform.bytes(),
                            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                        }));
                    }
                    _ => {}
                }
            }

            // access the textures and buffers in the order they were added
            let mut next_tex = 0;
            let mut next_buf = 0;

            // create all our bind group entries
            let mut entries: Vec<BindGroupEntry> = Vec::new();
            for (binding, value) in bindings.values.iter().enumerate() {
                entries.push(BindGroupEntry {
                    binding: binding as u32,
                    resource: match value {
                        BindingValue::Texture(_) => {
                            let i = next_tex;
                            next_tex += 1;
                            BindingResource::TextureView(&texture_views[i])
                        }
                        BindingValue::Sampler(sampler) => {
                            BindingResource::Sampler(samplers.get(sampler).unwrap())
                        }
                        BindingValue::Uniform(_) => {
                            let i = next_buf;
                            next_buf += 1;
                            BindingResource::Buffer(BufferBinding {
                                buffer: &buffers[i],
                                offset: 0,
                                size: None,
                            })
                        }
                    },
                });
            }

            // create the new bind group and return it
            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout,
                entries: &entries,
            });
            CachedGroup {
                buffers,
                bind_group,
            }
        };

        // mark the group as used and return the bind group
        self.used.push((key, group));
        &self.used.last().unwrap().1.bind_group
    }
}

#[derive(Debug)]
struct GroupCache {
    textures: Vec<Texture>,
    groups: Vec<CachedGroup>,
}

#[derive(Debug)]
struct CachedGroup {
    buffers: Vec<Buffer>,
    bind_group: BindGroup,
}
