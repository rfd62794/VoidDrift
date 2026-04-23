use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::render::camera::Viewport;
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
    pub drawer: ResMut<'w, DrawerState>,
    pub ui_layout: Res<'w, UiLayout>,
    pub world_view_rect: ResMut<'w, WorldViewRect>,
}

pub fn hud_ui_system(mut params: HudParams) {
    let Ok(mut ship) = params.ship_query.get_single_mut() else { return; };
    let ctx = params.contexts.ctx_mut();
    let is_docked = ship.state == ShipState::Docked;
    let opening_complete = params.opening.phase == OpeningPhase::Complete;

    // Drive drawer state from ship/game state
    if !opening_complete {
        *params.drawer = DrawerState::Collapsed;
    } else if is_docked && *params.drawer == DrawerState::Collapsed {
        *params.drawer = DrawerState::Expanded;
    } else if !is_docked && *params.drawer == DrawerState::Expanded {
        *params.drawer = DrawerState::Collapsed;
    }
    let drawer = *params.drawer;
    let layout = *params.ui_layout;

    // PANEL REGISTRATION ORDER: bottom-up.
    // Signal strip → content → secondary tabs → primary tabs → handle bar → CentralPanel.

    // ── 1. SIGNAL STRIP (always visible, bottommost) ─────────────────────────
    let signal_height = if params.expanded.0 { layout.signal_height * 3.0 } else { layout.signal_height };
    egui::TopBottomPanel::bottom("signal_strip")
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_black_alpha(220))
            .inner_margin(4.0))
        .exact_height(signal_height)
        .show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let response = ui.interact(rect, ui.id().with("strip_click"), egui::Sense::click());
            if response.clicked() {
                params.expanded.0 = !params.expanded.0;
            }
            let display_count = if params.expanded.0 { 8 } else { 3 };
            let entries: Vec<&String> = params.signal_log.entries.iter().rev().take(display_count).collect();
            ui.vertical(|ui| {
                for line in entries.iter().rev() {
                    ui.label(egui::RichText::new(*line)
                        .monospace()
                        .size(11.0)
                        .color(egui::Color32::from_rgb(0, 204, 102)));
                }
            });
        });

    if !opening_complete {
        return;
    }

    if let Ok((_station_ent, mut station, mut queues)) = params.station_query.get_single_mut() {

        // ── 2. CONTENT AREA (Expanded + docked only) ──────────────────────────
        if drawer == DrawerState::Expanded && is_docked {
            egui::TopBottomPanel::bottom("content_area")
                .frame(egui::Frame::NONE
                    .fill(egui::Color32::from_black_alpha(200))
                    .inner_margin(egui::Margin::symmetric(8, 8)))
                .exact_height(layout.content_height)
                .show(ctx, |ui| {
                    ui.set_width(ui.available_width());
                    match *params.active_tab {
                        ActiveStationTab::Station => {
                            ui.heading("VOIDRIFT STATION");
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("ECHO: STATION AI - OPERATIONAL")
                                .color(egui::Color32::from_rgb(0, 204, 102)));
                        }
                        ActiveStationTab::Fleet => {
                            ui.heading("FLEET");
                            ui.add_space(8.0);
                            ui.label("DRONE MANAGEMENT - COMING ONLINE");
                        }
                        ActiveStationTab::Cargo => {
                            ui.vertical(|ui| {
                                ui.heading("CARGO BAY");
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
                            ui.heading("POWER");
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.label(format!("STATION: {:.1}/{:.0}", station.power, STATION_POWER_MAX));
                                ui.add(egui::ProgressBar::new(station.power / STATION_POWER_MAX).desired_width(150.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label(format!("SHIP:    {:.1}/{:.0}", ship.power, SHIP_POWER_MAX));
                                ui.add(egui::ProgressBar::new(ship.power / SHIP_POWER_MAX).desired_width(150.0));
                            });
                        }
                        ActiveStationTab::Refinery => {
                            ui.horizontal(|ui| {
                                render_queue_card(ui, &mut station, &mut queues.magnetite_refinery, ProcessingOperation::MagnetiteRefinery, REFINERY_RATIO as f32, POWER_COST_REFINERY as f32, REFINERY_MAGNETITE_TIME);
                                ui.add_space(16.0);
                                render_queue_card(ui, &mut station, &mut queues.carbon_refinery, ProcessingOperation::CarbonRefinery, HULL_PLATE_COST_CARBON as f32, POWER_COST_HULL_FORGE as f32, REFINERY_CARBON_TIME);
                            });
                        }
                        ActiveStationTab::Foundry => {
                            ui.horizontal(|ui| {
                                render_queue_card(ui, &mut station, &mut queues.hull_forge, ProcessingOperation::HullForge, SHIP_HULL_COST_PLATES as f32, POWER_COST_SHIP_FORGE as f32, FORGE_HULL_TIME);
                                ui.add_space(16.0);
                                render_queue_card(ui, &mut station, &mut queues.core_fabricator, ProcessingOperation::CoreFabricator, AI_CORE_COST_CELLS as f32, POWER_COST_AI_FABRICATE as f32, FORGE_CORE_TIME);
                            });
                        }
                        ActiveStationTab::Hangar => {
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
                    }
                });
        }

        // ── 3. SECONDARY TABS (Expanded + docked only) ────────────────────────
        if drawer == DrawerState::Expanded && is_docked {
            egui::TopBottomPanel::bottom("secondary_tabs")
                .frame(egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(15, 15, 20))
                    .inner_margin(egui::Margin::symmetric(4, 4)))
                .exact_height(layout.secondary_tab_height)
                .show(ctx, |ui| {
                    let available = ui.available_width();
                    let tab_w = (available / 5.0 - 4.0).max(60.0);
                    let tab_size = egui::vec2(tab_w, layout.secondary_tab_height - 8.0);
                    ui.horizontal(|ui| {
                        for (tab, label) in [
                            (ActiveStationTab::Power, "POWER"),
                            (ActiveStationTab::Cargo, "CARGO"),
                            (ActiveStationTab::Refinery, "REFINERY"),
                            (ActiveStationTab::Foundry, "FOUNDRY"),
                            (ActiveStationTab::Hangar, "HANGAR"),
                        ] {
                            if ui.add_sized(tab_size, egui::SelectableLabel::new(*params.active_tab == tab, label)).clicked() {
                                *params.active_tab = tab;
                            }
                            ui.add_space(2.0);
                        }
                    });
                });
        }

        // ── 4. PRIMARY TABS (TabsOnly + Expanded) ─────────────────────────────
        if drawer != DrawerState::Collapsed {
            egui::TopBottomPanel::bottom("primary_tabs")
                .frame(egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(10, 10, 18))
                    .inner_margin(egui::Margin::symmetric(4, 4)))
                .exact_height(layout.primary_tab_height)
                .show(ctx, |ui| {
                    let available = ui.available_width();
                    let tab_w = (available / 2.0 - 6.0).max(80.0);
                    let tab_size = egui::vec2(tab_w, layout.primary_tab_height - 8.0);
                    ui.horizontal(|ui| {
                        for (tab, label) in [
                            (ActiveStationTab::Station, "STATION"),
                            (ActiveStationTab::Fleet, "FLEET"),
                        ] {
                            if ui.add_sized(tab_size, egui::SelectableLabel::new(*params.active_tab == tab, label)).clicked() {
                                *params.active_tab = tab;
                            }
                            ui.add_space(4.0);
                        }
                    });
                });
        }

        // ── 5. HANDLE BAR (always visible, topmost of drawer) ─────────────────
        egui::TopBottomPanel::bottom("handle_bar")
            .resizable(false)
            .exact_height(layout.handle_height)
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(8, 10, 14)))
            .show(ctx, |ui| {
                // ONE interact on the full rect — no widgets
                let rect = ui.max_rect();
                let response = ui.interact(rect, ui.id(), egui::Sense::click());

                if response.clicked() {
                    *params.drawer = match drawer {
                        DrawerState::Collapsed => DrawerState::TabsOnly,
                        DrawerState::TabsOnly  => if is_docked { DrawerState::Expanded } else { DrawerState::Collapsed },
                        DrawerState::Expanded  => DrawerState::Collapsed,
                    };
                }

                // Visual only — painter, no widgets
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                    egui::FontId::monospace(10.0),
                    egui::Color32::from_gray(60),
                );
            });

        // ── 6. QUEST OVERLAY (window, above world) ────────────────────────────
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

    } // end station query

    // ── 7. CENTRAL PANEL (world view — MUST be last) ───────────────────────────
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            // Store the actual rendered rect so camera_viewport_system uses it exactly
            let r = ui.max_rect();
            params.world_view_rect.x = r.min.x;
            params.world_view_rect.y = r.min.y;
            params.world_view_rect.w = r.width();
            params.world_view_rect.h = r.height();

            if opening_complete {
                if ui.add(egui::Button::new("FOCUS").min_size(egui::vec2(80.0, 44.0))).clicked() {
                    params.pan_state.is_focused = true;
                    params.pan_state.cumulative_offset = Vec2::ZERO;
                    if let Ok(mut proj) = params.cam_query.get_single_mut() {
                        proj.scale = 1.0;
                    }
                }
            }
        });

    // ── 8. TUTORIAL POP-UP ───────────────────────────────────────────────────
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

/// Resizes the camera viewport each frame to exactly match the CentralPanel rect
/// written by hud_ui_system. Using the actual egui rect eliminates the one-frame
/// lag that caused the graphical clone glitch on drawer state changes.
pub fn camera_viewport_system(
    world_view: Res<WorldViewRect>,
    windows: Query<&Window>,
    mut cam_query: Query<&mut Camera, With<MainCamera>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Ok(mut camera) = cam_query.get_single_mut() else { return; };

    let scale = window.scale_factor() as f32;

    // Convert logical egui px to physical px
    let phys_x = (world_view.x * scale).round() as u32;
    let phys_y = (world_view.y * scale).round() as u32;
    let phys_w = (world_view.w * scale).round() as u32;
    let phys_h = (world_view.h * scale).round() as u32;

    if phys_w == 0 || phys_h == 0 { return; }

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(phys_x, phys_y),
        physical_size: UVec2::new(phys_w, phys_h),
        depth: 0.0..1.0,
    });
}
