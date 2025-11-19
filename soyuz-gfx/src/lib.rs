use std::sync::Arc;
use winit::window::Window;

mod pass;
mod pipeline;
mod shader;

pub use pass::RenderPassBuilder;
pub use pipeline::RenderPipelineBuilder;
pub use shader::Shader;

#[derive(Debug, Clone)]
pub struct GraphicsBuilder {
    pub(crate) required_features: wgpu::Features,
    pub(crate) required_limits: wgpu::Limits,
    pub(crate) power_preference: wgpu::PowerPreference,
    pub(crate) backends: wgpu::Backends,
    pub(crate) present_mode: wgpu::PresentMode,
    pub(crate) force_fallback_adapter: bool,
}

impl Default for GraphicsBuilder {
    fn default() -> Self {
        Self {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            power_preference: wgpu::PowerPreference::default(),
            backends: wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::METAL,
            present_mode: wgpu::PresentMode::Immediate,
            force_fallback_adapter: false,
        }
    }
}

impl GraphicsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn features(mut self, features: wgpu::Features) -> Self {
        self.required_features = features;
        self
    }

    pub fn feature(mut self, feature: wgpu::Features) -> Self {
        self.required_features |= feature;
        self
    }

    pub fn limits(mut self, limits: wgpu::Limits) -> Self {
        self.required_limits = limits;
        self
    }

    pub fn power_preference(mut self, preference: wgpu::PowerPreference) -> Self {
        self.power_preference = preference;
        self
    }

    pub fn backends(mut self, backends: wgpu::Backends) -> Self {
        self.backends = backends;
        self
    }

    pub fn present_mode(mut self, mode: wgpu::PresentMode) -> Self {
        self.present_mode = mode;
        self
    }

    pub fn force_fallback_adapter(mut self, force: bool) -> Self {
        self.force_fallback_adapter = force;
        self
    }
}

pub struct Context {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    _window: Arc<Window>,
}

impl Context {
    pub async fn new(window: Arc<Window>, graphics: &GraphicsBuilder) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: graphics.backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: graphics.power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: graphics.force_fallback_adapter,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Soyuz Device"),
                required_features: graphics.required_features,
                required_limits: graphics.required_limits.clone(),
                experimental_features: wgpu::ExperimentalFeatures::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::default(),
            })
            .await
            .unwrap_or_else(|e| panic!("Failed to request device: {:?}", e));

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: graphics.present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        tracing::info!("Soyuz graphics context initialized");
        tracing::info!("GPU: {}", adapter.get_info().name);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            _window: window,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            tracing::debug!("Surface resized to {}x{}", new_size.width, new_size.height);
        }
    }

    pub fn render<F>(&mut self, render_fn: F)
    where
        F: FnOnce(&mut Context, &wgpu::TextureView, &mut wgpu::CommandEncoder),
    {
        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost) => {
                tracing::warn!("Surface lost, reconfiguring...");
                self.surface.configure(&self.device, &self.config);
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                tracing::error!("Out of memory!");
                panic!("Out of memory!");
            }
            Err(e) => {
                tracing::error!("Surface error: {:?}", e);
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        render_fn(self, &view, &mut encoder);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn render_pass<'a>(
        &self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
    ) -> RenderPassBuilder<'a> {
        RenderPassBuilder::new(encoder, view)
    }

    pub fn shader(&self, source: &str) -> Shader {
        Shader::from_wgsl(&self.device, source)
    }

    pub fn render_pipeline(&self) -> RenderPipelineBuilder<'_> {
        RenderPipelineBuilder::new(&self.device, self.config.format)
    }
}
