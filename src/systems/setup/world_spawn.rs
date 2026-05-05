use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use bevy_egui::EguiContextSettings;
use rand::{Rng, SeedableRng};
use crate::components::*;
use crate::constants::*;
use crate::systems::setup::entity_setup::*;
use crate::systems::setup::quest_init::*;
use crate::config::{BalanceConfig, VisualConfig, QuestConfig};

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

/// Resets all runtime resources to a clean state before world spawn.
pub fn reset_game_resources(
    mut queue: ResMut<ShipQueue>,
    mut cam_delta: ResMut<CameraDelta>,
    mut signal_log: ResMut<SignalLog>,
    mut map_pan_state: ResMut<MapPanState>,
    mut opening_sequence: ResMut<OpeningSequence>,
    mut drawer_state: ResMut<DrawerState>,
    mut world_view_rect: ResMut<WorldViewRect>,
    mut content_state: ResMut<ContentState>,
    mut view_state: ResMut<ViewState>,
) {
    *queue = ShipQueue::default();
    *cam_delta = CameraDelta::default();
    *signal_log = SignalLog::default();
    *map_pan_state = MapPanState::default();
    *opening_sequence = OpeningSequence { phase: OpeningPhase::Adrift, timer: 0.0, beat_timer: 0.0 };
    *drawer_state = DrawerState::Collapsed;
    *world_view_rect = WorldViewRect::default();
    *content_state = ContentState::default();
    view_state.show_production_tree = false;
}

/// Spawns the world objects, ship, and HUD. Resource reset handled by reset_game_resources.
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    max_dispatch: ResMut<MaxDispatch>,
    cfg: Res<BalanceConfig>,
    vcfg: Res<VisualConfig>,
    quest_cfg: Res<QuestConfig>,
) {
    info!("[Voidrift Phase 4] Final Production Build. PresentMode: Fifo.");

    init_quest_log(&mut commands, &quest_cfg);
    spawn_starfield(&mut commands, &mut meshes, &mut materials, &vcfg);
    spawn_camera(&mut commands);
    spawn_opening_drone(&mut commands, &mut meshes, &mut materials, &asset_server, &cfg, &vcfg);
    spawn_station(&mut commands, &mut meshes, &mut materials, max_dispatch, &vcfg);
    spawn_berths(&mut commands);
    spawn_destination_highlight(&mut commands, &mut meshes, &mut materials);
    spawn_tutorial_highlight(&mut commands, &mut meshes, &mut materials);
}

fn spawn_starfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    vcfg: &VisualConfig,
) {
    let sf = &vcfg.starfield;
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
    let star_sm  = meshes.add(Rectangle::new(sf.far_size, sf.far_size));
    let star_lg  = meshes.add(Rectangle::new(sf.near_size, sf.near_size));

    for _ in 0..sf.far_count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(0.0..1.0_f32).sqrt() * sf.radius;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        commands.spawn((
            StarLayer { layer: sf.far_parallax, orig_pos: Vec2::new(x, y) },
            Mesh2d(star_sm.clone()),
            MeshMaterial2d(far_mat.clone()),
            Transform::from_xyz(x, y, Z_STARS_FAR),
        ));
    }
    for _ in 0..sf.near_count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(0.0..1.0_f32).sqrt() * sf.radius;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        commands.spawn((
            StarLayer { layer: sf.near_parallax, orig_pos: Vec2::new(x, y) },
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
