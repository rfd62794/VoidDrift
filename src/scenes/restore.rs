use bevy::prelude::*;
use crate::components::*;
use super::MainMenuState;
use crate::config::BalanceConfig;
use crate::config::VisualConfig;
use crate::config::visual::rgb;
use crate::systems::persistence::save::SaveData;
use crate::spawn_drone_core_children;
use crate::systems::setup::mesh_builder::triangle_mesh;

pub fn ingame_startup_system(
    mut menu_state: ResMut<MainMenuState>,
    mut opening: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
    mut station_query: Query<&mut Station, (With<Station>, Without<Ship>)>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut queue: ResMut<ShipQueue>,
    mut max_dispatch: ResMut<MaxDispatch>,
    opening_drone_query: Query<Entity, With<InOpeningSequence>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut requests_tab: ResMut<RequestsTabState>,
    mut tutorial: ResMut<TutorialState>,
    mut pan_state: ResMut<MapPanState>,
    cfg: Res<BalanceConfig>,
    vcfg: Res<VisualConfig>,
) {
    // Reset tutorial for every session (new game starts clean; load path overrides below)
    *tutorial = TutorialState::default();

    if let Some(save_data) = menu_state.pending_load.take() {
        restore_save_state(
            &save_data,
            &mut opening,
            &mut signal_log,
            &mut station_query,
            &mut active_tab,
            &mut queue,
            &mut max_dispatch,
            &opening_drone_query,
            &mut commands,
            &mut requests_tab,
            &mut pan_state,
        );
        spawn_saved_drones(&save_data, &mut commands, &mut meshes, &mut materials, &cfg, &vcfg);
        // Suppress all Phase 4a tutorial popups when loading an existing save
        for id in [101u32, 102, 103, 104, 105, 106] {
            tutorial.shown.insert(id);
        }
    }
    // NEW GAME PATH: opening sequence runs normally — tutorial starts empty.

    apply_dev_mode_signal(&mut menu_state, &mut signal_log);
}

/// Restores all game state from a save file. No entity spawning.
pub fn restore_save_state(
    save_data: &SaveData,
    opening: &mut OpeningSequence,
    signal_log: &mut SignalLog,
    station_query: &mut Query<&mut Station, (With<Station>, Without<Ship>)>,
    active_tab: &mut ActiveStationTab,
    queue: &mut ShipQueue,
    max_dispatch: &mut MaxDispatch,
    opening_drone_query: &Query<Entity, With<InOpeningSequence>>,
    commands: &mut Commands,
    requests_tab: &mut RequestsTabState,
    pan_state: &mut MapPanState,
) {
    opening.phase = match save_data.opening_phase.as_str() {
        "Adrift"           => OpeningPhase::Adrift,
        "SignalIdentified" => OpeningPhase::SignalIdentified,
        "AutoPiloting"     => OpeningPhase::AutoPiloting,
        "InRange"          => OpeningPhase::InRange,
        "Docked"           => OpeningPhase::Docked,
        "Complete"         => OpeningPhase::Complete,
        _                  => OpeningPhase::Complete,
    };
    opening.timer = 0.0;

    // Reset camera focus to station on load
    pan_state.is_focused = false;

    if opening.phase == OpeningPhase::Complete {
        for ent in opening_drone_query.iter() {
            commands.entity(ent).despawn_recursive();
        }
    }

    queue.available_count = save_data.ship_hulls as u32;
    if opening.phase == OpeningPhase::Complete && queue.available_count == 0 {
        queue.available_count = 1;
        info!("[Voidrift] Load sanity check: Gifting emergency drone to empty fleet.");
    }

    if let Ok(mut station) = station_query.get_single_mut() {
        station.online               = save_data.station_online;
        station.iron_reserves        = save_data.iron;
        station.iron_ingots          = save_data.iron_ingots;
        station.tungsten_reserves    = save_data.tungsten;
        station.tungsten_ingots      = save_data.tungsten_ingots;
        station.nickel_reserves      = save_data.nickel;
        station.nickel_ingots        = save_data.nickel_ingots;
        station.aluminum_reserves    = save_data.aluminum;
        station.aluminum_ingots      = save_data.aluminum_ingots;
        station.aluminum_canisters   = save_data.aluminum_canisters;
        station.hull_plate_reserves  = save_data.hull_plates;
        station.thruster_reserves    = save_data.thruster_reserves;
        station.ai_cores             = save_data.ai_cores;
        station.repair_progress      = save_data.repair_progress;
        station.drone_build_progress = save_data.drone_build_progress;
        station.power_multiplier     = if save_data.power_multiplier > 0.0 { save_data.power_multiplier } else { 1.0 };
        station.max_dispatch          = if save_data.max_dispatch > 0 { save_data.max_dispatch } else { 5 };
        max_dispatch.0 = station.max_dispatch;
    }

    *active_tab = match save_data.active_tab.as_str() {
        "Cargo"      => ActiveStationTab::Cargo,
        "Production" => ActiveStationTab::Production,
        "Requests"   => ActiveStationTab::Requests,
        "Logs"       => ActiveStationTab::Logs,
        _            => ActiveStationTab::Cargo,
    };

    requests_tab.collected_requests = save_data.collected_requests.clone();

    signal_log.fired = save_data.signal_fired_ids.iter().copied().collect();
    signal_log.entries.push_back("ECHO: SAVE LOADED SUCCESSFULLY.".to_string());
    signal_log.entries.push_back(format!("ECHO: {} RESTORED.", save_data.save_name.to_uppercase()));

    // Restore telemetry consent and session counter
    commands.insert_resource(crate::systems::telemetry::TelemetryConsent {
        opted_in: save_data.telemetry_consent,
    });
    commands.insert_resource(crate::systems::telemetry::TelemetrySessionCounter {
        sessions: save_data.telemetry_sessions,
    });
}

