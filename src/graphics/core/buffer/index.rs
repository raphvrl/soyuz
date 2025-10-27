use super::Buffer;

pub struct IndexBuffer {
    buffer: Buffer,
    format: wgpu::IndexFormat,
    count: u32,
}

impl IndexBuffer {
    pub fn new_u16(device: &wgpu::Device, label: Option<&str>, indices: &[u16]) -> Self {
        Self {
            buffer: Buffer::new(
                device,
                label,
                indices,
                wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            ),
            format: wgpu::IndexFormat::Uint16,
            count: indices.len() as u32,
        }
    }

    pub fn new_u32(device: &wgpu::Device, label: Option<&str>, indices: &[u32]) -> Self {
        Self {
            buffer: Buffer::new(
                device,
                label,
                indices,
                wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            ),
            format: wgpu::IndexFormat::Uint32,
            count: indices.len() as u32,
        }
    }

    pub fn format(&self) -> wgpu::IndexFormat {
        self.format
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        self.buffer.raw()
    }

    pub fn slice(
        &self,
        bounds: impl std::ops::RangeBounds<wgpu::BufferAddress>,
    ) -> wgpu::BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}
