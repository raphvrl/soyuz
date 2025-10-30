use crate::graphics::core::GpuContext;
use bevy_ecs::prelude::*;
use glam::Vec4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuPointLight {
    pub position: Vec4,
    pub color: Vec4,
    pub intensity: f32,
    pub radius: f32,
    pub _padding: [f32; 2],
}

pub const MAX_POINT_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightingData {
    pub point_lights: [GpuPointLight; MAX_POINT_LIGHTS],
    pub num_point_lights: u32,
    pub _padding: [f32; 3],
}

impl Default for LightingData {
    fn default() -> Self {
        Self {
            point_lights: [GpuPointLight {
                position: Vec4::ZERO,
                color: Vec4::ZERO,
                intensity: 0.0,
                radius: 0.0,
                _padding: [0.0; 2],
            }; MAX_POINT_LIGHTS],
            num_point_lights: 0,
            _padding: [0.0; 3],
        }
    }
}

#[derive(Resource)]
pub struct LightingBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl LightingBuffer {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = gpu.device();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lighting_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lighting_buffer"),
            size: std::mem::size_of::<LightingData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("lighting_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update(&self, gpu: &GpuContext, lighting_data: &LightingData) {
        gpu.queue()
            .write_buffer(&self.buffer, 0, bytemuck::bytes_of(lighting_data));
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
