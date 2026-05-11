use bevy_egui::egui;
use crate::components::resources::TutorialState;
use crate::systems::telemetry::{TelemetryOptInPrompt, TelemetryConsent, TelemetrySessionCounter};
use crate::systems::persistence::save::{SaveRequestEvent, SaveCategory};

pub fn render_overlays(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    tutorial: &mut TutorialState,
    telemetry_opt_in: &mut TelemetryOptInPrompt,
    telemetry_consent: &mut TelemetryConsent,
    telemetry_session_counter: &mut TelemetrySessionCounter,
    save_events: &mut bevy::ecs::event::EventWriter<SaveRequestEvent>,
    world_view_rect: &crate::components::resources::WorldViewRect,
    view_state: &crate::components::resources::ViewState,
) {
    // Tutorial popup as painted overlay (same pattern as Production Tree arrows)
    if let Some(popup) = tutorial.active.clone() {
        if !view_state.show_production_tree {
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("tutorial_overlay")
            ));
            
            // Center on viewport (world_view_rect) instead of screen to account for drawer
            let viewport_center = egui::pos2(
                world_view_rect.x + world_view_rect.w / 2.0,
                world_view_rect.y + world_view_rect.h / 2.0
            );
            
            // Background rect — centered, wider for text wrapping
            let w = 480.0;
            let h = 220.0;
            let bg_rect = egui::Rect::from_center_size(
                viewport_center,
                egui::vec2(w, h)
            );
            
            // Draw background
            painter.rect_filled(bg_rect, 6.0, egui::Color32::from_rgba_unmultiplied(5, 5, 10, 240));
            painter.rect_stroke(bg_rect, 6.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
            
            // Title — ECHO in amber
            painter.text(
                bg_rect.center_top() + egui::vec2(0.0, 16.0),
                egui::Align2::CENTER_TOP,
                &popup.title,
                egui::FontId::proportional(13.0),
                egui::Color32::from_rgb(180, 140, 50),
            );
            
            // Body text — break on punctuation and render multiple lines
            let body_lines: Vec<&str> = popup.body
                .split(|c| c == '.' || c == '!' || c == '?')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim())
                .collect();
            
            let line_height = 18.0;
            let start_y = bg_rect.min.y + 50.0;
            
            for (i, line) in body_lines.iter().enumerate() {
                let y = start_y + (i as f32) * line_height;
                painter.text(
                    egui::pos2(bg_rect.center().x, y),
                    egui::Align2::CENTER_TOP,
                    line,
                    egui::FontId::proportional(12.0),
                    egui::Color32::from_rgb(220, 215, 210),
                );
            }
            
            // Button rect
            let btn_rect = egui::Rect::from_center_size(
                bg_rect.center_bottom() - egui::vec2(0.0, 28.0),
                egui::vec2(120.0, 32.0)
            );
            
            // Draw button
            painter.rect_filled(btn_rect, 4.0, egui::Color32::from_rgb(40, 35, 15));
            painter.rect_stroke(btn_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
            painter.text(
                btn_rect.center(),
                egui::Align2::CENTER_CENTER,
                &popup.button_label,
                egui::FontId::proportional(13.0),
                egui::Color32::from_rgb(180, 140, 50),
            );
            
            // Click detection — same pattern as Production Tree arrows
            let response = ui.interact(
                btn_rect,
                egui::Id::new("tutorial_btn"),
                egui::Sense::click()
            );
            if response.clicked() {
                tutorial.shown.insert(popup.id);
                tutorial.active = None;
            }
        }
        return;
    }

    // Telemetry opt-in prompt as painted overlay (ECHO system interrupt style)
    if telemetry_opt_in.active && !view_state.show_production_tree {
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("telemetry_opt_in_overlay")
        ));
        
        // Center on viewport
        let viewport_center = egui::pos2(
            world_view_rect.x + world_view_rect.w / 2.0,
            world_view_rect.y + world_view_rect.h / 2.0
        );
        
        // Background rect — amber border, dark background
        let w = 420.0;
        let h = 180.0;
        let bg_rect = egui::Rect::from_center_size(
            viewport_center,
            egui::vec2(w, h)
        );
        
        // Draw background
        painter.rect_filled(bg_rect, 6.0, egui::Color32::from_rgba_unmultiplied(5, 5, 10, 240));
        painter.rect_stroke(bg_rect, 6.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
        
        // Text content — ECHO voice, lowercase, fixed-width, left-aligned
        let text_lines = [
            "echo: anonymous usage data helps improve the signal.",
            "no personal data collected. no identifiers stored.",
            "no account required. choice can be changed in settings.",
        ];
        let mut y_offset = 30.0;
        for line in &text_lines {
            painter.text(
                bg_rect.min + egui::vec2(20.0, y_offset),
                egui::Align2::LEFT_TOP,
                line,
                egui::FontId::monospace(13.0),
                egui::Color32::WHITE
            );
            y_offset += 18.0;
        }
        
        // Allow signal button
        let allow_btn_rect = egui::Rect::from_center_size(
            bg_rect.center_bottom() - egui::vec2(60.0, 25.0),
            egui::vec2(110.0, 32.0)
        );
        painter.rect_filled(allow_btn_rect, 4.0, egui::Color32::from_rgb(40, 35, 15));
        painter.rect_stroke(allow_btn_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
        painter.text(
            allow_btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            "allow signal",
            egui::FontId::monospace(13.0),
            egui::Color32::from_rgb(180, 140, 50)
        );
        
        let allow_response = ui.interact(
            allow_btn_rect,
            egui::Id::new("telemetry_allow_btn"),
            egui::Sense::click()
        );
        if allow_response.clicked() {
            telemetry_opt_in.active = false;
            telemetry_consent.opted_in = Some(true);
            telemetry_session_counter.sessions = 0; // Reset counter on choice
            // Trigger autosave
            save_events.send(SaveRequestEvent {
                name: "autosave".to_string(),
                category: SaveCategory::Auto,
                description: "Telemetry consent saved".to_string(),
            });
        }
        
        // Decline button
        let decline_btn_rect = egui::Rect::from_center_size(
            bg_rect.center_bottom() - egui::vec2(-60.0, 25.0),
            egui::vec2(110.0, 32.0)
        );
        painter.rect_filled(decline_btn_rect, 4.0, egui::Color32::from_rgb(40, 35, 15));
        painter.rect_stroke(decline_btn_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 140, 50)), egui::StrokeKind::Outside);
        painter.text(
            decline_btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            "decline",
            egui::FontId::monospace(13.0),
            egui::Color32::from_rgb(180, 140, 50)
        );
        
        let decline_response = ui.interact(
            decline_btn_rect,
            egui::Id::new("telemetry_decline_btn"),
            egui::Sense::click()
        );
        if decline_response.clicked() {
            telemetry_opt_in.active = false;
            telemetry_consent.opted_in = Some(false);
            telemetry_session_counter.sessions = 0; // Reset counter on choice
            // Trigger autosave
            save_events.send(SaveRequestEvent {
                name: "autosave".to_string(),
                category: SaveCategory::Auto,
                description: "Telemetry consent saved".to_string(),
            });
        }
        
        return;
    }
}
