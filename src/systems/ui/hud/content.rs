use bevy::prelude::EventWriter;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;

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
    queue: &ShipQueue,
    prod_tab: &mut ProductionTabState,
    req_tab: &mut RequestsTabState,
    repair_events: &mut EventWriter<RepairStationEvent>,
    fulfill_events: &mut EventWriter<FulfillRequestEvent>,
) {
    match active_tab {
        ActiveStationTab::Cargo => {
            ui.vertical(|ui| {
                ui.heading("CARGO BAY");
                ui.add_space(8.0);
                egui::Grid::new("res_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("IRON:"); ui.label(egui::RichText::new(format!("{:.1}", station.iron_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("TUNGSTEN:"); ui.label(egui::RichText::new(format!("{:.1}", station.tungsten_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("NICKEL:"); ui.label(egui::RichText::new(format!("{:.1}", station.nickel_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("ALUMINUM:"); ui.label(egui::RichText::new(format!("{:.1}", station.aluminum_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{:.1}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("THRUSTERS:"); ui.label(egui::RichText::new(format!("{:.1}", station.thruster_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{:.1}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                    ui.label("CANISTERS:"); ui.label(egui::RichText::new(format!("{:.1}", station.aluminum_canisters)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("FLEET READY:"); ui.label(egui::RichText::new(format!("{}", queue.available_count)).color(egui::Color32::from_rgb(0, 230, 120)).strong()); ui.end_row();
                });

                ui.add_space(8.0);
                
                // DRONE BUILD PROGRESS BAR
                let progress = station.drone_build_progress;
                let can_build = station.hull_plate_reserves >= DRONE_BUILD_COST_HULLS
                    && station.thruster_reserves >= DRONE_BUILD_COST_THRUSTERS
                    && station.ai_cores >= DRONE_BUILD_COST_CORES;

                let bar_color = if can_build {
                    egui::Color32::from_rgb(0, 180, 100)
                } else {
                    egui::Color32::from_rgb(180, 60, 0)
                };

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("NEXT DRONE:").color(egui::Color32::from_gray(160)).size(12.0));
                    let progress_bar = egui::ProgressBar::new(progress)
                        .fill(bar_color)
                        .text(if can_build { format!("{:.0}%", progress * 100.0) } else { "STALLED".to_string() });
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
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            match req.id {
                                RequestId::FirstLight => {
                                    ui.heading(egui::RichText::new("First Light").color(egui::Color32::GOLD));
                                    ui.label(egui::RichText::new("Something has been noticed about you.").italics());
                                    ui.add_space(8.0);
                                    ui.label("Requires: 25 Iron Ingots");
                                    ui.label(egui::RichText::new("Reward: Power +25%").color(egui::Color32::CYAN));
                                    ui.add_space(8.0);
                                    
                                    if req.fulfilled {
                                        ui.label(egui::RichText::new("COMPLETE").strong().color(egui::Color32::GREEN));
                                    } else {
                                        let can_afford = station.iron_ingots >= 25.0;
                                        if ui.add_enabled(can_afford, egui::Button::new("FULFILL").min_size(egui::vec2(120.0, 44.0))).clicked() {
                                            fulfill_events.send(FulfillRequestEvent {
                                                request_id: req.id,
                                                faction_id: req.faction,
                                            });
                                        }
                                    }
                                }
                            }
                        });
                    });
                    ui.add_space(8.0);
                }
            }
        }
    }
}
