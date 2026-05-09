use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;
use crate::config::{BalanceConfig, RequestConfig, LogsConfig};
use crate::systems::persistence::save::SaveData;

#[derive(PartialEq, Clone, Copy)]
enum SymbolState {
    Locked,           // not yet reachable — ghost outline, alpha 0.2
    ActiveEmpty,      // unlocked, count == 0 — dim color, alpha 0.5
    ActivePopulated,  // unlocked, count > 0 — full color, alpha 1.0
}

// Part D: Symbol bar drawing function
fn draw_symbol_bar(ui: &mut egui::Ui, _name: &str, state: SymbolState, count: f32, size: f32) {
    ui.vertical(|ui| {
        let rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(size, size));

        match state {
            SymbolState::Locked => {
                // Draw symbol shape at alpha 51 (0.2 * 255) — outline only
                let fill = egui::Color32::from_rgba_unmultiplied(100, 100, 100, 51);
                ui.painter().rect_filled(rect, 0.0, fill);
                // Do NOT show count label
            }
            SymbolState::ActiveEmpty => {
                // Draw symbol at alpha 128 (0.5 * 255)
                let fill = egui::Color32::from_rgba_unmultiplied(0, 200, 200, 128);
                ui.painter().rect_filled(rect, 0.0, fill);
                // Show count "0" at 40% opacity
                ui.label(egui::RichText::new("0")
                    .color(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 102))
                    .size(10.0));
            }
            SymbolState::ActivePopulated => {
                // Draw symbol at full alpha 255
                let fill = egui::Color32::from_rgba_unmultiplied(0, 200, 200, 255);
                ui.painter().rect_filled(rect, 0.0, fill);
                // Show count at full opacity
                ui.label(egui::RichText::new(format!("{:.0}", count))
                    .color(egui::Color32::WHITE)
                    .size(10.0));
            }
        }

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
) {
    match active_tab {
        ActiveStationTab::Cargo => {
            ui.vertical(|ui| {
                ui.heading("CARGO BAY");
                ui.add_space(8.0);

                // Part D: Drawer Symbol Status Bar
                ui.label(egui::RichText::new("PRODUCTION STATUS").color(egui::Color32::from_gray(140)).size(11.0));
                ui.add_space(4.0);

                // Symbol bar: 9 symbols, 24×24px each, 4px gap
                ui.horizontal_centered(|ui| {
                    const SYMBOL_SIZE: f32 = 24.0;
                    const GAP: f32 = 4.0;
                    const ROW_WIDTH: f32 = (SYMBOL_SIZE + GAP) * 9.0 - GAP;

                    // Ingots - locked until first ore of that type mined
                    let iron_ingot_state = if station.iron_reserves == 0.0 && station.iron_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.iron_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Iron Ingot", iron_ingot_state, station.iron_ingots, SYMBOL_SIZE);

                    let tungsten_ingot_state = if station.tungsten_reserves == 0.0 && station.tungsten_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.tungsten_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Tungsten Ingot", tungsten_ingot_state, station.tungsten_ingots, SYMBOL_SIZE);

                    let nickel_ingot_state = if station.nickel_reserves == 0.0 && station.nickel_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.nickel_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Nickel Ingot", nickel_ingot_state, station.nickel_ingots, SYMBOL_SIZE);

                    let aluminum_ingot_state = if station.aluminum_reserves == 0.0 && station.aluminum_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.aluminum_ingots > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Aluminum Ingot", aluminum_ingot_state, station.aluminum_ingots, SYMBOL_SIZE);

                    // Components - locked until their ingot exists
                    let hull_plate_state = if station.iron_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.hull_plate_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Hull Plate", hull_plate_state, station.hull_plate_reserves, SYMBOL_SIZE);

                    let thruster_state = if station.tungsten_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.thruster_reserves > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Thruster", thruster_state, station.thruster_reserves, SYMBOL_SIZE);

                    let ai_core_state = if station.nickel_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.ai_cores > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "AI Core", ai_core_state, station.ai_cores, SYMBOL_SIZE);

                    let canister_state = if station.aluminum_ingots == 0.0 {
                        SymbolState::Locked
                    } else if station.aluminum_canisters > 0.0 {
                        SymbolState::ActivePopulated
                    } else {
                        SymbolState::ActiveEmpty
                    };
                    draw_symbol_bar(ui, "Canister", canister_state, station.aluminum_canisters, SYMBOL_SIZE);

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
                    draw_symbol_bar(ui, "Drone Bay", drone_bay_state, station.drone_count as f32, SYMBOL_SIZE);
                });

                ui.add_space(8.0);

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
