use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;

/// Render the active tab content inside the content_area panel.
/// Called only when docked and drawer is Expanded.
pub fn render_tab_content(
    ui: &mut egui::Ui,
    active_tab: ActiveStationTab,
    station: &mut Station,
    _queues: &mut StationQueues,
    _ship: &mut Ship,
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
                    ui.label("IRON:"); ui.label(egui::RichText::new(format!("{:.1}", station.iron_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("TUNGSTEN:"); ui.label(egui::RichText::new(format!("{:.1}", station.tungsten_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("NICKEL:"); ui.label(egui::RichText::new(format!("{:.1}", station.nickel_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                    ui.label("SHIP HULLS:"); ui.label(egui::RichText::new(format!("{}", station.ship_hulls)).color(egui::Color32::GOLD)); ui.end_row();
                });
            });
            if !station.online {
                if ui.button("REPAIR STATION").clicked() {
                    station.repair_progress = 1.0;
                    station.online = true;
                }
            }
        }

        ActiveStationTab::Refinery => {
            ui.vertical(|ui| {
                ui.heading("REFINERY STATUS");
                ui.add_space(8.0);
                egui::Grid::new("refinery_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("IRON ORE:"); ui.label(egui::RichText::new(format!("{:.1}", station.iron_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("IRON INGOTS:"); ui.label(egui::RichText::new(format!("{:.1}", station.iron_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label(""); ui.end_row();
                    ui.label("TUNGSTEN ORE:"); ui.label(egui::RichText::new(format!("{:.1}", station.tungsten_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("TUNGSTEN INGOTS:"); ui.label(egui::RichText::new(format!("{:.1}", station.tungsten_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label(""); ui.end_row();
                    ui.label("NICKEL ORE:"); ui.label(egui::RichText::new(format!("{:.1}", station.nickel_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("NICKEL INGOTS:"); ui.label(egui::RichText::new(format!("{:.1}", station.nickel_ingots)).color(egui::Color32::WHITE)); ui.end_row();
                });
            });
        }
        ActiveStationTab::Foundry => {
            ui.vertical(|ui| {
                ui.heading("FOUNDRY STATUS");
                ui.add_space(8.0);
                egui::Grid::new("foundry_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{:.1}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{:.1}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                    ui.label("SHIP HULLS:"); ui.label(egui::RichText::new(format!("{:.1}", station.ship_hulls)).color(egui::Color32::GOLD)); ui.end_row();
                });
            });
        }
        ActiveStationTab::Hangar => {
            ui.horizontal(|ui| {
                if ui.button("ASSEMBLE & DEPLOY AUTONOMOUS SHIP").clicked()
                    && station.ship_hulls >= 1.0
                    && station.ai_cores >= 1.0
                {
                    station.ship_hulls -= 1.0;
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
        }
    }
}
