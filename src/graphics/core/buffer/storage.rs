use super::{Buffer, BufferData};

#[derive(Clone)]
pub struct StorageBuffer(Buffer);

impl StorageBuffer {
    pub fn new<T: BufferData>(device: &wgpu::Device, label: Option<&str>, data: &[T]) -> Self {
        Self(Buffer::new(
            device,
            label,
            data,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        ))
    }

    pub fn new_read_write<T: BufferData>(
        device: &wgpu::Device,
        label: Option<&str>,
        data: &[T],
    ) -> Self {
        Self(Buffer::new(
            device,
            label,
            data,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        ))
    }

    pub fn new_empty(
        device: &wgpu::Device,
        label: Option<&str>,
        size: wgpu::BufferAddress,
    ) -> Self {
        Self(Buffer::new_empty(
            device,
            label,
            size,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        ))
    }

    pub fn write<T: BufferData>(&self, queue: &wgpu::Queue, data: &[T]) {
        self.0.write(queue, data);
    }

    pub fn buffer(&self) -> &Buffer {
        &self.0
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        self.0.raw()
    }
}
