use crate::color::{FromRgb, Rgba8, Rgba16, Rgba32F};
use crate::core::Window;
use crate::gfx::{
    IndexBuffer, Shader, Surface, Texture, TextureFormat, TexturePixel, Vertex, VertexBuffer,
};
use crate::grid::Grid;
use crate::img::{DynImage, Image, ImageError, ImageRgba8};
use crate::math::Vec2U;
use dpi::PhysicalSize;
use pollster::FutureExt;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::sync::Arc;
use wgpu::{
    Adapter, BackendOptions, Backends, Device, DeviceDescriptor, ExperimentalFeatures, Features,
    Instance, InstanceDescriptor, InstanceFlags, Limits, MemoryBudgetThresholds, MemoryHints,
    PowerPreference, PresentMode, Queue, RequestAdapterOptions, SurfaceCapabilities,
    SurfaceConfiguration, TextureUsages, Trace,
};

/// Handle to the graphics state, used to create surfaces, textures, shaders, etc.
///
/// This handle can be cloned and passed around freely to give objects the ability to create
/// graphics resources.
#[derive(Clone)]
pub struct Graphics(Arc<GraphicsInner>);

impl Debug for Graphics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graphics").finish_non_exhaustive()
    }
}

struct GraphicsInner {
    window: Window,
    _instance: Instance,
    surface_caps: SurfaceCapabilities,
    pub(crate) surface: wgpu::Surface<'static>,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    limits: Limits,
    default_texture: Texture,
    default_shader: Shader,
}

fn config(size: PhysicalSize<u32>, caps: &SurfaceCapabilities) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::AutoVsync,
        desired_maximum_frame_latency: 2,
        alpha_mode: caps.alpha_modes[0],
        view_formats: Vec::new(),
    }
}

impl Graphics {
    pub(crate) fn new(window: Window) -> Self {
        // create the instance
        let instance = {
            let backends = if cfg!(target_os = "windows") {
                Backends::DX12
            } else if cfg!(target_os = "macos") {
                Backends::METAL
            } else {
                Backends::VULKAN
            };
            Instance::new(&InstanceDescriptor {
                backends,
                flags: InstanceFlags::DEBUG | InstanceFlags::VALIDATION,
                memory_budget_thresholds: MemoryBudgetThresholds::default(),

                // TODO: ship with (or detect) DX12 compiler DLL?
                backend_options: BackendOptions::default(),
            })
        };

        // create the window surface
        let surface = instance
            .create_surface(window.0.clone())
            .expect("failed to create window surface");

        // request an adapter to a graphics device
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .expect("failed to find a suitable graphics device");

        // request a graphics device and queue for it
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::default(),
                required_limits: Limits::default(),
                experimental_features: ExperimentalFeatures::default(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .block_on()
            .expect(&format!(
                "failed to establish a connection to the graphics device:\n{:#?}",
                adapter.get_info()
            ));
        let limits = device.limits();

        // create the surface configuration and configure the surface
        let surface_caps = surface.get_capabilities(&adapter);
        surface.configure(&device, &config(window.0.inner_size(), &surface_caps));

        // create the default shader
        let default_shader = Shader::new(&device, include_str!("shader_default.wgsl"));

        // create the default texture
        let default_texture = Texture::new(
            &device,
            queue.clone(),
            Vec2U::ONE,
            TextureFormat::Rgba8,
            false,
        );
        default_texture.upload_bytes(bytemuck::cast_slice(&[Rgba8::FUCHSIA]));

        Self(Arc::new(GraphicsInner {
            window,
            _instance: instance,
            surface_caps,
            surface,
            _adapter: adapter,
            device,
            queue,
            limits,
            default_shader,
            default_texture,
        }))
    }

    /// Handle to the window.
    #[inline]
    pub fn window(&self) -> &Window {
        &self.0.window
    }

    #[inline]
    pub(crate) fn surface(&self) -> &wgpu::Surface<'static> {
        &self.0.surface
    }

    #[inline]
    pub(crate) fn device(&self) -> &Device {
        &self.0.device
    }

    #[inline]
    pub(crate) fn queue(&self) -> &Queue {
        &self.0.queue
    }

