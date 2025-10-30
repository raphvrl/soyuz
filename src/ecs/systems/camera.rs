use crate::ecs::resources::{CameraBuffer, MainCamera, RenderingContext};
use bevy_ecs::prelude::*;

pub fn update_camera_buffer_system(
    ctx: Res<RenderingContext>,
    camera_buffer: Res<CameraBuffer>,
    camera: Res<MainCamera>,
) {
    let view_proj = camera.get().view_proj_matrix();
    camera_buffer.update(&ctx.gpu, view_proj);
}
