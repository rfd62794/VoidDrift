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
) {
    match active_tab {
        ActiveStationTab::Station => {
            ui.heading("VOIDRIFT STATION");
            ui.add_space(8.0);
            ui.label(egui::RichText::new("ECHO: STATION AI - OPERATIONAL")
                .color(egui::Color32::from_rgb(0, 204, 102)));
        }
        ActiveStationTab::Fleet => {
            ui.heading("FLEET");
            ui.add_space(8.0);

            // Ships ready
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("SHIPS READY:").color(egui::Color32::from_gray(180)));
                ui.label(egui::RichText::new(format!("{}", queue.available_count))
                    .color(egui::Color32::from_rgb(0, 230, 120))
                    .strong()
                    .size(16.0));
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Drone assembly toggle
            ui.horizontal(|ui| {
                ui.checkbox(&mut toggles.build_drones, "");
                ui.label(egui::RichText::new("AUTO-ASSEMBLE DRONES")
                    .color(egui::Color32::from_rgb(0, 204, 102)));
            });

            ui.add_space(4.0);

            // Build progress bar
            let progress = station.drone_build_progress;
            let can_build = station.hull_plate_reserves >= 3.0
                && station.thruster_reserves >= 1.0
                && station.ai_cores >= 1.0;

            let bar_color = if !toggles.build_drones {
                egui::Color32::from_gray(60)
            } else if can_build {
                egui::Color32::from_rgb(0, 180, 100)
            } else {
                egui::Color32::from_rgb(180, 60, 0)
            };

            let progress_bar = egui::ProgressBar::new(progress)
                .fill(bar_color)
                .text(if !can_build {
                    "WAITING FOR COMPONENTS".to_string()
                } else {
                    format!("{:.0}%", progress * 100.0)
                });
            ui.add(progress_bar);

            ui.add_space(8.0);

            // Component stock
            egui::Grid::new("fleet_components").spacing([12.0, 4.0]).show(ui, |ui| {
                ui.label(egui::RichText::new("HULL PLATES:").color(egui::Color32::from_gray(160)));
                ui.label(egui::RichText::new(format!("{:.1} (need 3)", station.hull_plate_reserves))
                    .color(if station.hull_plate_reserves >= 3.0 { egui::Color32::WHITE } else { egui::Color32::DARK_RED }));
                ui.end_row();
                ui.label(egui::RichText::new("THRUSTERS:").color(egui::Color32::from_gray(160)));
                ui.label(egui::RichText::new(format!("{:.1} (need 1)", station.thruster_reserves))
                    .color(if station.thruster_reserves >= 1.0 { egui::Color32::WHITE } else { egui::Color32::DARK_RED }));
                ui.end_row();
                ui.label(egui::RichText::new("AI CORES:").color(egui::Color32::from_gray(160)));
                ui.label(egui::RichText::new(format!("{:.1} (need 1)", station.ai_cores))
                    .color(if station.ai_cores >= 1.0 { egui::Color32::WHITE } else { egui::Color32::DARK_RED }));
                ui.end_row();
            });

            if queue.available_count >= crate::constants::MAX_DRONE_QUEUE {
                ui.add_space(4.0);
                ui.label(egui::RichText::new("QUEUE FULL (5/5)")
                    .color(egui::Color32::GOLD)
                    .italics());
            }
        }
        ActiveStationTab::Cargo => {
            ui.vertical(|ui| {
                ui.heading("CARGO BAY");
                ui.add_space(8.0);
                egui::Grid::new("res_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("IRON:"); ui.label(egui::RichText::new(format!("{:.1}", station.iron_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("TUNGSTEN:"); ui.label(egui::RichText::new(format!("{:.1}", station.tungsten_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("NICKEL:"); ui.label(egui::RichText::new(format!("{:.1}", station.nickel_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{:.1}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("THRUSTERS:"); ui.label(egui::RichText::new(format!("{:.1}", station.thruster_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{:.1}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
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
                    station.repair_progress = 1.0;
                    station.online = true;
                }
            }
        }
        ActiveStationTab::Iron => {
            ui.heading("IRON");
            render_ore_pipeline(
                ui,
                "IRON ORE",
                station.iron_reserves,
                "IRON INGOTS",
                station.iron_ingots,
                "HULLS",
                station.hull_plate_reserves,
                1.0 / REFINERY_RATIO as f32,
                HULL_PLATE_COST_IRON as f32,
                &mut toggles.refine_iron,
                &mut toggles.forge_hull,
            );
        }
        ActiveStationTab::Tungsten => {
            ui.heading("TUNGSTEN");
            render_ore_pipeline(
                ui,
                "TUNGSTEN ORE",
                station.tungsten_reserves,
                "TUNGSTEN INGOTS",
                station.tungsten_ingots,
                "THRUSTERS",
                station.thruster_reserves,
                1.0 / REFINERY_RATIO as f32,
                THRUSTER_COST_TUNGSTEN as f32,
                &mut toggles.refine_tungsten,
                &mut toggles.forge_thruster,
            );
        }
        ActiveStationTab::Nickel => {
            ui.heading("NICKEL");
            render_ore_pipeline(
                ui,
                "NICKEL ORE",
                station.nickel_reserves,
                "NICKEL INGOTS",
                station.nickel_ingots,
                "AI CORES",
                station.ai_cores,
                1.0 / REFINERY_RATIO as f32,
                AI_CORE_COST_NICKEL as f32,
                &mut toggles.refine_nickel,
                &mut toggles.forge_core,
            );
        }
        ActiveStationTab::Upgrades => {
            ui.heading("UPGRADES");
            ui.add_space(8.0);
            ui.label("System upgrades offline. Awaiting Phase 2 implementation.");
        }
    }
}
