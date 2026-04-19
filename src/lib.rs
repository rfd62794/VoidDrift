// Voidrift — Phase 4: Station UI & Refinery (Final Gate 4 Build)
// ============================================================================
// Goal: Final Phase 4 closure. Opt-C: Logic verified, text deferred.
// ============================================================================

use bevy::{
    prelude::*,
    render::mesh::Mesh2d,
    sprite::MeshMaterial2d,
};
use bevy_egui::{egui, EguiPlugin, EguiContextSettings, EguiContexts};
use rand::{Rng, SeedableRng};

mod constants;
pub use constants::*;

mod components;
pub use components::*;

pub mod systems;
use systems::setup::*;
use systems::visuals::*;
use systems::autopilot::*;
use systems::mining::*;
use systems::autonomous::*;
use systems::economy::*;

// ----------------------------------------------------------------------------
// APP SETUP
// ----------------------------------------------------------------------------

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                present_mode: bevy::window::PresentMode::Fifo,
                title: "Voidrift".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .insert_resource(CameraDelta::default())
        .add_systems(Startup, systems::setup::setup_world)
        .add_systems(Update, (
            systems::autopilot::autopilot_system, 
            camera_follow_system, 
            systems::visuals::starfield_scroll_system,
            systems::visuals::ship_rotation_system,
            systems::visuals::thruster_glow_system,
        ).chain())
        .add_systems(OnEnter(GameState::MapView), enter_map_view)
        .add_systems(OnExit(GameState::MapView), exit_map_view)
        .add_systems(Update, (
            systems::mining::mining_system, 
            hud_ui_system,
            station_visual_system,
            systems::autonomous::autonomous_ship_system,
            ship_cargo_display_system,
            autonomous_ship_cargo_display_system,
            systems::economy::station_status_system,
            systems::economy::ship_self_preservation_system,
            systems::economy::station_maintenance_system,
        ))
        .add_systems(Update, handle_input)
        .run();
}

// ----------------------------------------------------------------------------
// SYSTEMS
// ----------------------------------------------------------------------------

pub fn add_log_entry(station: &mut Station, entry: String) {
    // Avoid duplicate consecutive logs if possible (optional but cleaner)
    if station.log.back() == Some(&entry) { return; }
    station.log.push_back(entry);
    if station.log.len() > LOG_MAX_LINES {
        station.log.pop_front();
    }
}

fn ship_cargo_display_system(ship_query: Query<&Ship>, mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for (mut tr, parent, mat_handle) in fill_query.iter_mut() {
        if let Ok(ship) = ship_query.get(**parent) {
            let r = ship.cargo / ship.cargo_capacity as f32;
            tr.scale.x = r.max(0.001);
            tr.translation.x = -20.0 + (20.0 * r);
            
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = match ship.cargo_type {
                    OreType::Magnetite => Color::srgb(0.8, 0.3, 0.3),
                    OreType::Carbon => Color::srgb(0.3, 0.8, 0.3),
                    OreType::Empty => Color::srgb(0.5, 0.5, 0.5),
                };
            }
        }
    }
}

fn autonomous_ship_cargo_display_system(ship_query: Query<&AutonomousShip>, mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for (mut tr, parent, mat_handle) in fill_query.iter_mut() {
        if let Ok(ship) = ship_query.get(**parent) {
            let r = ship.cargo / CARGO_CAPACITY as f32;
            tr.scale.x = r.max(0.001);
            tr.translation.x = -15.0 + (15.0 * r);
            
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = Color::srgb(1.0, 0.5, 0.0); // Autonomous ships are orange
            }
        }
    }
}

