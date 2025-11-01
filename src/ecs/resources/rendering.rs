use crate::graphics::core::{GpuContext, Surface};
use crate::graphics::pipelines::{BasicPipeline, ShadowPipeline};
use crate::graphics::resources::GpuTexture;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct RenderingContext {
    pub gpu: GpuContext,
    pub surface: Surface,
    pub depth_texture: GpuTexture,
    pub basic_pipeline: BasicPipeline,
    pub shadow_pipeline: ShadowPipeline,
}

impl RenderingContext {
    pub fn new(
        gpu: GpuContext,
        surface: Surface,
        texture_layout: &wgpu::BindGroupLayout,
        shadow_map_layout: &wgpu::BindGroupLayout,
        light_camera_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let width = surface.width();
        let height = surface.height();

        let depth_texture =
            GpuTexture::new_depth_texture(&gpu, width, height, Some("Depth Texture"));

        let basic_pipeline = BasicPipeline::new(&gpu, &surface, texture_layout, shadow_map_layout);
        let shadow_pipeline = ShadowPipeline::new(&gpu, light_camera_layout);

        Self {
            gpu,
            surface,
            depth_texture,
            basic_pipeline,
            shadow_pipeline,
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
