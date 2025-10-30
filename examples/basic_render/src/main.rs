use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use soyuz::ecs::components::*;
use soyuz::ecs::resources::*;
use soyuz::engine::app::App;
use winit::keyboard::KeyCode;

fn main() {
    App::new()
        .with_title("Basic Render - Cube")
        .add_startup_system(setup_scene)
        .add_system(camera_movement_system)
        .with_size(800, 600)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    ctx: Res<RenderingContext>,
    mut asset_manager: ResMut<AssetManager>,
) {
    let gpu = &ctx.gpu;

    let gpu_mesh = asset_manager
        .load_mesh(gpu, "examples/basic_render/src/monkey.glb")
        .expect("Failed to load GLB mesh");

    let mesh = Mesh::new(gpu_mesh.clone());

    for x in -10..10 {
        for z in -10..10 {
            let spacing = 4.0;

            let transform = Transform {
                translation: Vec3::new(x as f32 * spacing, 0.0, z as f32 * spacing),
                rotation: Quat::from_rotation_y(0.0),
                scale: Vec3::ONE,
            };
            commands.spawn((mesh.clone(), transform));
        }
    }
}

fn camera_movement_system(input: Res<Input>, mouse: Res<Mouse>, mut camera: ResMut<MainCamera>) {
    let walk_speed = 0.1;
    let mouse_sensitivity = 0.1;

    if mouse.delta_x != 0.0 || mouse.delta_y != 0.0 {
        camera.rotate(
            mouse.delta_x * mouse_sensitivity,
            mouse.delta_y * mouse_sensitivity,
        );
    }

    let forward = camera.forward();
    let right = camera.right();

    if input.pressed(KeyCode::KeyW) {
        camera.translate(forward * walk_speed);
    }
    if input.pressed(KeyCode::KeyS) {
        camera.translate(-forward * walk_speed);
    }
    if input.pressed(KeyCode::KeyA) {
        camera.translate(-right * walk_speed);
    }
    if input.pressed(KeyCode::KeyD) {
        camera.translate(right * walk_speed);
    }

    if input.pressed(KeyCode::Space) {
        camera.translate(Vec3::Y * walk_speed);
    }
    if input.pressed(KeyCode::ShiftLeft) {
        camera.translate(-Vec3::Y * walk_speed);
    }

    camera.update();
}
