use crate::graphics::core::{GpuContext, Surface};
use crate::graphics::pipelines::BasicPipeline;
use crate::graphics::resources::Texture;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct RenderingContext {
    pub gpu: GpuContext,
    pub surface: Surface,
    pub depth_texture: Texture,

    pub basic_pipeline: BasicPipeline,
}

impl RenderingContext {
    pub fn new(gpu: GpuContext, surface: Surface) -> Self {
        let width = surface.width();
        let height = surface.height();

        let depth_texture = Texture::new_depth_texture(&gpu, width, height, Some("Depth Texture"));

        let basic_pipeline = BasicPipeline::new(&gpu, &surface);

        Self {
            gpu,
            surface,
            depth_texture,
            basic_pipeline,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(&self.gpu, width, height);
        self.depth_texture.resize(&self.gpu, width, height);
    }

    pub fn depth_view(&self) -> &wgpu::TextureView {
        self.depth_texture.view()
    }
}
