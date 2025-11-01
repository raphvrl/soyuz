use crate::assets::vertices::GltfVertex;
use crate::ecs::components::*;
use crate::graphics::core::*;
use crate::graphics::resources::GpuTexture;
use bevy_ecs::prelude::*;
use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct PushConstants {
    model_matrix: Mat4,
}

#[derive(Resource)]
pub struct ShadowPipeline {
    pipeline: RenderPipeline,
}

impl ShadowPipeline {
    pub fn new(gpu: &GpuContext, camera_layout: &wgpu::BindGroupLayout) -> Self {
        let shader = Shader::from_file(gpu, "src/shaders/shadow.wgsl").unwrap();

        let pipeline = RenderPipeline::builder()
            .label("Shadow Pipeline")
            .vertex_shader(shader.clone(), "vs_main")
            .vertex_layout(GltfVertex::desc())
            .push_constant_range(std::mem::size_of::<PushConstants>() as u32)
            .depth_stencil_state(wgpu::DepthStencilState {
                format: GpuTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            })
            .raw_bind_group_layout(camera_layout)
            .build(gpu)
            .unwrap();

        Self { pipeline }
    }

    pub fn draw(
        &self,
        pass: &mut RenderPass,
        camera_bind_group: &wgpu::BindGroup,
        query: Query<(&Transform, &Mesh, &Material)>,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_raw_bind_group(0, camera_bind_group, &[]);

        for (transform, mesh, _) in query.iter() {
            let model_matrix = transform.get_matrix();

            let push_constants = PushConstants { model_matrix };

            pass.set_push_constant(0, bytemuck::bytes_of(&push_constants));

            mesh.draw(pass);
        }
    }
}
