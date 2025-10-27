use super::core::buffer::BufferData;

pub trait Vertex: BufferData {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
