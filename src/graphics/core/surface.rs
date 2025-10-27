use super::{GpuContext, error::Result};
use std::sync::Arc;
use winit::window::Window;

pub struct Surface {
    inner: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    format: wgpu::TextureFormat,
    is_configured: bool,
}

impl Surface {
    pub fn new(window: Arc<Window>, context: &GpuContext) -> Result<Self> {
        let instance = &context.instance();
        let surface = instance.create_surface(window.clone())?;

        let capabilities = surface.get_capabilities(context.adapter());
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            inner: surface,
            config,
            format,
            is_configured: false,
        })
    }

    pub fn configure(&mut self, context: &GpuContext) {
        self.inner.configure(context.device(), &self.config);
        self.is_configured = true;
    }

    pub fn resize(&mut self, context: &GpuContext, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.configure(context);
        }
    }

    pub fn get_current_texture(&self) -> Result<SurfaceTexture> {
        let texture = self.inner.get_current_texture()?;
        Ok(SurfaceTexture { inner: texture })
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    pub fn is_configured(&self) -> bool {
        self.is_configured
    }

    pub fn width(&self) -> u32 {
        self.config.width
    }

    pub fn height(&self) -> u32 {
        self.config.height
    }
}

pub struct SurfaceTexture {
    inner: wgpu::SurfaceTexture,
}

impl SurfaceTexture {
    pub fn create_view(&self) -> wgpu::TextureView {
        self.inner
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn present(self) {
        self.inner.present();
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.inner.texture
    }
}
