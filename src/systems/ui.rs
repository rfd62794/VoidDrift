use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::constants::*;

pub fn add_log_entry(station: &mut Station, entry: String) {
    if station.log.back() == Some(&entry) { return; }
    station.log.push_back(entry);
    if station.log.len() > LOG_MAX_LINES {
        station.log.pop_front();
    }
}

pub fn ship_cargo_display_system(
    ship_query: Query<&Ship>, 
    mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), (With<ShipCargoBarFill>, Without<Ship>, Without<AutonomousShip>, Without<Station>, Without<AsteroidField>, Without<Berth>, Without<MainCamera>, Without<DestinationHighlight>)>, 
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
    mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), (With<ShipCargoBarFill>, Without<Ship>, Without<AutonomousShip>, Without<Station>, Without<AsteroidField>, Without<Berth>, Without<MainCamera>, Without<DestinationHighlight>)>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (mut tr, parent, mat_handle) in fill_query.iter_mut() {
        if let Ok(ship) = ship_query.get(**parent) {
            let r = ship.cargo / CARGO_CAPACITY as f32;
            tr.scale.x = r.max(0.001);
            tr.translation.x = -15.0 + (15.0 * r);
            
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = Color::srgb(1.0, 0.5, 0.0);
            }
        }
    }
}

