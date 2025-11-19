use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use soyuz_gfx::Context;

pub trait App: 'static + Sized {
    /// Initializes the application with the graphics context.
    ///
    /// This function is called once during application creation, after the graphics
    /// context has been initialized. This is the ideal place to create resources
    /// needed by the application (buffers, pipelines, textures, etc.).
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`], which provides
    ///   access to the surface, device, queue, and render configuration.
    ///
    /// # Returns
    ///
    /// Returns an initialized instance of the application.
    fn init(ctx: &mut Context) -> Self;

    /// Called every frame to update and render the application.
    ///
    /// This function is called continuously on every render frame. It is responsible
    /// for updating the application logic and rendering the current frame.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`], allowing access to
    ///   rendering resources (surface, device, queue) to perform rendering operations.
    /// * `dt` - The delta time in seconds since the last frame. This value can be used
    ///   to create animations and updates that are independent of framerate. For example,
    ///   a typical value would be ~0.016 for 60 FPS.
    fn frame(&mut self, _ctx: &mut Context, _dt: f32) {}

    /// Called when the window is resized.
    ///
    /// This function is called whenever the window size changes. The default implementation
    /// updates the graphics context with the new size. Override this function to handle
    /// window resizing in your application (e.g., updating viewport, recalculating aspect ratios).
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `width` - The new width of the window in physical pixels.
    /// * `height` - The new height of the window in physical pixels.
    fn resize(&mut self, ctx: &mut Context, width: u32, height: u32) {
        ctx.resize(winit::dpi::PhysicalSize::new(width, height));
    }

    /// Called when a keyboard key is pressed.
    ///
    /// This function is called once when a key is first pressed down. Use this to handle
    /// keyboard input for your application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `key` - The [`KeyCode`](winit::keyboard::KeyCode) of the pressed key.
    fn key_pressed(&mut self, _ctx: &mut Context, _key: winit::keyboard::KeyCode) {}

    /// Called when a keyboard key is released.
    ///
    /// This function is called when a previously pressed key is released. Use this to handle
    /// key release events in your application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `key` - The [`KeyCode`](winit::keyboard::KeyCode) of the released key.
    fn key_released(&mut self, _ctx: &mut Context, _key: winit::keyboard::KeyCode) {}

    /// Called when text is input via IME (Input Method Editor).
    ///
    /// This function is called when text is committed through an IME, typically used for
    /// inputting text in languages that require composition (e.g., Japanese, Chinese).
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `text` - The committed text string.
    fn text_input(&mut self, _ctx: &mut Context, _text: &str) {}

    /// Called when the mouse cursor moves within the window.
    ///
    /// This function is called continuously as the mouse cursor moves. The coordinates are
    /// in physical pixels relative to the top-left corner of the window.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `x` - The x-coordinate of the cursor position in physical pixels.
    /// * `y` - The y-coordinate of the cursor position in physical pixels.
    fn mouse_moved(&mut self, _ctx: &mut Context, _x: f64, _y: f64) {}

    /// Called when a mouse button is pressed.
    ///
    /// This function is called once when a mouse button is first pressed down.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `button` - The [`MouseButton`](winit::event::MouseButton) that was pressed.
    fn mouse_pressed(&mut self, _ctx: &mut Context, _button: winit::event::MouseButton) {}

    /// Called when a mouse button is released.
    ///
    /// This function is called when a previously pressed mouse button is released.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `button` - The [`MouseButton`](winit::event::MouseButton) that was released.
    fn mouse_released(&mut self, _ctx: &mut Context, _button: winit::event::MouseButton) {}

    /// Called when the mouse wheel is scrolled.
    ///
    /// This function is called when the user scrolls the mouse wheel. The delta values
    /// can be either in lines (for typical mouse wheels) or pixels (for trackpads).
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `delta_x` - The horizontal scroll delta (positive = right, negative = left).
    /// * `delta_y` - The vertical scroll delta (positive = down, negative = up).
    fn mouse_scrolled(&mut self, _ctx: &mut Context, _delta_x: f32, _delta_y: f32) {}

    /// Called when the mouse cursor enters the window.
    ///
    /// This function is called once when the mouse cursor enters the window boundaries.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn cursor_entered(&mut self, _ctx: &mut Context) {}

    /// Called when the mouse cursor leaves the window.
    ///
    /// This function is called once when the mouse cursor leaves the window boundaries.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn cursor_left(&mut self, _ctx: &mut Context) {}

    /// Called when a touch input starts.
    ///
    /// This function is called when a touch point first makes contact with the screen.
    /// Useful for handling touch-based input on mobile devices and touchscreens.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `id` - A unique identifier for this touch point.
    /// * `x` - The x-coordinate of the touch position in physical pixels.
    /// * `y` - The y-coordinate of the touch position in physical pixels.
    fn touch_started(&mut self, _ctx: &mut Context, _id: u64, _x: f64, _y: f64) {}

    /// Called when a touch input moves.
    ///
    /// This function is called continuously as a touch point moves across the screen.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `id` - The unique identifier of the moving touch point.
    /// * `x` - The x-coordinate of the touch position in physical pixels.
    /// * `y` - The y-coordinate of the touch position in physical pixels.
    fn touch_moved(&mut self, _ctx: &mut Context, _id: u64, _x: f64, _y: f64) {}

    /// Called when a touch input ends.
    ///
    /// This function is called when a touch point is lifted from the screen.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `id` - The unique identifier of the touch point that ended.
    /// * `x` - The x-coordinate of the final touch position in physical pixels.
    /// * `y` - The y-coordinate of the final touch position in physical pixels.
    fn touch_ended(&mut self, _ctx: &mut Context, _id: u64, _x: f64, _y: f64) {}

    /// Called when a touch input is cancelled.
    ///
    /// This function is called when a touch input is interrupted or cancelled (e.g., by
    /// a system gesture or another application taking focus).
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `id` - The unique identifier of the touch point that was cancelled.
    /// * `x` - The x-coordinate of the touch position in physical pixels.
    /// * `y` - The y-coordinate of the touch position in physical pixels.
    fn touch_cancelled(&mut self, _ctx: &mut Context, _id: u64, _x: f64, _y: f64) {}

    /// Called when the window gains focus.
    ///
    /// This function is called when the window receives keyboard focus. Use this to resume
    /// paused operations or update the application state.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn focused(&mut self, _ctx: &mut Context) {}

    /// Called when the window loses focus.
    ///
    /// This function is called when the window loses keyboard focus. Use this to pause
    /// operations or save state if needed.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn unfocused(&mut self, _ctx: &mut Context) {}

    /// Called when the window is moved.
    ///
    /// This function is called when the window position changes on the screen.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `x` - The new x-coordinate of the window position in screen coordinates.
    /// * `y` - The new y-coordinate of the window position in screen coordinates.
    fn moved(&mut self, _ctx: &mut Context, _x: i32, _y: i32) {}

    /// Called when the window's scale factor changes.
    ///
    /// This function is called when the DPI scale factor of the window changes, typically
    /// when moving between displays with different DPI settings or when the system DPI changes.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    /// * `scale_factor` - The new scale factor (e.g., 1.0 for 96 DPI, 2.0 for 192 DPI).
    fn scale_factor_changed(&mut self, _ctx: &mut Context, _scale_factor: f64) {}

    /// Called when the application is suspended.
    ///
    /// This function is called when the application is suspended (e.g., when the device
    /// goes to sleep or the application is backgrounded). Use this to pause rendering
    /// or save state.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn suspended(&mut self, _ctx: &mut Context) {}

    /// Called when the application is resumed.
    ///
    /// This function is called when the application is resumed after being suspended.
    /// Use this to restore state or resume rendering operations.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn resumed(&mut self, _ctx: &mut Context) {}

    /// Called when the application is about to close.
    ///
    /// This function is called when the window is closed or the application is shutting down.
    /// Use this to perform cleanup operations, such as saving data or releasing resources.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the graphics [`Context`].
    fn cleanup(&mut self, _ctx: &mut Context) {}
}