/// Spawns saved drone entities from save data. State restore only — no game logic.
pub fn spawn_saved_drones(
    save_data: &SaveData,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    cfg: &BalanceConfig,
    vcfg: &VisualConfig,
) {
    for d in save_data.drones.iter() {
        // DroneSaveData no longer stores state, ore_type, cargo - use defaults
        let state = ShipState::Idle;
        let ore_type = OreDeposit::Iron;
        let cargo = 0.0;

        let ship_ent = commands.spawn((
            Ship {
                state,
                speed: cfg.mining.ship_speed,
                cargo,
                cargo_type: ore_type,
                cargo_capacity: cfg.mining.cargo_capacity,
                laser_tier: LaserTier::Basic,
                current_mining_target: None,
            },
            AutonomousShipTag,
            LastHeading(d.heading),
            Transform::from_xyz(d.pos_x, d.pos_y, vcfg.z_layer.z_ship),
            Mesh2d(meshes.add(triangle_mesh(vcfg.drone.mission.hull_w, vcfg.drone.mission.hull_h))),
            MeshMaterial2d(materials.add(rgb(vcfg.drone.mission.color_hull))),
        )).id();

        if d.assignment_pos_x != 0.0 || d.assignment_pos_y != 0.0 {
            commands.entity(ship_ent).insert(AutopilotTarget {
                destination: Vec2::new(d.assignment_pos_x, d.assignment_pos_y),
                target_entity: None,
            });
        }

        let md = &vcfg.drone.mission;
        commands.entity(ship_ent).with_children(|parent| {
            spawn_drone_core_children!(parent, meshes, materials, md, vcfg);
        });
    }
}

/// Writes developer mode signal log entries (once per session).
pub fn apply_dev_mode_signal(menu_state: &mut MainMenuState, signal_log: &mut SignalLog) {
    if menu_state.developer_mode && !menu_state.dev_mode_signal_fired {
        signal_log.entries.push_back("ECHO: DEVELOPER MODE ENABLED.".to_string());
        signal_log.entries.push_back("ECHO: STAGE SAVES NOW ACCESSIBLE.".to_string());
        menu_state.dev_mode_signal_fired = true;
    }
}
