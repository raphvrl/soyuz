use bevy_ecs::prelude::*;

#[derive(Message)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}
