use crate::graphics::resources::camera::Camera;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct MainCamera(Camera);

impl MainCamera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self(Camera::new_perspective(fov, aspect, near, far))
    }
    
    pub fn get(&self) -> &Camera {
        &self.0
    }
    
    pub fn get_mut(&mut self) -> &mut Camera {
        &mut self.0
    }
}