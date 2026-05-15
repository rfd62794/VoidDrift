pub mod state_machine;
pub mod content;
pub mod prod_tree;
pub mod overlays;
pub mod buttons;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::components::resources::{MaxDispatch, ProdTreeViewState};
use crate::scenes::main_menu::MainMenuState;
use crate::config::{BalanceConfig, VisualConfig};
use crate::config::visual::rgb;
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
    pub prod_tree_view_state: ResMut<'w, ProdTreeViewState>,
    pub telemetry_opt_in: ResMut<'w, TelemetryOptInPrompt>,
    pub telemetry_consent: ResMut<'w, TelemetryConsent>,
    pub telemetry_session_counter: ResMut<'w, TelemetrySessionCounter>,
    pub scout_enabled: ResMut<'w, crate::components::resources::ScoutEnabled>,
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
        let signal_height = if params.expanded.0 { layout.signal_height * 1.0 } else { layout.signal_height };
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

        // Calculate dynamic content height based on window height
        // Use fixed 200px height for drawer content
        let content_height = 250.0;

        egui::TopBottomPanel::bottom("content_area")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(8, 10, 16))
                .inner_margin(egui::Margin::symmetric(8, 8)))
            .exact_height(content_height)
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
                        &params.visual_cfg,
                        &mut params.scout_enabled,
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
                let tab_w = ui.available_width() / 5.0;
                let tab_size = egui::vec2(tab_w, layout.secondary_tab_height - 8.0);
                ui.horizontal(|ui| {
                    for (tab, label) in [
                        (ActiveStationTab::Cargo,      "CARGO"),
                        (ActiveStationTab::Hangar,     "HANGAR"),
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
                prod_tree::render_production_tree(
                    ui,
                    &mut params.station_query,
                    &mut params.toggles,
                    &params.visual_cfg,
                    &mut params.world_view_rect,
                    &mut params.view_state,
                    &mut params.prod_tree_view_state,
                );
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

            overlays::render_overlays(
                ui,
                ctx,
                &mut params.tutorial,
                &mut params.telemetry_opt_in,
                &mut params.telemetry_consent,
                &mut params.telemetry_session_counter,
                &mut params.save_events,
                &params.world_view_rect,
                &params.view_state,
            );

            buttons::render_hud_buttons(
                ui,
                &params.autonomous_ships,
                &params.queue,
                &params.station_query,
                &mut params.view_state,
                &mut params.menu_state,
                &mut params.pan_state,
                &mut params.cam_query,
            );
        });
}
