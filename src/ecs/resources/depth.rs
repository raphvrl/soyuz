use crate::graphics::{core::GpuContext, resources::Texture};
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct DepthTexture {
    texture: Texture,
}

impl DepthTexture {
    pub fn new(gpu: &GpuContext, width: u32, height: u32) -> Self {
        let texture =
            Texture::new_depth_texture(gpu.device(), width, height, Some("Depth Texture"));
        Self { texture }
    }

    pub fn view(&self) -> &wgpu::TextureView {
        self.texture.view()
    }

    pub fn resize(&mut self, gpu: &GpuContext, width: u32, height: u32) {
        self.texture.resize(gpu.device(), width, height);
    }

    pub fn size(&self) -> (u32, u32) {
        let size = self.texture.size();
        (size.width, size.height)
    }
}
