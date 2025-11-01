use crate::ecs::components::{DirectionalLight, Mesh};
use crate::ecs::resources::{LightCameraBuffer, RenderingContext, ShadowMap};
use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3};

const SHADOW_NEAR: f32 = 0.1;
const SHADOW_FAR: f32 = 100.0;
const SHADOW_ORTHO_SIZE: f32 = 50.0;

pub fn update_shadow_map_system(
    rendering_context: Res<RenderingContext>,
    shadow_map: Res<ShadowMap>,
    light_camera_buffer: Res<LightCameraBuffer>,
    query: Query<(&DirectionalLight,), Without<Mesh>>,
) {
    if let Some((light,)) = query.iter().next() {
        let light_direction = light.direction.normalize();
        let light_position = -light_direction * SHADOW_FAR * 0.5;
        let light_target = Vec3::ZERO;

        let light_view = Mat4::look_at_rh(light_position, light_target, Vec3::Y);

        let light_proj = Mat4::orthographic_rh(
            -SHADOW_ORTHO_SIZE / 2.0,
            SHADOW_ORTHO_SIZE / 2.0,
            -SHADOW_ORTHO_SIZE / 2.0,
            SHADOW_ORTHO_SIZE / 2.0,
            SHADOW_NEAR,
            SHADOW_FAR,
        );

        let light_space_matrix = light_proj * light_view;

        light_camera_buffer.update(&rendering_context.gpu, &light_space_matrix);

        shadow_map.update_light_space(&rendering_context.gpu, &light_space_matrix);
    }
}
