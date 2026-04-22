use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::constants::*;
use crate::systems::station_tabs::render_queue_card;
use crate::scenes::main_menu::MainMenuState;

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
    pub ship_query: Query<'w, 's, &'static mut Ship, (With<PlayerShip>, Without<AutonomousShipTag>, Without<Station>)>,
    pub station_query: Query<'w, 's, (Entity, &'static mut Station, &'static mut StationQueues), (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    pub state: Res<'w, State<GameState>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
    pub signal_log: Res<'w, SignalLog>,
    pub opening: Res<'w, OpeningSequence>,
    pub active_tab: ResMut<'w, ActiveStationTab>,
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
    pub menu_state: ResMut<'w, MainMenuState>,
}

pub fn hud_ui_system(mut params: HudParams) {
    let mut ship = params.ship_query.single_mut();
    let ctx = params.contexts.ctx_mut();

    // ── 1. SIGNAL STRIP (Bottom) ──────────────────────────────────────────────
// [PHASE B] SIGNAL STRIP (Bottom) - MIGRATED TO BEVY UI
    let strip_height = if params.expanded.0 { 180.0 } else { 60.0 };

    egui::TopBottomPanel::bottom("signal_strip")
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_black_alpha(200))
            .inner_margin(4.0))
        .exact_height(strip_height)
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

    if params.opening.phase != OpeningPhase::Complete {
        return;
    }

    if let Ok((_station_ent, mut station, mut queues)) = params.station_query.get_single_mut() {
        // ── 2. LEFT PANEL (MAP + TABS) ──────────────────────────────────────────────
        egui::SidePanel::left("left_panel")
            .frame(egui::Frame::NONE)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(16.0);
                
                let label = if *params.state.get() == GameState::SpaceView { "MAP" } else { "EXIT MAP" };
                if ui.add(egui::Button::new(label).min_size(egui::vec2(80.0, 40.0))).clicked() {
                    if *params.state.get() == GameState::SpaceView {
                        params.next_state.set(GameState::MapView);
                    } else {
                        params.next_state.set(GameState::SpaceView);
                    }
                    params.quest_log.panel_open = false;
                }

                ui.add_space(8.0);

                let quest_label = if params.quest_log.panel_open { "CLOSE Q" } else { "QUEST" };
                if ui.add(egui::Button::new(quest_label).min_size(egui::vec2(80.0, 40.0))).clicked() {
                    params.quest_log.panel_open = !params.quest_log.panel_open;
                    if params.quest_log.panel_open {
                        params.next_state.set(GameState::SpaceView);
                    }
                }

                if ship.state == ShipState::Docked {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    let tab_size = egui::vec2(80.0, 44.0);
                    let tabs = [
                        (ActiveStationTab::Reserves, "RESERVES"),
                        (ActiveStationTab::Power, "POWER"),
                        (ActiveStationTab::Smelter, "REFINERY"),
                        (ActiveStationTab::Forge, "FORGE"),
                        (ActiveStationTab::ShipPort, "SHIP PORT"),
                    ];

                    for (tab, label) in tabs {
                        if ui.add_sized(tab_size, egui::SelectableLabel::new(*params.active_tab == tab, label)).clicked() {
                            *params.active_tab = tab;
                        }
                        ui.add_space(8.0);
                    }
                }

                // FOCUS AND SAVE BUTTONS (Bottom Left)
                let (focus_rect, focus_response) = ui.allocate_exact_size(egui::vec2(80.0, 40.0), egui::Sense::click());
                let (save_rect, save_response) = ui.allocate_exact_size(egui::vec2(80.0, 40.0), egui::Sense::click());
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(16.0);
                    if focus_response.clicked() {
                        params.pan_state.is_focused = true;
                        params.pan_state.cumulative_offset = Vec2::ZERO;
                        if let Ok(mut proj) = params.cam_query.get_single_mut() {
                            proj.scale = 1.0;
                        }
                    }
                    if focus_response.hovered() {
                        ui.painter().rect_filled(
                            focus_rect,
                            0.0,
                            egui::Color32::from_rgb(40, 120, 40)
                        );
                    }
                    
                    ui.add_space(8.0);
                    
                    if save_response.clicked() {
                        params.menu_state.show_save_overlay = !params.menu_state.show_save_overlay;
                    }
                    if save_response.hovered() {
                        ui.painter().rect_filled(
                            save_rect,
                            0.0,
                            egui::Color32::from_rgb(120, 40, 40)
                        );
                    }
                });
                
                // Custom drawing after UI allocation to avoid borrowing conflicts
                let painter = ui.painter();
                
                // Draw focus lines symbol
                let focus_center = focus_rect.center();
                let focus_line_length = focus_rect.width() * 0.3;
                let focus_line_width = 2.0;
                
                painter.line_segment(
                    [focus_center - egui::vec2(focus_line_length, 0.0), focus_center + egui::vec2(focus_line_length, 0.0)],
                    egui::Stroke::new(focus_line_width, egui::Color32::WHITE)
                );
                painter.line_segment(
                    [focus_center - egui::vec2(0.0, focus_line_length), focus_center + egui::vec2(0.0, focus_line_length)],
                    egui::Stroke::new(focus_line_width, egui::Color32::WHITE)
                );
                
                // Draw gear symbol (7 circles in hex pattern)
                let gear_center = save_rect.center();
                let gear_radius = save_rect.width() * 0.08;
                
                // Calculate hex positions (6 around center, 1 in center)
                let hex_positions = [
                    gear_center + egui::vec2(0.0, -gear_radius * 2.0),           // top
                    gear_center + egui::vec2(gear_radius * 1.73, -gear_radius),      // top-right
                    gear_center + egui::vec2(gear_radius * 1.73, gear_radius),       // bottom-right
                    gear_center + egui::vec2(0.0, gear_radius * 2.0),            // bottom
                    gear_center + egui::vec2(-gear_radius * 1.73, gear_radius),      // bottom-left
                    gear_center + egui::vec2(-gear_radius * 1.73, -gear_radius),     // top-left
                ];
                
                // Draw outer 6 filled circles
                for pos in &hex_positions {
                    painter.circle_filled(*pos, gear_radius, egui::Color32::WHITE);
                }
                
                // Draw center empty circle (outline only)
                painter.circle_stroke(
                    gear_center,
                    gear_radius * 0.7,
                    egui::Stroke::new(1.5, egui::Color32::WHITE)
                );
            });

        // ── 3. QUEST PANEL ────────────────────────────────────────────────────────
        if params.quest_log.panel_open {
            egui::Window::new("OBJECTIVES")
                .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.set_min_height(300.0);
                    ui.heading(egui::RichText::new("ACTIVE").color(egui::Color32::WHITE));
                    ui.separator();
                    for obj in params.quest_log.objectives.iter().filter(|o| o.state == ObjectiveState::Active) {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("▶").color(egui::Color32::CYAN));
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
                        ui.label(egui::RichText::new(format!("✓ {}", obj.description)).color(egui::Color32::from_gray(140)));
                    }
                });
        }

        // ── 4. TAB DETAIL PANEL ───────────────────────────────────────────────────
        if ship.state == ShipState::Docked {
            egui::TopBottomPanel::bottom("tab_detail_panel")
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        match *params.active_tab {
                            ActiveStationTab::Reserves => {
                                ui.vertical(|ui| {
                                    ui.heading("STATION RESOURCES");
                                    ui.add_space(8.0);
                                    egui::Grid::new("res_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                                        ui.label("MAGNETITE:"); ui.label(egui::RichText::new(format!("{:.1}", station.magnetite_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                                        ui.label("CARBON:"); ui.label(egui::RichText::new(format!("{:.1}", station.carbon_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                                        ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                                        ui.label("POWER CELLS:"); ui.label(egui::RichText::new(format!("{}", station.power_cells)).color(egui::Color32::GREEN)); ui.end_row();
                                        ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                                        ui.label("SHIP HULLS:"); ui.label(egui::RichText::new(format!("{}", station.ship_hulls)).color(egui::Color32::GOLD)); ui.end_row();
                                    });
                                    ui.add_space(16.0);
                                    ui.separator();
                                    ui.heading("AUTO-DOCK SETTINGS");
                                    ui.checkbox(&mut params.auto_dock_settings.auto_unload, "Auto-Unload Cargo");
                                    ui.checkbox(&mut params.auto_dock_settings.auto_smelt_magnetite, "Auto-Smelt Magnetite");
                                    ui.checkbox(&mut params.auto_dock_settings.auto_smelt_carbon, "Auto-Smelt Carbon");
                                });
                                if !station.online {
                                    if ui.button(format!("REPAIR STATION [{} CELLS]", REPAIR_COST)).clicked() && station.power_cells >= REPAIR_COST {
                                        station.power_cells -= REPAIR_COST; station.repair_progress = 1.0; station.online = true;
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
                            }
                            ActiveStationTab::Smelter => {
                                ui.horizontal(|ui| {
                                    render_queue_card(ui, &mut station, &mut queues.magnetite_refinery, ProcessingOperation::MagnetiteRefinery, REFINERY_RATIO as f32, POWER_COST_REFINERY as f32, REFINERY_MAGNETITE_TIME);
                                    ui.add_space(16.0);
                                    render_queue_card(ui, &mut station, &mut queues.carbon_refinery, ProcessingOperation::CarbonRefinery, HULL_PLATE_COST_CARBON as f32, POWER_COST_HULL_FORGE as f32, REFINERY_CARBON_TIME);
                                });
                            }
                            ActiveStationTab::Forge => {
                                ui.horizontal(|ui| {
                                    render_queue_card(ui, &mut station, &mut queues.hull_forge, ProcessingOperation::HullForge, SHIP_HULL_COST_PLATES as f32, POWER_COST_SHIP_FORGE as f32, FORGE_HULL_TIME);
                                    ui.add_space(16.0);
                                    render_queue_card(ui, &mut station, &mut queues.core_fabricator, ProcessingOperation::CoreFabricator, AI_CORE_COST_CELLS as f32, POWER_COST_AI_FABRICATE as f32, FORGE_CORE_TIME);
                                });
                            }
                            ActiveStationTab::ShipPort => {
                                ui.horizontal(|ui| {
                                    if ui.button("ASSEMBLE & DEPLOY AUTONOMOUS SHIP").clicked() && station.ship_hulls >= 1 && station.ai_cores >= 1 {
                                        station.ship_hulls -= 1; station.ai_cores -= 1;
                                        let (target_pos, ore, name) = if station.ai_cores >= 1 { (SECTOR_3_POS, OreType::Carbon, "Sector 3") } else { (SECTOR_1_POS, OreType::Magnetite, "Sector 1") };
                                        params.commands.spawn((AutonomousShipTag, LastHeading(0.0), AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore, power: SHIP_POWER_MAX }, AutonomousAssignment { target_pos, ore_type: ore, sector_name: name.to_string() }, Mesh2d(params.meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))), MeshMaterial2d(params.materials.add(Color::srgb(1.0, 0.5, 0.0))), Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_SHIP)))
                                        .with_children(|parent| {
                                            parent.spawn((ThrusterGlow, Mesh2d(params.meshes.add(Rectangle::new(6.0, 8.0))), MeshMaterial2d(params.materials.add(Color::srgb(0.0, 1.0, 1.0))), Transform::from_xyz(0.0, -18.0, 0.1), Visibility::Hidden));
                                            parent.spawn((MiningBeam, Mesh2d(params.meshes.add(Rectangle::new(2.0, 1.0))), MeshMaterial2d(params.materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))), Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP), Visibility::Hidden));
                                            parent.spawn((Mesh2d(params.meshes.add(Rectangle::new(30.0, 4.0))), MeshMaterial2d(params.materials.add(Color::srgb(0.2, 0.2, 0.2))), Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP)));
                                            parent.spawn((ShipCargoBarFill, Mesh2d(params.meshes.add(Rectangle::new(30.0, 4.0))), MeshMaterial2d(params.materials.add(Color::srgb(1.0, 0.5, 0.0))), Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05)));
                                            parent.spawn((MapElement, Mesh2d(params.meshes.add(crate::systems::setup::triangle_mesh(12.0, 16.0))), MeshMaterial2d(params.materials.add(ColorMaterial { color: Color::srgb(1.0, 0.5, 0.0), alpha_mode: bevy::sprite::AlphaMode2d::Opaque, ..default() })), Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)), Visibility::Hidden));
                                        });
                                    }
                                    ui.separator();
                                    if ui.button("TOP UP SHIP [3 CELLS]").clicked() && station.power_cells >= 3 && ship.power_cells < 5 {
                                        station.power_cells -= 3; ship.power_cells = (ship.power_cells + 3).min(5);
                                    }
                                });
                            }
                            _ => { ui.label("Unavailable."); }
                        }
                    });
                    ui.add_space(8.0);
                });
        }
    }

    // ── 5. TUTORIAL POP-UP ───────────────────────────────────────────────────
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
