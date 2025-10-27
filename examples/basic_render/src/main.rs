use soyuz::Vertex;
use soyuz::graphics::core::{
    buffer::{IndexBuffer, VertexBuffer},
    gpu::GpuContext,
    pipeline::{PipelineBuilder, RenderPipeline},
    render_pass::RenderPass,
    shader::Shader,
    surface::Surface,
};

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use std::sync::Arc;

#[repr(C)]
#[derive(Vertex)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.5],
        color: [0.0, 0.0, 1.0],
    },
];

const INDICES: [u16; 3] = [0, 1, 2];

struct State {
    window: Arc<Window>,
    gpu_context: GpuContext,
    surface: Surface,
    pipeline: RenderPipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
}

struct App {
    state: Option<State>,
}

impl App {
    fn new() -> Self {
        Self { state: None }
    }

    pub fn render(&mut self) {
        if let Some(state) = &self.state {
            let output = state.surface.get_current_texture().unwrap();
            let view = output.create_view();

            let mut encoder = state
                .gpu_context
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let mut render_pass = RenderPass::builder()
                    .label("Triangle Render Pass")
                    .clear_color(0.1, 0.1, 0.1, 1.0)
                    .begin(&mut encoder, &view, None);

                render_pass.set_pipeline(&state.pipeline);
                render_pass.set_vertex_buffer(&state.vertex_buffer);
                render_pass.set_index_buffer(&state.index_buffer);
                render_pass.draw_indexed(0..state.index_buffer.count(), 0, 0..1);
            }

            state.gpu_context.queue().submit(Some(encoder.finish()));
            output.present();
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(state) = &mut self.state {
            if width > 0 && height > 0 {
                state.surface.resize(&state.gpu_context, width, height);
            }
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
                    &gpu_context.device(),
                    "examples/basic_render/src/shader.wgsl",
                )
                .unwrap(),
            );

            let vertex_buffer = VertexBuffer::new(
                &gpu_context.device(),
                Some("Triangle Vertex Buffer"),
                &VERTICES,
            );

            let index_buffer = IndexBuffer::new_u16(
                &gpu_context.device(),
                Some("Triangle Index Buffer"),
                &INDICES,
            );

            let pipeline = PipelineBuilder::new()
                .label("Triangle Pipeline")
                .vertex_shader(Arc::clone(&shader), "vs_main")
                .fragment_shader(Arc::clone(&shader), "fs_main")
                .vertex_layout(Vertex::desc())
                .color_target(surface.format())
                .build(&gpu_context.device())
                .unwrap();

            self.state = Some(State {
                window: window.clone(),
                gpu_context,
                surface,
                pipeline,
                vertex_buffer,
                index_buffer,
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
