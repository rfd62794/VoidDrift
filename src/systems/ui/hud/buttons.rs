use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::scenes::main_menu::MainMenuState;
use crate::components::resources::{ShipQueue, ViewState, MapPanState};

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
            // PIPELINE renders first but appears leftmost due to right_to_left
            let pipeline_response = ui.add(egui::Button::new("PIPELINE").min_size(egui::vec2(80.0, 44.0)));
            if pipeline_response.clicked() {
                view_state.show_production_tree = true;
                view_state.production_tree_ever_opened = true;
            }

            // Pipeline highlight: amber pulsing stroke when drone built and production tree never opened
            let station = station_query.get_single();
            let drone_built = station.as_ref().ok().map(|(_, _, queues)| {
                queues.hull_forge.is_some() || queues.core_fabricator.is_some()
            }).unwrap_or(false);
            if drone_built && !view_state.production_tree_ever_opened {
                let t = ui.ctx().input(|i| i.time as f32);
                let alpha = ((t * 2.0).sin() * 0.3 + 0.7) * 255.0;
                let center_rect = egui::Rect::from_center_size(pipeline_response.rect.center(), egui::vec2(200.0, pipeline_response.rect.height()));
                let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("pipeline_highlight"));
                let painter = ui.ctx().layer_painter(layer_id);
                painter.rect_stroke(
                    center_rect,
                    0.0,
                    egui::Stroke::new(8.0, egui::Color32::from_rgba_unmultiplied(255, 200, 50, alpha as u8)),
                    egui::StrokeKind::Outside,
                );
            }

            // SAVE renders second but appears rightmost due to right_to_left
            if ui.add(egui::Button::new("SAVE").min_size(egui::vec2(80.0, 44.0))).clicked() {
                menu_state.show_save_overlay = true;
            }
            // FOCUS renders third but appears left of SAVE
            if ui.add(egui::Button::new("FOCUS").min_size(egui::vec2(80.0, 44.0))).clicked() {
                pan_state.is_focused = true;
                pan_state.cumulative_offset = Vec2::ZERO;
                if let Ok(mut proj) = cam_query.get_single_mut() {
                    proj.scale = 1.0;
                }
            }
        });
    });
}
