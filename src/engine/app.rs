use crate::engine::render::RenderSystem;
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

#[derive(Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Soyuz Engine".to_string(),
            width: 800,
            height: 600,
        }
    }
}

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    pub fn run<T: RenderSystem + 'static>(self, render_system: T) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut handler = AppHandler {
            config: self.config,
            state: None,
            render_system,
        };
        event_loop.run_app(&mut handler).unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

struct AppHandler<T: RenderSystem> {
    config: AppConfig,
    state: Option<State>,
    render_system: T,
}

impl<T: RenderSystem> ApplicationHandler for AppHandler<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = winit::window::Window::default_attributes()
                .with_title(self.config.title.clone())
                .with_inner_size(winit::dpi::LogicalSize::new(
                    self.config.width,
                    self.config.height,
                ));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let gpu_context =
                pollster::block_on(async { GpuContext::builder().build().await.unwrap() });

            let surface = Surface::new(window.clone(), &gpu_context).unwrap();

            self.render_system.init(&gpu_context, &surface);

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
                    self.render_system
                        .render(&state.gpu_context, &state.surface, &state.window);
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

                    self.render_system.resize(
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
