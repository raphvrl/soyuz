use crate::graphics::core::*;
use bevy_ecs::prelude::*;
use glam::Vec3;
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuDirectionalLight {
    pub direction: Vec3,
    pub _pad0: f32,
    pub color: Vec3,
    pub intensity: f32,
}

pub const MAX_POINT_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightingData {
    pub point_lights: [GpuPointLight; MAX_POINT_LIGHTS],
    pub num_point_lights: u32,
    pub _pad_after_count: [f32; 3],
    pub directional_light: GpuDirectionalLight,
    pub _padding: [f32; 4],
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
            _pad_after_count: [0.0; 3],
            directional_light: GpuDirectionalLight {
                direction: Vec3::new(0.0, 0.0, 0.0),
                _pad0: 0.0,
                color: Vec3::ZERO,
                intensity: 0.0,
            },
            _padding: [0.0; 4],
        }
    }
}

#[derive(Resource)]
pub struct LightingBuffer {
    pub buffer: UniformBuffer,
    pub bind_group: BindGroup,
}

impl LightingBuffer {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = gpu.device();

        let buffer = UniformBuffer::new_empty(
            device,
            Some("lighting_buffer"),
            std::mem::size_of::<LightingData>() as u64,
        );

        let bind_group = BindGroup::builder()
            .label("lighting_bind_group")
            .uniform(0, wgpu::ShaderStages::FRAGMENT)
            .build(device, &[BindResource::Buffer(0, buffer.raw())])
            .expect("Failed to create lighting bind group");

        Self { buffer, bind_group }
    }

    pub fn update(&self, gpu: &GpuContext, lighting_data: &LightingData) {
        self.buffer.write(gpu.queue(), lighting_data);
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.bind_group.layout()
    }
}
