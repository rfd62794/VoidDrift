use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use super::MainMenuState;
use crate::components::resources::AppState;
use crate::systems::persistence::save::{SaveRequestEvent, SaveCategory};

pub fn save_overlay_system(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut save_events: EventWriter<SaveRequestEvent>,
) {
    let ctx = contexts.ctx_mut();

    if menu_state.show_save_overlay {
        egui::Window::new("save_overlay")
            .fixed_pos([
                ctx.screen_rect().width() / 2.0 - 160.0,
                ctx.screen_rect().height() / 2.0 - 200.0,
            ])
            .fixed_size([320.0, 400.0])
            .title_bar(false)
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgba_premultiplied(4, 8, 12, 240))
                .stroke(egui::Stroke::new(1.0,
                    egui::Color32::from_rgb(0, 100, 50))))
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("SAVE / LOAD")
                    .size(14.0)
                    .color(egui::Color32::from_rgb(0, 200, 100)));

                ui.separator();

                // Save name input
                ui.label(egui::RichText::new("SAVE NAME:")
                    .size(11.0)
                    .color(egui::Color32::from_rgb(80, 120, 80)));

                ui.text_edit_singleline(&mut menu_state.save_name_input);

                ui.add_space(8.0);

                // Save as Play Save
                if ui.add_sized([300.0, 44.0],
                    egui::Button::new("SAVE - PLAY")).clicked() {
                    let name = if menu_state.save_name_input.is_empty() {
                        "quicksave".to_string()
                    } else {
                        menu_state.save_name_input.clone()
                    };
                    save_events.send(SaveRequestEvent {
                        name,
                        category: SaveCategory::Play,
                        description: String::new(),
                    });
                    menu_state.show_save_overlay = false;
                }

                // Save as Stage Save - developer mode only
                if menu_state.developer_mode {
                    if ui.add_sized([300.0, 44.0],
                        egui::Button::new(
                            egui::RichText::new("SAVE - STAGE")
                                .color(egui::Color32::from_rgb(200, 160, 0))
                        )).clicked() {
                        if !menu_state.save_name_input.is_empty() {
                            save_events.send(SaveRequestEvent {
                                name: menu_state.save_name_input.clone(),
                                category: SaveCategory::Stage,
                                description: format!("Stage save - {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
                            });
                            menu_state.show_save_overlay = false;
                        }
                    }
                }

                ui.separator();

                // Return to main menu
                if ui.add_sized([300.0, 44.0],
                    egui::Button::new("MAIN MENU")).clicked() {
                    next_state.set(AppState::MainMenu);
                    menu_state.show_save_overlay = false;
                }

                ui.add_space(4.0);

                // Close overlay
                if ui.add_sized([300.0, 36.0],
                    egui::Button::new(
                        egui::RichText::new("CLOSE")
                            .size(12.0)
                            .color(egui::Color32::from_rgb(80, 80, 80))
                    )).clicked() {
                    menu_state.show_save_overlay = false;
                }
            });
    }
}
