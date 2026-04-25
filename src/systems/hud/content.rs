use bevy::prelude::*;
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
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
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
            ui.horizontal(|ui| {
                let can_build = station.ship_hulls >= 1.0 && station.thruster_reserves >= 1.0 && station.ai_cores >= 1.0;
                if ui.add_enabled(can_build, egui::Button::new("ASSEMBLE & DEPLOY AUTONOMOUS SHIP")).clicked() {
                    station.ship_hulls -= 1.0;
                    station.thruster_reserves -= 1.0;
                    station.ai_cores -= 1.0;
                    let (target_pos, ore, name) = if station.ai_cores >= 1.0 {
                        (SECTOR_3_POS, OreDeposit::Nickel, "Sector 3")
                    } else {
                        (SECTOR_1_POS, OreDeposit::Iron, "Sector 1")
                    };
                    commands.spawn((
                        AutonomousShipTag,
                        LastHeading(0.0),
                        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore },
                        AutonomousAssignment { target_pos, ore_type: ore, sector_name: name.to_string() },
                        Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))),
                        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                        Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_SHIP),
                    )).with_children(|parent| {
                        parent.spawn((ThrusterGlow, Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))), MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), Transform::from_xyz(0.0, -18.0, 0.1), Visibility::Hidden));
                        parent.spawn((MiningBeam, Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))), MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))), Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP), Visibility::Hidden));
                        parent.spawn((Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))), MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))), Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP)));
                        parent.spawn((ShipCargoBarFill, Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))), MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))), Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05)));
                        parent.spawn((MapElement, Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(12.0, 16.0))), MeshMaterial2d(materials.add(ColorMaterial { color: Color::srgb(1.0, 0.5, 0.0), alpha_mode: bevy::sprite::AlphaMode2d::Opaque, ..default() })), Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)), Visibility::Hidden));
                    });
                }
            });
            ui.add_space(8.0);
            ui.label(format!("DRONES IN QUEUE: {:.1}", station.ship_hulls)); // Just as a simple placeholder stat
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
                    ui.label("SHIP HULLS:"); ui.label(egui::RichText::new(format!("{:.1}", station.ship_hulls)).color(egui::Color32::GOLD)); ui.end_row();
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
