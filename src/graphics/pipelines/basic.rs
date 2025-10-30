use crate::assets::vertices::GltfVertex;
use crate::ecs::components::Material;
use crate::ecs::components::Mesh;
use crate::ecs::components::Transform;
use crate::ecs::resources::AssetManager;
use crate::ecs::resources::MainCamera;
use crate::graphics::core::{GpuContext, RenderPass, RenderPipeline, Shader, Surface};
use crate::graphics::resources::GpuTexture;
use bevy_ecs::prelude::*;
use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct PushConstants {
    transform: Mat4,
    base_color: [f32; 4],
    texture_index: u32,
    _padding: [u32; 3],
}

pub struct BasicPipeline {
    pipeline: RenderPipeline,
}

impl BasicPipeline {
    pub fn new(
        gpu: &GpuContext,
        surface: &Surface,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader = Shader::from_file(gpu, "src/shaders/basic.wgsl").unwrap();

        let pipeline = RenderPipeline::builder()
            .vertex_shader(shader.clone(), "vs_main")
            .fragment_shader(shader.clone(), "fs_main")
            .vertex_layout(GltfVertex::desc())
            .color_target(surface.format())
            .push_constant_range(std::mem::size_of::<PushConstants>() as u32)
            .depth_stencil(GpuTexture::DEPTH_FORMAT)
            .raw_bind_group_layout(texture_layout)
            .build(gpu)
            .unwrap();

        Self { pipeline }
    }

    pub fn draw(
        &self,
        pass: &mut RenderPass,
        asset_manager: &AssetManager,
        camera: Res<MainCamera>,
        query: Query<(&Transform, &Mesh, &Material)>,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, asset_manager.texture_bind_group(), &[]);

        for (transform, mesh, material) in query.iter() {
            let model_matrix = transform.get_matrix();
            let camera = camera.get();
            let view_matrix = camera.view_matrix();
            let projection_matrix = camera.projection_matrix();
            let transform_matrix = projection_matrix * view_matrix * model_matrix;

            let push_constants = PushConstants {
                transform: transform_matrix,
                base_color: material.base_color,
                texture_index: material.texture_index,
                _padding: [0; 3],
            };

            pass.set_push_constant(0, bytemuck::bytes_of(&push_constants));

            mesh.draw(pass);
        }
    }
}
