use crate::graphics::core::{GpuContext, IndexBuffer, RenderPass, VertexBuffer};
use crate::graphics::resources::vertex::VertexTrait;

pub struct GpuMesh {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    index_count: u32,
}

impl GpuMesh {
    pub fn new<V: VertexTrait>(
        gpu: &GpuContext,
        vertices: &[V],
        indices: &[u16],
        label: Option<&str>,
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(
            &gpu.device,
            label.map(|l| format!("{} Vertices", l)).as_deref(),
            vertices,
        );

        let index_buffer = IndexBuffer::new_u16(
            &gpu.device,
            label.map(|l| format!("{} Indices", l)).as_deref(),
            indices,
        );

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }

    pub fn draw(&self, pass: &mut RenderPass) {
        pass.set_vertex_buffer(&self.vertex_buffer);
        pass.set_index_buffer(&self.index_buffer);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

impl Clone for GpuMesh {
    fn clone(&self) -> Self {
        Self {
            vertex_buffer: self.vertex_buffer.clone(),
            index_buffer: self.index_buffer.clone(),
            index_count: self.index_count,
        }
    }
}
