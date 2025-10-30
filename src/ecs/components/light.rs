use bevy_ecs::prelude::Component;
use glam::Vec3;

#[derive(Component, Clone)]
pub struct PointLight {
    pub color: Vec3,
    pub intensity: f32,
    pub radius: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            color: Vec3::ONE,
            intensity: 1.0,
            radius: 10.0,
        }
    }
}
