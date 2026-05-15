use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::config::{BalanceConfig, RequestConfig, LogsConfig, VisualConfig};
use crate::systems::visuals::ore_polygon::{self, OrePolygonConfig};
use crate::systems::visuals::ingot_node::{self, IngotNodeConfig};
use crate::systems::visuals::component_nodes::{self, ThrusterConfig, HullConfig, CanisterConfig, AICoreConfig, DroneBayConfig};
use crate::systems::persistence::save::SaveData;

#[derive(PartialEq, Clone, Copy)]
enum SymbolState {
    Locked,           // not yet reachable — ghost outline, alpha 0.2
    ActiveEmpty,      // unlocked, count == 0 — dim color, alpha 0.5
    ActivePopulated,  // unlocked, count > 0 — full color, alpha 1.0
}

#[derive(PartialEq, Clone, Copy)]
enum SymbolType {
    IronOre,
    TungstenOre,
    NickelOre,
    AluminumOre,
    IronIngot,
    TungstenIngot,
    NickelIngot,
    AluminumIngot,
    HullPlate,
    Thruster,
    AICore,
    Canister,
}

fn render_ore_pipeline(
    ui: &mut egui::Ui,
    ore_name: &str,
    ore_count: f32,
    ingot_name: &str,
    ingot_count: f32,
    product_name: &str,
    product_count: f32,
    ingot_ratio: f32,
    product_cost: f32,
    refine_toggle: &mut bool,
    forge_toggle: &mut bool,
) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            // ORE SECTION
            ui.heading(ore_name);
            ui.horizontal(|ui| {
                ui.label(format!("{}: {:.1}", ore_name, ore_count));
                ui.checkbox(refine_toggle, "Refine");
            });
            ui.label(format!("→ {:.1}x ingots per ore", ingot_ratio));
            ui.separator();
            
            // INGOT SECTION
            ui.heading(ingot_name);
            ui.label(format!("{}: {:.1}", ingot_name, ingot_count));
            ui.separator();
            
            // PRODUCT SECTION
            ui.heading(product_name);
            ui.horizontal(|ui| {
                ui.label(format!("{}: {:.1}", product_name, product_count));
                ui.checkbox(forge_toggle, "Forge");
            });
            ui.label(format!("← {:.1} ingots per {}", product_cost, product_name.to_lowercase()));
        });
    });
}

