use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use bevy_egui::EguiContextSettings;
use rand::{Rng, SeedableRng};
use crate::constants::*;
use crate::components::*;
use crate::systems::setup::entity_setup::*;
use crate::systems::setup::quest_init::*;

/// Clean up all entities before setting up a new game
pub fn cleanup_world_entities(
    mut commands: Commands,
    entities: Query<Entity, With<Transform>>, // Remove all entities with Transform (game objects)
) {
    info!("Cleaning up {} entities before new game", entities.iter().count());
    
    let entities_to_despawn: Vec<Entity> = entities.iter().collect();
    
    for entity in entities_to_despawn {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Resets runtime resources to a clean state (call on OnExit(InGame))
pub fn reset_game_resources(
    mut queue: ResMut<ShipQueue>,
    mut cam_delta: ResMut<CameraDelta>,
    mut signal_log: ResMut<SignalLog>,
) {
    *queue = ShipQueue::default();
    *cam_delta = CameraDelta::default();
    *signal_log = SignalLog::default();
}

/// Spawns the world objects, ship, and HUD.
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut camera_delta: ResMut<CameraDelta>,
    mut map_pan_state: ResMut<MapPanState>,
    mut opening_sequence: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
    mut drawer_state: ResMut<DrawerState>,
    mut world_view_rect: ResMut<WorldViewRect>,
    mut queue: ResMut<ShipQueue>,
) {
    info!("[Voidrift Phase 4] Final Production Build. PresentMode: Fifo.");

    // Reset resources to clean state
    *camera_delta = CameraDelta::default();
    *map_pan_state = MapPanState::default();
    *opening_sequence = OpeningSequence { phase: OpeningPhase::Adrift, timer: 0.0, beat_timer: 0.0 };
    *drawer_state = DrawerState::Collapsed;
    *world_view_rect = WorldViewRect::default();
    *queue = ShipQueue::default(); // Always start with 0 — opening sequence gifts count=1 on complete
    
    // Reset SignalLog completely
    *signal_log = SignalLog::default();

    init_quest_log(&mut commands);
    spawn_starfield(&mut commands, &mut meshes, &mut materials);
    spawn_camera(&mut commands);
    spawn_opening_drone(&mut commands, &mut meshes, &mut materials, &asset_server);
    // queue starts at 0 — opening sequence will gift available_count += 1 on completion
    spawn_station(&mut commands, &mut meshes, &mut materials);
    spawn_berths(&mut commands);
    spawn_destination_highlight(&mut commands, &mut meshes, &mut materials);
}

fn spawn_starfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEAD_BEEF_u64);
    let far_mat  = materials.add(ColorMaterial {
        color: Color::srgba(1.0, 1.0, 1.0, 1.0),
        alpha_mode: AlphaMode2d::Opaque,
        ..default()
    });
    let near_mat = materials.add(ColorMaterial {
        color: Color::srgba(1.0, 1.0, 1.0, 1.0),
        alpha_mode: AlphaMode2d::Opaque,
        ..default()
    });
    // Stars are fully opaque and pushed far back to ensure Opaque2d phase
    // and avoid Z-fighting/shimmering on mobile hardware.
    let star_sm  = meshes.add(Rectangle::new(2.0, 2.0));
    let star_lg  = meshes.add(Rectangle::new(3.0, 3.0));
    
    let radius = 2400.0;
    
    for _ in 0..800 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(0.0..1.0_f32).sqrt() * radius;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        commands.spawn((
            StarLayer { layer: 0.05, orig_pos: Vec2::new(x, y) },
            Mesh2d(star_sm.clone()),
            MeshMaterial2d(far_mat.clone()),
            Transform::from_xyz(x, y, Z_STARS_FAR),
        ));
    }
    for _ in 0..300 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(0.0..1.0_f32).sqrt() * radius;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        commands.spawn((
            StarLayer { layer: 0.15, orig_pos: Vec2::new(x, y) },
            Mesh2d(star_lg.clone()),
            MeshMaterial2d(near_mat.clone()),
            Transform::from_xyz(x, y, Z_STARS_NEAR),
        ));
    }
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            far: 1200.0, // Headroom for Z_STARS_FAR (-100) from Z=1000
            ..OrthographicProjection::default_2d()
        },
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 1000.0),
        EguiContextSettings {
            scale_factor: EGUI_SCALE,
            ..default()
        },
    ));
}
