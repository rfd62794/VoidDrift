// Voidrift — Phase 1 World Scaffold
// ============================================================================
// Scope: Phase 1 ONLY.
// Deliverable: 2D Space Scene with static Ship, Asteroid, and Station.
// Hard Scope: No movement, no grid, no camera controller.
//
// Gate 1 behaviours this file must satisfy:
//   TB-P1-01: Screenshot shows Ship (origin), Asteroid (nearby), Station (offset)
// ============================================================================

use bevy::{prelude::*, sprite::{Mesh2d, MeshMaterial2d}};

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                title: "Voidrift".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .add_systems(Startup, setup_world)
        .add_systems(Update, log_touches)
        .run();
}

/// Spawns the camera and the three core visual entities at static positions.
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2D Camera centered at origin.
    commands.spawn(Camera2d::default());

    // 1. SHIP (World Origin)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), // Cyan
        Transform::from_xyz(0.0, 0.0, 1.0), // Above background
    ));

    // 2. ASTEROID (Nearby)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(48.0, 48.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))), // Grey
        Transform::from_xyz(150.0, 100.0, 0.5),
    ));

    // 3. STATION (Offset - focused for mobile visibility)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))), // Yellow
        Transform::from_xyz(-150.0, -200.0, 0.2),
    ));

    info!("[Voidrift Phase 1] World Scaffolding Initialized.");
}

/// Reads all active touches each frame and prints their position to logcat.
fn log_touches(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
            "[Voidrift Phase 1] Touch detected — id: {}, position: ({:.1}, {:.1})",
            touch.id(),
            touch.position().x,
            touch.position().y,
        );
    }
}