    #[inline]
    pub fn max_texture_size(&self) -> u32 {
        self.0.limits.max_texture_dimension_2d
    }

    /// Shader that is used by default, which is:
    ///
    /// ```wgsl
    /// @vertex
    /// fn vert_main(vert: Vertex) -> Fragment {
    ///     return vert_default(vert);
    /// }
    ///
    /// @fragment
    /// fn frag_main(frag: Fragment) -> @location(0) vec4f {
    ///     return frag_default(frag);
    /// }
    /// ```
    #[inline]
    pub fn default_shader(&self) -> &Shader {
        &self.0.default_shader
    }

    /// Texture that is used by default (a single `Rgba8::FUCHSIA` pixel).
    #[inline]
    pub fn default_texture(&self) -> &Texture {
        &self.0.default_texture
    }

    /// Create a new shader from the provided [WGSL](https://www.w3.org/TR/WGSL/) source code.
    ///
    /// See [`default_shader`](Self::default_shader) for a starting point.
    pub fn create_shader(&self, source: &str) -> Shader {
        Shader::new(&self.0.device, source)
    }

    /// Create a new shader from the provided [WGSL](https://www.w3.org/TR/WGSL/) source file.
    ///
    /// See [`default_shader`](Self::default_shader) for a starting point.
    pub fn load_shader(&self, path: impl AsRef<Path>) -> Result<Shader, std::io::Error> {
        let source = std::fs::read_to_string(path)?;
        Ok(self.create_shader(&source))
    }

    /// Create a new surface that can be rendered to.
    pub fn create_surface(&self, size: impl Into<Vec2U>, format: TextureFormat) -> Surface {
        Surface(Texture::new(
            &self.0.device,
            self.0.queue.clone(),
            size.into(),
            format,
            true,
        ))
    }

    /// Create a new [`Rgba8`](TextureFormat::Rgba8) surface.
    pub fn create_rgba8_surface(&self, size: impl Into<Vec2U>) -> Surface {
        self.create_surface(size, TextureFormat::Rgba8)
    }

    /// Create a new texture.
    pub fn create_texture<P: TexturePixel>(&self, size: Vec2U, pixels: &[P]) -> Texture {
        let texture = Texture::new(
            &self.0.device,
            self.0.queue.clone(),
            size,
            P::TEXTURE_FORMAT,
            false,
        );
        texture.upload_bytes(bytemuck::cast_slice(pixels));
        texture
    }

    /// Create a new texture from a PNG file. The texture's format will be determined by
    /// the PNG's format:
    ///
    /// | `ImageFormat::`      | `TextureFormat::` |
    /// | -------------------- | ----------------- |
    /// | `Grey8`              | `R8`              |
    /// | `Grey16`             | `R16`             |
    /// | `Grey32F`            | `R32F`            |
    /// | `GreyAlpha8`         | `Rg8`             |
    /// | `GreyAlpha16`        | `Rg16`            |
    /// | `GreyAlpha32F`       | `Rg32F`           |
    /// | `Rgb8` / `Rgba8`     | `Rgba8`           |
    /// | `Rgb16` / `Rgba16`   | `Rgba16`          |
    /// | `Rgb32F` / `Rgba32F` | `Rgba32F`         |
    ///
    /// The main thing to notice here is that RGB textures are not supported, and so any RGB-format
    /// images will be promoted to their RGBA equivalents.
    pub fn load_png_from_file(
        &self,
        path: impl AsRef<Path>,
        premultiply: bool,
    ) -> Result<Texture, ImageError> {
        let mut img = DynImage::load_png_from_file(path)?;
        if premultiply {
            img.premultiply();
        }
        Ok(self.create_texture_from_dyn_img(&img))
    }

