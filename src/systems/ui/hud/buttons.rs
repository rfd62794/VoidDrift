use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::scenes::main_menu::MainMenuState;
use crate::components::resources::{ShipQueue, ViewState, MapPanState};
use crate::ui_kit::{primitives::vd_button, styles::{ButtonStyle, HighlightKind}};

pub fn render_hud_buttons(
    ui: &mut egui::Ui,
    autonomous_ships: &Query<Entity, With<AutonomousShipTag>>,
    queue: &ShipQueue,
    station_query: &Query<(Entity, &mut Station, &mut StationQueues), (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    view_state: &mut ViewState,
    menu_state: &mut MainMenuState,
    pan_state: &mut MapPanState,
    cam_query: &mut Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    ui.horizontal(|ui| {
        // Left: Fleet count indicator (ready/total)
        let deployed = autonomous_ships.iter().count();
        let total = queue.available_count as usize + deployed;
        ui.label(egui::RichText::new(format!("Fleet: {}/{}", queue.available_count, total))
            .color(egui::Color32::from_rgb(0, 200, 200))
            .size(16.0));

        // Push buttons to right edge using right_to_left layout
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Pipeline highlight gating — evaluated BEFORE the button so the
            // amber pulse can be passed in as a vd_button argument.
            let station = station_query.get_single();
            let drone_built = station.as_ref().ok().map(|(_, _, queues)| {
                queues.hull_forge.is_some() || queues.core_fabricator.is_some()
            }).unwrap_or(false);
            let highlight = if drone_built && !view_state.production_tree_ever_opened {
                Some(HighlightKind::Amber)
            } else {
                None
            };

            // PIPELINE renders first but appears leftmost due to right_to_left
            let pipeline_response = vd_button(ui, "PIPELINE", ButtonStyle::primary(), true, highlight);
            if pipeline_response.clicked() {
                view_state.show_production_tree = true;
                view_state.production_tree_ever_opened = true;
            }

            // SAVE renders second but appears rightmost due to right_to_left
            let save_response = vd_button(ui, "SAVE", ButtonStyle::primary(), true, None);
            if save_response.clicked() {
                menu_state.show_save_overlay = true;
            }
            // FOCUS renders third but appears left of SAVE
            let focus_response = vd_button(ui, "FOCUS", ButtonStyle::primary(), true, None);
            if focus_response.clicked() {
                pan_state.is_focused = true;
                pan_state.cumulative_offset = Vec2::ZERO;
                if let Ok(mut proj) = cam_query.get_single_mut() {
                    proj.scale = 1.0;
                }
            }
        });
    });
}
