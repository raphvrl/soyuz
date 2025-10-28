use crate::graphics::core::GpuContext;
use crate::graphics::core::surface::Surface;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use std::sync::Arc;

pub struct State {
    window: Arc<Window>,
    gpu_context: GpuContext,
    surface: Surface,
}

pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut handler = AppHandler { state: self.state };
        event_loop.run_app(&mut handler).unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

struct AppHandler {
    state: Option<State>,
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = winit::window::Window::default_attributes()
                .with_title("Soyuz Engine")
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

            window.request_redraw();
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
                if let Some(state) = &self.state {
                    state.window.request_redraw();
                }
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(state) = &mut self.state {
                    state.surface.resize(
                        &state.gpu_context,
                        physical_size.width,
                        physical_size.height,
                    );
                }
            }
            _ => {}
        }
    }
}
