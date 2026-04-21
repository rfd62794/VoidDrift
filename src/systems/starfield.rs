// Procedural infinite starfield using a custom Material2d
// Three parallax layers in a single shader pass
// Stars are generated deterministically from camera position
// No hard boundaries - infinite in all directions

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use crate::constants::*;
use crate::components::*;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct StarfieldMaterial {
    #[uniform(0)]
    pub camera_pos: Vec2,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub screen_size: Vec2,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/starfield.wgsl".into()
    }
}

pub fn setup_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    // Large quad - covers much more than screen to handle zoom out
    let size = Vec2::new(20000.0, 20000.0);
    
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(materials.add(StarfieldMaterial {
            camera_pos: Vec2::ZERO,
            time: 0.0,
            screen_size: Vec2::new(window.width(), window.height()),
        })),
        Transform::from_xyz(0.0, 0.0, Z_STARS_FAR),
    ));
}

pub fn update_starfield(
    time: Res<Time>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
) {
    let Ok(cam_transform) = camera_query.get_single() else { return };
    let cam_pos = cam_transform.translation.truncate();
    
    for material in materials.iter_mut() {
        material.1.camera_pos = cam_pos;
        material.1.time = time.elapsed_secs();
    }
}
