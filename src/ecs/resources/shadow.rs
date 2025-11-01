use crate::graphics::core::*;
use crate::graphics::resources::{GpuTexture, TextureSampler};
use bevy_ecs::prelude::*;
use glam::Mat4;

pub const SHADOW_MAP_SIZE: u32 = 1024;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightSpaceUniform {
    pub light_space_matrix: Mat4,
}

#[derive(Resource)]
pub struct ShadowMap {
    pub texture: GpuTexture,
    pub sampler: TextureSampler,
    pub bind_group: BindGroup,
    pub light_space_buffer: UniformBuffer,
    pub dummy_color: GpuTexture,
}

impl ShadowMap {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = gpu.device();

        let texture = GpuTexture::new_depth_texture(
            gpu,
            SHADOW_MAP_SIZE,
            SHADOW_MAP_SIZE,
            Some("Shadow Map Texture"),
        );

        let sampler = TextureSampler::depth_comparison(device, Some("Shadow Map Sampler"));

        let dummy_color = GpuTexture::new_render_target(
            gpu,
            SHADOW_MAP_SIZE,
            SHADOW_MAP_SIZE,
            wgpu::TextureFormat::R8Unorm,
            Some("Shadow Dummy Color"),
        );

        let initial_data = LightSpaceUniform {
            light_space_matrix: Mat4::IDENTITY,
        };
        let light_space_buffer =
            UniformBuffer::new(device, Some("Light Space Buffer"), &initial_data);

        let bind_group = BindGroup::builder()
            .label("shadow_map_bind_group")
            .texture_depth(0, wgpu::ShaderStages::FRAGMENT)
            .sampler_comparison(1, wgpu::ShaderStages::FRAGMENT)
            .uniform(2, wgpu::ShaderStages::FRAGMENT)
            .build(
                device,
                &[
                    BindResource::TextureView(0, texture.view()),
                    BindResource::Sampler(1, sampler.raw()),
                    BindResource::Buffer(2, light_space_buffer.raw()),
                ],
            )
            .expect("Failed to create shadow map bind group");

        Self {
            texture,
            sampler,
            bind_group,
            light_space_buffer,
            dummy_color,
        }
    }

    pub fn update_light_space(&self, gpu: &GpuContext, light_space_matrix: &Mat4) {
        let uniform = LightSpaceUniform {
            light_space_matrix: *light_space_matrix,
        };
        self.light_space_buffer.write(gpu.queue(), &uniform);
    }

    pub fn view(&self) -> &wgpu::TextureView {
        self.texture.view()
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.bind_group.layout()
    }
}

#[derive(Resource)]
pub struct LightCameraBuffer {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl LightCameraBuffer {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = gpu.device();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("light_camera_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("light_camera_buffer"),
            size: std::mem::size_of::<crate::ecs::resources::camera::CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light_camera_bind_group"),
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

    pub fn update(&self, gpu: &GpuContext, view_proj: &Mat4) {
        let uniform = crate::ecs::resources::camera::CameraUniform {
            view_proj: *view_proj,
        };
        gpu.queue()
            .write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