pub fn render_tab_content(
    ui: &mut egui::Ui,
    active_tab: ActiveStationTab,
    station: &mut Station,
    toggles: &mut ProductionToggles,
    _queue: &ShipQueue,
    prod_tab: &mut ProductionTabState,
    req_tab: &mut RequestsTabState,
    _repair_events: &mut EventWriter<RepairStationEvent>,
    fulfill_events: &mut EventWriter<FulfillRequestEvent>,
    cfg: &BalanceConfig,
    request_cfg: &RequestConfig,
    logs_cfg: &LogsConfig,
    save_data: &SaveData,
    vcfg: &VisualConfig,
    scout_enabled: &mut crate::components::resources::ScoutEnabled,
) {
    match active_tab {
        ActiveStationTab::Cargo => {
            ui.heading("CARGO BAY");
            ui.add_space(8.0);

            // Dynamic layout - calculate based on available height
            // Fixed x positions
            const X_ORE: f32 = 40.0;
            const X_INGOT: f32 = 220.0;
            const X_COMPONENT: f32 = 400.0;
            const X_ARROW_START: f32 = 428.0;
            const X_ARROW_END: f32 = 648.0;
            const X_DRONE: f32 = 660.0;
            const CONTENT_TOP: f32 = 0.0;

            // Calculate dynamic sizing based on available height
            let available_height = ui.available_height() - 10.0; // leave margin
            let row_height = available_height / 3.8; // 4 rows
            let symbol_size = (row_height * 0.38).clamp(13.0, 17.0);
            let drone_size = (row_height * 0.6).clamp(32.0, 56.0);

            // Allocate full available height
            let (rect, _response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), available_height), egui::Sense::hover());
            let painter = ui.painter_at(rect);

            let content_top_y = rect.min.y + CONTENT_TOP;

            // Row y positions - spread evenly across available height
            let row_centers = [
                content_top_y + (0.2 * row_height),
                content_top_y + (1.2 * row_height),
                content_top_y + (2.2 * row_height),
                content_top_y + (3.0 * row_height),
            ];
            
            // Drone vertical center (middle of all 4 rows)
            let drone_y = (row_centers[0] + row_centers[1] + row_centers[2]) / 3.0;

            // Row states
            let iron_state = if station.iron_reserves > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let iron_ingot_state = if station.iron_reserves == 0.0 && station.iron_ingots == 0.0 { SymbolState::Locked } else if station.iron_ingots > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let hull_plate_state = if station.iron_ingots == 0.0 { SymbolState::Locked } else if station.hull_plate_reserves > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };

            let tungsten_state = if station.tungsten_reserves > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let tungsten_ingot_state = if station.tungsten_reserves == 0.0 && station.tungsten_ingots == 0.0 { SymbolState::Locked } else if station.tungsten_ingots > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let thruster_state = if station.tungsten_ingots == 0.0 { SymbolState::Locked } else if station.thruster_reserves > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };

            let nickel_state = if station.nickel_reserves > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let nickel_ingot_state = if station.nickel_reserves == 0.0 && station.nickel_ingots == 0.0 { SymbolState::Locked } else if station.nickel_ingots > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let ai_core_state = if station.nickel_ingots == 0.0 { SymbolState::Locked } else if station.ai_cores > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };

            let aluminum_state = if station.aluminum_reserves > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let aluminum_ingot_state = if station.aluminum_reserves == 0.0 && station.aluminum_ingots == 0.0 { SymbolState::Locked } else if station.aluminum_ingots > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };
            let canister_state = if station.aluminum_ingots == 0.0 { SymbolState::Locked } else if station.aluminum_canisters > 0.0 { SymbolState::ActivePopulated } else { SymbolState::ActiveEmpty };

            // Helper to draw symbol with text
            let draw_symbol_text = |painter: &egui::Painter, x: f32, y: f32, symbol_type: SymbolType, state: SymbolState, name: &str, count: f32, vcfg: &VisualConfig, symbol_size: f32| {
                let alpha = match state {
                    SymbolState::Locked => 51,
                    SymbolState::ActiveEmpty => 128,
                    SymbolState::ActivePopulated => 255,
                };

                let symbol_pos = egui::pos2(x, y);
                let symbol_rect = egui::Rect::from_center_size(symbol_pos, egui::vec2(symbol_size, symbol_size));
                let symbol_painter = painter.with_clip_rect(symbol_rect);

                match symbol_type {
                    SymbolType::IronOre => {
                        let ore_cfg = &vcfg.production_tree.ore_node;
                        let ore_config = OrePolygonConfig {
                            radius: symbol_size * 0.4,
                            vertex_count: ore_cfg.vertex_count,
                            jaggedness: ore_cfg.jaggedness,
                            color_body: egui::Color32::from_rgba_unmultiplied(100, 100, 100, alpha),
                            color_vein: egui::Color32::from_rgba_unmultiplied(200, 100, 100, alpha),
                            band_count: 3,
                            band_width_min: 2.0,
                            band_width_max: 4.0,
                            grain_angle_deg: 30.0,
                            seed: 1,
                        };
                        ore_polygon::draw_ore_polygon(&symbol_painter, symbol_rect.center(), &ore_config, 0.8);
                    },
                    SymbolType::TungstenOre => {
                        let ore_cfg = &vcfg.production_tree.ore_node;
                        let ore_config = OrePolygonConfig {
                            radius: symbol_size * 0.4,
                            vertex_count: ore_cfg.vertex_count,
                            jaggedness: ore_cfg.jaggedness,
                            color_body: egui::Color32::from_rgba_unmultiplied(100, 100, 100, alpha),
                            color_vein: egui::Color32::from_rgba_unmultiplied(150, 150, 200, alpha),
                            band_count: 3,
                            band_width_min: 2.0,
                            band_width_max: 4.0,
                            grain_angle_deg: 30.0,
                            seed: 2,
                        };
                        ore_polygon::draw_ore_polygon(&symbol_painter, symbol_rect.center(), &ore_config, 0.8);
                    },
                    SymbolType::NickelOre => {
                        let ore_cfg = &vcfg.production_tree.ore_node;
                        let ore_config = OrePolygonConfig {
                            radius: symbol_size * 0.4,
                            vertex_count: ore_cfg.vertex_count,
                            jaggedness: ore_cfg.jaggedness,
                            color_body: egui::Color32::from_rgba_unmultiplied(100, 100, 100, alpha),
                            color_vein: egui::Color32::from_rgba_unmultiplied(180, 140, 50, alpha),
                            band_count: 3,
                            band_width_min: 2.0,
                            band_width_max: 4.0,
                            grain_angle_deg: 30.0,
                            seed: 3,
                        };
                        ore_polygon::draw_ore_polygon(&symbol_painter, symbol_rect.center(), &ore_config, 0.8);
                    },
                    SymbolType::AluminumOre => {
                        let ore_cfg = &vcfg.production_tree.ore_node;
                        let ore_config = OrePolygonConfig {
                            radius: symbol_size * 0.4,
                            vertex_count: ore_cfg.vertex_count,
                            jaggedness: ore_cfg.jaggedness,
                            color_body: egui::Color32::from_rgba_unmultiplied(100, 100, 100, alpha),
                            color_vein: egui::Color32::from_rgba_unmultiplied(200, 200, 150, alpha),
                            band_count: 3,
                            band_width_min: 2.0,
                            band_width_max: 4.0,
                            grain_angle_deg: 30.0,
                            seed: 4,
                        };
                        ore_polygon::draw_ore_polygon(&symbol_painter, symbol_rect.center(), &ore_config, 0.8);
                    },
                    SymbolType::IronIngot => {
                        let ingot_config = IngotNodeConfig {
                            width: symbol_size * 0.7,
                            height: symbol_size * 0.7,
                            depth_offset_x: symbol_size * 0.15,
                            depth_offset_y: symbol_size * 0.15,
                            color_face_light_factor: 1.0,
                            color_face_dark_factor: 0.7,
                        };
                        let base_color = egui::Color32::from_rgba_unmultiplied(150, 100, 100, alpha);
                        ingot_node::draw_ingot_node(&symbol_painter, symbol_rect.center(), &ingot_config, base_color);
                    },
                    SymbolType::TungstenIngot => {
                        let ingot_config = IngotNodeConfig {
                            width: symbol_size * 0.7,
                            height: symbol_size * 0.7,
                            depth_offset_x: symbol_size * 0.15,
                            depth_offset_y: symbol_size * 0.15,
                            color_face_light_factor: 1.0,
                            color_face_dark_factor: 0.7,
                        };
                        let base_color = egui::Color32::from_rgba_unmultiplied(130, 130, 180, alpha);
                        ingot_node::draw_ingot_node(&symbol_painter, symbol_rect.center(), &ingot_config, base_color);
                    },
                    SymbolType::NickelIngot => {
                        let ingot_config = IngotNodeConfig {
                            width: symbol_size * 0.7,
                            height: symbol_size * 0.7,
                            depth_offset_x: symbol_size * 0.15,
                            depth_offset_y: symbol_size * 0.15,
                            color_face_light_factor: 1.0,
                            color_face_dark_factor: 0.7,
                        };
                        let base_color = egui::Color32::from_rgba_unmultiplied(180, 150, 80, alpha);
                        ingot_node::draw_ingot_node(&symbol_painter, symbol_rect.center(), &ingot_config, base_color);
                    },
                    SymbolType::AluminumIngot => {
                        let ingot_config = IngotNodeConfig {
                            width: symbol_size * 0.7,
                            height: symbol_size * 0.7,
                            depth_offset_x: symbol_size * 0.15,
                            depth_offset_y: symbol_size * 0.15,
                            color_face_light_factor: 1.0,
                            color_face_dark_factor: 0.7,
                        };
                        let base_color = egui::Color32::from_rgba_unmultiplied(180, 180, 140, alpha);
                        ingot_node::draw_ingot_node(&symbol_painter, symbol_rect.center(), &ingot_config, base_color);
                    },
                    SymbolType::HullPlate => {
                        let hull_cfg = &vcfg.component.hull;
                        let hull_config = HullConfig {
                            width: symbol_size * 0.8,
                            rib_count: hull_cfg.rib_count,
                            color_frame: egui::Color32::from_rgba_unmultiplied(150, 100, 100, alpha),
                            color_outline: egui::Color32::from_rgba_unmultiplied(200, 130, 130, alpha),
                            stroke_width: 1.5,
                        };
                        component_nodes::draw_hull(&symbol_painter, symbol_rect.center(), &hull_config);
                    },
                    SymbolType::Thruster => {
                        let thruster_cfg = &vcfg.component.thruster;
                        let thruster_config = ThrusterConfig {
                            width: symbol_size * 0.8,
                            color_nozzle: egui::Color32::from_rgba_unmultiplied(100, 100, 150, alpha),
                            color_body: egui::Color32::from_rgba_unmultiplied(130, 130, 180, alpha),
                            color_wire: egui::Color32::from_rgba_unmultiplied(200, 50, 50, alpha),
                            wire_count: thruster_cfg.wire_count,
                            nozzle_width_ratio: thruster_cfg.nozzle_width_ratio,
                            body_width_ratio: thruster_cfg.body_width_ratio,
                        };
                        component_nodes::draw_thruster(&symbol_painter, symbol_rect.center(), &thruster_config);
                    },
                    SymbolType::AICore => {
                        let ai_core_cfg = &vcfg.component.ai_core;
                        let ai_core_config = AICoreConfig {
                            radius: symbol_size * 0.4,
                            fin_count: ai_core_cfg.fin_count,
                            fin_length: ai_core_cfg.fin_length,
                            fin_width: ai_core_cfg.fin_width,
                            color_body: egui::Color32::from_rgba_unmultiplied(180, 150, 80, alpha),
                            color_fins: egui::Color32::from_rgba_unmultiplied(200, 170, 100, alpha),
                            color_fan_housing: egui::Color32::from_rgba_unmultiplied(100, 100, 100, alpha),
                            fan_radius_ratio: ai_core_cfg.fan_radius_ratio,
                            fan_blade_count: ai_core_cfg.fan_blade_count,
                        };
                        component_nodes::draw_ai_core(&symbol_painter, symbol_rect.center(), &ai_core_config);
                    },
                    SymbolType::Canister => {
                        let canister_cfg = &vcfg.component.canister;
                        let canister_config = CanisterConfig {
                            width: symbol_size * 0.6,
                            height: symbol_size * 0.8,
                            lid_height_ratio: canister_cfg.lid_height_ratio,
                            color_body: egui::Color32::from_rgba_unmultiplied(180, 180, 140, alpha),
                            color_lid: egui::Color32::from_rgba_unmultiplied(150, 150, 110, alpha),
                            color_highlight: egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                            color_handle: egui::Color32::from_rgba_unmultiplied(100, 100, 80, alpha),
                        };
                        component_nodes::draw_canister(&symbol_painter, symbol_rect.center(), &canister_config);
                    },
                }

                let name_pos = egui::pos2(x, y + symbol_size / 2.0 + 6.0);
                painter.text(name_pos, egui::Align2::CENTER_TOP, name, egui::FontId::proportional(10.0), egui::Color32::from_rgba_unmultiplied(0, 200, 200, alpha));

                let count_pos = egui::pos2(x, y + symbol_size / 2.0 + 14.0);
                painter.text(count_pos, egui::Align2::CENTER_TOP, &format!("{}", count.floor()), egui::FontId::proportional(10.0), egui::Color32::from_rgba_unmultiplied(0, 204, 102, alpha));
            };

            // Row 1: Iron Ore → Iron Ingot → Hull Plate
            let row_y = row_centers[0];
            draw_symbol_text(&painter, X_ORE, row_y, SymbolType::IronOre, iron_state, "Iron Ore", station.iron_reserves, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ORE + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_INGOT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_INGOT, row_y, SymbolType::IronIngot, iron_ingot_state, "Iron Ingot", station.iron_ingots, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_INGOT + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_COMPONENT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_COMPONENT, row_y, SymbolType::HullPlate, hull_plate_state, "Hull Plate", station.hull_plate_reserves, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ARROW_START, row_y), egui::pos2(X_ARROW_END, drone_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            painter.add(egui::Shape::convex_polygon(vec![egui::pos2(X_ARROW_END, drone_y), egui::pos2(X_ARROW_END - 6.0, drone_y - 3.0), egui::pos2(X_ARROW_END - 6.0, drone_y + 3.0)], egui::Color32::from_rgb(0, 200, 200), egui::Stroke::NONE));

            // Row 2: Tungsten Ore → Tungsten Ingot → Thruster
            let row_y = row_centers[1];
            draw_symbol_text(&painter, X_ORE, row_y, SymbolType::TungstenOre, tungsten_state, "Tungsten Ore", station.tungsten_reserves, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ORE + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_INGOT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_INGOT, row_y, SymbolType::TungstenIngot, tungsten_ingot_state, "Tungsten Ingot", station.tungsten_ingots, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_INGOT + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_COMPONENT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_COMPONENT, row_y, SymbolType::Thruster, thruster_state, "Thruster", station.thruster_reserves, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ARROW_START, row_y), egui::pos2(X_ARROW_END, drone_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            painter.add(egui::Shape::convex_polygon(vec![egui::pos2(X_ARROW_END, drone_y), egui::pos2(X_ARROW_END - 6.0, drone_y - 3.0), egui::pos2(X_ARROW_END - 6.0, drone_y + 3.0)], egui::Color32::from_rgb(0, 200, 200), egui::Stroke::NONE));

            // Row 3: Nickel Ore → Nickel Ingot → AI Core
            let row_y = row_centers[2];
            draw_symbol_text(&painter, X_ORE, row_y, SymbolType::NickelOre, nickel_state, "Nickel Ore", station.nickel_reserves, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ORE + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_INGOT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_INGOT, row_y, SymbolType::NickelIngot, nickel_ingot_state, "Nickel Ingot", station.nickel_ingots, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_INGOT + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_COMPONENT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_COMPONENT, row_y, SymbolType::AICore, ai_core_state, "AI Core", station.ai_cores, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ARROW_START, row_y), egui::pos2(X_ARROW_END, drone_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            painter.add(egui::Shape::convex_polygon(vec![egui::pos2(X_ARROW_END, drone_y), egui::pos2(X_ARROW_END - 6.0, drone_y - 3.0), egui::pos2(X_ARROW_END - 6.0, drone_y + 3.0)], egui::Color32::from_rgb(0, 200, 200), egui::Stroke::NONE));

            // Row 4: Aluminum Ore → Aluminum Ingot → Canister
            let row_y = row_centers[3];
            draw_symbol_text(&painter, X_ORE, row_y, SymbolType::AluminumOre, aluminum_state, "Aluminum Ore", station.aluminum_reserves, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_ORE + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_INGOT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_INGOT, row_y, SymbolType::AluminumIngot, aluminum_ingot_state, "Aluminum Ingot", station.aluminum_ingots, vcfg, symbol_size);
            painter.line_segment([egui::pos2(X_INGOT + symbol_size / 2.0 + 8.0, row_y), egui::pos2(X_COMPONENT - symbol_size / 2.0 - 8.0, row_y)], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 200, 200)));
            draw_symbol_text(&painter, X_COMPONENT, row_y, SymbolType::Canister, canister_state, "Canister", station.aluminum_canisters, vcfg, symbol_size);

            // Single drone at x=660, dynamic y
            let drone_pos = egui::pos2(X_DRONE, drone_y);
            let drone_rect = egui::Rect::from_center_size(drone_pos, egui::vec2(drone_size, drone_size));
            let drone_painter = painter.with_clip_rect(drone_rect);
            let drone_bay_cfg = &vcfg.component.drone_bay;
            let drone_bay_config = DroneBayConfig {
                width: drone_size * 0.8,
                height: drone_size * 0.8,
                color_ready: egui::Color32::from_rgb(0, 200, 100),
                color_empty: egui::Color32::from_rgb(100, 100, 100),
                nose_height_ratio: drone_bay_cfg.nose_height_ratio,
                fin_width_ratio: drone_bay_cfg.fin_width_ratio,
                fin_height_ratio: drone_bay_cfg.fin_height_ratio,
                porthole_radius: drone_bay_cfg.porthole_radius,
                porthole_offset_y: drone_bay_cfg.porthole_offset_y,
                exhaust_radius: drone_bay_cfg.exhaust_radius,
            };
            let drone_is_ready = station.drone_count > 0;
            component_nodes::draw_drone_bay(&drone_painter, drone_rect.center(), &drone_bay_config, drone_is_ready);

            painter.text(egui::pos2(X_DRONE, drone_pos.y + drone_size / 2.0 + 4.0), egui::Align2::CENTER_TOP, "Drone Bay", egui::FontId::proportional(12.0), egui::Color32::from_rgb(0, 200, 200));
            painter.text(egui::pos2(X_DRONE, drone_pos.y + drone_size / 2.0 + 20.0), egui::Align2::CENTER_TOP, &format!("Fleet: {}", station.drone_count), egui::FontId::proportional(12.0), egui::Color32::from_rgb(0, 204, 102));
        }
        ActiveStationTab::Hangar => {
            ui.heading("HANGAR");
            ui.add_space(8.0);
            
            // Scout Mk I toggle row
            if scout_enabled.unlocked {
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("SCOUT Mk I").size(14.0).color(egui::Color32::from_rgb(0, 200, 200)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn_text = if scout_enabled.active { "ON" } else { "OFF" };
                        let btn_color = if scout_enabled.active { egui::Color32::from_rgb(0, 204, 102) } else { egui::Color32::from_rgb(100, 100, 100) };
                        let btn_pos = ui.cursor().right_bottom();
                        let btn_rect = egui::Rect::from_min_max(btn_pos, btn_pos + egui::vec2(40.0, 24.0));
                        let response = ui.interact(btn_rect, egui::Id::new("scout_toggle"), egui::Sense::click());
                        if response.clicked() {
                            scout_enabled.active = !scout_enabled.active;
                        }
                        ui.painter().rect_filled(btn_rect, 0.0, btn_color);
                        ui.painter().text(btn_rect.center(), egui::Align2::CENTER_CENTER, btn_text, egui::FontId::proportional(12.0), egui::Color32::WHITE);
                        ui.add_space(50.0); // Spacer before label
                    });
                    ui.label(egui::RichText::new("Auto-dispatch to Inner Ring").size(12.0).color(egui::Color32::from_gray(160)));
                });
                ui.add_space(4.0);
            }
        }
        ActiveStationTab::Production => {
            ui.heading("FORGE");
            ui.add_space(8.0);
            
            egui::ComboBox::from_id_salt("ore_combo")
                .selected_text(match prod_tab.selected_ore {
                    OreType::Iron => "Iron Pipeline",
                    OreType::Tungsten => "Tungsten Pipeline",
                    OreType::Nickel => "Nickel Pipeline",
                    OreType::Aluminum => "Aluminum Pipeline",
                })
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut prod_tab.selected_ore, OreType::Iron, "Iron Pipeline").on_hover_text("Process Iron");
                    ui.selectable_value(&mut prod_tab.selected_ore, OreType::Tungsten, "Tungsten Pipeline").on_hover_text("Process Tungsten");
                    ui.selectable_value(&mut prod_tab.selected_ore, OreType::Nickel, "Nickel Pipeline").on_hover_text("Process Nickel");
                    ui.selectable_value(&mut prod_tab.selected_ore, OreType::Aluminum, "Aluminum Pipeline").on_hover_text("Process Aluminum");
                });
            
            ui.add_space(16.0);
            
            match prod_tab.selected_ore {
                OreType::Iron => {
                    render_ore_pipeline(ui, "IRON ORE", station.iron_reserves, "IRON INGOTS", station.iron_ingots, "HULLS", station.hull_plate_reserves, 1.0 / cfg.refinery.ratio as f32, cfg.forge.hull_plate_cost_iron as f32, &mut toggles.refine_iron, &mut toggles.forge_hull);
                }
                OreType::Tungsten => {
                    render_ore_pipeline(ui, "TUNGSTEN ORE", station.tungsten_reserves, "TUNGSTEN INGOTS", station.tungsten_ingots, "THRUSTERS", station.thruster_reserves, 1.0 / cfg.refinery.ratio as f32, cfg.forge.thruster_cost_tungsten as f32, &mut toggles.refine_tungsten, &mut toggles.forge_thruster);
                }
                OreType::Nickel => {
                    render_ore_pipeline(ui, "NICKEL ORE", station.nickel_reserves, "NICKEL INGOTS", station.nickel_ingots, "AI CORES", station.ai_cores, 1.0 / cfg.refinery.ratio as f32, cfg.forge.ai_core_cost_nickel as f32, &mut toggles.refine_nickel, &mut toggles.forge_core);
                }
                OreType::Aluminum => {
                    render_ore_pipeline(ui, "ALUMINUM ORE", station.aluminum_reserves, "ALUMINUM INGOTS", station.aluminum_ingots, "CANISTERS", station.aluminum_canisters, 1.0 / cfg.refinery.ratio as f32, cfg.forge.aluminum_canister_cost_aluminum as f32, &mut toggles.refine_aluminum, &mut toggles.forge_aluminum_canister);
                }
            }
        }
        ActiveStationTab::Requests => {
            ui.heading("SIGNAL DECRYPTION");
            ui.add_space(8.0);
            
            egui::ComboBox::from_id_salt("faction_combo")
                .selected_text("FACTION: SIGNAL")
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut req_tab.selected_faction, FactionId::Signal, "FACTION: SIGNAL");
                });
                
            ui.add_space(16.0);
            
            let mut filtered: Vec<&mut CollectedRequest> = req_tab.collected_requests.iter_mut().filter(|r| r.faction == req_tab.selected_faction).collect();
            
            if filtered.is_empty() {
                ui.label(egui::RichText::new("No signals received.").color(egui::Color32::GRAY));
                ui.label(egui::RichText::new("Something may be out there.").color(egui::Color32::GRAY).italics());
            } else {
                for req in filtered.iter_mut().rev() {
                    // Look up request definition from config
                    if let Some(req_def) = request_cfg.faction_requests.iter().find(|r| r.id == format!("{:?}", req.id)) {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.heading(egui::RichText::new(&req_def.title).color(egui::Color32::GOLD));
                                ui.label(egui::RichText::new(&req_def.flavor).italics());
                                ui.add_space(8.0);
                                
                                // Build requirements string
                                for req_item in &req_def.requirements {
                                    ui.label(format!("Requires: {:.0} {}", req_item.amount, req_item.resource));
                                }
                                
                                // Build rewards string
                                for reward in &req_def.rewards {
                                    ui.label(egui::RichText::new(format!("Reward: {} {:.2}", reward.r#type, reward.value)).color(egui::Color32::CYAN));
                                }
                                
                                ui.add_space(8.0);
                                
                                if req.fulfilled {
                                    ui.label(egui::RichText::new("COMPLETE").strong().color(egui::Color32::GREEN));
                                } else {
                                    // Check if player can afford (simplified for FirstLight: iron_ingots >= 25)
                                    let can_afford = req_def.requirements.iter().all(|r| {
                                        match r.resource.as_str() {
                                            "iron_ingots" => station.iron_ingots >= r.amount,
                                            _ => false,
                                        }
                                    });
                                    
                                    if ui.add_enabled(can_afford, egui::Button::new("FULFILL").min_size(egui::vec2(120.0, 44.0))).clicked() {
                                        fulfill_events.send(FulfillRequestEvent {
                                            request_id: req.id,
                                            faction_id: req.faction,
                                        });
                                    }
                                }
                            });
                        });
                        ui.add_space(8.0);
                    }
                }
            }
        }
        ActiveStationTab::Logs => {
            ui.heading("SIGNAL LOGS");
            ui.add_space(8.0);

            let unlocked_logs: Vec<_> = logs_cfg.logs.iter()
                .filter(|log| save_data.unlocked_logs.contains(&log.id))
                .collect();

            if unlocked_logs.is_empty() {
                ui.label(egui::RichText::new("No signals received.").color(egui::Color32::GRAY));
            } else {
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        for log in unlocked_logs {
                            ui.add_space(12.0);
                            ui.label(egui::RichText::new(&log.title)
                                .color(egui::Color32::from_rgb(180, 140, 50))
                                .size(14.0));
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(&log.body)
                                .color(egui::Color32::from_rgb(220, 215, 210))
                                .size(12.0));
                            ui.add_space(8.0);
                            ui.separator();
                        }
                    });
            }
        }
    }
}
