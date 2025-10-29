use bevy_ecs::prelude::*;
use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Resource, Default)]
pub struct Input {
    pressed_keys: HashSet<KeyCode>,
    just_pressed_keys: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn update(&mut self) {
        self.just_pressed_keys.clear();
    }

    pub(crate) fn press_key(&mut self, key: KeyCode) {
        if !self.pressed_keys.contains(&key) {
            self.just_pressed_keys.insert(key);
        }

        self.pressed_keys.insert(key);
    }

    pub(crate) fn release_key(&mut self, key: KeyCode) {
        self.pressed_keys.remove(&key);
    }
}
