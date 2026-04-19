use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::constants::*;

pub fn add_log_entry(station: &mut Station, entry: String) {
    // Avoid duplicate consecutive logs if possible (optional but cleaner)
    if station.log.back() == Some(&entry) { return; }
    station.log.push_back(entry);
    if station.log.len() > LOG_MAX_LINES {
        station.log.pop_front();
    }
}

pub fn ship_cargo_display_system(
    ship_query: Query<&Ship>, 
    mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
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

pub fn autonomous_ship_cargo_display_system(
    ship_query: Query<&AutonomousShip>, 
    mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
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

pub fn hud_ui_system(
    mut contexts: EguiContexts,
    mut ship_query: Query<&mut Ship>,
    mut station_query: Query<(Entity, &mut Station)>,
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
                            let refine_mag_allowed = station.magnetite_reserves >= REFINERY_RATIO as f32;
                            let btn_label = if bootstrap_mode { "CRITICAL REFINE [10 MAG]" } else { "REFINE MAG [1 PWR]" };
                            if ui.add_sized([160.0, 30.0], egui::Button::new(btn_label)).clicked() && refine_mag_allowed {
                                if bootstrap_mode {
                                    station.magnetite_reserves -= REFINERY_RATIO as f32;
                                    station.power_cells += 1;
                                    add_log_entry(&mut station, "[STATION AI] Bootstrap Refine complete. (+1 Cell)".to_string());
                                } else if station.power >= POWER_COST_REFINERY as f32 {
                                    station.magnetite_reserves -= REFINERY_RATIO as f32;
                                    station.power -= POWER_COST_REFINERY as f32;
                                    station.power_cells += 1;
                                    add_log_entry(&mut station, "[STATION AI] Refine complete. (+1 Cell)".to_string());
                                }
                            }
                            
                            // [PHASE 9] FORGE HULL PLATE Action
                            let can_forge_hull = station.carbon_reserves >= HULL_REFINERY_RATIO as f32 && station.power >= POWER_COST_HULL_FORGE as f32;
                            if ui.add_sized([160.0, 30.0], egui::Button::new("FORGE HULL [2 PWR]")).clicked() && can_forge_hull && !automation_suspended {
                                station.carbon_reserves -= HULL_REFINERY_RATIO as f32;
                                station.power -= POWER_COST_HULL_FORGE as f32;
                                station.hull_plate_reserves += 1;
                                add_log_entry(&mut station, "[STATION AI] SynthForge cycle complete. (+1 Hull Plate)".to_string());
                            }
                        });
                        
                        ui.add_space(4.0);

                        // SECTION 4: FABRICATION ROW
                        ui.horizontal(|ui| {
                            // [PHASE 9] ASSEMBLE SHIP Action
                            let can_assemble = station.hull_plate_reserves >= SHIP_HULL_COST_PLATES && station.power >= POWER_COST_SHIP_FORGE as f32;
                            if ui.add_sized([160.0, 30.0], egui::Button::new("ASSEMBLE SHIP [3 PWR]")).clicked() && can_assemble && !automation_suspended {
                                station.hull_plate_reserves -= SHIP_HULL_COST_PLATES;
                                station.power -= POWER_COST_SHIP_FORGE as f32;
                                station.ship_hulls += 1;
                                add_log_entry(&mut station, "[STATION AI] Hull Assembly complete. (+1 Ship Hull)".to_string());
                            }

                            // [PHASE 9] FABRICATE CORE Action
                            let can_fabricate_core = station.power_cells >= AI_CORE_COST_CELLS && station.power >= POWER_COST_AI_FABRICATE as f32;
                            if ui.add_sized([160.0, 30.0], egui::Button::new("FABRICATE CORE [5 PWR]")).clicked() && can_fabricate_core && !automation_suspended {
                                station.power_cells -= AI_CORE_COST_CELLS;
                                station.power -= POWER_COST_AI_FABRICATE as f32;
                                station.ai_cores += 1;
                                add_log_entry(&mut station, "[STATION AI] AI Core fabrication complete. (+1 Core)".to_string());
                                
                                // Unlock Sector 7 mapping coordinates upon first fabrication
                                if station.ai_cores == 1 {
                                    if let Ok(s7_ent) = carbon_field_query.get_single() {
                                        commands.entity(s7_ent).insert((MapMarker, Visibility::Visible));
                                        add_log_entry(&mut station, "[STATION AI] Core diagnostic: New mapping coordinates acquired. Sector 7 added to routing table.".to_string());
                                    }
                                }
                            }
                        });

                        ui.add_space(4.0);

                        // SECTION 5: DEPLOYMENT ROW
                        ui.horizontal(|ui| {
                            let can_deploy = station.ship_hulls >= 1 && station.ai_cores >= 1;
                            if ui.add_sized([320.0, 30.0], egui::Button::new("DEPLOYS AUTONOMOUS SHIP")).clicked() && can_deploy && !automation_suspended {
                                station.ship_hulls -= 1;
                                station.ai_cores -= 1;
                                
                                // Route to Sector 7 once unlocked, else Sector 1
                                let (target_pos, ore, name) = if station.ai_cores >= 1 {
                                    (SECTOR_7_POS, OreType::Carbon, "Sector 7".to_string())
                                } else {
                                    (SECTOR_1_POS, OreType::Magnetite, "Sector 1".to_string())
                                };

                                commands.spawn((
                                    AutonomousShipTag,
                                    LastHeading(0.0),
                                    AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore, power: SHIP_POWER_MAX },
                                    AutonomousAssignment { target_pos, ore_type: ore, sector_name: name.clone() },
                                    Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))), // Uses setup's mesh method
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

pub fn station_visual_system(
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
