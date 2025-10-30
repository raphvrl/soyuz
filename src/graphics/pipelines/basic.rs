use crate::assets::vertices::GltfVertex;
use crate::ecs::components::Mesh;
use crate::ecs::components::Transform;
use crate::ecs::resources::MainCamera;
use crate::graphics::core::{GpuContext, RenderPass, RenderPipeline, Shader, Surface};
use crate::graphics::resources::Texture;
use bevy_ecs::prelude::*;
use glam::Mat4;

pub struct BasicPipeline {
    pipeline: RenderPipeline,
}

impl BasicPipeline {
    pub fn new(gpu: &GpuContext, surface: &Surface) -> Self {
        let shader = Shader::from_file(gpu, "src/shaders/basic.wgsl").unwrap();

        let pipeline = RenderPipeline::builder()
            .vertex_shader(shader.clone(), "vs_main")
            .fragment_shader(shader.clone(), "fs_main")
            .vertex_layout(GltfVertex::desc())
            .color_target(surface.format())
            .push_constant_range(std::mem::size_of::<Mat4>() as u32)
            .depth_stencil(Texture::DEPTH_FORMAT)
            .build(gpu)
            .unwrap();

        Self { pipeline }
    }

    pub fn draw(
        &self,
        pass: &mut RenderPass,
        camera: Res<MainCamera>,
        query: Query<(&Transform, &Mesh)>,
    ) {
        for (transform, mesh) in query.iter() {
            let model_matrix = transform.get_matrix();

            let camera = camera.get();

            let view_matrix = camera.view_matrix();
            let projection_matrix = camera.projection_matrix();

            let transform_matrix = projection_matrix * view_matrix * model_matrix;
            let bytes = bytemuck::bytes_of(&transform_matrix);

            pass.set_pipeline(&self.pipeline);
            pass.set_push_constant(0, bytes);

            mesh.draw(pass);
        }
    }
}
