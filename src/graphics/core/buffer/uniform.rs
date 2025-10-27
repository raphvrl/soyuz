use super::{Buffer, BufferData};

pub struct UniformBuffer(Buffer);

impl UniformBuffer {
    pub fn new<T: BufferData>(device: &wgpu::Device, label: Option<&str>, data: &T) -> Self {
        Self(Buffer::new(
            device,
            label,
            std::slice::from_ref(data),
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        ))
    }

    pub fn write<T: BufferData>(&self, queue: &wgpu::Queue, data: &T) {
        self.0.write(queue, std::slice::from_ref(data));
    }

    pub fn buffer(&self) -> &Buffer {
        &self.0
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        self.0.raw()
    }
}