struct AppHandler<A: App> {
    app: Option<A>,
    context: Option<Context>,
    window: Option<Arc<Window>>,
    last_frame: Option<std::time::Instant>,
    title: String,
}

impl<A: App> AppHandler<A> {
    fn new(title: String) -> Self {
        Self {
            app: None,
            context: None,
            window: None,
            last_frame: None,
            title,
        }
    }
}

impl<A: App> ApplicationHandler for AppHandler<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes =
                winit::window::Window::default_attributes().with_title(&self.title);

            let window = match event_loop.create_window(window_attributes) {
                Ok(window) => Arc::new(window),
                Err(e) => {
                    tracing::error!("Failed to create window: {}", e);
                    event_loop.exit();
                    return;
                }
            };

            self.window = Some(window);
        }

        if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
            app.resumed(ctx);
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
            app.suspended(ctx);
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
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.cleanup(ctx);
                }
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(ctx) = self.context.as_mut() {
                    ctx.resize(physical_size);
                    if let Some(app) = self.app.as_mut() {
                        app.resize(ctx, physical_size.width, physical_size.height);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if self.context.is_none() {
                    let window = self.window.as_ref().expect("Window should exist");
                    let context = pollster::block_on(Context::new(window.clone()));
                    self.context = Some(context);

                    if let Some(ctx) = self.context.as_mut() {
                        let app = A::init(ctx);
                        self.app = Some(app);
                    }

                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                }

                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    let now = std::time::Instant::now();
                    let dt = if let Some(last_frame) = self.last_frame {
                        now.duration_since(last_frame).as_secs_f32()
                    } else {
                        0.016
                    };
                    self.last_frame = Some(now);

                    app.frame(ctx, dt);
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut())
                    && let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key {
                        if event.state.is_pressed() {
                            app.key_pressed(ctx, key_code);
                        } else {
                            app.key_released(ctx, key_code);
                        }
                    }
            }

            WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.text_input(ctx, &text);
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.mouse_moved(ctx, position.x, position.y);
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    if state.is_pressed() {
                        app.mouse_pressed(ctx, button);
                    } else {
                        app.mouse_released(ctx, button);
                    }
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    let (delta_x, delta_y) = match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            (pos.x as f32, pos.y as f32)
                        }
                    };
                    app.mouse_scrolled(ctx, delta_x, delta_y);
                }
            }

            WindowEvent::CursorEntered { .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.cursor_entered(ctx);
                }
            }

            WindowEvent::CursorLeft { .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.cursor_left(ctx);
                }
            }

            WindowEvent::Touch(touch) => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    match touch.phase {
                        winit::event::TouchPhase::Started => {
                            app.touch_started(ctx, touch.id, touch.location.x, touch.location.y);
                        }
                        winit::event::TouchPhase::Moved => {
                            app.touch_moved(ctx, touch.id, touch.location.x, touch.location.y);
                        }
                        winit::event::TouchPhase::Ended => {
                            app.touch_ended(ctx, touch.id, touch.location.x, touch.location.y);
                        }
                        winit::event::TouchPhase::Cancelled => {
                            app.touch_cancelled(ctx, touch.id, touch.location.x, touch.location.y);
                        }
                    }
                }
            }

            WindowEvent::Focused(focused) => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    if focused {
                        app.focused(ctx);
                    } else {
                        app.unfocused(ctx);
                    }
                }
            }

            WindowEvent::Moved(position) => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.moved(ctx, position.x, position.y);
                }
            }

            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if let (Some(app), Some(ctx)) = (self.app.as_mut(), self.context.as_mut()) {
                    app.scale_factor_changed(ctx, scale_factor);
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

pub fn run<A: App>(title: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Soyuz App...");

    let event_loop = EventLoop::new().unwrap();
    let mut app_handler = AppHandler::<A>::new(title.to_string());

    event_loop.run_app(&mut app_handler).unwrap();
}
