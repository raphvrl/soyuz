use crate::graphics::core::{IndexBuffer, VertexBuffer};
use bevy_ecs::prelude::Component;
use glam::Vec3;
use soyuz_macros::Vertex;

#[derive(Vertex)]
pub struct VertexData {
    pub position: Vec3,
    pub color: Vec3,
}

#[derive(Component, Clone)]
pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub index_count: u32,
}
