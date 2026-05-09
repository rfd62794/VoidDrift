pub mod state_machine;
pub mod content;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::components::resources::MaxDispatch;
use crate::scenes::main_menu::MainMenuState;
use crate::config::{BalanceConfig, VisualConfig};
use crate::config::visual::{rgb, rgb_u8_to_egui};
use crate::systems::visuals::ore_polygon::{self, OrePolygonConfig};
use crate::systems::visuals::ingot_node::{self, IngotNodeConfig};
use crate::systems::visuals::component_nodes::{self, ThrusterConfig, HullConfig, CanisterConfig, AICoreConfig, DroneBayConfig};
use crate::systems::telemetry::{TelemetryOptInPrompt, TelemetryConsent, TelemetrySessionCounter};
use crate::systems::persistence::save::SaveRequestEvent;

// ── Non-egui systems (kept here for module cohesion) ──────────────────────────

pub fn ship_cargo_display_system(
    time: Res<Time>,
    ship_query: Query<(&Ship, &Children), Without<Station>>,
    mut fill_query: Query<(&mut Transform, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vcfg: Res<VisualConfig>,
) {
    for (ship, children) in ship_query.iter() {
        for child in children.iter() {
            if let Ok((mut transform, mat_handle)) = fill_query.get_mut(*child) {
                let ratio = (ship.cargo / ship.cargo_capacity as f32).clamp(0.0, 1.0);
                transform.scale.x = ratio.max(0.001);
                transform.translation.x = -20.0 * (1.0 - ratio);
                if let Some(mat) = materials.get_mut(&mat_handle.0) {
                    let av = &vcfg.asteroid;
                    mat.color = if ship.cargo >= ship.cargo_capacity as f32 * 0.95 {
                        let pulse = (time.elapsed_secs() * 10.0).sin() * 0.2 + 0.8;
                        Color::srgba(0.0, pulse, pulse, 1.0)
                    } else {
                        match ship.cargo_type {
                            OreDeposit::Iron     => rgb(av.color_iron),
                            OreDeposit::Tungsten => rgb(av.color_tungsten),
                            OreDeposit::Nickel   => rgb(av.color_nickel),
                            OreDeposit::Aluminum => rgb(av.color_aluminum),
                        }
                    };
                }
            }
        }
    }
}

pub fn cargo_label_system(
    ship_query: Query<(&Ship, &Children), Without<Station>>,
    mut ore_label_query: Query<&mut Text2d, (With<CargoOreLabel>, Without<CargoCountLabel>)>,
    mut count_label_query: Query<&mut Text2d, (With<CargoCountLabel>, Without<CargoOreLabel>)>,
) {
    for (ship, children) in ship_query.iter() {
        for &child in children.iter() {
            if let Ok(mut ore_text) = ore_label_query.get_mut(child) {
                ore_text.0 = match ship.cargo_type {
                    OreDeposit::Iron => "IRON".to_string(),
                    OreDeposit::Tungsten => "TUNGSTEN".to_string(),
                    OreDeposit::Nickel    => "NICKEL".to_string(),
                    OreDeposit::Aluminum  => "ALUMINUM".to_string(),
                };
            }
            if let Ok(mut count_text) = count_label_query.get_mut(child) {
                count_text.0 = format!("{:.0} / {}", ship.cargo, ship.cargo_capacity);
            }
        }
    }
}

pub fn station_visual_system(
    station_query: Query<&Station>,
    mut hub_query: Query<&MeshMaterial2d<ColorMaterial>, With<StationHub>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vcfg: Res<VisualConfig>,
) {
    if let Ok(station) = station_query.get_single() {
        if let Ok(material_handle) = hub_query.get_single_mut() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let sv = &vcfg.station;
                let target_color = if station.online { rgb(sv.color_hub_online) } else { rgb(sv.color_hub_offline) };
                if material.color != target_color { material.color = target_color; }
            }
        }
    }
}

/// Syncs station.max_dispatch to MaxDispatch resource for HUD display
/// Runs before HUD to avoid query conflicts
pub fn sync_max_drones_system(
    station_query: Query<&Station, (With<Station>, Without<Ship>)>,
    mut max_dispatch: ResMut<MaxDispatch>,
) {
    if let Ok(station) = station_query.get_single() {
        max_dispatch.0 = station.max_dispatch;
    }
}

#[derive(SystemParam)]
pub struct HudParams<'w, 's> {
    pub contexts: EguiContexts<'w, 's>,
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
    pub toggles: ResMut<'w, ProductionToggles>,
    pub tutorial: ResMut<'w, TutorialState>,
    pub pan_state: ResMut<'w, MapPanState>,
    pub cam_query: Query<'w, 's, &'static mut OrthographicProjection, With<MainCamera>>,
    pub menu_state: ResMut<'w, MainMenuState>,
    pub drawer: ResMut<'w, DrawerState>,
    pub ui_layout: Res<'w, UiLayout>,
    pub world_view_rect: ResMut<'w, WorldViewRect>,
    pub queue: Res<'w, ShipQueue>,
    pub max_dispatch: Res<'w, MaxDispatch>,
    pub autonomous_ships: Query<'w, 's, Entity, With<AutonomousShipTag>>,
    pub prod_tab: ResMut<'w, ProductionTabState>,
    pub req_tab: ResMut<'w, RequestsTabState>,
    pub repair_events: EventWriter<'w, RepairStationEvent>,
    pub fulfill_events: EventWriter<'w, FulfillRequestEvent>,
    pub save_events: EventWriter<'w, SaveRequestEvent>,
    pub balance_cfg: Res<'w, BalanceConfig>,
    pub visual_cfg: Res<'w, VisualConfig>,
    pub request_cfg: Res<'w, crate::config::RequestConfig>,
    pub logs_cfg: Res<'w, crate::config::LogsConfig>,
    pub save_data: ResMut<'w, crate::systems::persistence::save::SaveData>,
    pub view_state: ResMut<'w, ViewState>,
    pub telemetry_opt_in: ResMut<'w, TelemetryOptInPrompt>,
    pub telemetry_consent: ResMut<'w, TelemetryConsent>,
    pub telemetry_session_counter: ResMut<'w, TelemetrySessionCounter>,
}

