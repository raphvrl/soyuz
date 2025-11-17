use std::sync::Arc;
use winit::window::Window;

mod shader;
mod pipeline;

pub use shader::Shader;
pub use pipeline::RenderPipelineBuilder;

pub struct Context {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    _window: Arc<Window>,
}

impl Context {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::METAL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Soyuz Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
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
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        log::info!("Soyuz graphics context initialized");
        log::info!("GPU: {}", adapter.get_info().name);

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
            log::debug!("Surface resized to {}x{}", new_size.width, new_size.height);
        }
    }

    pub fn render<F>(&mut self, render_fn: F)
    where
        F: FnOnce(&wgpu::TextureView, &wgpu::Device, &wgpu::Queue, &mut wgpu::CommandEncoder),
    {
        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost) => {
                log::warn!("Surface lost, reconfiguring...");
                self.surface.configure(&self.device, &self.config);
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::error!("Out of memory!");
                panic!("Out of memory!");
            }
            Err(e) => {
                log::error!("Surface error: {:?}", e);
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

        render_fn(&view, &self.device, &self.queue, &mut encoder);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn shader(&self, source: &str) -> Shader {
        Shader::from_wgsl(&self.device, source)
    }

    pub fn render_pipeline(&self) -> RenderPipelineBuilder<'_> {
        RenderPipelineBuilder::new(&self.device, self.config.format)
    }
}
