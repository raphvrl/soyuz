use bevy_ecs::prelude::Component;

#[derive(Component, Clone)]
pub struct Material {
    pub texture_index: u32,
    pub base_color: [f32; 4],
}

impl Material {
    pub fn new(texture_index: u32) -> Self {
        Self {
            texture_index,
            base_color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn with_color(color: [f32; 4]) -> Self {
        Self {
            texture_index: 0,
            base_color: color,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::with_color([1.0, 1.0, 1.0, 1.0])
    }
}
