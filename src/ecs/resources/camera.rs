use crate::graphics::resources::camera::Camera;

use bevy_ecs::prelude::*;
use glam::Vec3;

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

    pub fn position(&self) -> Vec3 {
        self.0.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.0.set_position(position);
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.0.translate(offset);
    }

    pub fn forward(&self) -> Vec3 {
        self.0.forward()
    }

    pub fn right(&self) -> Vec3 {
        self.0.right()
    }

    pub fn rotate(&mut self, x_offset: f32, y_offset: f32) {
        self.0.rotate(x_offset, y_offset);
    }

    pub fn update(&mut self) {
        self.0.update();
    }
}
