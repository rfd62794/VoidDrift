use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;
use crate::config::{BalanceConfig, RequestConfig, LogsConfig, VisualConfig};
use crate::config::visual::{rgb_u8_to_egui};
use crate::systems::visuals::ore_polygon::{self, OrePolygonConfig};
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
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config);
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
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config);
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
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config);
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
                ore_polygon::draw_ore_polygon(&painter, rect.center(), &ore_config);
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
            ui.vertical(|ui| {
                ui.heading("CARGO BAY");
                ui.add_space(8.0);

                // Chain row layout
                const SYMBOL_SIZE: f32 = 32.0;

                // Row 1: Iron / Iron Ingot / Hull Plate
                ui.horizontal(|ui| {
                    let col_width = ui.available_width() / 3.0;

                    // Iron Ore
                    ui.set_width(col_width);
                    let iron_state = if station.iron_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::IronOre, "Iron Ore", station.iron_reserves, iron_state, SYMBOL_SIZE, vcfg);

                    // Iron Ingot
                    ui.set_width(col_width);
                    let iron_ingot_state = if station.iron_reserves == 0.0 && station.iron_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.iron_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::IronIngot, "Iron Ingot", station.iron_ingots, iron_ingot_state, SYMBOL_SIZE, vcfg);

                    // Hull Plate
                    ui.set_width(col_width);
                    let hull_plate_state = if station.iron_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.hull_plate_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::HullPlate, "Hull Plate", station.hull_plate_reserves, hull_plate_state, SYMBOL_SIZE, vcfg);
                });

                ui.add_space(6.0);

                // Row 2: Tungsten / Tungsten Ingot / Thruster
                ui.horizontal(|ui| {
                    let col_width = ui.available_width() / 3.0;

                    // Tungsten Ore
                    ui.set_width(col_width);
                    let tungsten_state = if station.tungsten_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::TungstenOre, "Tungsten Ore", station.tungsten_reserves, tungsten_state, SYMBOL_SIZE, vcfg);

                    // Tungsten Ingot
                    ui.set_width(col_width);
                    let tungsten_ingot_state = if station.tungsten_reserves == 0.0 && station.tungsten_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.tungsten_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::TungstenIngot, "Tungsten Ingot", station.tungsten_ingots, tungsten_ingot_state, SYMBOL_SIZE, vcfg);

                    // Thruster
                    ui.set_width(col_width);
                    let thruster_state = if station.tungsten_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.thruster_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::Thruster, "Thruster", station.thruster_reserves, thruster_state, SYMBOL_SIZE, vcfg);
                });

                ui.add_space(6.0);

                // Row 3: Nickel / Nickel Ingot / AI Core
                ui.horizontal(|ui| {
                    let col_width = ui.available_width() / 3.0;

                    // Nickel Ore
                    ui.set_width(col_width);
                    let nickel_state = if station.nickel_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::NickelOre, "Nickel Ore", station.nickel_reserves, nickel_state, SYMBOL_SIZE, vcfg);

                    // Nickel Ingot
                    ui.set_width(col_width);
                    let nickel_ingot_state = if station.nickel_reserves == 0.0 && station.nickel_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.nickel_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::NickelIngot, "Nickel Ingot", station.nickel_ingots, nickel_ingot_state, SYMBOL_SIZE, vcfg);

                    // AI Core
                    ui.set_width(col_width);
                    let ai_core_state = if station.nickel_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.ai_cores > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::AICore, "AI Core", station.ai_cores, ai_core_state, SYMBOL_SIZE, vcfg);
                });

                ui.add_space(6.0);

                // Row 4: Aluminum / Aluminum Ingot / Canister
                ui.horizontal(|ui| {
                    let col_width = ui.available_width() / 3.0;

                    // Aluminum Ore
                    ui.set_width(col_width);
                    let aluminum_state = if station.aluminum_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::AluminumOre, "Aluminum Ore", station.aluminum_reserves, aluminum_state, SYMBOL_SIZE, vcfg);

                    // Aluminum Ingot
                    ui.set_width(col_width);
                    let aluminum_ingot_state = if station.aluminum_reserves == 0.0 && station.aluminum_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.aluminum_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::AluminumIngot, "Aluminum Ingot", station.aluminum_ingots, aluminum_ingot_state, SYMBOL_SIZE, vcfg);

                    // Canister
                    ui.set_width(col_width);
                    let canister_state = if station.aluminum_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.aluminum_canisters > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_chain_column(ui, SymbolType::Canister, "Canister", station.aluminum_canisters, canister_state, SYMBOL_SIZE, vcfg);
                });

                ui.add_space(6.0);

                // Fleet row at bottom - right aligned
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(format!("Fleet Total: {}", station.drone_count))
                            .color(egui::Color32::from_rgb(0, 204, 102))
                            .size(14.0)
                            .strong());
                        let drone_bay_state = if station.hull_plate_reserves == 0.0
                            && station.thruster_reserves == 0.0
                            && station.ai_cores == 0.0
                            && station.aluminum_canisters == 0.0 {
                            SymbolState::Locked
                        } else if station.drone_count > 0 {
                            SymbolState::ActivePopulated
                        } else {
                            SymbolState::ActiveEmpty
                        };
                        draw_chain_column(ui, SymbolType::DroneBay, "Drone Bay", station.drone_count as f32, drone_bay_state, SYMBOL_SIZE, vcfg);
                    });
                });
            });
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
