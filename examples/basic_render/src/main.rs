use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use soyuz::ecs::components::{Mesh, Transform, VertexData};
use soyuz::ecs::resources::RenderingContext;
use soyuz::engine::app::App;
use soyuz::graphics::core::{IndexBuffer, VertexBuffer};

fn main() {
    App::new()
        .with_title("Basic Render - Cube")
        .add_startup_system(setup_scene)
        .with_size(800, 600)
        .run();
}

fn setup_scene(mut commands: Commands, rendering_context: Res<RenderingContext>) {
    let gpu = &rendering_context.gpu;

    let vertices = vec![
        VertexData {
            position: Vec3::new(-0.5, -0.5, 0.5),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        VertexData {
            position: Vec3::new(0.5, -0.5, 0.5),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        VertexData {
            position: Vec3::new(0.5, 0.5, 0.5),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
        VertexData {
            position: Vec3::new(-0.5, 0.5, 0.5),
            color: Vec3::new(1.0, 1.0, 0.0),
        },
        VertexData {
            position: Vec3::new(-0.5, -0.5, -0.5),
            color: Vec3::new(1.0, 0.0, 1.0),
        },
        VertexData {
            position: Vec3::new(0.5, -0.5, -0.5),
            color: Vec3::new(0.0, 1.0, 1.0),
        },
        VertexData {
            position: Vec3::new(0.5, 0.5, -0.5),
            color: Vec3::new(1.0, 1.0, 1.0),
        },
        VertexData {
            position: Vec3::new(-0.5, 0.5, -0.5),
            color: Vec3::new(0.5, 0.5, 0.5),
        },
    ];

    #[rustfmt::skip]
    let indices: Vec<u16> = vec![
        0, 1, 2,  2, 3, 0,
        5, 4, 7,  7, 6, 5,
        4, 0, 3,  3, 7, 4,
        1, 5, 6,  6, 2, 1,
        3, 2, 6,  6, 7, 3,
        4, 5, 1,  1, 0, 4,
    ];

    let vertex_buffer = VertexBuffer::new(&gpu.device, Some("Cube Vertices"), &vertices);
    let index_buffer = IndexBuffer::new_u16(&gpu.device, Some("Cube Indices"), &indices);

    let mesh = Mesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    };

    let transform = Transform {
        translation: Vec3::new(0.0, 0.0, -3.0),
        rotation: Quat::from_rotation_y(0.0),
        scale: Vec3::ONE,
    };

    commands.spawn((mesh.clone(), transform));

    let transform2 = Transform {
        translation: Vec3::new(3.0, 0.0, -3.0),
        rotation: Quat::from_rotation_y(0.0),
        scale: Vec3::ONE,
    };

    commands.spawn((mesh.clone(), transform2));
}
