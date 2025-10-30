use crate::ecs::{events::*, resources::*, systems::*};
use crate::graphics::core::{GpuContext, Surface};
use bevy_ecs::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use std::sync::Arc;

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
    startup_schedule: Schedule,
    update_schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
            startup_schedule: Schedule::default(),
            update_schedule: Schedule::default(),
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

    pub fn add_startup_system<Marker>(
        mut self,
        system: impl IntoSystem<(), (), Marker> + 'static,
    ) -> Self {
        self.startup_schedule.add_systems(system);
        self
    }

    pub fn add_system<Marker>(mut self, system: impl IntoSystem<(), (), Marker> + 'static) -> Self {
        self.update_schedule.add_systems(system);
        self
    }

    pub fn run(mut self) {
        let world = World::new();

        self.update_schedule.add_systems(render_system);

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut handler = EcsAppHandler {
            config: self.config,
            world,
            startup_schedule: self.startup_schedule,
            update_schedule: self.update_schedule,
            state: None,
        };

        event_loop.run_app(&mut handler).unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

struct EcsAppHandler {
    config: AppConfig,
    world: World,
    startup_schedule: Schedule,
    update_schedule: Schedule,
    state: Option<AppState>,
}

struct AppState {
    window: Arc<Window>,
}

impl ApplicationHandler for EcsAppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = winit::window::Window::default_attributes()
                .with_title(self.config.title.clone())
                .with_inner_size(winit::dpi::LogicalSize::new(
                    self.config.width,
                    self.config.height,
                ));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let gpu_context = pollster::block_on(async {
                GpuContext::builder()
                    .add_features(wgpu::Features::PUSH_CONSTANTS)
                    .add_features(wgpu::Features::TEXTURE_BINDING_ARRAY)
                    .limits(wgpu::Limits {
                        max_push_constant_size: 128,
                        max_sampled_textures_per_shader_stage: 512,
                        max_binding_array_elements_per_shader_stage: 512,
                        ..Default::default()
                    })
                    .build()
                    .await
                    .unwrap()
            });

            let mut surface = Surface::new(window.clone(), &gpu_context).unwrap();
            surface.configure(&gpu_context);

            let asset_manager = AssetManager::new(&gpu_context);
            self.world.insert_resource(asset_manager);

            let asset_manager = self.world.resource::<AssetManager>();
            let rendering_context = RenderingContext::new(gpu_context, surface, asset_manager);
            self.world.insert_resource(rendering_context);

            self.world.init_resource::<Messages<WindowResizeEvent>>();

            self.world.insert_resource(Input::new());
            self.world.insert_resource(Mouse::new());

            self.world.insert_resource(MainCamera::new(
                45.0,
                self.config.width as f32 / self.config.height as f32,
                0.1,
                100.0,
            ));

            self.startup_schedule.run(&mut self.world);

            self.state = Some(AppState {
                window: window.clone(),
            });

            window
                .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                .unwrap();

            window.set_cursor_visible(false);

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
                if let Some(mut messages) =
                    self.world.get_resource_mut::<Messages<WindowResizeEvent>>()
                {
                    messages.update();
                }

                self.update_schedule.run(&mut self.world);

                if let Some(mut input) = self.world.get_resource_mut::<Input>() {
                    input.update();
                }

                if let Some(mut mouse) = self.world.get_resource_mut::<Mouse>() {
                    mouse.update();
                }

                if let Some(state) = &self.state {
                    state.window.request_redraw();
                }
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(mut messages) =
                    self.world.get_resource_mut::<Messages<WindowResizeEvent>>()
                {
                    messages.write(WindowResizeEvent {
                        width: physical_size.width,
                        height: physical_size.height,
                    });
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(mut input) = self.world.get_resource_mut::<Input>() {
                    let keycode = match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(code) => code,
                        winit::keyboard::PhysicalKey::Unidentified(_) => return,
                    };

                    match event.state {
                        winit::event::ElementState::Pressed => {
                            input.press_key(keycode);
                        }
                        winit::event::ElementState::Released => {
                            input.release_key(keycode);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event
            && let Some(mut mouse) = self.world.get_resource_mut::<Mouse>()
        {
            mouse.add_delta(delta.0 as f32, delta.1 as f32);
        }
    }
}
