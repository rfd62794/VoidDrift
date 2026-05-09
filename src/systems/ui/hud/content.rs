use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;
use crate::config::{BalanceConfig, RequestConfig, LogsConfig, VisualConfig};
use crate::config::visual::{rgb_u8_to_egui};
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

// Part D: Symbol bar drawing function
fn draw_symbol_bar(
    ui: &mut egui::Ui,
    symbol_type: SymbolType,
    state: SymbolState,
    count: f32,
    size: f32,
    vcfg: &VisualConfig,
) {
    ui.vertical(|ui| {
        let rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(size, size));
        let painter = ui.painter();

        // Determine alpha based on state
        let alpha = match state {
            SymbolState::Locked => 51,
            SymbolState::ActiveEmpty => 128,
            SymbolState::ActivePopulated => 255,
        };

        // Draw procedural visual based on symbol type
        match symbol_type {
            SymbolType::IronIngot | SymbolType::TungstenIngot | SymbolType::NickelIngot | SymbolType::AluminumIngot => {
                let base_color = match symbol_type {
                    SymbolType::IronIngot => rgb_u8_to_egui(vcfg.ore.metal.color_vein),
                    SymbolType::TungstenIngot => rgb_u8_to_egui(vcfg.ore.h3_gas.color_vein),
                    SymbolType::NickelIngot => rgb_u8_to_egui(vcfg.ore.void_essence.color_vein),
                    SymbolType::AluminumIngot => rgb_u8_to_egui(vcfg.ore.metal.color_vein),
                    _ => egui::Color32::WHITE,
                };
                let base_color = egui::Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), alpha);
                // For now, use simple rectangle for ingots - the ingot_node drawing is complex and needs color support
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
                component_nodes::draw_hull(painter, rect.center(), &hull_config);
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
                component_nodes::draw_thruster(painter, rect.center(), &thruster_config);
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
                component_nodes::draw_ai_core(painter, rect.center(), &ai_core_config);
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
                component_nodes::draw_canister(painter, rect.center(), &canister_config);
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
                component_nodes::draw_drone_bay(painter, rect.center(), &drone_bay_config, is_ready);
            }
        }

        // Draw count label below symbol (not for locked state)
        let count_y = rect.center().y + 8.0;
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
            painter.text(
                egui::pos2(rect.center().x, count_y),
                egui::Align2::CENTER_CENTER,
                &count_text,
                egui::FontId::proportional(10.0),
                count_color,
            );
        }

        // Draw short label below count
        let label_y = count_y + 12.0;
        let label_text = match symbol_type {
            SymbolType::IronIngot => "FE",
            SymbolType::TungstenIngot => "W",
            SymbolType::NickelIngot => "NI",
            SymbolType::AluminumIngot => "AL",
            SymbolType::HullPlate => "HULL",
            SymbolType::Thruster => "THRU",
            SymbolType::AICore => "A.I.",
            SymbolType::Canister => "CANS",
            SymbolType::DroneBay => "BAY",
        };
        let label_color = match state {
            SymbolState::Locked => egui::Color32::from_rgba_unmultiplied(0, 200, 200, 51),
            SymbolState::ActiveEmpty => egui::Color32::from_rgba_unmultiplied(0, 200, 200, 128),
            SymbolState::ActivePopulated => egui::Color32::from_rgba_unmultiplied(0, 200, 200, 255),
        };
        painter.text(
            egui::pos2(rect.center().x, label_y),
            egui::Align2::CENTER_CENTER,
            label_text,
            egui::FontId::proportional(9.0),
            label_color,
        );

        ui.advance_cursor_after_rect(rect);
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

                // Part D: Drawer Symbol Status Bar
                ui.label(egui::RichText::new("PRODUCTION STATUS").color(egui::Color32::from_gray(140)).size(11.0));
                ui.add_space(4.0);

                // Symbol bar: 9 symbols, 32×32px each, 4px gap
                ui.horizontal_centered(|ui| {
                    const SYMBOL_SIZE: f32 = 32.0;

                    // Ingots - locked until first ore of that type mined
                    let iron_ingot_state = if station.iron_reserves == 0.0 && station.iron_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.iron_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::IronIngot, iron_ingot_state, station.iron_ingots, SYMBOL_SIZE, vcfg);

                    let tungsten_ingot_state = if station.tungsten_reserves == 0.0 && station.tungsten_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.tungsten_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::TungstenIngot, tungsten_ingot_state, station.tungsten_ingots, SYMBOL_SIZE, vcfg);

                    let nickel_ingot_state = if station.nickel_reserves == 0.0 && station.nickel_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.nickel_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::NickelIngot, nickel_ingot_state, station.nickel_ingots, SYMBOL_SIZE, vcfg);

                    let aluminum_ingot_state = if station.aluminum_reserves == 0.0 && station.aluminum_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.aluminum_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::AluminumIngot, aluminum_ingot_state, station.aluminum_ingots, SYMBOL_SIZE, vcfg);

                    // Components - locked until their ingot exists
                    let hull_plate_state = if station.iron_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.hull_plate_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::HullPlate, hull_plate_state, station.hull_plate_reserves, SYMBOL_SIZE, vcfg);

                    let thruster_state = if station.tungsten_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.thruster_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::Thruster, thruster_state, station.thruster_reserves, SYMBOL_SIZE, vcfg);

                    let ai_core_state = if station.nickel_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.ai_cores > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::AICore, ai_core_state, station.ai_cores, SYMBOL_SIZE, vcfg);

                    let canister_state = if station.aluminum_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.aluminum_canisters > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, SymbolType::Canister, canister_state, station.aluminum_canisters, SYMBOL_SIZE, vcfg);

                    // Drone Bay - locked until first component exists
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
                    draw_symbol_bar(ui, SymbolType::DroneBay, drone_bay_state, station.drone_count as f32, SYMBOL_SIZE, vcfg);
                });

                // Legacy text display (keep for now, can be removed later)
                ui.label(egui::RichText::new("DETAILED INVENTORY").color(egui::Color32::from_gray(140)).size(11.0));
                egui::Grid::new("ore_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("IRON:"); ui.label(egui::RichText::new(format!("{:.1} / {:.1}", station.iron_reserves, station.iron_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("TUNGSTEN:"); ui.label(egui::RichText::new(format!("{:.1} / {:.1}", station.tungsten_reserves, station.tungsten_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("NICKEL:"); ui.label(egui::RichText::new(format!("{:.1} / {:.1}", station.nickel_reserves, station.nickel_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("ALUMINUM:"); ui.label(egui::RichText::new(format!("{:.1} / {:.1}", station.aluminum_reserves, station.aluminum_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                });

                ui.add_space(4.0);
                ui.label(egui::RichText::new("PARTS").color(egui::Color32::from_gray(140)).size(11.0));
                egui::Grid::new("parts_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{:.1}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("THRUSTERS:"); ui.label(egui::RichText::new(format!("{:.1}", station.thruster_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{:.1}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                    ui.label("CANISTERS:"); ui.label(egui::RichText::new(format!("{:.1}", station.aluminum_canisters)).color(egui::Color32::WHITE)); ui.end_row();
                });

                ui.add_space(8.0);
                
                // DRONE BUILD PROGRESS BAR
                let progress = station.drone_build_progress;
                let can_build = station.hull_plate_reserves >= cfg.drone.cost_hulls
                    && station.thruster_reserves >= cfg.drone.cost_thrusters
                    && station.ai_cores >= cfg.drone.cost_cores;

                let bar_color = if can_build {
                    egui::Color32::from_rgb(0, 180, 100)
                } else {
                    egui::Color32::from_rgb(180, 60, 0)
                };

                let stall_label = if station.hull_plate_reserves < cfg.drone.cost_hulls {
                    "Needs Hull Plates"
                } else if station.thruster_reserves < cfg.drone.cost_thrusters {
                    "Needs Thrusters"
                } else {
                    "Needs AI Core"
                };

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("NEXT DRONE:").color(egui::Color32::from_gray(160)).size(12.0));
                    let progress_bar = egui::ProgressBar::new(progress)
                        .fill(bar_color)
                        .text(if can_build { format!("{:.0}%", progress * 100.0) } else { stall_label.to_string() });
                    ui.add(progress_bar);
                });
            });
            if !station.online {
                if ui.button("REPAIR STATION").clicked() {
                    repair_events.send(RepairStationEvent);
                }
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