fn hud_ui_system(
    mut contexts: EguiContexts,
    mut ship_query: Query<&mut Ship>,
    mut station_query: Query<(Entity, &mut Station)>,
    auto_ship_query: Query<&AutonomousShip>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    carbon_field_query: Query<Entity, (With<AsteroidField>, Without<MapMarker>)>,
) {
    let mut ship = ship_query.single_mut();
    let ctx = contexts.ctx_mut();

    // 1. MAP TOGGLE (Always available)
    egui::SidePanel::left("navigation_panel")
        .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(0)))
        .show(ctx, |ui| {
            ui.add_space(16.0);
            let label = if *state.get() == GameState::SpaceView { "MAP" } else { "EXIT MAP" };
            if ui.add(egui::Button::new(label).min_size(egui::vec2(80.0, 40.0))).clicked() {
                if *state.get() == GameState::SpaceView {
                    next_state.set(GameState::MapView);
                } else {
                    next_state.set(GameState::SpaceView);
                }
            }
        });

    // 2. REFINERY & LOG UI (Only when docked)
    if ship.state == ShipState::Docked {
        egui::TopBottomPanel::bottom("refinery_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                
                if let Ok((_station_ent, mut station)) = station_query.get_single_mut() {
                    ui.vertical_centered(|ui| {
                        // SECTION 1: SYSTEM LOG (Fixed 5-line height)
                        ui.group(|ui| {
                            ui.set_height(60.0); 
                            egui::ScrollArea::vertical()
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    for line in &station.log {
                                        ui.label(egui::RichText::new(line).monospace().size(9.0).color(egui::Color32::LIGHT_GRAY));
                                    }
                                });
                        });
                        ui.add_space(4.0);

                        // SECTION 2: RESOURCE STATUS BAR (Phase 9 Expansion)
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 6.0;
                            ui.label(format!("MAG: {:.0}", station.magnetite_reserves));
                            ui.separator();
                            ui.label(format!("CAR: {:.0}", station.carbon_reserves));
                            ui.separator();
                            ui.label(egui::RichText::new(format!("CELLS: {}", station.power_cells)).color(egui::Color32::LIGHT_BLUE));
                            ui.separator();
                            ui.label(format!("PLT: {}", station.hull_plate_reserves));
                            ui.separator();
                            ui.label(format!("HUL: {}", station.ship_hulls));
                            ui.separator();
                            ui.label(format!("COR: {}", station.ai_cores));
                        });
                        
                        // [PHASE 8b] POWER STATUS
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("STATION POWER: {:.1}/{:.0}", station.power, STATION_POWER_MAX)).color(if station.power < STATION_POWER_FLOOR { egui::Color32::RED } else { egui::Color32::LIGHT_BLUE }));
                            ui.separator();
                            ui.label(egui::RichText::new(format!("SHIP POWER: {:.1}/{:.0}", ship.power, SHIP_POWER_MAX)).color(egui::Color32::LIGHT_GREEN));
                        });
                        ui.add_space(4.0);

                        // SHARED LOGIC
                        let bootstrap_mode = station.power_cells == 0;
                        let automation_suspended = station.power < STATION_POWER_FLOOR && !bootstrap_mode;

                        // SECTION 3: REFINERY ROW
                        ui.horizontal(|ui| {
                            let can_refine_mag = station.magnetite_reserves >= REFINERY_RATIO as f32;
                            let has_power_mag = station.power_cells >= POWER_COST_REFINERY;
                            let label_mag = if bootstrap_mode { "BOOTSTRAP MODE".to_string() } else if automation_suspended { "SUSPENDED".to_string() } else if has_power_mag { "REFINE CELLS".to_string() } else { "REFINERY OFFLINE".to_string() };
                            
                            if ui.add_sized([115.0, 30.0], egui::Button::new(label_mag)).clicked() && can_refine_mag {
                                if bootstrap_mode || (has_power_mag && !automation_suspended) {
                                    let cells = (station.magnetite_reserves as u32) / REFINERY_RATIO;
                                    station.magnetite_reserves -= (cells * REFINERY_RATIO) as f32;
                                    station.power_cells += cells;
                                    if !bootstrap_mode {
                                        station.power_cells -= POWER_COST_REFINERY;
                                        add_log_entry(&mut station, format!("[STATION AI] Magnetite refined -> {} cells.", cells));
                                    } else {
                                        add_log_entry(&mut station, "[STATION AI] Emergency bootstrap. Refinery running on reserve.".to_string());
                                    }
                                }
                            }

                            let can_refine_carb = station.carbon_reserves >= HULL_REFINERY_RATIO as f32;
                            let has_power_hull = station.power_cells >= POWER_COST_HULL_FORGE;
                            let label_hull = if automation_suspended { "SUSPENDED".to_string() } else if has_power_hull { "REFINE HULL".to_string() } else { "FORGE OFFLINE".to_string() };

                            if ui.add_sized([115.0, 30.0], egui::Button::new(label_hull)).clicked() && can_refine_carb && has_power_hull && !automation_suspended {
                                let plates = (station.carbon_reserves as u32) / HULL_REFINERY_RATIO;
                                station.carbon_reserves -= (plates * HULL_REFINERY_RATIO) as f32;
                                station.hull_plate_reserves += plates;
                                station.power_cells -= POWER_COST_HULL_FORGE;
                                add_log_entry(&mut station, format!("[STATION AI] Hull synthesis complete: {} units.", plates));
                            }
                        });
                        ui.add_space(4.0);

                        // SECTION 4: MANUFACTURING ROW (Hull Forge & Core Fabrication)
                        ui.horizontal(|ui| {
                            let can_forge_hull = station.hull_plate_reserves >= SHIP_HULL_COST_PLATES && station.power_cells >= POWER_COST_SHIP_FORGE;
                            let forge_hull_label = if automation_suspended { "SUSPENDED".to_string() } 
                                                  else if station.hull_plate_reserves < SHIP_HULL_COST_PLATES { format!("FORGE HULL (need {} plt)", SHIP_HULL_COST_PLATES) }
                                                  else if station.power_cells < POWER_COST_SHIP_FORGE { "FORGE HULL (insufficient power)".to_string() }
                                                  else { "FORGE HULL".to_string() };

                            if ui.add_sized([125.0, 30.0], egui::Button::new(forge_hull_label)).clicked() && can_forge_hull && !automation_suspended {
                                station.hull_plate_reserves -= SHIP_HULL_COST_PLATES;
                                station.power_cells -= POWER_COST_SHIP_FORGE;
                                station.ship_hulls += 1;
                                add_log_entry(&mut station, "[STATION AI] Ship Hull fabricated. Structural assembly ready.".to_string());
                            }

                            let total_core_cost = AI_CORE_COST_CELLS + POWER_COST_AI_FABRICATE;
                            let can_fab_core = station.power_cells >= total_core_cost && station.ai_cores < 2;
                            let fab_core_label = if automation_suspended { "SUSPENDED".to_string() }
                                                else if station.ai_cores >= 2 { "CORE STOCKPILE FULL".to_string() }
                                                else if station.power_cells < total_core_cost { format!("FABRICATE CORE (need {} cells)", total_core_cost) }
                                                else { "FABRICATE CORE".to_string() };

                            if ui.add_sized([135.0, 30.0], egui::Button::new(fab_core_label)).clicked() && can_fab_core && !automation_suspended {
                                station.power_cells -= total_core_cost;
                                station.ai_cores += 1;
                                add_log_entry(&mut station, "[STATION AI] AI Core fabricated. Unit ready for assembly.".to_string());
                                
                                // [PHASE 9] Immediate discovery upon first fabrication
                                if let Ok(s7_ent) = carbon_field_query.get_single() {
                                    commands.entity(s7_ent).insert((MapMarker, Visibility::Visible));
                                    add_log_entry(&mut station, "[STATION AI] Carbon signature detected. Designation: Sector 7.".to_string());
                                }
                            }
                        });
                        ui.add_space(4.0);

                        // SECTION 5: ASSEMBLY & MAINTENANCE ROW
                        ui.horizontal(|ui| {
                            let ship_count = auto_ship_query.iter().count();
                            if ship_count < 2 {
                                let can_assemble = station.ship_hulls >= 1 && station.ai_cores >= 1;
                                let assemble_label = if automation_suspended { "SUSPENDED".to_string() }
                                                     else if station.ship_hulls < 1 { "ASSEMBLE SHIP (no hull)".to_string() }
                                                     else if station.ai_cores < 1 { "ASSEMBLE SHIP (no core)".to_string() }
                                                     else { "ASSEMBLE SHIP".to_string() };

                                if ui.add_sized([120.0, 30.0], egui::Button::new(assemble_label)).clicked() && can_assemble && !automation_suspended {
                                    station.ship_hulls -= 1;
                                    station.ai_cores -= 1;
                                    
                                    let (target_pos, ore, name) = if ship_count == 0 {
                                        (SECTOR_1_POS, OreType::Magnetite, "Sector 1".to_string())
                                    } else {
                                        (SECTOR_7_POS, OreType::Carbon, "Sector 7".to_string())
                                    };

                                    commands.spawn((
                                        AutonomousShipTag,
                                        LastHeading(0.0),
                                        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore, power: SHIP_POWER_MAX },
                                        AutonomousAssignment { target_pos, ore_type: ore, sector_name: name.clone() },
                                        Mesh2d(meshes.add(triangle_mesh(20.0, 28.0))),
                                        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                                        Transform::from_xyz(STATION_POS.x, STATION_POS.y, 0.5),
                                    ))
                                    .with_children(|parent| {
                                        // [POLISH] Thruster Glow
                                        parent.spawn((
                                            ThrusterGlow,
                                            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
                                            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), // Cyan for autonomous
                                            Transform::from_xyz(0.0, -18.0, -0.1),
                                            Visibility::Hidden,
                                        ));
                                        // [POLISH] Mining Beam
                                        parent.spawn((
                                            MiningBeam,
                                            Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
                                            MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))), // Orange for autonomous
                                            Transform::from_xyz(0.0, 0.0, -0.2),
                                            Visibility::Hidden,
                                        ));
                                        parent.spawn((
                                            Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                                            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
                                            Transform::from_xyz(0.0, 24.0, 1.1),
                                        ));
                                        parent.spawn((
                                            ShipCargoBarFill,
                                            Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                                            Transform::from_xyz(0.0, 24.0, 1.2),
                                        ));
                                    });
                                    add_log_entry(&mut station, format!("[STATION AI] Ship assembly complete. {}. {} extraction.", name, if ore == OreType::Magnetite { "Magnetite" } else { "Carbon" }));
                                }
                            }

                            // TOP UP SHIP action
                            let can_top_up = station.power_cells >= 3 && ship.power_cells < 5;
                            if ui.add_sized([100.0, 30.0], egui::Button::new("TOP UP SHIP")).clicked() && can_top_up {
                                station.power_cells -= 3;
                                ship.power_cells = (ship.power_cells + 3).min(5);
                                add_log_entry(&mut station, "[STATION AI] Ship cells replenished.".to_string());
                            }

                            if !station.online {
                                let can_repair = station.power_cells >= REPAIR_COST;
                                if ui.add_sized([80.0, 30.0], egui::Button::new("REPAIR")).clicked() && can_repair {
                                    station.power_cells -= REPAIR_COST;
                                    station.repair_progress = 1.0;
                                    station.online = true;
                                    add_log_entry(&mut station, "[STATION AI] Repair complete. Power grid online.".to_string());
                                }
                            }
                        });
                    });
                }
                ui.add_space(8.0);
            });
    }
}

