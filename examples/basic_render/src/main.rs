use glam::{Mat4, Vec3};
use soyuz::engine::app::App;
use soyuz::engine::render::RenderSystem;
use soyuz::graphics::Camera;
use soyuz::graphics::core::bind_group::BindGroupBuilder;
use soyuz::graphics::core::buffer::{IndexBuffer, UniformBuffer, VertexBuffer};
use soyuz::graphics::core::pipeline::{PipelineBuilder, RenderPipeline};
use soyuz::graphics::core::render_pass::RenderPass;
use soyuz::graphics::core::shader::Shader;
use soyuz::graphics::core::{GpuContext, Surface};
use soyuz::graphics::resources::Texture;
use std::sync::Arc;
use winit::window::Window;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeVertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformUniform {
    transform: [[f32; 4]; 4],
}

impl From<Mat4> for TransformUniform {
    fn from(matrix: Mat4) -> Self {
        Self {
            transform: matrix.to_cols_array_2d(),
        }
    }
}

impl CubeVertex {
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CubeVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

struct BasicRenderSystem {
    pipeline: Option<RenderPipeline>,
    vertex_buffer: Option<VertexBuffer>,
    index_buffer: Option<IndexBuffer>,
    uniform_buffer: Option<UniformBuffer>,
    bind_group: Option<wgpu::BindGroup>,
    depth_texture: Option<Texture>,
    camera: Option<Camera>,
    start_time: std::time::Instant,
}

impl BasicRenderSystem {
    fn new() -> Self {
        Self {
            pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            uniform_buffer: None,
            bind_group: None,
            depth_texture: None,
            camera: None,
            start_time: std::time::Instant::now(),
        }
    }
}

impl RenderSystem for BasicRenderSystem {
    fn init(&mut self, gpu_context: &GpuContext, surface: &Surface) {
        let shader = Shader::from_file(
            gpu_context.device(),
            "examples/basic_render/src/shader.wgsl",
        )
        .expect("Failed to load shader");

        let vertices = vec![
            CubeVertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 0.0, 0.0],
            },
            CubeVertex {
                position: [0.5, -0.5, 0.5],
                color: [0.0, 1.0, 0.0],
            },
            CubeVertex {
                position: [0.5, 0.5, 0.5],
                color: [0.0, 0.0, 1.0],
            },
            CubeVertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 0.0],
            },
            CubeVertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 0.0, 1.0],
            },
            CubeVertex {
                position: [0.5, -0.5, -0.5],
                color: [0.0, 1.0, 1.0],
            },
            CubeVertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0],
            },
            CubeVertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.5, 0.5, 0.5],
            },
        ];

        #[rustfmt::skip]
        let indices: Vec<u16> = vec![
            0, 1, 2, 2, 3, 0,
            1, 5, 6, 6, 2, 1,
            5, 4, 7, 7, 6, 5,
            4, 0, 3, 3, 7, 4,
            3, 2, 6, 6, 7, 3,
            4, 5, 1, 1, 0, 4,
        ];

        self.vertex_buffer = Some(VertexBuffer::new(
            gpu_context.device(),
            Some("Cube Vertex Buffer"),
            &vertices,
        ));

        self.index_buffer = Some(IndexBuffer::new_u16(
            gpu_context.device(),
            Some("Cube Index Buffer"),
            &indices,
        ));

        self.uniform_buffer = Some(UniformBuffer::new(
            gpu_context.device(),
            Some("Transform Uniform Buffer"),
            &[TransformUniform::from(Mat4::IDENTITY)],
        ));

        self.camera = Some(Camera::new_perspective(45.0, 1280.0 / 720.0, 0.1, 100.0));
        if let Some(camera) = &mut self.camera {
            camera.set_position(Vec3::new(0.0, 0.0, 2.0));
            camera.set_target(Vec3::new(0.0, 0.0, 0.0));
        }

        let depth_texture =
            Texture::new_depth_texture(gpu_context.device(), 1280, 720, Some("Depth Texture"));

        self.depth_texture = Some(depth_texture);

        let bind_group_layout = BindGroupBuilder::new()
            .uniform(0, wgpu::ShaderStages::VERTEX)
            .layout_only(gpu_context.device());

        if let Some(uniform_buffer) = &self.uniform_buffer {
            let bind_group = gpu_context
                .device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Transform Bind Group"),
                    layout: &bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.raw().as_entire_binding(),
                    }],
                });

            self.bind_group = Some(bind_group);
        }

        let vertex_layout = CubeVertex::vertex_layout();
        let shader_arc = Arc::new(shader);

        let pipeline = PipelineBuilder::new()
            .vertex_shader(shader_arc.clone(), "vs_main")
            .fragment_shader(shader_arc, "fs_main")
            .vertex_layouts(&[vertex_layout])
            .raw_bind_group_layout(&bind_group_layout)
            .color_target(surface.format())
            .topology(wgpu::PrimitiveTopology::TriangleList)
            .front_face(wgpu::FrontFace::Ccw)
            .cull_mode(Some(wgpu::Face::Back))
            .polygon_mode(wgpu::PolygonMode::Fill)
            .depth_stencil(wgpu::TextureFormat::Depth32Float)
            .build(gpu_context.device())
            .expect("Failed to create pipeline");

        self.pipeline = Some(pipeline);
    }

    fn render(&mut self, gpu_context: &GpuContext, surface: &Surface, _window: &Arc<Window>) {
        let surface_texture = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(_) => return,
        };

        let view = surface_texture
            .texture()
            .create_view(&wgpu::TextureViewDescriptor::default());

        let elapsed = self.start_time.elapsed().as_secs_f32();
        let rotation = Mat4::from_rotation_y(elapsed) * Mat4::from_rotation_x(elapsed * 0.7);

        if let Some(camera) = &mut self.camera {
            camera.update();
            let view_proj = camera.view_proj_matrix();
            let transform = view_proj * rotation;

            if let Some(uniform_buffer) = &self.uniform_buffer {
                uniform_buffer.write(gpu_context.queue(), &[TransformUniform::from(transform)]);
            }
        }

        let mut encoder =
            gpu_context
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let depth_view = self.depth_texture.as_ref().unwrap().view();
            let mut render_pass = RenderPass::builder()
                .clear_color(0.0, 0.0, 0.0, 1.0)
                .clear_depth(1.0)
                .begin(&mut encoder, &view, Some(depth_view));

            if let Some(pipeline) = &self.pipeline {
                render_pass.set_pipeline(pipeline);

                if let Some(bind_group) = &self.bind_group {
                    render_pass.set_bind_group(0, bind_group, &[]);
                }

                if let Some(vertex_buffer) = &self.vertex_buffer {
                    render_pass.set_vertex_buffer(vertex_buffer);
                }

                if let Some(index_buffer) = &self.index_buffer {
                    render_pass.set_index_buffer(index_buffer);
                }

                render_pass.draw_indexed(0..36, 0, 0..1);
            }
        }

        gpu_context.queue().submit(Some(encoder.finish()));
        surface_texture.present();
    }

    fn resize(&mut self, gpu_context: &GpuContext, width: u32, height: u32) {
        if let Some(camera) = &mut self.camera {
            camera.set_aspect_ratio(width as f32 / height as f32);
        }

        if let Some(depth_texture) = &mut self.depth_texture {
            depth_texture.resize(gpu_context.device(), width, height);
        }
    }
}

fn main() {
    let render_system = BasicRenderSystem::new();

    App::new()
        .with_title("Basic Render - Cube")
        .with_size(1280, 720)
        .run(render_system);
}
