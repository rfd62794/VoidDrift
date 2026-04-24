use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;
use crate::systems::station_tabs::render_queue_card;

/// Render the active tab content inside the content_area panel.
/// Called only when docked and drawer is Expanded.
pub fn render_tab_content(
    ui: &mut egui::Ui,
    active_tab: ActiveStationTab,
    station: &mut Station,
    queues: &mut StationQueues,
    ship: &mut Ship,
    auto_dock_settings: &mut AutoDockSettings,
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
            ui.label("DRONE MANAGEMENT - COMING ONLINE");
        }
        ActiveStationTab::Cargo => {
            ui.vertical(|ui| {
                ui.heading("CARGO BAY");
                ui.add_space(8.0);
                egui::Grid::new("res_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("MAGNETITE:"); ui.label(egui::RichText::new(format!("{:.1}", station.magnetite_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("CARBON:"); ui.label(egui::RichText::new(format!("{:.1}", station.carbon_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("POWER CELLS:"); ui.label(egui::RichText::new(format!("{}", station.power_cells)).color(egui::Color32::GREEN)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                    ui.label("SHIP HULLS:"); ui.label(egui::RichText::new(format!("{}", station.ship_hulls)).color(egui::Color32::GOLD)); ui.end_row();
                });
                ui.add_space(16.0);
                ui.separator();
                ui.heading("AUTO-DOCK SETTINGS");
                ui.checkbox(&mut auto_dock_settings.auto_unload, "Auto-Unload Cargo");
                ui.checkbox(&mut auto_dock_settings.auto_smelt_magnetite, "Auto-Smelt Magnetite");
                ui.checkbox(&mut auto_dock_settings.auto_smelt_carbon, "Auto-Smelt Carbon");
            });
            if !station.online {
                if ui.button(format!("REPAIR STATION [{} CELLS]", REPAIR_COST)).clicked() && station.power_cells >= REPAIR_COST {
                    station.power_cells -= REPAIR_COST;
                    station.repair_progress = 1.0;
                    station.online = true;
                }
            }
        }
        ActiveStationTab::Power => {
            ui.heading("POWER");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(format!("STATION: {:.1}/{:.0}", station.power, STATION_POWER_MAX));
                ui.add(egui::ProgressBar::new(station.power / STATION_POWER_MAX).desired_width(150.0));
            });
            ui.horizontal(|ui| {
                ui.label(format!("SHIP:    {:.1}/{:.0}", ship.power, SHIP_POWER_MAX));
                ui.add(egui::ProgressBar::new(ship.power / SHIP_POWER_MAX).desired_width(150.0));
            });
        }
        ActiveStationTab::Refinery => {
            ui.horizontal(|ui| {
                render_queue_card(ui, station, &mut queues.magnetite_refinery, ProcessingOperation::MagnetiteRefinery, REFINERY_RATIO as f32, POWER_COST_REFINERY as f32, REFINERY_MAGNETITE_TIME);
                ui.add_space(16.0);
                render_queue_card(ui, station, &mut queues.carbon_refinery, ProcessingOperation::CarbonRefinery, HULL_PLATE_COST_CARBON as f32, POWER_COST_HULL_FORGE as f32, REFINERY_CARBON_TIME);
            });
        }
        ActiveStationTab::Foundry => {
            ui.horizontal(|ui| {
                render_queue_card(ui, station, &mut queues.hull_forge, ProcessingOperation::HullForge, SHIP_HULL_COST_PLATES as f32, POWER_COST_SHIP_FORGE as f32, FORGE_HULL_TIME);
                ui.add_space(16.0);
                render_queue_card(ui, station, &mut queues.core_fabricator, ProcessingOperation::CoreFabricator, AI_CORE_COST_CELLS as f32, POWER_COST_AI_FABRICATE as f32, FORGE_CORE_TIME);
            });
        }
        ActiveStationTab::Hangar => {
            ui.horizontal(|ui| {
                if ui.button("ASSEMBLE & DEPLOY AUTONOMOUS SHIP").clicked()
                    && station.ship_hulls >= 1
                    && station.ai_cores >= 1
                {
                    station.ship_hulls -= 1;
                    station.ai_cores -= 1;
                    let (target_pos, ore, name) = if station.ai_cores >= 1 {
                        (SECTOR_3_POS, OreType::Carbon, "Sector 3")
                    } else {
                        (SECTOR_1_POS, OreType::Magnetite, "Sector 1")
                    };
                    commands.spawn((
                        AutonomousShipTag,
                        LastHeading(0.0),
                        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore, power: SHIP_POWER_MAX },
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
                ui.separator();
                if ui.button("TOP UP SHIP [3 CELLS]").clicked()
                    && station.power_cells >= 3
                    && ship.power_cells < 5
                {
                    station.power_cells -= 3;
                    ship.power_cells = (ship.power_cells + 3).min(5);
                }
            });
        }
    }
}
