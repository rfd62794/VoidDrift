use bevy_egui::egui;
use crate::ui_kit::styles::{ButtonStyle, HighlightKind};

pub fn vd_button(
    ui: &mut egui::Ui,
    label: &str,
    style: ButtonStyle,
    enabled: bool,
    highlight: Option<HighlightKind>,
) -> egui::Response {
    // Wire text_color (None = inherit ui.style())
    let text = match style.text_color {
        Some(c) => egui::RichText::new(label).color(c),
        None    => egui::RichText::new(label),
    };

    // Build button. Stroke applied unconditionally — zero-width transparent
    // stroke is a visual no-op, and egui::Stroke does not impl PartialEq.
    let mut button = egui::Button::new(text)
        .min_size(style.min_size)
        .stroke(style.stroke)
        .corner_radius(egui::CornerRadius::same(style.corner_radius));

    if let Some(fill) = style.fill {
        button = button.fill(fill);
    }

    // Disabled state delegates to egui::Visuals::widgets.noninteractive.
    let response = ui.add_enabled(enabled, button);

    // Amber pulse — replicates timing/alpha from
    // src/systems/ui/hud/buttons.rs:39-51, but scales width to 2.5× the
    // button's actual width (existing call site is a 200px overlay on an
    // 80px button = 2.5×) so the visual extension ratio is preserved
    // for any button size.
    if let Some(HighlightKind::Amber) = highlight {
        let t = ui.ctx().input(|i| i.time as f32);
        let alpha = ((t * 2.0).sin() * 0.3 + 0.7) * 255.0;
        let pulse_width = response.rect.width() * 2.5;
        let center_rect = egui::Rect::from_center_size(
            response.rect.center(),
            egui::vec2(pulse_width, response.rect.height()),
        );
        let layer_id = egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new(("vd_button_amber_pulse", response.id)),
        );
        let painter = ui.ctx().layer_painter(layer_id);
        painter.rect_stroke(
            center_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(8.0, egui::Color32::from_rgba_unmultiplied(255, 200, 50, alpha as u8)),
            egui::StrokeKind::Outside,
        );
    }

    response
}
