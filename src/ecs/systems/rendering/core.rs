use crate::ecs::events::WindowResizeEvent;
use crate::ecs::{components::*, resources::*};
use crate::graphics::core::*;
use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct RenderingState {
    pub frame_count: u64,
    pub draw_calls: u32,
    pub triangles: u32,
}

pub fn resize_system(
    mut events: MessageReader<WindowResizeEvent>,
    mut rendering_context: ResMut<RenderingContext>,
    mut camera: ResMut<MainCamera>,
) {
    for event in events.read() {
        if event.width > 0 && event.height > 0 {
            rendering_context.resize(event.width, event.height);

            camera
                .get_mut()
                .set_aspect_ratio(event.width as f32 / event.height as f32);
        }
    }
}

pub fn render_system(
    rendering_context: Res<RenderingContext>,
    mut asset_manager: ResMut<AssetManager>,
    camera_buffer: Res<CameraBuffer>,
    lighting_buffer: Res<LightingBuffer>,
    shadow_map: Res<ShadowMap>,
    light_camera_buffer: Res<LightCameraBuffer>,
    query: Query<(&Transform, &Mesh, &Material)>,
) {
    let gpu = &rendering_context.gpu;
    let surface = &rendering_context.surface;
    let basic_pipeline = &rendering_context.basic_pipeline;

    asset_manager.update_texture_bindings(gpu);

    let surface_texture = match surface.get_current_texture() {
        Ok(texture) => texture,
        Err(_) => return,
    };

    let view = surface_texture.create_view();

    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut shadow_pass = RenderPass::builder()
            .label("Shadow Pass")
            .clear_depth(1.0)
            .begin_depth_only(&mut encoder, shadow_map.view());

        rendering_context.shadow_pipeline.draw(
            &mut shadow_pass,
            light_camera_buffer.bind_group(),
            query,
        );
    }

    {
        let mut render_pass = RenderPass::builder()
            .label("Main Render Pass")
            .clear_color(0.1, 0.2, 0.3, 1.0)
            .clear_depth(1.0)
            .begin(&mut encoder, &view, Some(rendering_context.depth_view()));

        basic_pipeline.draw(
            &mut render_pass,
            &asset_manager,
            camera_buffer.as_ref(),
            lighting_buffer.as_ref(),
            &shadow_map,
            query,
        );
    }

    gpu.submit(Some(encoder.finish()));
    surface_texture.present();
}
