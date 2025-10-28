pub mod camera;
pub mod sampler;
pub mod texture;

pub use sampler::TextureSampler;
pub use texture::Texture;

use super::core::buffer::BufferData;

pub trait Vertex: BufferData {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
