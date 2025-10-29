mod index;
mod storage;
mod uniform;
mod vertex;

use wgpu::util::DeviceExt;

#[derive(Clone)]
pub struct Buffer {
    buffer: wgpu::Buffer,
    size: wgpu::BufferAddress,
    usage: wgpu::BufferUsages,
}

impl Buffer {
    pub fn new<T: BufferData>(
        device: &wgpu::Device,
        label: Option<&str>,
        data: &[T],
        usage: wgpu::BufferUsages,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(data),
            usage,
        });

        Self {
            buffer,
            size: (std::mem::size_of_val(data)) as wgpu::BufferAddress,
            usage,
        }
    }

    pub fn new_empty(
        device: &wgpu::Device,
        label: Option<&str>,
        size: wgpu::BufferAddress,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            size,
            usage,
        }
    }

    pub fn write<T: BufferData>(&self, queue: &wgpu::Queue, data: &[T]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }

    pub fn write_at<T: BufferData>(
        &self,
        queue: &wgpu::Queue,
        offset: wgpu::BufferAddress,
        data: &[T],
    ) {
        queue.write_buffer(&self.buffer, offset, bytemuck::cast_slice(data));
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn size(&self) -> wgpu::BufferAddress {
        self.size
    }

    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }

    pub fn slice(
        &'_ self,
        bounds: impl std::ops::RangeBounds<wgpu::BufferAddress>,
    ) -> wgpu::BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}

pub use index::IndexBuffer;
pub use storage::StorageBuffer;
pub use uniform::UniformBuffer;
pub use vertex::VertexBuffer;

pub trait BufferData: bytemuck::Pod + bytemuck::Zeroable {}

impl<T> BufferData for T where T: bytemuck::Pod + bytemuck::Zeroable {}
