use crate::graphics::core::{GpuContext, Surface};
use std::sync::Arc;
use winit::window::Window;

pub trait RenderSystem {
    fn init(&mut self, gpu_context: &GpuContext, surface: &Surface);

    fn render(&mut self, gpu_context: &GpuContext, surface: &Surface, window: &Arc<Window>);

    fn resize(&mut self, gpu_context: &GpuContext, width: u32, height: u32);
}