fn station_visual_system(
    station_query: Query<(&Station, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (station, material_handle) in &station_query {
        if station.online {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                if material.color != Color::srgb(0.0, 0.8, 1.0) {
                    material.color = Color::srgb(0.0, 0.8, 1.0);
                    info!("[Voidrift Phase 5] Station visual: online state activated.");
                }
            }
        }
    }
}


fn camera_follow_system(
    state: Res<State<GameState>>,
    ship: Query<&Transform, (With<Ship>, Without<MainCamera>)>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    mut cam_delta: ResMut<CameraDelta>,
) {
    let st = ship.single();
    let mut ct = cam.single_mut();
    let old_pos = ct.translation.truncate();
    if *state.get() == GameState::SpaceView {
        ct.translation.x = st.translation.x;
        ct.translation.y = st.translation.y;
    } else {
        ct.translation.x = 0.0;
        ct.translation.y = 0.0;
    }
    // Write camera delta so starfield_scroll_system can parallax-scroll each layer.
    cam_delta.0 = ct.translation.truncate() - old_pos;
}

fn enter_map_view(mut cam: Query<&mut OrthographicProjection, With<MainCamera>>) { cam.single_mut().scale = MAP_OVERVIEW_SCALE; }
fn exit_map_view(mut cam: Query<&mut OrthographicProjection, With<MainCamera>>) { cam.single_mut().scale = 1.0; }

fn handle_input(
    touches: Res<Touches>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>)>,
    mut ship_query: Query<(Entity, &mut Ship), With<Ship>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            for (mt, me) in marker_query.iter() {
                let mp = mt.translation.truncate();
                if world_pos.distance(mp) < 80.0 {
                    let (ship_entity, mut ship) = ship_query.single_mut();
                    
                    // Avoid docking redundancy
                    if ship.state == ShipState::Docked && mp.distance(STATION_POS) < 10.0 { 
                        continue; 
                    }

                    ship.state = ShipState::Navigating;
                    ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                    commands.entity(ship_entity).insert(AutopilotTarget { 
                        destination: mp, 
                        target_entity: Some(me) 
                    });

                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}

}
