use crate::graphics::core::RenderPass;
use crate::graphics::resources::GpuMesh;
use bevy_ecs::prelude::Component;
use soyuz_macros::Vertex;

use glam::Vec3;

use std::sync::Arc;

#[derive(Vertex)]
pub struct VertexData {
    pub position: Vec3,
    pub color: Vec3,
}

#[derive(Component, Clone)]
pub struct Mesh {
    pub gpu_mesh: Arc<GpuMesh>,
}

impl Mesh {
    pub fn new(gpu_mesh: Arc<GpuMesh>) -> Self {
        Self { gpu_mesh }
    }

    pub fn draw(&self, pass: &mut RenderPass) {
        self.gpu_mesh.draw(pass);
    }
}
