use crate::graphics::core::GpuContext;
use crate::graphics::resources::camera::Camera;

use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3};

#[derive(Resource)]
pub struct MainCamera(Camera);

impl MainCamera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self(Camera::new_perspective(fov, aspect, near, far))
    }

    pub fn get(&self) -> &Camera {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Camera {
        &mut self.0
    }

    pub fn position(&self) -> Vec3 {
        self.0.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.0.set_position(position);
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.0.translate(offset);
    }

    pub fn forward(&self) -> Vec3 {
        self.0.forward()
    }

    pub fn right(&self) -> Vec3 {
        self.0.right()
    }

    pub fn rotate(&mut self, x_offset: f32, y_offset: f32) {
        self.0.rotate(x_offset, y_offset);
    }

    pub fn update(&mut self) {
        self.0.update();
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: Mat4,
}

#[derive(Resource)]
pub struct CameraBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraBuffer {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = gpu.device();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
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
            label: Some("camera_buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
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
        let uniform = CameraUniform {
            view_proj: *view_proj,
        };
        gpu.queue()
            .write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
