use soyuz_app::prelude::*;

struct SettingsDemo {
    vsync_enabled: bool,
}

impl App for SettingsDemo {
    fn init(_ctx: &mut Context) -> Self {
        println!("Controls:");
        println!("  F11 or F - Toggle fullscreen");
        println!("  V - Toggle VSync");
        println!("  1 - Set window to 800x600");
        println!("  2 - Set window to 1280x720");
        println!("  3 - Set window to 1920x1080");
        println!("  M - Toggle maximize");
        println!("  ESC - Exit fullscreen");

        Self {
            vsync_enabled: false, // Par défaut Immediate mode
        }
    }

    fn frame(&mut self, ctx: &mut Context, _dt: f32) {
        ctx.render(|ctx, view, encoder| {
            let _pass = ctx
                .render_pass(encoder, view)
                .clear_rgb(0.1, 0.2, 0.3)
                .label("Settings Demo Pass")
                .begin();
        });
    }

    fn key_pressed(&mut self, ctx: &mut Context, key: KeyCode) {
        match key {
            // Toggle fullscreen
            KeyCode::F11 | KeyCode::KeyF => {
                ctx.toggle_fullscreen();
                println!("Fullscreen: {}", ctx.is_fullscreen());
            }

            // Toggle VSync
            KeyCode::KeyV => {
                self.vsync_enabled = !self.vsync_enabled;
                let mode = if self.vsync_enabled {
                    wgpu::PresentMode::AutoVsync
                } else {
                    wgpu::PresentMode::Immediate
                };
                ctx.set_present_mode(mode);
                println!("VSync: {} (mode: {:?})", self.vsync_enabled, mode);
            }

            // Preset resolutions
            KeyCode::Digit1 => {
                if !ctx.is_fullscreen() {
                    ctx.set_window_size(800, 600);
                    println!("Window size set to 800x600");
                }
            }
            KeyCode::Digit2 => {
                if !ctx.is_fullscreen() {
                    ctx.set_window_size(1280, 720);
                    println!("Window size set to 1280x720");
                }
            }
            KeyCode::Digit3 => {
                if !ctx.is_fullscreen() {
                    ctx.set_window_size(1920, 1080);
                    println!("Window size set to 1920x1080");
                }
            }

            // Toggle maximize
            KeyCode::KeyM => {
                if !ctx.is_fullscreen() {
                    let maximized = !ctx.is_maximized();
                    ctx.set_maximized(maximized);
                    println!("Maximized: {}", maximized);
                }
            }

            // Exit fullscreen with ESC
            KeyCode::Escape => {
                if ctx.is_fullscreen() {
                    ctx.set_fullscreen(false);
                    println!("Exited fullscreen");
                }
            }

            _ => {}
        }
    }

    fn resize(&mut self, ctx: &mut Context, width: u32, height: u32) {
        ctx.resize(winit::dpi::PhysicalSize::new(width, height));
        println!("Window resized to {}x{}", width, height);
    }
}

fn main() {
    builder()
        .title("Settings Demo - Press F11 for fullscreen, V for VSync")
        .size(1280, 720)
        .run::<SettingsDemo>();
}
