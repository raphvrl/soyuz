use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct Mouse {
    pub delta_x: f32,
    pub delta_y: f32,
    initialized: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            delta_x: 0.0,
            delta_y: 0.0,
            initialized: false,
        }
    }

    pub fn update(&mut self) {
        self.delta_x = 0.0;
        self.delta_y = 0.0;
        self.initialized = true;
    }

    pub(crate) fn add_delta(&mut self, delta_x: f32, delta_y: f32) {
        if self.initialized {
            self.delta_x += delta_x;
            self.delta_y += delta_y;
        }
    }
}
