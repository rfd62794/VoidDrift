use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::{egui, EguiContexts};
use crate::systems::station_tabs::render_queue_card;
use crate::components::*;
use crate::constants::*;

pub fn ui_layout_system(
    windows: Query<&Window>,
    mut layout: ResMut<UiLayout>,
) {
    let Ok(window) = windows.get_single() else { return };

    let scale = window.scale_factor() as f32;
    let w = window.physical_width() as f32 / scale;
    let h = window.physical_height() as f32 / scale;
    let landscape = w > h;

    // Bottom drawer dimensions
    let handle_height = 32.0;
    let tab_bar_height = 48.0;
    let signal_strip_height = if landscape { 56.0 } else { 64.0 };
    let world_view_min = h * 0.45;
    let content_area_height = h - handle_height - tab_bar_height - signal_strip_height - world_view_min;

    *layout = UiLayout {
        screen_width: w,
        screen_height: h,
        is_landscape: landscape,
        
        // Bottom drawer dimensions
        handle_height,
        tab_bar_height,
        content_area_height,
        signal_strip_height,
        world_view_min_height: world_view_min,
        
        // Content dimensions (full width, no side panel)
        content_width: w,
        
        button_height: 44.0,
        tab_button_height: 44.0,
        font_size_body: if landscape { 13.0 } else { 12.0 },
        font_size_label: if landscape { 11.0 } else { 10.0 },
        font_size_title: if landscape { 15.0 } else { 14.0 },
    };
}

