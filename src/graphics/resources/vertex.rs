use crate::graphics::core::buffer::BufferData;
use soyuz_macros::Vertex;

use glam::{Vec2, Vec3};

pub trait VertexTrait: BufferData {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Vertex)]
pub struct VertexData {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl VertexData {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }
}
