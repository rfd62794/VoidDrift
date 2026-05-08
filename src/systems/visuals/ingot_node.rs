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
    let w = config.width / 2.0;
    let h = config.height / 2.0;
    let dx = config.depth_offset_x;
    let dy = config.depth_offset_y;

    let depth_face_color = multiply_color(base_color, config.color_face_dark_factor);
    let side_face_color = multiply_color(base_color, 0.85);
    let front_face_color = multiply_color(base_color, config.color_face_light_factor);

    let top_left = egui::pos2(center.x - w + dx, center.y - h + dy);
    let top_right = egui::pos2(center.x + w + dx, center.y - h + dy);
    let bottom_right = egui::pos2(center.x + w, center.y - h);
    let bottom_left = egui::pos2(center.x - w, center.y - h);

    painter.add(egui::Shape::convex_polygon(
        vec![top_left, top_right, bottom_right, bottom_left],
        depth_face_color,
        egui::Stroke::NONE,
    ));

    let right_top_left = egui::pos2(center.x + w, center.y - h);
    let right_top_right = egui::pos2(center.x + w + dx, center.y - h + dy);
    let right_bottom_right = egui::pos2(center.x + w + dx, center.y + h + dy);
    let right_bottom_left = egui::pos2(center.x + w, center.y + h);

    painter.add(egui::Shape::convex_polygon(
        vec![right_top_left, right_top_right, right_bottom_right, right_bottom_left],
        side_face_color,
        egui::Stroke::NONE,
    ));

    let front_top_left = egui::pos2(center.x - w, center.y - h);
    let front_top_right = egui::pos2(center.x + w, center.y - h);
    let front_bottom_right = egui::pos2(center.x + w, center.y + h);
    let front_bottom_left = egui::pos2(center.x - w, center.y + h);

    painter.add(egui::Shape::convex_polygon(
        vec![front_top_left, front_top_right, front_bottom_right, front_bottom_left],
        front_face_color,
        egui::Stroke::NONE,
    ));
}

fn multiply_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    let [r, g, b, a] = color.to_srgba_unmultiplied();
    let r = ((r as f32 * factor).min(255.0)) as u8;
    let g = ((g as f32 * factor).min(255.0)) as u8;
    let b = ((b as f32 * factor).min(255.0)) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}