    /// Create a new texture from the bytes of a PNG file.. The texture's format will be determined
    /// by the PNG's format:
    ///
    /// | `ImageFormat::`      | `TextureFormat::` |
    /// | -------------------- | ----------------- |
    /// | `Grey8`              | `R8`              |
    /// | `Grey16`             | `R16`             |
    /// | `Grey32F`            | `R32F`            |
    /// | `GreyAlpha8`         | `Rg8`             |
    /// | `GreyAlpha16`        | `Rg16`            |
    /// | `GreyAlpha32F`       | `Rg32F`           |
    /// | `Rgb8` / `Rgba8`     | `Rgba8`           |
    /// | `Rgb16` / `Rgba16`   | `Rgba16`          |
    /// | `Rgb32F` / `Rgba32F` | `Rgba32F`         |
    ///
    /// The main thing to notice here is that RGB textures are not supported, and so any RGB-format
    /// images will be promoted to their RGBA equivalents.
    pub fn load_png_from_memory(
        &self,
        bytes: &[u8],
        premultiply: bool,
    ) -> Result<Texture, ImageError> {
        let mut img = DynImage::load_png_from_memory(bytes)?;
        if premultiply {
            img.premultiply();
        }
        Ok(self.create_texture_from_dyn_img(&img))
    }

    /// Create a new texture from an [`Image`].
    pub fn create_texture_from_img<P: TexturePixel, S: AsRef<[P::Channel]>>(
        &self,
        image: &Image<P, S>,
    ) -> Texture {
        self.create_texture(image.size(), image.pixels())
    }

    /// Create a new texture from a [`DynImage`]. The texture's format will be determined by
    /// the image's format as follows:
    ///
    /// | `DynImage::`         | `TextureFormat::` |
    /// | -------------------- | ----------------- |
    /// | `Grey8`              | `R8`              |
    /// | `Grey16`             | `R16`             |
    /// | `Grey32F`            | `R32F`            |
    /// | `GreyAlpha8`         | `Rg8`             |
    /// | `GreyAlpha16`        | `Rg16`            |
    /// | `GreyAlpha32F`       | `Rg32F`           |
    /// | `Rgb8` / `Rgba8`     | `Rgba8`           |
    /// | `Rgb16` / `Rgba16`   | `Rgba16`          |
    /// | `Rgb32F` / `Rgba32F` | `Rgba32F`         |
    ///
    /// The main thing to notice here is that RGB textures are not supported, and so any RGB-format
    /// images will be promoted to their RGBA equivalents.
    pub fn create_texture_from_dyn_img(&self, image: &DynImage) -> Texture {
        match image {
            DynImage::Grey8(img) => self.create_texture_from_img(img),
            DynImage::Grey16(img) => self.create_texture_from_img(img),
            DynImage::Grey32F(img) => self.create_texture_from_img(img),
            DynImage::GreyAlpha8(img) => self.create_texture_from_img(img),
            DynImage::GreyAlpha16(img) => self.create_texture_from_img(img),
            DynImage::GreyAlpha32F(img) => self.create_texture_from_img(img),
            DynImage::Rgb8(img) => self.create_texture_from_img(&img.to_rgba8()),
            DynImage::Rgb16(img) => self.create_texture_from_img(&img.map(Rgba16::from_rgb)),
            DynImage::Rgb32F(img) => self.create_texture_from_img(&img.map(Rgba32F::from_rgb)),
            DynImage::Rgba8(img) => self.create_texture_from_img(img),
            DynImage::Rgba16(img) => self.create_texture_from_img(img),
            DynImage::Rgba32F(img) => self.create_texture_from_img(img),
        }
    }

    /// Create an [`Rgba8`](TextureFormat::Rgba8) texture.
    pub fn create_rgba8_texture(&self, image: &ImageRgba8) -> Texture {
        self.create_texture_from_img(image)
    }

    /// Create a new index buffer from the provided indices.
    pub fn create_index_buffer(&self, indices: &[u32]) -> IndexBuffer {
        let buffer = IndexBuffer::new(&self.0.device, self.0.queue.clone(), indices.len());
        buffer.upload(indices).unwrap();
        buffer
    }

    /// Create a new vertex buffer from the provided vertices.
    pub fn create_vertex_buffer(&self, vertices: &[Vertex]) -> VertexBuffer {
        let buffer = VertexBuffer::new(&self.0.device, self.0.queue.clone(), vertices.len());
        buffer.upload(vertices).unwrap();
        buffer
    }

    pub(crate) fn resized(&self, new_size: PhysicalSize<u32>) {
        // only configure surface if the window has an actual size
        if new_size.width > 0 && new_size.height > 0 {
            let config = config(new_size, &self.0.surface_caps);
            self.0.surface.configure(&self.0.device, &config);
        }
    }
}
