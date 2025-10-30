use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use soyuz::ecs::components::*;
use soyuz::ecs::resources::*;
use soyuz::engine::app::App;
use soyuz::graphics::resources::mesh::GpuMesh;
use std::sync::Arc;
use winit::keyboard::KeyCode;

fn main() {
    App::new()
        .with_title("Basic Render - Cube")
        .add_startup_system(setup_scene)
        .add_system(camera_movement_system)
        .with_size(1280, 720)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    ctx: Res<RenderingContext>,
    mut asset_manager: ResMut<AssetManager>,
) {
    let gpu = &ctx.gpu;

    let gltf_asset = asset_manager
        .load_gltf_asset(gpu, "examples/basic_render/src/cube.glb")
        .expect("Failed to load GLTF asset");

    if let Some(mesh_data) = gltf_asset.meshes.first() {
        let gpu_mesh = Arc::new(GpuMesh::new(
            gpu,
            &mesh_data.vertices,
            &mesh_data.indices,
            Some("Cube Mesh"),
        ));
        let mesh = Mesh::new(gpu_mesh);

        let material = gltf_asset
            .materials
            .first()
            .map(|mat| asset_manager.load_material_from_gltf(gpu, mat, &gltf_asset.textures))
            .unwrap_or_else(|| Material::default());

        let transform = Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_y(0.0),
            scale: Vec3::ONE,
        };

        commands.spawn((mesh, material, transform));
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