pub fn ship_cargo_display_system(
    time: Res<Time>,
    ship_query: Query<&Ship, (With<PlayerShip>, Without<Station>, Without<AutonomousShipTag>, Without<AsteroidField>)>,
    mut fill_query: Query<(&mut Transform, &mut MeshMaterial2d<ColorMaterial>), (With<ShipCargoBarFill>, Without<PlayerShip>, Without<Station>, Without<AutonomousShipTag>, Without<AsteroidField>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(ship) = ship_query.get_single() {
        if let Ok((mut transform, mat_handle)) = fill_query.get_single_mut() {
            let ratio = (ship.cargo / ship.cargo_capacity as f32).clamp(0.0, 1.0);
            transform.scale.x = ratio;
            transform.translation.x = -20.0 * (1.0 - ratio);

            // Update color based on fullness + pulse
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = if ship.cargo >= ship.cargo_capacity as f32 * 0.95 {
                    // Pulse Cyan when full
                    let pulse = (time.elapsed_secs() * 10.0).sin() * 0.2 + 0.8;
                    Color::srgba(0.0, pulse, pulse, 1.0)
                } else {
                    match ship.cargo_type {
                        OreType::Magnetite => COLOR_MAGNETITE,
                        OreType::Carbon    => COLOR_CARBON,
                        _ => Color::srgb(0.0, 1.0, 1.0),
                    }
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

pub fn cargo_label_system(
    ship_query: Query<(&Ship, &Children), (With<PlayerShip>, Without<Station>, Without<AsteroidField>)>,
    mut ore_label_query: Query<&mut Text2d, (With<CargoOreLabel>, Without<CargoCountLabel>)>,
    mut count_label_query: Query<&mut Text2d, (With<CargoCountLabel>, Without<CargoOreLabel>)>,
) {
    let Ok((ship, children)) = ship_query.get_single() else { return; };

    for &child in children.iter() {
        if let Ok(mut ore_text) = ore_label_query.get_mut(child) {
            ore_text.0 = match ship.cargo_type {
                OreType::Empty => "EMPTY".to_string(),
                OreType::Magnetite => "MAGNETITE".to_string(),
                OreType::Carbon => "CARBON".to_string(),
            };
        }
        if let Ok(mut count_text) = count_label_query.get_mut(child) {
            count_text.0 = format!("{:.0} / {}", ship.cargo, ship.cargo_capacity);
        }
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
                let target_color = if station.online { Color::srgb(1.0, 0.84, 0.0) } else { Color::srgb(0.33, 0.27, 0.0) };
                if material.color != target_color { material.color = target_color; }
            }
        }
    }
}

#[derive(SystemParam)]
pub struct HudParams<'w, 's> {
    pub contexts: EguiContexts<'w, 's>,
    pub ship_query: Query<'w, 's, (Entity, &'static mut Ship), (With<PlayerShip>, Without<Station>, Without<AutonomousShipTag>, Without<AsteroidField>)>,
    pub station_query: Query<'w, 's, (Entity, &'static mut Station, &'static mut StationQueues), (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    pub state: Res<'w, State<GameState>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
    pub signal_log: Res<'w, SignalLog>,
    pub opening: Res<'w, OpeningSequence>,
    pub drawer_state: ResMut<'w, DrawerState>,
    pub active_tab: ResMut<'w, ActiveStationTab>,
    pub layout: Res<'w, UiLayout>,
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub expanded: ResMut<'w, SignalStripExpanded>,
    pub quest_log: ResMut<'w, QuestLog>,
    pub forge_settings: Res<'w, ForgeSettings>,
    pub auto_dock_settings: ResMut<'w, AutoDockSettings>,
    pub tutorial: ResMut<'w, TutorialState>,
    pub pan_state: ResMut<'w, MapPanState>,
    pub cam_query: Query<'w, 's, &'static mut OrthographicProjection, With<MainCamera>>,
}

pub fn hud_ui_system(mut params: HudParams) {
    let ctx = params.contexts.ctx_mut();
    let layout = params.layout.into_inner();

    // ---- 1. SIGNAL STRIP (Bottom) ----
    egui::TopBottomPanel::bottom("signal_strip")
        .resizable(false)
        .exact_height(layout.signal_strip_height)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let display_count = if params.expanded.0 { 20 } else { 3 };
            let entries: Vec<&String> = params.signal_log.entries.iter().rev().take(display_count).collect();
            
            let rect = ui.available_rect_before_wrap();
            let response = ui.interact(rect, ui.id().with("strip_click"), egui::Sense::click());
            if response.clicked() {
                params.expanded.0 = !params.expanded.0;
            }

            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for line in entries.iter().rev() {
                            ui.label(egui::RichText::new(*line)
                                .monospace()
                                .size(11.0)
                                .color(egui::Color32::from_rgb(0, 255, 128)));
                        }
                    });
                });
        });

    // ---- 2. CONTENT AREA (Bottom) ----
    if *params.drawer_state == DrawerState::Expanded {
        if let Ok((_station_ent, mut station, mut queues)) = params.station_query.get_single_mut() {
            egui::TopBottomPanel::bottom("content_area")
                .resizable(false)
                .exact_height(layout.content_area_height)
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .show(ui, |ui| {
                            ui.set_width(layout.content_width);
                            ui.add_space(8.0);
                            
                            // Render RESERVES and REFINERY tabs for now
                            match *params.active_tab {
                                ActiveStationTab::Reserves => {
                                    ui.heading("STATION RESOURCES");
                                    ui.add_space(8.0);
                                    
                                    // Resource grid in pairs
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label("MAGNETITE:");
                                            ui.label(egui::RichText::new(format!("{:.1}", station.magnetite_reserves)).color(egui::Color32::WHITE));
                                        });
                                        ui.add_space(20.0);
                                        ui.vertical(|ui| {
                                            ui.label("CARBON:");
                                            ui.label(egui::RichText::new(format!("{:.1}", station.carbon_reserves)).color(egui::Color32::WHITE));
                                        });
                                    });
                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label("HULL PLATES:");
                                            ui.label(egui::RichText::new(format!("{}", station.hull_plate_reserves)).color(egui::Color32::WHITE));
                                        });
                                        ui.add_space(20.0);
                                        ui.vertical(|ui| {
                                            ui.label("POWER CELLS:");
                                            ui.label(egui::RichText::new(format!("{}", station.power_cells)).color(egui::Color32::GREEN));
                                        });
                                    });
                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label("AI CORES:");
                                            ui.label(egui::RichText::new(format!("{}", station.ai_cores)).color(egui::Color32::CYAN));
                                        });
                                        ui.add_space(20.0);
                                        ui.vertical(|ui| {
                                            ui.label("SHIP HULLS:");
                                            ui.label(egui::RichText::new(format!("{}", station.ship_hulls)).color(egui::Color32::GOLD));
                                        });
                                    });
                                    ui.add_space(8.0);
                                    ui.vertical(|ui| {
                                        ui.label("HELIUM:");
                                        ui.label(egui::RichText::new(format!("{}", station.power)).color(egui::Color32::WHITE));
                                    });
                                    
                                    ui.add_space(16.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    
                                    ui.heading("AUTO-DOCK SETTINGS");
                                    ui.checkbox(&mut params.auto_dock_settings.auto_unload, "Auto-Unload Cargo");
                                    ui.checkbox(&mut params.auto_dock_settings.auto_smelt_magnetite, "Auto-Smelt Magnetite");
                                    ui.checkbox(&mut params.auto_dock_settings.auto_smelt_carbon, "Auto-Smelt Carbon");
                                    
                                    if !station.online {
                                        ui.add_space(16.0);
                                        if ui.button(format!("REPAIR STATION [{} CELLS]", REPAIR_COST)).clicked() && station.power_cells >= REPAIR_COST {
                                            station.power_cells -= REPAIR_COST; 
                                            station.repair_progress = 1.0; 
                                            station.online = true;
                                        }
                                    }
                                }
                                ActiveStationTab::Refinery => {
                                    // Magnetite -> Power Cells queue card
                                    render_queue_card(
                                        ui, 
                                        &layout, 
                                        &mut station, 
                                        &mut queues.magnetite_refinery, 
                                        ProcessingOperation::MagnetiteRefinery, 
                                        REFINERY_RATIO as f32, 
                                        POWER_COST_REFINERY as f32, 
                                        REFINERY_MAGNETITE_TIME
                                    );
                                    
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    
                                    // Carbon -> Hull Plates queue card
                                    render_queue_card(
                                        ui, 
                                        &layout, 
                                        &mut station, 
                                        &mut queues.carbon_refinery, 
                                        ProcessingOperation::CarbonRefinery, 
                                        HULL_PLATE_COST_CARBON as f32, 
                                        POWER_COST_HULL_FORGE as f32, 
                                        REFINERY_CARBON_TIME
                                    );
                                }
                                ActiveStationTab::Power => {
                                    ui.heading("STATION POWER STATUS");
                                    ui.add_space(8.0);
                                    
                                    ui.horizontal(|ui| {
                                        ui.label(format!("STATION POWER: {:.1}/{:.0}", station.power, STATION_POWER_MAX));
                                        ui.add(egui::ProgressBar::new(station.power / STATION_POWER_MAX).desired_width(120.0));
                                    });
                                    ui.add_space(8.0);
                                    
                                    let (_, ship) = params.ship_query.single_mut();
                                    ui.horizontal(|ui| {
                                        ui.label(format!("SHIP POWER: {:.1}/{:.0}", ship.power, SHIP_POWER_MAX));
                                        ui.add(egui::ProgressBar::new(ship.power / SHIP_POWER_MAX).desired_width(120.0));
                                    });
                                    
                                    ui.add_space(16.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    
                                    ui.heading("POWER CONSUMPTION");
                                    ui.add_space(8.0);
                                    
                                    ui.label("Station Systems:");
                                    ui.label("  - Life Support: 10.0 PWR/s");
                                    ui.label("  - Communications: 5.0 PWR/s");
                                    ui.label("  - Docking Systems: 15.0 PWR/s");
                                    ui.label("  - Processing: 25.0 PWR/s");
                                }
                                ActiveStationTab::Forge => {
                                    // Hull Plates -> Ship Hull queue card
                                    render_queue_card(
                                        ui, 
                                        &layout, 
                                        &mut station, 
                                        &mut queues.hull_forge, 
                                        ProcessingOperation::HullForge, 
                                        SHIP_HULL_COST_PLATES as f32, 
                                        POWER_COST_SHIP_FORGE as f32, 
                                        FORGE_HULL_TIME
                                    );
                                    
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    
                                    // Power Cells -> AI Core queue card
                                    render_queue_card(
                                        ui, 
                                        &layout, 
                                        &mut station, 
                                        &mut queues.core_fabricator, 
                                        ProcessingOperation::CoreFabricator, 
                                        AI_CORE_COST_CELLS as f32, 
                                        POWER_COST_AI_FABRICATE as f32, 
                                        FORGE_CORE_TIME
                                    );
                                }
                                ActiveStationTab::ShipPort => {
                                    ui.heading("FLEET ASSEMBLY");
                                    ui.add_space(8.0);
                                    
                                    ui.horizontal(|ui| {
                                        if ui.button("ASSEMBLE & DEPLOY AUTONOMOUS SHIP").clicked() && station.ship_hulls >= 1 && station.ai_cores >= 1 {
                                            station.ship_hulls -= 1; 
                                            station.ai_cores -= 1;
                                            let (target_pos, ore, name) = if station.ai_cores >= 1 { 
                                                (SECTOR_3_POS, OreType::Carbon, "Sector 3") 
                                            } else { 
                                                (SECTOR_1_POS, OreType::Magnetite, "Sector 1") 
                                            };
                                            params.commands.spawn((
                                                AutonomousShipTag, 
                                                LastHeading(0.0), 
                                                AutonomousShip { 
                                                    state: AutonomousShipState::Holding, 
                                                    cargo: 0.0, 
                                                    cargo_type: ore, 
                                                    power: SHIP_POWER_MAX 
                                                }, 
                                                AutonomousAssignment { 
                                                    target_pos, 
                                                    ore_type: ore, 
                                                    sector_name: name.to_string() 
                                                }, 
                                                Mesh2d(params.meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))), 
                                                MeshMaterial2d(params.materials.add(Color::srgb(1.0, 0.5, 0.0))), 
                                                Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_SHIP)
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn((
                                                    ThrusterGlow, 
                                                    Mesh2d(params.meshes.add(Rectangle::new(6.0, 8.0))), 
                                                    MeshMaterial2d(params.materials.add(Color::srgb(0.0, 1.0, 1.0))), 
                                                    Transform::from_xyz(0.0, -18.0, 0.1), 
                                                    Visibility::Hidden
                                                ));
                                                parent.spawn((
                                                    MiningBeam, 
                                                    Mesh2d(params.meshes.add(Rectangle::new(2.0, 1.0))), 
                                                    MeshMaterial2d(params.materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))), 
                                                    Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP), 
                                                    Visibility::Hidden
                                                ));
                                                parent.spawn((
                                                    Mesh2d(params.meshes.add(Rectangle::new(30.0, 4.0))), 
                                                    MeshMaterial2d(params.materials.add(Color::srgb(0.2, 0.2, 0.2))), 
                                                    Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP)
                                                ));
                                                parent.spawn((
                                                    ShipCargoBarFill, 
                                                    Mesh2d(params.meshes.add(Rectangle::new(30.0, 4.0))), 
                                                    MeshMaterial2d(params.materials.add(Color::srgb(1.0, 0.5, 0.0))), 
                                                    Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05)
                                                ));
                                                parent.spawn((
                                                    MapElement, 
                                                    Mesh2d(params.meshes.add(crate::systems::setup::triangle_mesh(12.0, 16.0))), 
                                                    MeshMaterial2d(params.materials.add(ColorMaterial { 
                                                        color: Color::srgb(1.0, 0.5, 0.0), 
                                                        alpha_mode: bevy::sprite::AlphaMode2d::Opaque, 
                                                        ..default() 
                                                    })), 
                                                    Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)), 
                                                    Visibility::Hidden
                                                ));
                                            });
                                        }
                                        ui.separator();
                                        let (_, mut ship) = params.ship_query.single_mut();
                                        if ui.button("TOP UP SHIP [3 CELLS]").clicked() && station.power_cells >= 3 && ship.power_cells < 5 {
                                            station.power_cells -= 3; 
                                            ship.power_cells = (ship.power_cells + 3).min(5);
                                        }
                                    });
                                    
                                    ui.add_space(16.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    
                                    ui.heading("FLEET STATUS");
                                    ui.add_space(8.0);
                                    ui.label("Autonomous Ships: 1 deployed");
                                    ui.label("Status: Active mining operations");
                                }
                                ActiveStationTab::Quest => {
                                    ui.heading("QUEST LOG");
                                    ui.add_space(8.0);
                                    
                                    ui.heading(egui::RichText::new("ACTIVE").color(egui::Color32::WHITE));
                                    ui.separator();
                                    for obj in params.quest_log.objectives.iter().filter(|o| o.state == ObjectiveState::Active) {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(">").color(egui::Color32::CYAN));
                                            ui.label(egui::RichText::new(&obj.description).strong());
                                        });
                                        if let (Some(curr), Some(target)) = (obj.progress_current, obj.progress_target) {
                                            ui.add(egui::ProgressBar::new(curr as f32 / target as f32).text(format!("{}/{}", curr, target)));
                                        }
                                        ui.add_space(8.0);
                                    }
                                    ui.add_space(12.0);
                                    
                                    ui.heading(egui::RichText::new("COMPLETED").color(egui::Color32::GRAY));
                                    ui.separator();
                                    for obj in params.quest_log.objectives.iter().filter(|o| o.state == ObjectiveState::Complete) {
                                        ui.label(egui::RichText::new(format!(" {}", obj.description)).color(egui::Color32::from_gray(140)));
                                    }
                                    ui.add_space(12.0);
                                    
                                    ui.heading(egui::RichText::new("UPCOMING").color(egui::Color32::from_gray(120)));
                                    ui.separator();
                                    for obj in params.quest_log.objectives.iter().filter(|o| o.state == ObjectiveState::Locked) {
                                        ui.label(egui::RichText::new(format!(" {}", obj.description)).color(egui::Color32::from_gray(100)));
                                    }
                                }
                                ActiveStationTab::Routes => {
                                    ui.heading("NAVIGATION MAP");
                                    ui.add_space(8.0);
                                    
                                    // Map toggle button
                                    let map_label = if *params.state.get() == GameState::SpaceView { "OPEN MAP" } else { "CLOSE MAP" };
                                    if ui.button(map_label).clicked() {
                                        if *params.state.get() == GameState::SpaceView {
                                            params.next_state.set(GameState::MapView);
                                        } else {
                                            params.next_state.set(GameState::SpaceView);
                                        }
                                    }
                                    
                                    ui.add_space(16.0);
                                    
                                    // Navigation targets
                                    ui.heading("SECTORS");
                                    ui.add_space(8.0);
                                    
                                    ui.horizontal(|ui| {
                                        if ui.button("SECTOR 1").clicked() {
                                            let (ship_entity, _) = params.ship_query.single_mut();
                                            params.commands.entity(ship_entity).insert(AutopilotTarget {
                                                destination: SECTOR_1_POS,
                                                target_entity: None,
                                            });
                                        }
                                        ui.separator();
                                        if ui.button("SECTOR 2").clicked() {
                                            let (ship_entity, _) = params.ship_query.single_mut();
                                            params.commands.entity(ship_entity).insert(AutopilotTarget {
                                                destination: SECTOR_2_POS,
                                                target_entity: None,
                                            });
                                        }
                                    });
                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        if ui.button("SECTOR 3").clicked() {
                                            let (ship_entity, _) = params.ship_query.single_mut();
                                            params.commands.entity(ship_entity).insert(AutopilotTarget {
                                                destination: SECTOR_3_POS,
                                                target_entity: None,
                                            });
                                        }
                                        ui.separator();
                                        if ui.button("STATION").clicked() {
                                            let (ship_entity, _) = params.ship_query.single_mut();
                                            params.commands.entity(ship_entity).insert(AutopilotTarget {
                                                destination: STATION_POS,
                                                target_entity: None,
                                            });
                                        }
                                    });
                                    
                                    ui.add_space(16.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    
                                    // Map controls
                                    ui.heading("MAP CONTROLS");
                                    ui.add_space(8.0);
                                    
                                    if let Ok(mut map_cam) = params.cam_query.get_single_mut() {
                                        ui.label(format!("Zoom: {:.1}x", map_cam.scale));
                                        ui.add_space(4.0);
                                        if ui.button("RESET ZOOM").clicked() {
                                            map_cam.scale = MAP_OVERVIEW_SCALE;
                                        }
                                    }
                                    
                                    ui.add_space(8.0);
                                    ui.label("Tap sectors to navigate");
                                    ui.label("Use pinch to zoom on map");
                                }
                                _ => {
                                    ui.label("Content not implemented yet.");
                                }
                            }
                            
                            ui.add_space(8.0);
                        });
                });
        }
    }

    // ---- 3. TAB BAR (Bottom) ----
    if *params.drawer_state != DrawerState::Collapsed {
        egui::TopBottomPanel::bottom("tab_bar")
            .resizable(false)
            .exact_height(layout.tab_bar_height)
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let is_docked = params.ship_query.single().1.state == ShipState::Docked;
                
                // Show all tabs for Step 7
                let tabs = [
                    (ActiveStationTab::Routes, "ROUTES"),
                    (ActiveStationTab::Quest, "QUEST"),
                    (ActiveStationTab::Reserves, "RESERVES"),
                    (ActiveStationTab::Power, "POWER"),
                    (ActiveStationTab::Refinery, "REFINERY"),
                    (ActiveStationTab::Forge, "FORGE"),
                    (ActiveStationTab::ShipPort, "SHIP PORT"),
                ];
                
                let tab_width = layout.screen_width / tabs.len() as f32;
                
                ui.horizontal(|ui| {
                    for (tab, label) in tabs {
                        let selected = *params.active_tab == tab;
                        let button = egui::Button::new(label)
                            .min_size(egui::vec2(tab_width, layout.tab_bar_height))
                            .fill(if selected { 
                                egui::Color32::from_rgb(40, 40, 60) 
                            } else { 
                                egui::Color32::from_rgb(20, 20, 40) 
                            })
                            .stroke(if selected { 
                                egui::Stroke::new(2.0, egui::Color32::CYAN) 
                            } else { 
                                egui::Stroke::new(1.0, egui::Color32::from_gray(60)) 
                            });
                        
                        if ui.add(button).clicked() {
                            if *params.drawer_state == DrawerState::TabsOnly {
                                *params.drawer_state = DrawerState::Expanded;
                            }
                            *params.active_tab = tab;
                        }
                    }
                });
            });
    }

    // ---- 4. HANDLE BAR (Bottom) ----
    egui::TopBottomPanel::bottom("drawer_handle")
        .resizable(false)
        .exact_height(layout.handle_height)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let handle_rect = ui.available_rect_before_wrap();
            let response = ui.allocate_rect(handle_rect, egui::Sense::click());
            
            if response.clicked() {
                *params.drawer_state = match *params.drawer_state {
                    DrawerState::Collapsed => DrawerState::TabsOnly,
                    DrawerState::TabsOnly => DrawerState::Collapsed,
                    DrawerState::Expanded => DrawerState::TabsOnly,
                };
            }

            ui.horizontal(|ui| {
                // [ ] button on left
                if ui.add(egui::Button::new("[ ]")
                    .min_size(egui::vec2(44.0, layout.handle_height))
                    .fill(egui::Color32::from_rgb(40, 40, 60))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100)))).clicked() {
                    *params.drawer_state = match *params.drawer_state {
                        DrawerState::Collapsed => DrawerState::TabsOnly,
                        DrawerState::TabsOnly => DrawerState::Collapsed,
                        DrawerState::Expanded => DrawerState::TabsOnly,
                    };
                }

                // Decorative line fill
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let available_width = ui.available_width();
                    let line_text = "=".repeat((available_width / 8.0) as usize);
                    ui.label(egui::RichText::new(line_text)
                        .color(egui::Color32::from_gray(80))
                        .size(12.0));
                });
            });
        });

    // ---- 5. CENTRAL PANEL (World View) ----
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |_ui| {
            // World rendered by Bevy, not egui
        });

    // ---- 6. TUTORIAL POP-UP ----
    if let Some(popup) = params.tutorial.active.clone() {
        egui::Window::new(egui::RichText::new(&popup.title).strong().color(egui::Color32::CYAN))
            .id(egui::Id::new("tutorial_popup"))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([300.0, 180.0])
            .frame(egui::Frame::window(&ctx.style())
                .fill(egui::Color32::from_black_alpha(240))
                .stroke(egui::Stroke::new(2.0, egui::Color32::CYAN))
                .inner_margin(16.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(&popup.body).size(13.0).color(egui::Color32::WHITE));
                    ui.add_space(20.0);
                    if ui.add(egui::Button::new(egui::RichText::new(&popup.button_label).strong()).min_size(egui::vec2(120.0, 40.0))).clicked() {
                        params.tutorial.shown.insert(popup.id);
                        params.tutorial.active = None;
                    }
                });
            });

        // Dismiss on tap anywhere else if not consumed by the window
        if ctx.input(|i| i.pointer.any_click()) && !ctx.is_pointer_over_area() {
            params.tutorial.shown.insert(popup.id);
            params.tutorial.active = None;
        }
    }
}
