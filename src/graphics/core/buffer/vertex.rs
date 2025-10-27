use super::{Buffer, BufferData};

pub struct VertexBuffer(Buffer);

impl VertexBuffer {
    pub fn new<T: BufferData>(device: &wgpu::Device, label: Option<&str>, vertices: &[T]) -> Self {
        Self(Buffer::new(
            device,
            label,
            vertices,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        ))
    }

    pub fn write<T: BufferData>(&self, queue: &wgpu::Queue, vertices: &[T]) {
        self.0.write(queue, vertices);
    }

    pub fn buffer(&self) -> &Buffer {
        &self.0
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        self.0.raw()
    }

    pub fn slice(
        &self,
        bounds: impl std::ops::RangeBounds<wgpu::BufferAddress>,
    ) -> wgpu::BufferSlice<'_> {
        self.0.slice(bounds)
    }
}