pub fn hud_ui_system(mut params: HudParams, mut was_docked: Local<bool>) {
    let ctx = params.contexts.ctx_mut();

    // Record egui canvas size every frame for viewport scaling
    let screen = ctx.screen_rect();
    params.world_view_rect.canvas_w = screen.width();
    params.world_view_rect.canvas_h = screen.height();

    let is_docked = true;
    let opening_complete = params.opening.phase == OpeningPhase::Complete;

    // ── STATE MACHINE ─────────────────────────────────────────────────────────
    state_machine::update_drawer_state(
        is_docked,
        opening_complete,
        &mut was_docked,
        &mut params.drawer,
    );
    let drawer = *params.drawer;
    let layout = *params.ui_layout;

    // ── PANEL REGISTRATION ORDER: bottom-up ───────────────────────────────────
    // 1. Signal strip (always)
    // 2. Content area (if Expanded)
    // 3. Secondary tabs (if Expanded && docked)
    // 4. Primary tabs (if Expanded)
    // 5. Handle bar (always when opening_complete)
    // 6. Central panel (always — MUST be last)

    // ── 1. SIGNAL STRIP ───────────────────────────────────────────────────────
    if !params.view_state.show_production_tree {
        let signal_height = if params.expanded.0 { layout.signal_height * 3.0 } else { layout.signal_height };
        egui::TopBottomPanel::bottom("signal_strip")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(5, 8, 12))
                .inner_margin(4.0))
            .exact_height(signal_height)
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let response = ui.interact(rect, ui.id().with("strip_click"), egui::Sense::click());
                if response.clicked() { params.expanded.0 = !params.expanded.0; }
                let display_count = if params.expanded.0 { 8 } else { 3 };
                let entries: Vec<&String> = params.signal_log.entries.iter().rev().take(display_count).collect();
                ui.vertical(|ui| {
                    for line in entries.iter().rev() {
                        ui.label(egui::RichText::new(*line).monospace().size(11.0).color(egui::Color32::from_rgb(0, 204, 102)));
                    }
                });
            });
    }

    // Register CentralPanel early during opening sequence and return
    if !opening_complete {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let r = ui.max_rect();
                params.world_view_rect.x = r.min.x;
                params.world_view_rect.y = r.min.y;
                params.world_view_rect.w = r.width();
                params.world_view_rect.h = r.height();
            });
        return;
    }

    // ── 2. CONTENT AREA (if Expanded) ─────────────────────────────────────────
    if drawer == DrawerState::Expanded && !params.view_state.show_production_tree {
        // Get station data — available only when docked
        let station_result = params.station_query.get_single_mut();

        egui::TopBottomPanel::bottom("content_area")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(8, 10, 16))
                .inner_margin(egui::Margin::symmetric(8, 8)))
            .exact_height(layout.content_height)
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());
                if let Ok((_ent, mut station, _queues)) = station_result {
                    content::render_tab_content(
                        ui,
                        *params.active_tab,
                        &mut station,
                        &mut params.toggles,
                        &params.queue,
                        &mut params.prod_tab,
                        &mut params.req_tab,
                        &mut params.repair_events,
                        &mut params.fulfill_events,
                        &params.balance_cfg,
                        &params.request_cfg,
                        &params.logs_cfg,
                        &params.save_data,
                    );
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.heading(egui::RichText::new("APPROACHING STATION")
                            .color(egui::Color32::from_rgb(0, 204, 102)));
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Dock to access station systems.")
                            .color(egui::Color32::from_gray(160)));
                    });
                }
            });
    }

    // ── 3. SECONDARY TABS (if Expanded && docked) ─────────────────────────────
    if drawer == DrawerState::Expanded && is_docked && !params.view_state.show_production_tree {
        let no_popup = params.tutorial.active.is_none();
        let show_forge_hl    = no_popup && params.tutorial.shown.contains(&104) && !params.tutorial.shown.contains(&105);
        let show_requests_hl = no_popup && params.tutorial.shown.contains(&106) && !params.req_tab.collected_requests.is_empty() && !params.req_tab.visited_after_t106;
        let tab_hl_fill   = egui::Color32::from_rgba_unmultiplied(0, 220, 220, 35);
        let tab_hl_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 220, 220));

        egui::TopBottomPanel::bottom("secondary_tabs")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(15, 15, 20))
                .inner_margin(egui::Margin { left: 0, right: 4, top: 4, bottom: 4 }))
            .exact_height(layout.secondary_tab_height)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let tab_w = ui.available_width() / 4.0;
                let tab_size = egui::vec2(tab_w, layout.secondary_tab_height - 8.0);
                ui.horizontal(|ui| {
                    for (tab, label) in [
                        (ActiveStationTab::Cargo,      "CARGO"),
                        (ActiveStationTab::Production, "FORGE"),
                        (ActiveStationTab::Requests,   "QUESTS"),
                        (ActiveStationTab::Logs,       "LOGS"),
                    ] {
                        let response = ui.add_sized(
                            tab_size,
                            egui::SelectableLabel::new(*params.active_tab == tab, label),
                        );
                        let highlight = match tab {
                            ActiveStationTab::Production => show_forge_hl && *params.active_tab != tab,
                            ActiveStationTab::Requests   => show_requests_hl && *params.active_tab != tab,
                            _                            => false,
                        };
                        if highlight {
                            let p = ui.ctx().layer_painter(egui::LayerId::new(
                                egui::Order::Foreground,
                                egui::Id::new("tab_hl"),
                            ));
                            p.rect_filled(response.rect, 0.0, tab_hl_fill);
                            p.rect_stroke(response.rect, 0.0, tab_hl_stroke, egui::StrokeKind::Outside);
                        }
                        if response.clicked() {
                            *params.active_tab = tab;
                            if tab == ActiveStationTab::Requests && params.tutorial.shown.contains(&106) {
                                params.req_tab.visited_after_t106 = true;
                            }
                        }
                    }
                });
            });
    }

    // ── 4. PRIMARY TABS REMOVED ─────────────────────────────

    // ── 5. HANDLE BAR (always when opening_complete) ──────────────────────────
    if !params.view_state.show_production_tree {
        egui::TopBottomPanel::bottom("handle_bar")
            .resizable(false)
            .exact_height(layout.handle_height)
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(8, 10, 14)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let response = ui.interact(rect, ui.id(), egui::Sense::click());
                if response.clicked() {
                    *params.drawer = match *params.drawer {
                        DrawerState::Collapsed => DrawerState::Expanded,
                        DrawerState::Expanded  => DrawerState::Collapsed,
                    };
                }
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                    egui::FontId::monospace(10.0),
                    egui::Color32::from_gray(60),
                );

                // Drawer highlight during T-103 popup (when player reads "Tap the grey bar")
                let should_highlight = params.tutorial.active.as_ref().map(|p| p.id == 103).unwrap_or(false);
                if should_highlight {
                    let t = ui.ctx().input(|i| i.time as f32);
                    let alpha = ((t * 2.0).sin() * 0.3 + 0.7) * 255.0;
                    let center_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(200.0, rect.height()));
                    let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("drawer_highlight"));
                    let painter = ui.ctx().layer_painter(layer_id);
                    painter.rect_stroke(
                        center_rect,
                        0.0,
                        egui::Stroke::new(8.0, egui::Color32::from_rgba_unmultiplied(255, 200, 50, alpha as u8)),
                        egui::StrokeKind::Outside,
                    );
                }
            });
    }

    // ── 6. QUEST OVERLAY (window, above world) ────────────────────────────────
    if params.quest_log.panel_open && !params.view_state.show_production_tree {
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


    // ── 8. PRODUCTION TREE VIEWPORT ───────────────────────────────────────────
    if params.view_state.show_production_tree {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(2, 4, 8)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                
                // Update WorldViewRect to zero — hide world view
                params.world_view_rect.w = 0.0;
                params.world_view_rect.h = 0.0;
                
                let painter = ui.painter();
                let text_color = egui::Color32::WHITE;
                
                // Grid dimensions
                let col_width = rect.width() / 4.0;
                let row_height = rect.height() / 5.0;
                let node_size = egui::vec2(100.0, 40.0);
                let drone_bay_size = egui::vec2(200.0, 40.0);
                
                // Access station data for active states
                let (station, mut toggles) = if let Ok((_, st, _)) = params.station_query.get_single() {
                    (Some(st), params.toggles.clone())
                } else {
                    (None, params.toggles.clone())
                };
                
                // Compute all node centers first
                let node_center = |col: usize, row: usize| -> egui::Pos2 {
                    egui::pos2(
                        rect.min.x + col_width * (col as f32 + 0.5),
                        rect.min.y + row_height * (row as f32 + 0.5),
                    )
                };
                
                // Named centers for arrow routing
                let iron_ore     = node_center(0, 0);
                let tungsten_ore = node_center(1, 0);
                let nickel_ore   = node_center(2, 0);
                let aluminum_ore = node_center(3, 0);
                
                let iron_ingot     = node_center(0, 1);
                let tungsten_ingot = node_center(1, 1);
                let nickel_ingot   = node_center(2, 1);
                let aluminum_ingot = node_center(3, 1);
                
                let hull_plate = node_center(0, 2);
                let thruster   = node_center(1, 2);
                let ai_core    = node_center(2, 2);
                let canister   = node_center(3, 2);
                
                let drone_bay  = node_center(1, 3); // wide, centered
                
                // Arrow drawing helper with three-state colors and ON/OFF label
                let draw_arrow = |from: egui::Pos2, to: egui::Pos2, toggle_on: bool, has_inventory: bool| -> egui::Rect {
                    let color = if !toggle_on {
                        egui::Color32::from_rgb(180, 40, 40)  // Red — OFF
                    } else if has_inventory {
                        egui::Color32::from_rgb(40, 180, 40)  // Green — ON + flowing
                    } else {
                        egui::Color32::from_rgb(80, 80, 80)   // Gray — ON but nothing to flow
                    };
                    let stroke = egui::Stroke::new(1.5, color);
                    
                    // Arrow shaft — from bottom of source node to top of target node
                    let shaft_from = egui::pos2(from.x, from.y + 20.0); // bottom of node
                    let shaft_to = egui::pos2(to.x, to.y - 20.0);       // top of node
                    painter.line_segment([shaft_from, shaft_to], stroke);
                    
                    // Arrowhead — small triangle at shaft_to
                    let tip = shaft_to;
                    let left = egui::pos2(tip.x - 5.0, tip.y - 8.0);
                    let right = egui::pos2(tip.x + 5.0, tip.y - 8.0);
                    painter.add(egui::Shape::convex_polygon(
                        vec![tip, left, right],
                        color,
                        egui::Stroke::NONE,
                    ));
                    
                    // ON/OFF label centered on shaft
                    let mid = egui::pos2(
                        (shaft_from.x + shaft_to.x) / 2.0,
                        (shaft_from.y + shaft_to.y) / 2.0,
                    );
                    let label = if toggle_on { "ON" } else { "OFF" };
                    painter.text(mid, egui::Align2::CENTER_CENTER, label, 
                        egui::FontId::proportional(9.0), color);
                    
                    // Return clickable rect for Part 3
                    egui::Rect::from_center_size(mid, egui::vec2(50.0, 30.0))
                };
                
                // Ore → Ingot (toggle-based)
                let st = station.as_ref();
                let refine_iron_rect = draw_arrow(iron_ore, iron_ingot, toggles.refine_iron, st.map_or(false, |s| s.iron_reserves > 0.0));
                let refine_tungsten_rect = draw_arrow(tungsten_ore, tungsten_ingot, toggles.refine_tungsten, st.map_or(false, |s| s.tungsten_reserves > 0.0));
                let refine_nickel_rect = draw_arrow(nickel_ore, nickel_ingot, toggles.refine_nickel, st.map_or(false, |s| s.nickel_reserves > 0.0));
                let refine_aluminum_rect = draw_arrow(aluminum_ore, aluminum_ingot, toggles.refine_aluminum, st.map_or(false, |s| s.aluminum_reserves > 0.0));
                
                // Ingot → Part (toggle-based)
                let forge_hull_rect = draw_arrow(iron_ingot, hull_plate, toggles.forge_hull, st.map_or(false, |s| s.iron_ingots > 0.0));
                let forge_thruster_rect = draw_arrow(tungsten_ingot, thruster, toggles.forge_thruster, st.map_or(false, |s| s.tungsten_ingots > 0.0));
                let forge_core_rect = draw_arrow(nickel_ingot, ai_core, toggles.forge_core, st.map_or(false, |s| s.nickel_ingots > 0.0));
                let forge_canister_rect = draw_arrow(aluminum_ingot, canister, toggles.forge_aluminum_canister, st.map_or(false, |s| s.aluminum_ingots > 0.0));
                
                // Part → Drone Bay (shared build_drones toggle)
                let hull_drone_rect = if let Some(s) = station {
                    draw_arrow(hull_plate, drone_bay, toggles.build_drones, s.hull_plate_reserves > 0.0)
                } else {
                    egui::Rect::NOTHING
                };
                let thruster_drone_rect = if let Some(s) = station {
                    draw_arrow(thruster, drone_bay, toggles.build_drones, s.thruster_reserves > 0.0)
                } else {
                    egui::Rect::NOTHING
                };
                let core_drone_rect = if let Some(s) = station {
                    draw_arrow(ai_core, drone_bay, toggles.build_drones, s.ai_cores > 0.0)
                } else {
                    egui::Rect::NOTHING
                };
                
                // Click detection for arrows
                let response = ui.interact(refine_iron_rect, ui.id().with("toggle_refine_iron"), egui::Sense::click());
                if response.clicked() { toggles.refine_iron = !toggles.refine_iron; }
                
                let response = ui.interact(refine_tungsten_rect, ui.id().with("toggle_refine_tungsten"), egui::Sense::click());
                if response.clicked() { toggles.refine_tungsten = !toggles.refine_tungsten; }
                
                let response = ui.interact(refine_nickel_rect, ui.id().with("toggle_refine_nickel"), egui::Sense::click());
                if response.clicked() { toggles.refine_nickel = !toggles.refine_nickel; }
                
                let response = ui.interact(refine_aluminum_rect, ui.id().with("toggle_refine_aluminum"), egui::Sense::click());
                if response.clicked() { toggles.refine_aluminum = !toggles.refine_aluminum; }
                
                let response = ui.interact(forge_hull_rect, ui.id().with("toggle_forge_hull"), egui::Sense::click());
                if response.clicked() { toggles.forge_hull = !toggles.forge_hull; }
                
                let response = ui.interact(forge_thruster_rect, ui.id().with("toggle_forge_thruster"), egui::Sense::click());
                if response.clicked() { toggles.forge_thruster = !toggles.forge_thruster; }
                
                let response = ui.interact(forge_core_rect, ui.id().with("toggle_forge_core"), egui::Sense::click());
                if response.clicked() { toggles.forge_core = !toggles.forge_core; }
                
                let response = ui.interact(forge_canister_rect, ui.id().with("toggle_forge_canister"), egui::Sense::click());
                if response.clicked() { toggles.forge_aluminum_canister = !toggles.forge_aluminum_canister; }
                
                // Shared build_drones toggle — any of the three arrows toggles it
                let response = ui.interact(hull_drone_rect, ui.id().with("toggle_build_drones"), egui::Sense::click());
                if response.clicked() { toggles.build_drones = !toggles.build_drones; }
                let response = ui.interact(thruster_drone_rect, ui.id().with("toggle_build_drones_thruster"), egui::Sense::click());
                if response.clicked() { toggles.build_drones = !toggles.build_drones; }
                let response = ui.interact(core_drone_rect, ui.id().with("toggle_build_drones_core"), egui::Sense::click());
                if response.clicked() { toggles.build_drones = !toggles.build_drones; }
                
                // Canister → future (no arrow for now)
                
                // Title at top (smaller to make room for grid)
                painter.text(
                    egui::pos2(rect.center().x, rect.min.y + 30.0),
                    egui::Align2::CENTER_CENTER,
                    "PRODUCTION PIPELINE",
                    egui::FontId::proportional(16.0),
                    egui::Color32::from_rgb(0, 200, 200),
                );
                
                // Node rendering helper with active state and inventory display
                let render_node = |col: usize, row: usize, label: &str, inventory: String, is_wide: bool, active: bool, ore_type: Option<OreDeposit>, is_ingot: bool, component_type: Option<&str>| {
                    let border_color = if active {
                        egui::Color32::from_rgb(0, 200, 200) // Echo cyan
                    } else {
                        egui::Color32::from_rgb(40, 80, 80) // Dimmed cyan
                    };
                    let fill_color = if active {
                        egui::Color32::from_rgb(4, 16, 20) // Slightly brighter
                    } else {
                        egui::Color32::from_rgb(4, 8, 16) // Dark
                    };
                    let border_stroke = egui::Stroke::new(1.5, border_color);
                    
                    let cell_center = egui::pos2(
                        rect.min.x + col_width * (col as f32 + 0.5),
                        rect.min.y + row_height * (row as f32 + 0.5),
                    );
                    let size = if is_wide { drone_bay_size } else { node_size };
                    let node_rect = egui::Rect::from_center_size(cell_center, size);
                    
                    painter.rect_filled(node_rect, 4.0, fill_color);
                    painter.rect_stroke(node_rect, 4.0, border_stroke, egui::StrokeKind::Outside);
                    
                    // Draw procedural visuals for ore/ingot nodes
                    if let Some(ore) = ore_type {
                        if is_ingot {
                            // Draw ingot node (3-rect isometric)
                            let ingot_cfg = &params.visual_cfg.production_tree.ingot_node;
                            let base_color = match ore {
                                OreDeposit::Iron => rgb_u8_to_egui(params.visual_cfg.ore.metal.color_vein),
                                OreDeposit::Tungsten => rgb_u8_to_egui(params.visual_cfg.ore.h3_gas.color_vein),
                                OreDeposit::Nickel => rgb_u8_to_egui(params.visual_cfg.ore.void_essence.color_vein),
                                OreDeposit::Aluminum => rgb_u8_to_egui(params.visual_cfg.ore.metal.color_vein),
                            };
                            let ingot_config = IngotNodeConfig {
                                width: ingot_cfg.width,
                                height: ingot_cfg.height,
                                depth_offset_x: ingot_cfg.depth_offset_x,
                                depth_offset_y: ingot_cfg.depth_offset_y,
                                color_face_light_factor: ingot_cfg.color_face_light_factor,
                                color_face_dark_factor: ingot_cfg.color_face_dark_factor,
                            };
                            ingot_node::draw_ingot_node(painter, node_rect.center(), &ingot_config, base_color);
                        } else {
                            // Draw ore node (procedural polygon)
                            let ore_cfg = &params.visual_cfg.production_tree.ore_node;
                            let (body_color, vein_color, band_count, band_width_min, band_width_max, grain_angle_deg) = match ore {
                                OreDeposit::Iron => {
                                    let cfg = &params.visual_cfg.ore.metal;
                                    (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                                },
                                OreDeposit::Tungsten => {
                                    let cfg = &params.visual_cfg.ore.h3_gas;
                                    (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                                },
                                OreDeposit::Nickel => {
                                    let cfg = &params.visual_cfg.ore.void_essence;
                                    (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                                },
                                OreDeposit::Aluminum => {
                                    let cfg = &params.visual_cfg.ore.metal;
                                    (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                                },
                            };
                            let ore_seed = match ore {
                                OreDeposit::Iron => 1u64,
                                OreDeposit::Tungsten => 2u64,
                                OreDeposit::Nickel => 3u64,
                                OreDeposit::Aluminum => 4u64,
                            };
                            let ore_config = OrePolygonConfig {
                                radius: ore_cfg.radius,
                                vertex_count: ore_cfg.vertex_count,
                                jaggedness: ore_cfg.jaggedness,
                                color_body: body_color,
                                color_vein: vein_color,
                                band_count,
                                band_width_min,
                                band_width_max,
                                grain_angle_deg,
                                seed: ore_seed,
                            };
                            ore_polygon::draw_ore_polygon(painter, node_rect.center(), &ore_config);
                        }
                    }
                    
                    if let Some(component) = component_type {
                        let node_center = node_rect.center();
                        match component {
                            "hull" => {
                                let hull_cfg = &params.visual_cfg.component.hull;
                                let hull_config = HullConfig {
                                    width: hull_cfg.width,
                                    rib_count: hull_cfg.rib_count,
                                    color_frame: rgb_u8_to_egui(hull_cfg.color_frame),
                                    color_outline: rgb_u8_to_egui(hull_cfg.color_outline),
                                    stroke_width: hull_cfg.stroke_width,
                                };
                                component_nodes::draw_hull(painter, node_center, &hull_config);
                            },
                            "thruster" => {
                                let thruster_cfg = &params.visual_cfg.component.thruster;
                                let thruster_config = ThrusterConfig {
                                    width: thruster_cfg.width,
                                    color_nozzle: rgb_u8_to_egui(thruster_cfg.color_nozzle),
                                    color_body: rgb_u8_to_egui(thruster_cfg.color_body),
                                    color_wire: rgb_u8_to_egui(thruster_cfg.color_wire),
                                    wire_count: thruster_cfg.wire_count,
                                    nozzle_width_ratio: thruster_cfg.nozzle_width_ratio,
                                    body_width_ratio: thruster_cfg.body_width_ratio,
                                };
                                component_nodes::draw_thruster(painter, node_center, &thruster_config);
                            },
                            "ai_core" => {
                                let ai_core_cfg = &params.visual_cfg.component.ai_core;
                                let ai_core_config = AICoreConfig {
                                    radius: ai_core_cfg.radius,
                                    fin_count: ai_core_cfg.fin_count,
                                    fin_length: ai_core_cfg.fin_length,
                                    fin_width: ai_core_cfg.fin_width,
                                    color_body: rgb_u8_to_egui(ai_core_cfg.color_body),
                                    color_fins: rgb_u8_to_egui(ai_core_cfg.color_fins),
                                    color_fan_housing: rgb_u8_to_egui(ai_core_cfg.color_fan_housing),
                                    fan_radius_ratio: ai_core_cfg.fan_radius_ratio,
                                    fan_blade_count: ai_core_cfg.fan_blade_count,
                                };
                                component_nodes::draw_ai_core(painter, node_center, &ai_core_config);
                            },
                            "canister" => {
                                let canister_cfg = &params.visual_cfg.component.canister;
                                let canister_config = CanisterConfig {
                                    width: canister_cfg.width,
                                    height: canister_cfg.height,
                                    lid_height_ratio: canister_cfg.lid_height_ratio,
                                    color_body: rgb_u8_to_egui(canister_cfg.color_body),
                                    color_lid: rgb_u8_to_egui(canister_cfg.color_lid),
                                    color_highlight: rgb_u8_to_egui(canister_cfg.color_highlight),
                                    color_handle: rgb_u8_to_egui(canister_cfg.color_handle),
                                };
                                component_nodes::draw_canister(painter, node_center, &canister_config);
                            },
                            "drone_bay" => {
                                let drone_bay_cfg = &params.visual_cfg.component.drone_bay;
                                let drone_bay_config = DroneBayConfig {
                                    width: drone_bay_cfg.width,
                                    height: drone_bay_cfg.height,
                                    color_ready: rgb_u8_to_egui(drone_bay_cfg.color_ready),
                                    color_empty: rgb_u8_to_egui(drone_bay_cfg.color_empty),
                                    nose_height_ratio: drone_bay_cfg.nose_height_ratio,
                                    fin_width_ratio: drone_bay_cfg.fin_width_ratio,
                                    fin_height_ratio: drone_bay_cfg.fin_height_ratio,
                                    porthole_radius: drone_bay_cfg.porthole_radius,
                                    porthole_offset_y: drone_bay_cfg.porthole_offset_y,
                                    exhaust_radius: drone_bay_cfg.exhaust_radius,
                                };
                                let is_ready = if let Some(st) = station { st.drone_build_progress >= 1.0 } else { false };
                                component_nodes::draw_drone_bay(painter, node_center, &drone_bay_config, is_ready);
                            },
                            _ => {}
                        }
                    }
                    
                    let display = format!("{} ({})", label, inventory);
                    painter.text(
                        node_rect.center() + egui::vec2(0.0, 35.0),
                        egui::Align2::CENTER_CENTER,
                        &display,
                        egui::FontId::proportional(10.0),
                        text_color,
                    );
                };
                
                // Render nodes with active states and inventory
                if let Some(st) = station {
                    // Row 0: Ore nodes
                    render_node(0, 0, "IRON", format!("{:.1}", st.iron_reserves), false, st.iron_reserves > 0.0, Some(OreDeposit::Iron), false, None);
                    render_node(1, 0, "TUNGSTEN", format!("{:.1}", st.tungsten_reserves), false, st.tungsten_reserves > 0.0, Some(OreDeposit::Tungsten), false, None);
                    render_node(2, 0, "NICKEL", format!("{:.1}", st.nickel_reserves), false, st.nickel_reserves > 0.0, Some(OreDeposit::Nickel), false, None);
                    render_node(3, 0, "ALUMINUM", format!("{:.1}", st.aluminum_reserves), false, st.aluminum_reserves > 0.0, Some(OreDeposit::Aluminum), false, None);
                    
                    // Row 1: Ingot nodes
                    render_node(0, 1, "IRON INGOT", format!("{:.1}", st.iron_ingots), false, st.iron_ingots > 0.0, Some(OreDeposit::Iron), true, None);
                    render_node(1, 1, "TUNGSTEN INGOT", format!("{:.1}", st.tungsten_ingots), false, st.tungsten_ingots > 0.0, Some(OreDeposit::Tungsten), true, None);
                    render_node(2, 1, "NICKEL INGOT", format!("{:.1}", st.nickel_ingots), false, st.nickel_ingots > 0.0, Some(OreDeposit::Nickel), true, None);
                    render_node(3, 1, "ALUMINUM INGOT", format!("{:.1}", st.aluminum_ingots), false, st.aluminum_ingots > 0.0, Some(OreDeposit::Aluminum), true, None);
                    
                    // Row 2: Part nodes
                    render_node(0, 2, "HULL PLATE", format!("{:.0}", st.hull_plate_reserves), false, st.hull_plate_reserves > 0.0, None, false, Some("hull"));
                    render_node(1, 2, "THRUSTER", format!("{:.0}", st.thruster_reserves), false, st.thruster_reserves > 0.0, None, false, Some("thruster"));
                    render_node(2, 2, "AI CORE", format!("{:.0}", st.ai_cores), false, st.ai_cores > 0.0, None, false, Some("ai_core"));
                    render_node(3, 2, "CANISTER", format!("{:.0}", st.aluminum_canisters), false, st.aluminum_canisters > 0.0, None, false, Some("canister"));
                    
                    // Row 3: Convergence (DRONE BAY) — no inventory number
                    render_node(1, 3, "DRONE BAY", String::new(), true, 
                        st.hull_plate_reserves > 0.0 && 
                        st.thruster_reserves > 0.0 && 
                        st.ai_cores > 0.0, None, false, Some("drone_bay"));
                } else {
                    // Render all nodes as inactive when station not accessible
                    render_node(0, 0, "IRON", String::new(), false, false, Some(OreDeposit::Iron), false, None);
                    render_node(1, 0, "TUNGSTEN", String::new(), false, false, Some(OreDeposit::Tungsten), false, None);
                    render_node(2, 0, "NICKEL", String::new(), false, false, Some(OreDeposit::Nickel), false, None);
                    render_node(3, 0, "ALUMINUM", String::new(), false, false, Some(OreDeposit::Aluminum), false, None);
                    
                    render_node(0, 1, "IRON INGOT", String::new(), false, false, Some(OreDeposit::Iron), true, None);
                    render_node(1, 1, "TUNGSTEN INGOT", String::new(), false, false, Some(OreDeposit::Tungsten), true, None);
                    render_node(2, 1, "NICKEL INGOT", String::new(), false, false, Some(OreDeposit::Nickel), true, None);
                    render_node(3, 1, "ALUMINUM INGOT", String::new(), false, false, Some(OreDeposit::Aluminum), true, None);
                    
                    render_node(0, 2, "HULL PLATE", String::new(), false, false, None, false, Some("hull"));
                    render_node(1, 2, "THRUSTER", String::new(), false, false, None, false, Some("thruster"));
                    render_node(2, 2, "AI CORE", String::new(), false, false, None, false, Some("ai_core"));
                    render_node(3, 2, "CANISTER", String::new(), false, false, None, false, Some("canister"));
                    
                    render_node(1, 3, "DRONE BAY", String::new(), true, false, None, false, Some("drone_bay"));
                }
                
                // Write toggles back to resource after any clicks
                *params.toggles = toggles;
                
                // BACK button — bottom center, above DRONE BAY
                let button_rect = egui::Rect::from_center_size(
                    rect.center_bottom() - egui::vec2(0.0, 60.0),
                    egui::vec2(120.0, 44.0),
                );
                if ui.put(button_rect, egui::Button::new("BACK").min_size(egui::vec2(120.0, 44.0))).clicked() {
                    params.view_state.show_production_tree = false;
                    // Restore WorldViewRect on next frame naturally
                }
            });
        return; // Skip normal HUD rendering
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let r = ui.max_rect();
            params.world_view_rect.x = r.min.x;
            params.world_view_rect.y = r.min.y;
            params.world_view_rect.w = r.width();
            params.world_view_rect.h = r.height();

            // Tutorial popup as painted overlay (same pattern as Production Tree arrows)
            if let Some(popup) = params.tutorial.active.clone() {
                if !params.view_state.show_production_tree {
                    let painter = ctx.layer_painter(egui::LayerId::new(
                        egui::Order::Foreground,
                        egui::Id::new("tutorial_overlay")
                    ));
                    
                    // Center on viewport (world_view_rect) instead of screen to account for drawer
                    let viewport_center = egui::pos2(
                        params.world_view_rect.x + params.world_view_rect.w / 2.0,
                        params.world_view_rect.y + params.world_view_rect.h / 2.0
                    );
                    
                    // Background rect — centered, wider for text wrapping
                    let w = 480.0;
                    let h = 220.0;
                    let bg_rect = egui::Rect::from_center_size(
                        viewport_center,
                        egui::vec2(w, h)
                    );
                    
                    // Draw background
                    painter.rect_filled(bg_rect, 6.0, egui::Color32::from_rgba_unmultiplied(5, 5, 10, 240));
                    painter.rect_stroke(bg_rect, 6.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
                    
                    // Title — ECHO in amber
                    painter.text(
                        bg_rect.center_top() + egui::vec2(0.0, 16.0),
                        egui::Align2::CENTER_TOP,
                        &popup.title,
                        egui::FontId::proportional(13.0),
                        egui::Color32::from_rgb(180, 140, 50),
                    );
                    
                    // Body text — break on punctuation and render multiple lines
                    let body_lines: Vec<&str> = popup.body
                        .split(|c| c == '.' || c == '!' || c == '?')
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.trim())
                        .collect();
                    
                    let line_height = 18.0;
                    let start_y = bg_rect.min.y + 50.0;
                    
                    for (i, line) in body_lines.iter().enumerate() {
                        let y = start_y + (i as f32) * line_height;
                        painter.text(
                            egui::pos2(bg_rect.center().x, y),
                            egui::Align2::CENTER_TOP,
                            line,
                            egui::FontId::proportional(12.0),
                            egui::Color32::from_rgb(220, 215, 210),
                        );
                    }
                    
                    // Button rect
                    let btn_rect = egui::Rect::from_center_size(
                        bg_rect.center_bottom() - egui::vec2(0.0, 28.0),
                        egui::vec2(120.0, 32.0)
                    );
                    
                    // Draw button
                    painter.rect_filled(btn_rect, 4.0, egui::Color32::from_rgb(40, 35, 15));
                    painter.rect_stroke(btn_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
                    painter.text(
                        btn_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &popup.button_label,
                        egui::FontId::proportional(13.0),
                        egui::Color32::from_rgb(180, 140, 50),
                    );
                    
                    // Click detection — same pattern as Production Tree arrows
                    let response = ui.interact(
                        btn_rect,
                        egui::Id::new("tutorial_btn"),
                        egui::Sense::click()
                    );
                    if response.clicked() {
                        params.tutorial.shown.insert(popup.id);
                        params.tutorial.active = None;
                    }
                }
                return;
            }

            // Telemetry opt-in prompt as painted overlay (ECHO system interrupt style)
            if params.telemetry_opt_in.active && !params.view_state.show_production_tree {
                let painter = ctx.layer_painter(egui::LayerId::new(
                    egui::Order::Foreground,
                    egui::Id::new("telemetry_opt_in_overlay")
                ));
                
                // Center on viewport
                let viewport_center = egui::pos2(
                    params.world_view_rect.x + params.world_view_rect.w / 2.0,
                    params.world_view_rect.y + params.world_view_rect.h / 2.0
                );
                
                // Background rect — amber border, dark background
                let w = 420.0;
                let h = 180.0;
                let bg_rect = egui::Rect::from_center_size(
                    viewport_center,
                    egui::vec2(w, h)
                );
                
                // Draw background
                painter.rect_filled(bg_rect, 6.0, egui::Color32::from_rgba_unmultiplied(5, 5, 10, 240));
                painter.rect_stroke(bg_rect, 6.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
                
                // Text content — ECHO voice, lowercase, fixed-width, left-aligned
                let text_lines = [
                    "echo: anonymous usage data helps improve the signal.",
                    "no personal data collected. no identifiers stored.",
                    "no account required. choice can be changed in settings.",
                ];
                let mut y_offset = 30.0;
                for line in &text_lines {
                    painter.text(
                        bg_rect.min + egui::vec2(20.0, y_offset),
                        egui::Align2::LEFT_TOP,
                        line,
                        egui::FontId::monospace(13.0),
                        egui::Color32::WHITE
                    );
                    y_offset += 18.0;
                }
                
                // Allow signal button
                let allow_btn_rect = egui::Rect::from_center_size(
                    bg_rect.center_bottom() - egui::vec2(60.0, 25.0),
                    egui::vec2(110.0, 32.0)
                );
                painter.rect_filled(allow_btn_rect, 4.0, egui::Color32::from_rgb(40, 35, 15));
                painter.rect_stroke(allow_btn_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
                painter.text(
                    allow_btn_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "allow signal",
                    egui::FontId::monospace(13.0),
                    egui::Color32::from_rgb(180, 140, 50)
                );
                
                let allow_response = ui.interact(
                    allow_btn_rect,
                    egui::Id::new("telemetry_allow_btn"),
                    egui::Sense::click()
                );
                if allow_response.clicked() {
                    params.telemetry_opt_in.active = false;
                    params.telemetry_consent.opted_in = Some(true);
                    params.telemetry_session_counter.sessions = 0; // Reset counter on choice
                    // Trigger autosave
                    params.save_events.send(SaveRequestEvent {
                        name: "autosave".to_string(),
                        category: crate::systems::persistence::save::SaveCategory::Auto,
                        description: "Telemetry consent saved".to_string(),
                    });
                }
                
                // Decline button
                let decline_btn_rect = egui::Rect::from_center_size(
                    bg_rect.center_bottom() - egui::vec2(-60.0, 25.0),
                    egui::vec2(110.0, 32.0)
                );
                painter.rect_filled(decline_btn_rect, 4.0, egui::Color32::from_rgb(40, 35, 15));
                painter.rect_stroke(decline_btn_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
                painter.text(
                    decline_btn_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "decline",
                    egui::FontId::monospace(13.0),
                    egui::Color32::from_rgb(180, 140, 50)
                );
                
                let decline_response = ui.interact(
                    decline_btn_rect,
                    egui::Id::new("telemetry_decline_btn"),
                    egui::Sense::click()
                );
                if decline_response.clicked() {
                    params.telemetry_opt_in.active = false;
                    params.telemetry_consent.opted_in = Some(false);
                    params.telemetry_session_counter.sessions = 0; // Reset counter on choice
                    // Trigger autosave
                    params.save_events.send(SaveRequestEvent {
                        name: "autosave".to_string(),
                        category: crate::systems::persistence::save::SaveCategory::Auto,
                        description: "Telemetry consent saved".to_string(),
                    });
                }
                
                return;
            }

            ui.horizontal(|ui| {
                // Left: Fleet count indicator (ready/total)
                let deployed = params.autonomous_ships.iter().count();
                let total = params.queue.available_count as usize + deployed;
                ui.label(egui::RichText::new(format!("Fleet: {}/{}", params.queue.available_count, total))
                    .color(egui::Color32::from_rgb(0, 200, 200))
                    .size(16.0));

                // Push buttons to right edge using right_to_left layout
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // PIPELINE renders first but appears leftmost due to right_to_left
                    let pipeline_response = ui.add(egui::Button::new("PIPELINE").min_size(egui::vec2(80.0, 44.0)));
                    if pipeline_response.clicked() {
                        params.view_state.show_production_tree = true;
                        params.view_state.production_tree_ever_opened = true;
                    }

                    // Pipeline highlight: amber pulsing stroke when drone built and production tree never opened
                    let station = params.station_query.get_single();
                    let drone_built = station.as_ref().ok().map(|(_, _, queues)| {
                        queues.hull_forge.is_some() || queues.core_fabricator.is_some()
                    }).unwrap_or(false);
                    if drone_built && !params.view_state.production_tree_ever_opened {
                        let t = ui.ctx().input(|i| i.time as f32);
                        let alpha = ((t * 2.0).sin() * 0.3 + 0.7) * 255.0;
                        let center_rect = egui::Rect::from_center_size(pipeline_response.rect.center(), egui::vec2(200.0, pipeline_response.rect.height()));
                        let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("pipeline_highlight"));
                        let painter = ui.ctx().layer_painter(layer_id);
                        painter.rect_stroke(
                            center_rect,
                            0.0,
                            egui::Stroke::new(8.0, egui::Color32::from_rgba_unmultiplied(255, 200, 50, alpha as u8)),
                            egui::StrokeKind::Outside,
                        );
                    }

                    // SAVE renders second but appears rightmost due to right_to_left
                    if ui.add(egui::Button::new("SAVE").min_size(egui::vec2(80.0, 44.0))).clicked() {
                        params.menu_state.show_save_overlay = true;
                    }
                    // FOCUS renders third but appears left of SAVE
                    if ui.add(egui::Button::new("FOCUS").min_size(egui::vec2(80.0, 44.0))).clicked() {
                        params.pan_state.is_focused = true;
                        params.pan_state.cumulative_offset = Vec2::ZERO;
                        if let Ok(mut proj) = params.cam_query.get_single_mut() {
                            proj.scale = 1.0;
                        }
                    }
                });
            });
        });
}
