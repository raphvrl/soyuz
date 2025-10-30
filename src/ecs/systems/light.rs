use crate::ecs::components::{PointLight, Transform};
use crate::ecs::resources::RenderingContext;
use crate::ecs::resources::lighting::{
    GpuPointLight, LightingBuffer, LightingData, MAX_POINT_LIGHTS,
};
use bevy_ecs::prelude::*;
use glam::Vec4;

pub fn update_lighting_buffer_system(
    rendering_context: Res<RenderingContext>,
    lighting_buffer: Res<LightingBuffer>,
    query: Query<(&Transform, &PointLight)>,
) {
    let mut lighting_data = LightingData::default();

    for (i, (transform, light)) in query.iter().enumerate() {
        if i >= MAX_POINT_LIGHTS {
            break;
        }

        lighting_data.point_lights[i] = GpuPointLight {
            position: Vec4::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                1.0,
            ),
            color: Vec4::new(light.color.x, light.color.y, light.color.z, 1.0),
            intensity: light.intensity,
            radius: light.radius,
            _padding: [0.0; 2],
        };

        lighting_data.num_point_lights += 1;
    }

    lighting_buffer.update(&rendering_context.gpu, &lighting_data);
}