pub fn hud_ui_system(
    mut contexts: EguiContexts,
    mut ship_query: Query<&mut Ship, (With<PlayerShip>, Without<AutonomousShipTag>)>,
    mut station_query: Query<(Entity, &mut Station)>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    signal_log: Res<SignalLog>,
    opening: Res<OpeningSequence>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    auto_ships: Query<&AutonomousShip, With<AutonomousShipTag>>,
    mut expanded: ResMut<SignalStripExpanded>,
    mut quest_log: ResMut<QuestLog>,
) {
    let mut ship = ship_query.single_mut();
    let ctx = contexts.ctx_mut();

    // ── 1. SIGNAL STRIP (Always visible) ────────────────────────────────────────
    
    let strip_height = if expanded.0 { 180.0 } else { 48.0 };

    egui::TopBottomPanel::bottom("signal_strip")
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_rgba_premultiplied(13, 13, 13, 217))) // 0.05, 0.05, 0.05, 0.85 approx
        .min_height(strip_height)
        .show(ctx, |ui| {
            ui.add_space(6.0);
            
            let display_count = if expanded.0 { 20 } else if ship.state == ShipState::Docked { 3 } else { 2 };
            let entries: Vec<&String> = signal_log.entries.iter().rev().take(display_count).collect();
            
            // Interaction area for the whole strip
            let rect = ui.available_rect_before_wrap();
            let response = ui.interact(rect, ui.id().with("strip_click"), egui::Sense::click());
            if response.clicked() {
                expanded.0 = !expanded.0;
            }

            if expanded.0 {
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .max_height(160.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for line in entries.iter().rev() {
                                ui.label(egui::RichText::new(*line)
                                    .monospace()
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(0, 204, 102))); // #00CC66
                            }
                        });
                    });
            } else {
                ui.vertical(|ui| {
                    for line in entries.iter().rev() {
                        ui.label(egui::RichText::new(*line)
                            .monospace()
                            .size(10.0)
                            .color(egui::Color32::from_rgb(0, 204, 102))); // #00CC66
                    }
                });
            }
            
            ui.add_space(6.0);
        });

    // If opening sequence not complete, suppress main UI
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    // ── 2. LEFT PANEL (MAP + TABS) ──────────────────────────────────────────────

    egui::SidePanel::left("left_panel")
        .frame(egui::Frame::NONE)
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.add_space(16.0);
            
            // Map Toggle
            let label = if *state.get() == GameState::SpaceView { "MAP" } else { "EXIT MAP" };
            if ui.add(egui::Button::new(label).min_size(egui::vec2(80.0, 40.0))).clicked() {
                if *state.get() == GameState::SpaceView {
                    next_state.set(GameState::MapView);
                } else {
                    next_state.set(GameState::SpaceView);
                }
                // Opening MAP closes QUEST
                quest_log.panel_open = false;
            }

            ui.add_space(8.0);

            // Quest Toggle
            let quest_label = if quest_log.panel_open { "CLOSE Q" } else { "QUEST" };
            if ui.add(egui::Button::new(quest_label).min_size(egui::vec2(80.0, 40.0))).clicked() {
                quest_log.panel_open = !quest_log.panel_open;
                // Opening QUEST closes MAP
                if quest_log.panel_open {
                    next_state.set(GameState::SpaceView);
                }
            }

            // Department Tabs (Only when docked)
            if ship.state == ShipState::Docked {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);
                
                if let Ok((_, station)) = station_query.get_single() {
                    let departments = [
                        (ActiveStationTab::Reserves, "RESERVES", true),
                        (ActiveStationTab::Power, "POWER", station.online),
                        (ActiveStationTab::Smelter, "SMELTER", true),
                        (ActiveStationTab::Forge, "FORGE", true),
                        (ActiveStationTab::ShipPort, "SHIP PORT", auto_ships.iter().count() > 0),
                    ];

                    for (tab, name, unlocked) in departments {
                        ui.add_enabled_ui(unlocked, |ui| {
                            let is_active = *active_tab == tab;
                            let color = if is_active {
                                egui::Color32::WHITE
                            } else if unlocked {
                                egui::Color32::from_gray(180)
                            } else {
                                egui::Color32::from_gray(100)
                            };

                            let btn = egui::Button::new(egui::RichText::new(name).color(color))
                                .min_size(egui::vec2(80.0, 32.0))
                                .frame(is_active);

                            if ui.add(btn).clicked() {
                                *active_tab = tab;
                            }
                        });
                        ui.add_space(4.0);
                    }
                }
            }
        });

    // ── 3. TAB DETAIL PANEL (Docked Context) ────────────────────────────────────

    if ship.state == ShipState::Docked {
        egui::TopBottomPanel::bottom("tab_detail_panel")
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                if let Ok((_station_ent, mut station)) = station_query.get_single_mut() {
                    ui.vertical_centered(|ui| {
                        match *active_tab {
                            ActiveStationTab::Reserves => {
                                ui.horizontal(|ui| {
                                    ui.label(format!("MAG: {:.0}", station.magnetite_reserves));
                                    ui.separator();
                                    ui.label(format!("CAR: {:.0}", station.carbon_reserves));
                                    ui.separator();
                                    ui.label(egui::RichText::new(format!("CELLS: {}", station.power_cells)).color(egui::Color32::LIGHT_BLUE));
                                    ui.separator();
                                    ui.label(format!("PLATES: {}", station.hull_plate_reserves));
                                    ui.separator();
                                    ui.label(format!("HULLS: {}", station.ship_hulls));
                                    ui.separator();
                                    ui.label(format!("CORES: {}", station.ai_cores));
                                });
                                
                                if !station.online {
                                    ui.add_space(8.0);
                                    let can_repair = station.power_cells >= REPAIR_COST;
                                    if ui.add(egui::Button::new(format!("REPAIR STATION [{} CELLS]", REPAIR_COST)).min_size(egui::vec2(200.0, 40.0)))
                                        .clicked() && can_repair {
                                        station.power_cells -= REPAIR_COST;
                                        station.repair_progress = 1.0;
                                        station.online = true;
                                    }
                                }
                            }
                            ActiveStationTab::Power => {
                                ui.horizontal(|ui| {
                                    ui.label(format!("STATION POWER: {:.1}/{:.0}", station.power, STATION_POWER_MAX));
                                    ui.add(egui::ProgressBar::new(station.power / STATION_POWER_MAX).desired_width(120.0));
                                    ui.separator();
                                    ui.label(format!("SHIP POWER: {:.1}/{:.0}", ship.power, SHIP_POWER_MAX));
                                    ui.add(egui::ProgressBar::new(ship.power / SHIP_POWER_MAX).desired_width(120.0));
                                });
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("CONSUMPTION: 4 cells/cycle (base)").size(9.0).italics());
                            }
                            ActiveStationTab::Smelter => {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label("MAGNETITE → CELLS (10:1)");
                                        let can_refine = station.magnetite_reserves >= REFINERY_RATIO as f32 && station.power >= 1.0;
                                        if ui.add(egui::Button::new("REFINE CELLS [1 PWR]").min_size(egui::vec2(140.0, 32.0)))
                                            .clicked() && can_refine {
                                            station.magnetite_reserves -= REFINERY_RATIO as f32;
                                            station.power -= 1.0;
                                            station.power_cells += 1;
                                        }
                                    });
                                    ui.add_space(20.0);
                                    ui.vertical(|ui| {
                                        ui.label("CARBON → PLATES (5:1)");
                                        let can_forge = station.carbon_reserves >= HULL_REFINERY_RATIO as f32 && station.power >= 2.0;
                                        if ui.add(egui::Button::new("FORGE PLATES [2 PWR]").min_size(egui::vec2(140.0, 32.0)))
                                            .clicked() && can_forge {
                                            station.carbon_reserves -= HULL_REFINERY_RATIO as f32;
                                            station.power -= 2.0;
                                            station.hull_plate_reserves += 1;
                                        }
                                    });
                                });
                            }
                            ActiveStationTab::Forge => {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label("PLATES → HULL (3:1)");
                                        let can_hull = station.hull_plate_reserves >= SHIP_HULL_COST_PLATES && station.power >= 3.0;
                                        if ui.add(egui::Button::new("FORGE HULL [3 PWR]").min_size(egui::vec2(140.0, 32.0)))
                                            .clicked() && can_hull {
                                            station.hull_plate_reserves -= SHIP_HULL_COST_PLATES;
                                            station.power -= 3.0;
                                            station.ship_hulls += 1;
                                        }
                                    });
                                    ui.add_space(20.0);
                                    ui.vertical(|ui| {
                                        ui.label("CELLS → CORE (50 total)");
                                        let can_core = station.power_cells >= AI_CORE_COST_CELLS && station.power >= 5.0;
                                        if ui.add(egui::Button::new("FABRICATE CORE [5 PWR]").min_size(egui::vec2(140.0, 32.0)))
                                            .clicked() && can_core {
                                            station.power_cells -= AI_CORE_COST_CELLS;
                                            station.power -= 5.0;
                                            station.ai_cores += 1;
                                        }
                                    });
                                });
                            }
                            ActiveStationTab::ShipPort => {
                                ui.horizontal(|ui| {
                                    let can_deploy = station.ship_hulls >= 1 && station.ai_cores >= 1;
                                    if ui.add(egui::Button::new("ASSEMBLE & DEPLOY AUTONOMOUS SHIP").min_size(egui::vec2(280.0, 40.0)))
                                        .clicked() && can_deploy {
                                        station.ship_hulls -= 1;
                                        station.ai_cores -= 1;
                                        
                                        // Spawn autonomous ship logic (using SECTOR_7_POS if unlocked, etc)
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
                                            Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))),
                                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                                            Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_SHIP),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                ThrusterGlow,
                                                Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
                                                MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), 
                                                Transform::from_xyz(0.0, -18.0, 0.1), 
                                                Visibility::Hidden,
                                            ));
                                            parent.spawn((
                                                MiningBeam,
                                                Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
                                                MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))), 
                                                Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP), 
                                                Visibility::Hidden,
                                            ));
                                            parent.spawn((
                                                Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                                                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
                                                Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP),
                                            ));
                                            parent.spawn((
                                                ShipCargoBarFill,
                                                Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                                                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                                                Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05),
                                            ));
                                            parent.spawn((
                                                MapElement,
                                                Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(12.0, 16.0))),
                                                MeshMaterial2d(materials.add(ColorMaterial {
                                                    color: Color::srgb(1.0, 0.5, 0.0),
                                                    alpha_mode: bevy::sprite::AlphaMode2d::Opaque,
                                                    ..default()
                                                })),
                                                Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)),
                                                Visibility::Hidden,
                                            ));
                                        });
                                    }
                                    
                                    ui.separator();
                                    let can_top_up = station.power_cells >= 3 && ship.power_cells < 5;
                                    if ui.add(egui::Button::new("TOP UP SHIP [3 CELLS]").min_size(egui::vec2(120.0, 40.0)))
                                        .clicked() && can_top_up {
                                        station.power_cells -= 3;
                                        ship.power_cells = (ship.power_cells + 3).min(5);
                                    }
                                });
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("FLEET STATUS: Management coming soon").size(9.0).italics().color(egui::Color32::GRAY));
                            }
                        }
                    });
                }
                ui.add_space(8.0);
            });
    }
}

pub fn station_visual_system(
    station_query: Query<&Station>,
    mut hub_query: Query<&MeshMaterial2d<ColorMaterial>, With<StationHub>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(station) = station_query.get_single() {
        if let Ok(material_handle) = hub_query.get_single_mut() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let target_color = if station.online {
                    Color::srgb(1.0, 0.84, 0.0) // #FFD700 — powered yellow
                } else {
                    Color::srgb(0.33, 0.27, 0.0) // #554400 — dark amber derelict
                };

                if material.color != target_color {
                    material.color = target_color;
                }
            }
        }
    }
}
