use crate::ecs::components::{DirectionalLight, PointLight, Transform};
use crate::ecs::resources::RenderingContext;
use crate::ecs::resources::lighting::{
    GpuDirectionalLight, GpuPointLight, LightingBuffer, LightingData, MAX_POINT_LIGHTS,
};
use bevy_ecs::prelude::*;
use glam::Vec4;

pub fn update_lighting_buffer_system(
    rendering_context: Res<RenderingContext>,
    lighting_buffer: Res<LightingBuffer>,
    point_light_query: Query<(&Transform, &PointLight)>,
    directional_light_query: Query<&DirectionalLight, Without<PointLight>>,
) {
    let mut lighting_data = LightingData::default();

    for (i, (transform, light)) in point_light_query.iter().enumerate() {
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

    if let Some(directional_light) = directional_light_query.iter().next() {
        lighting_data.directional_light = GpuDirectionalLight {
            direction: directional_light.direction,
            _pad0: 0.0,
            color: directional_light.color,
            intensity: directional_light.intensity,
        };
    }

    lighting_buffer.update(&rendering_context.gpu, &lighting_data);
}
