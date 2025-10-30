use glam::{Vec2, Vec3};
use soyuz_macros::Vertex;

#[repr(C)]
#[derive(Vertex)]
pub struct GltfVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl GltfVertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }
}
