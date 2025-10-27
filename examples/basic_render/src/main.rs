use soyuz::graphics::core::{gpu::GpuContext, render_pass::RenderPass, surface::Surface};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use std::sync::Arc;

struct State {
    window: Arc<Window>,
    gpu_context: GpuContext,
    surface: Surface,
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
                let _render_pass = RenderPass::builder()
                    .label("Clear Render Pass")
                    .clear_color(0.0, 0.0, 0.0, 1.0)
                    .begin(&mut encoder, &view, None);
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

            self.state = Some(State {
                window: window.clone(),
                gpu_context,
                surface,
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
