# Soyuz

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![wgpu](https://img.shields.io/badge/wgpu-27.0.1-green.svg)

Soyuz is a low-level and minimalist Rust toolkit designed for personal use to facilitate the initialization and development of [wgpu](https://wgpu.rs/) projects.

This project provides a light abstraction layer on top of wgpu and winit to simplify boilerplate code while maintaining direct access to the wgpu API. It's a personal tool designed to accelerate the startup of new graphics projects by avoiding the need to rewrite the same initialization code repeatedly.

## Purpose

Soyuz aims to reduce the boilerplate typically required when starting a new wgpu project. Instead of manually setting up window creation, event loop management, surface initialization, and context handling every time, Soyuz provides a simple framework that handles these common tasks while still allowing full access to the underlying wgpu and winit APIs when needed.

The toolkit is intentionally minimalist and low-level. It doesn't provide a complete rendering engine or high-level abstractions. Instead, it focuses on the essential setup and management tasks, leaving you free to build your rendering logic on top.

## Structure

Soyuz is organized into two main modules:

The `soyuz-gfx` crate handles the low-level graphics context, providing wrappers around wgpu components like the device, queue, and surface. It includes builder patterns for creating render pipelines and render passes in a more ergonomic way, and utilities for compiling shaders from WGSL source code.

The `soyuz-app` crate provides the application framework, including the App trait that structures your application logic with hooks for initialization, frame rendering, and various input events. It manages the winit event loop automatically and handles the graphics context initialization transparently.

## Personal Use Only

This toolkit is designed specifically for personal use as a quick starting point for wgpu projects. It doesn't claim to be a general-purpose or complete library suitable for production use by others. The API may change without notice, and there's no commitment to maintain backward compatibility or provide support for external users.

## Installation

To use Soyuz in your project, add it as a dependency directly from GitHub. Add the following to your `Cargo.toml` file:

```toml
[dependencies]
soyuz-app = { git = "https://github.com/raphvrl/soyuz" }
soyuz-gfx = { git = "https://github.com/raphvrl/soyuz" }
```

Since Soyuz uses a workspace structure with multiple crates, you need to specify the path for each crate when installing from GitHub. The paths `soyuz-app` and `soyuz-gfx` point to the respective crate directories within the repository.

After adding these dependencies, run `cargo build` and Cargo will automatically clone the repository and compile the necessary components.

## Example

Here's a simple example to get started with Soyuz:

```rust
use soyuz_app::prelude::*;

struct MyApp;

impl App for MyApp {
    fn init(_ctx: &mut Context) -> Self {
        Self
    }

    fn frame(&mut self, ctx: &mut Context, _dt: f32) {
        ctx.render(|ctx, view, encoder| {
            let _render_pass = ctx
                .render_pass(encoder, view)
                .clear_rgb(0.1, 0.2, 0.3)
                .label("Clear Pass")
                .begin();
        });
    }
}

fn main() {
    soyuz_app::run::<MyApp>("My App");
}
```

This example creates a simple application that clears the screen with a dark blue color each frame.

## Dependencies

Soyuz relies on several key dependencies:

- `wgpu` version 27.0.1 for the cross-platform graphics API
- `winit` version 0.30.12 for window and event management
- `tracing` for structured logging throughout the framework
- `pollster` as an async runtime utility for handling the asynchronous initialization process

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
