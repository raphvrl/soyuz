pub use soyuz_gfx::{Context, GraphicsBuilder, RenderPassBuilder};

pub use wgpu;
pub use winit;

pub mod app;

pub use app::{new, run, AppBuilder};

pub mod prelude {
    pub use crate::wgpu;

    pub use crate::winit;
    pub use winit::keyboard::KeyCode;

    pub use crate::Context;
    pub use crate::app::*;
    pub use crate::{new, run, AppBuilder, GraphicsBuilder};
}
