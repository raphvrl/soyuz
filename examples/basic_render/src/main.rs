use soyuz::Vertex;
use soyuz::graphics::Camera;
use soyuz::graphics::core::{
    buffer::{IndexBuffer, UniformBuffer, VertexBuffer},
    gpu::GpuContext,
    pipeline::{PipelineBuilder, RenderPipeline},
    render_pass::RenderPass,
    shader::Shader,
    surface::Surface,
};
use soyuz::graphics::resources::{Texture, TextureSampler};

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use glam::Mat4;
use std::sync::Arc;

#[repr(C)]
#[derive(Vertex)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}

const VERTICES: [Vertex; 24] = [
    Vertex {
        position: [-0.5, -0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        uv: [0.0, 1.0],
    },
];

#[rustfmt::skip]
const INDICES: [u16; 36] = [
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4,
    8, 9, 10, 10, 11, 8,
    12, 13, 14, 14, 15, 12,
    16, 17, 18, 18, 19, 16,
    20, 21, 22, 22, 23, 20,
];

struct State {
    window: Arc<Window>,
    gpu_context: GpuContext,
    surface: Surface,
    pipeline: RenderPipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    camera: Camera,
    uniform_buffer: UniformBuffer,
    texture: Texture,
    sampler: TextureSampler,
    depth_texture: Texture,
    rotation_angle: f32,
}

struct App {
    state: Option<State>,
}

impl App {
    fn new() -> Self {
        Self { state: None }
    }

    pub fn render(&mut self) {
        if let Some(state) = &mut self.state {
            state.rotation_angle += 0.02;

            let model_matrix = Mat4::from_rotation_y(state.rotation_angle);

            state.camera.update();

            let view_proj = state.camera.view_proj_matrix();
            let model_view_proj = *view_proj * model_matrix;

            let uniform_data = model_view_proj.to_cols_array();

            state
                .uniform_buffer
                .write(state.gpu_context.queue(), &uniform_data);

            let bind_group_layout = state.gpu_context.device().create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("Transform Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                },
            );

            let bind_group =
                state
                    .gpu_context
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("Transform Bind Group"),
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: state.uniform_buffer.raw().as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(state.texture.view()),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(state.sampler.raw()),
                            },
                        ],
                    });

            let output = state.surface.get_current_texture().unwrap();
            let view = output.create_view();

            let mut encoder = state
                .gpu_context
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let mut render_pass = RenderPass::builder()
                    .label("Cube Render Pass")
                    .clear_color(0.0, 0.0, 0.0, 1.0)
                    .clear_depth(1.0)
                    .begin(&mut encoder, &view, Some(state.depth_texture.view()));

                render_pass.set_pipeline(&state.pipeline);
                render_pass.set_vertex_buffer(&state.vertex_buffer);
                render_pass.set_index_buffer(&state.index_buffer);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw_indexed(0..state.index_buffer.count(), 0, 0..1);
            }

            state.gpu_context.queue().submit(Some(encoder.finish()));
            output.present();
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(state) = &mut self.state
            && width > 0
            && height > 0
        {
            state.surface.resize(&state.gpu_context, width, height);
            state.camera.set_aspect_ratio(width as f32 / height as f32);
            state
                .depth_texture
                .resize(state.gpu_context.device(), width, height);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Soyuz - Basic Render")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let gpu_context =
                pollster::block_on(async { GpuContext::builder().build().await.unwrap() });

            let surface = Surface::new(window.clone(), &gpu_context).unwrap();

            let shader = Arc::new(
                Shader::from_file(
                    gpu_context.device(),
                    "examples/basic_render/src/shader.wgsl",
                )
                .unwrap(),
            );

            let vertex_buffer = VertexBuffer::new(
                gpu_context.device(),
                Some("Triangle Vertex Buffer"),
                &VERTICES,
            );

            let index_buffer = IndexBuffer::new_u16(
                gpu_context.device(),
                Some("Triangle Index Buffer"),
                &INDICES,
            );

            let camera = Camera::new_perspective(
                45.0,
                window.inner_size().width as f32 / window.inner_size().height as f32,
                0.1,
                100.0,
            );

            let uniform_buffer = UniformBuffer::new(
                gpu_context.device(),
                Some("Camera Uniform Buffer"),
                &[0.0f32; 32],
            );

            let img = image::open("examples/basic_render/src/image.png").unwrap();

            let texture = Texture::from_image(
                gpu_context.device(),
                gpu_context.queue(),
                img,
                Some("Cube Texture"),
            );

            let sampler = TextureSampler::linear(gpu_context.device(), Some("Cube Sampler"));

            let window_size = window.inner_size();
            let depth_texture = Texture::new_depth_texture(
                gpu_context.device(),
                window_size.width,
                window_size.height,
                Some("Depth Texture"),
            );

            let bind_group_layout =
                gpu_context
                    .device()
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("Transform Bind Group Layout"),
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::VERTEX,
                                ty: wgpu::BindingType::Buffer {
                                    ty: wgpu::BufferBindingType::Uniform,
                                    has_dynamic_offset: false,
                                    min_binding_size: None,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    sample_type: wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    multisampled: false,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 2,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                                count: None,
                            },
                        ],
                    });

            let pipeline = PipelineBuilder::new()
                .label("Cube Pipeline")
                .vertex_shader(Arc::clone(&shader), "vs_main")
                .fragment_shader(Arc::clone(&shader), "fs_main")
                .vertex_layout(Vertex::desc())
                .raw_bind_group_layout(&bind_group_layout)
                .color_target(surface.format())
                .depth_stencil(wgpu::TextureFormat::Depth32Float)
                .build(gpu_context.device())
                .unwrap();

            self.state = Some(State {
                window: window.clone(),
                gpu_context,
                surface,
                pipeline,
                vertex_buffer,
                index_buffer,
                camera,
                uniform_buffer,
                texture,
                sampler,
                depth_texture,
                rotation_angle: 0.0,
            });
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                if let Some(state) = &self.state {
                    state.window.request_redraw();
                }
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size.width, physical_size.height);
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
