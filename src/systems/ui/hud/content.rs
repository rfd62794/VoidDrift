use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;
use crate::config::{BalanceConfig, RequestConfig, LogsConfig, VisualConfig};
use crate::config::visual::{rgb_u8_to_egui};
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
    DroneBay,
}

fn draw_chain_column(
    ui: &mut egui::Ui,
    symbol_type: SymbolType,
    name: &str,
    count: f32,
    state: SymbolState,
    size: f32,
    vcfg: &VisualConfig,
) {
    ui.vertical_centered(|ui| {
        // Determine alpha based on state
        let alpha = match state {
            SymbolState::Locked => 51,
            SymbolState::ActiveEmpty => 128,
            SymbolState::ActivePopulated => 255,
        };

        // Draw symbol
        let desired_size = egui::vec2(size, size);
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Draw procedural visual
        match symbol_type {
            SymbolType::IronOre => {
                let ore_cfg = &vcfg.production_tree.ore_node;
                let mut ore_config = OrePolygonConfig {
                    radius: size * 0.4,
                    vertex_count: ore_cfg.vertex_count,
                    jaggedness: ore_cfg.jaggedness,
                    color_body: rgb_u8_to_egui(vcfg.ore.metal.color_body),
                    color_vein: rgb_u8_to_egui(vcfg.ore.metal.color_vein),
                    band_count: 5,
                    band_width_min: 0.02,
                    band_width_max: 0.05,
                    grain_angle_deg: 45.0,
                    seed: 1u64,
                };
                ore_config.color_body = egui::Color32::from_rgba_unmultiplied(ore_config.color_body.r(), ore_config.color_body.g(), ore_config.color_body.b(), alpha);
                ore_config.color_vein = egui::Color32::from_rgba_unmultiplied(ore_config.color_vein.r(), ore_config.color_vein.g(), ore_config.color_vein.b(), alpha);
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config, 0.8);
            }
            SymbolType::TungstenOre => {
                let ore_cfg = &vcfg.production_tree.ore_node;
                let mut ore_config = OrePolygonConfig {
                    radius: size * 0.4,
                    vertex_count: ore_cfg.vertex_count,
                    jaggedness: ore_cfg.jaggedness,
                    color_body: rgb_u8_to_egui(vcfg.ore.h3_gas.color_body),
                    color_vein: rgb_u8_to_egui(vcfg.ore.h3_gas.color_vein),
                    band_count: 5,
                    band_width_min: 0.02,
                    band_width_max: 0.05,
                    grain_angle_deg: 45.0,
                    seed: 2u64,
                };
                ore_config.color_body = egui::Color32::from_rgba_unmultiplied(ore_config.color_body.r(), ore_config.color_body.g(), ore_config.color_body.b(), alpha);
                ore_config.color_vein = egui::Color32::from_rgba_unmultiplied(ore_config.color_vein.r(), ore_config.color_vein.g(), ore_config.color_vein.b(), alpha);
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config, 0.8);
            }
            SymbolType::NickelOre => {
                let ore_cfg = &vcfg.production_tree.ore_node;
                let mut ore_config = OrePolygonConfig {
                    radius: size * 0.4,
                    vertex_count: ore_cfg.vertex_count,
                    jaggedness: ore_cfg.jaggedness,
                    color_body: rgb_u8_to_egui(vcfg.ore.void_essence.color_body),
                    color_vein: rgb_u8_to_egui(vcfg.ore.void_essence.color_vein),
                    band_count: 5,
                    band_width_min: 0.02,
                    band_width_max: 0.05,
                    grain_angle_deg: 45.0,
                    seed: 3u64,
                };
                ore_config.color_body = egui::Color32::from_rgba_unmultiplied(ore_config.color_body.r(), ore_config.color_body.g(), ore_config.color_body.b(), alpha);
                ore_config.color_vein = egui::Color32::from_rgba_unmultiplied(ore_config.color_vein.r(), ore_config.color_vein.g(), ore_config.color_vein.b(), alpha);
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config, 0.8);
            }
            SymbolType::AluminumOre => {
                let ore_cfg = &vcfg.production_tree.ore_node;
                let mut ore_config = OrePolygonConfig {
                    radius: size * 0.4,
                    vertex_count: ore_cfg.vertex_count,
                    jaggedness: ore_cfg.jaggedness,
                    color_body: rgb_u8_to_egui(vcfg.ore.metal.color_body),
                    color_vein: rgb_u8_to_egui(vcfg.ore.metal.color_vein),
                    band_count: 5,
                    band_width_min: 0.02,
                    band_width_max: 0.05,
                    grain_angle_deg: 45.0,
                    seed: 4u64,
                };
                ore_config.color_body = egui::Color32::from_rgba_unmultiplied(ore_config.color_body.r(), ore_config.color_body.g(), ore_config.color_body.b(), alpha);
                ore_config.color_vein = egui::Color32::from_rgba_unmultiplied(ore_config.color_vein.r(), ore_config.color_vein.g(), ore_config.color_vein.b(), alpha);
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config, 0.8);
            }
            SymbolType::IronIngot | SymbolType::TungstenIngot | SymbolType::NickelIngot | SymbolType::AluminumIngot => {
                let base_color = match symbol_type {
                    SymbolType::IronIngot => rgb_u8_to_egui(vcfg.ore.metal.color_vein),
                    SymbolType::TungstenIngot => rgb_u8_to_egui(vcfg.ore.h3_gas.color_vein),
                    SymbolType::NickelIngot => rgb_u8_to_egui(vcfg.ore.void_essence.color_vein),
                    SymbolType::AluminumIngot => rgb_u8_to_egui(vcfg.ore.metal.color_vein),
                    _ => egui::Color32::WHITE,
                };
                let base_color = egui::Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), alpha);
                painter.rect_filled(rect, 2.0, base_color);
            }
            SymbolType::HullPlate => {
                let hull_cfg = &vcfg.component.hull;
                let mut hull_config = HullConfig {
                    width: size * 0.8,
                    rib_count: hull_cfg.rib_count,
                    color_frame: rgb_u8_to_egui(hull_cfg.color_frame),
                    color_outline: rgb_u8_to_egui(hull_cfg.color_outline),
                    stroke_width: hull_cfg.stroke_width,
                };
                hull_config.color_frame = egui::Color32::from_rgba_unmultiplied(hull_config.color_frame.r(), hull_config.color_frame.g(), hull_config.color_frame.b(), alpha);
                hull_config.color_outline = egui::Color32::from_rgba_unmultiplied(hull_config.color_outline.r(), hull_config.color_outline.g(), hull_config.color_outline.b(), alpha);
                component_nodes::draw_hull(&painter, rect.center(), &hull_config);
            }
            SymbolType::Thruster => {
                let thruster_cfg = &vcfg.component.thruster;
                let mut thruster_config = ThrusterConfig {
                    width: size * 0.8,
                    color_nozzle: rgb_u8_to_egui(thruster_cfg.color_nozzle),
                    color_body: rgb_u8_to_egui(thruster_cfg.color_body),
                    color_wire: rgb_u8_to_egui(thruster_cfg.color_wire),
                    wire_count: thruster_cfg.wire_count,
                    nozzle_width_ratio: thruster_cfg.nozzle_width_ratio,
                    body_width_ratio: thruster_cfg.body_width_ratio,
                };
                thruster_config.color_nozzle = egui::Color32::from_rgba_unmultiplied(thruster_config.color_nozzle.r(), thruster_config.color_nozzle.g(), thruster_config.color_nozzle.b(), alpha);
                thruster_config.color_body = egui::Color32::from_rgba_unmultiplied(thruster_config.color_body.r(), thruster_config.color_body.g(), thruster_config.color_body.b(), alpha);
                thruster_config.color_wire = egui::Color32::from_rgba_unmultiplied(thruster_config.color_wire.r(), thruster_config.color_wire.g(), thruster_config.color_wire.b(), alpha);
                component_nodes::draw_thruster(&painter, rect.center(), &thruster_config);
            }
            SymbolType::AICore => {
                let ai_core_cfg = &vcfg.component.ai_core;
                let mut ai_core_config = AICoreConfig {
                    radius: size * 0.4,
                    fin_count: ai_core_cfg.fin_count,
                    fin_length: ai_core_cfg.fin_length,
                    fin_width: ai_core_cfg.fin_width,
                    color_body: rgb_u8_to_egui(ai_core_cfg.color_body),
                    color_fins: rgb_u8_to_egui(ai_core_cfg.color_fins),
                    color_fan_housing: rgb_u8_to_egui(ai_core_cfg.color_fan_housing),
                    fan_radius_ratio: ai_core_cfg.fan_radius_ratio,
                    fan_blade_count: ai_core_cfg.fan_blade_count,
                };
                ai_core_config.color_body = egui::Color32::from_rgba_unmultiplied(ai_core_config.color_body.r(), ai_core_config.color_body.g(), ai_core_config.color_body.b(), alpha);
                ai_core_config.color_fins = egui::Color32::from_rgba_unmultiplied(ai_core_config.color_fins.r(), ai_core_config.color_fins.g(), ai_core_config.color_fins.b(), alpha);
                ai_core_config.color_fan_housing = egui::Color32::from_rgba_unmultiplied(ai_core_config.color_fan_housing.r(), ai_core_config.color_fan_housing.g(), ai_core_config.color_fan_housing.b(), alpha);
                component_nodes::draw_ai_core(&painter, rect.center(), &ai_core_config);
            }
            SymbolType::Canister => {
                let canister_cfg = &vcfg.component.canister;
                let mut canister_config = CanisterConfig {
                    width: size * 0.5,
                    height: size * 0.7,
                    lid_height_ratio: canister_cfg.lid_height_ratio,
                    color_body: rgb_u8_to_egui(canister_cfg.color_body),
                    color_lid: rgb_u8_to_egui(canister_cfg.color_lid),
                    color_highlight: rgb_u8_to_egui(canister_cfg.color_highlight),
                    color_handle: rgb_u8_to_egui(canister_cfg.color_handle),
                };
                canister_config.color_body = egui::Color32::from_rgba_unmultiplied(canister_config.color_body.r(), canister_config.color_body.g(), canister_config.color_body.b(), alpha);
                canister_config.color_lid = egui::Color32::from_rgba_unmultiplied(canister_config.color_lid.r(), canister_config.color_lid.g(), canister_config.color_lid.b(), alpha);
                canister_config.color_highlight = egui::Color32::from_rgba_unmultiplied(canister_config.color_highlight.r(), canister_config.color_highlight.g(), canister_config.color_highlight.b(), alpha);
                canister_config.color_handle = egui::Color32::from_rgba_unmultiplied(canister_config.color_handle.r(), canister_config.color_handle.g(), canister_config.color_handle.b(), alpha);
                component_nodes::draw_canister(&painter, rect.center(), &canister_config);
            }
            SymbolType::DroneBay => {
                let drone_bay_cfg = &vcfg.component.drone_bay;
                let mut drone_bay_config = DroneBayConfig {
                    width: size * 0.8,
                    height: size * 0.6,
                    color_ready: rgb_u8_to_egui(drone_bay_cfg.color_ready),
                    color_empty: rgb_u8_to_egui(drone_bay_cfg.color_empty),
                    nose_height_ratio: drone_bay_cfg.nose_height_ratio,
                    fin_width_ratio: drone_bay_cfg.fin_width_ratio,
                    fin_height_ratio: drone_bay_cfg.fin_height_ratio,
                    porthole_radius: drone_bay_cfg.porthole_radius,
                    porthole_offset_y: drone_bay_cfg.porthole_offset_y,
                    exhaust_radius: drone_bay_cfg.exhaust_radius,
                };
                let is_ready = count > 0.0;
                let color = if is_ready { drone_bay_config.color_ready } else { drone_bay_config.color_empty };
                let color = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
                drone_bay_config.color_ready = color;
                drone_bay_config.color_empty = color;
                component_nodes::draw_drone_bay(&painter, rect.center(), &drone_bay_config, is_ready);
            }
        }

        // Draw full name below symbol
        let name_color = match state {
            SymbolState::Locked => egui::Color32::from_rgba_unmultiplied(0, 200, 200, 51),
            SymbolState::ActiveEmpty => egui::Color32::from_rgba_unmultiplied(0, 200, 200, 128),
            SymbolState::ActivePopulated => egui::Color32::from_rgba_unmultiplied(0, 200, 200, 255),
        };
        ui.label(egui::RichText::new(name).color(name_color).size(11.0));

        // Draw count below name
        let count_text = match state {
            SymbolState::Locked => String::new(),
            SymbolState::ActiveEmpty => "0".to_string(),
            SymbolState::ActivePopulated => format!("{:.0}", count),
        };
        let count_color = match state {
            SymbolState::Locked => egui::Color32::TRANSPARENT,
            SymbolState::ActiveEmpty => egui::Color32::from_rgba_unmultiplied(0, 204, 102, 102),
            SymbolState::ActivePopulated => egui::Color32::from_rgb(0, 204, 102),
        };
        if !count_text.is_empty() {
            ui.label(egui::RichText::new(count_text).color(count_color).size(12.0).strong());
        }
    });
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
    repair_events: &mut EventWriter<RepairStationEvent>,
    fulfill_events: &mut EventWriter<FulfillRequestEvent>,
    cfg: &BalanceConfig,
    request_cfg: &RequestConfig,
    logs_cfg: &LogsConfig,
    save_data: &SaveData,
    vcfg: &VisualConfig,
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
            let symbol_size = (row_height * 0.4).clamp(14.0, 19.0);
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
                content_top_y + (3.2 * row_height),
            ];
            
            // Drone vertical center (middle of all 4 rows)
            let drone_y = content_top_y + (2.0 * row_height);

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
                    _ => {},
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
                    render_ore_pipeline(ui, "IRON ORE", station.iron_reserves, "IRON INGOTS", station.iron_ingots, "HULLS", station.hull_plate_reserves, 1.0 / REFINERY_RATIO as f32, HULL_PLATE_COST_IRON as f32, &mut toggles.refine_iron, &mut toggles.forge_hull);
                }
                OreType::Tungsten => {
                    render_ore_pipeline(ui, "TUNGSTEN ORE", station.tungsten_reserves, "TUNGSTEN INGOTS", station.tungsten_ingots, "THRUSTERS", station.thruster_reserves, 1.0 / REFINERY_RATIO as f32, THRUSTER_COST_TUNGSTEN as f32, &mut toggles.refine_tungsten, &mut toggles.forge_thruster);
                }
                OreType::Nickel => {
                    render_ore_pipeline(ui, "NICKEL ORE", station.nickel_reserves, "NICKEL INGOTS", station.nickel_ingots, "AI CORES", station.ai_cores, 1.0 / REFINERY_RATIO as f32, AI_CORE_COST_NICKEL as f32, &mut toggles.refine_nickel, &mut toggles.forge_core);
                }
                OreType::Aluminum => {
                    render_ore_pipeline(ui, "ALUMINUM ORE", station.aluminum_reserves, "ALUMINUM INGOTS", station.aluminum_ingots, "CANISTERS", station.aluminum_canisters, 1.0 / REFINERY_RATIO as f32, ALUMINUM_CANISTER_COST_ALUMINUM as f32, &mut toggles.refine_aluminum, &mut toggles.forge_aluminum_canister);
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
