use bevy_egui::egui;

pub struct IngotNodeConfig {
    pub width: f32,
    pub height: f32,
    pub depth_offset_x: f32,
    pub depth_offset_y: f32,
    pub color_face_light_factor: f32,
    pub color_face_dark_factor: f32,
}

pub fn draw_ingot_node(
    painter: &egui::Painter,
    center: egui::Pos2,
    config: &IngotNodeConfig,
    base_color: egui::Color32,
) {
    let depth_face_color = multiply_color(base_color, config.color_face_dark_factor);
    let side_face_color = multiply_color(base_color, 0.85);
    let front_face_color = multiply_color(base_color, config.color_face_light_factor);

    let depth_offset = egui::vec2(config.depth_offset_x, config.depth_offset_y);
    let side_offset = depth_offset * 0.5;

    let depth_center = center + depth_offset;
    let side_center = center + side_offset;
    let front_center = center;

    let depth_rect = egui::Rect::from_center_size(depth_center, egui::vec2(config.width, config.height));
    let side_rect = egui::Rect::from_center_size(side_center, egui::vec2(config.width, config.height));
    let front_rect = egui::Rect::from_center_size(front_center, egui::vec2(config.width, config.height));

    painter.rect_filled(depth_rect, 2.0, depth_face_color);
    painter.rect_filled(side_rect, 2.0, side_face_color);
    painter.rect_filled(front_rect, 2.0, front_face_color);
}

fn multiply_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    let [r, g, b, a] = color.to_srgba_unmultiplied();
    let r = ((r as f32 * factor).min(255.0)) as u8;
    let g = ((g as f32 * factor).min(255.0)) as u8;
    let b = ((b as f32 * factor).min(255.0)) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}
