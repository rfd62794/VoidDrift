use bevy_egui::egui;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f32::consts::{PI, TAU};

pub struct ThrusterConfig {
    pub width: f32,
    pub color_nozzle: egui::Color32,
    pub color_body: egui::Color32,
    pub color_wire: egui::Color32,
    pub wire_count: usize,
    pub nozzle_width_ratio: f32,
    pub body_width_ratio: f32,
}

pub struct HullConfig {
    pub width: f32,
    pub rib_count: usize,
    pub color_frame: egui::Color32,
    pub color_outline: egui::Color32,
    pub stroke_width: f32,
}

pub struct CanisterConfig {
    pub width: f32,
    pub height: f32,
    pub lid_height_ratio: f32,
    pub color_body: egui::Color32,
    pub color_lid: egui::Color32,
    pub color_highlight: egui::Color32,
    pub color_handle: egui::Color32,
}

pub struct AICoreConfig {
    pub radius: f32,
    pub fin_count: usize,
    pub fin_length: f32,
    pub fin_width: f32,
    pub color_body: egui::Color32,
    pub color_fins: egui::Color32,
    pub color_fan_housing: egui::Color32,
    pub fan_radius_ratio: f32,
    pub fan_blade_count: usize,
}

pub struct RocketConfig {
    pub width: f32,
    pub height: f32,
    pub color_body: egui::Color32,
    pub color_nose: egui::Color32,
    pub color_fins: egui::Color32,
    pub color_exhaust: egui::Color32,
    pub nose_height_ratio: f32,
    pub fin_width_ratio: f32,
    pub fin_height_ratio: f32,
    pub exhaust_radius: f32,
    pub porthole_radius: f32,
    pub porthole_offset_y: f32,
}

pub struct DroneBayConfig {
    pub width: f32,
    pub height: f32,
    pub color_ready: egui::Color32,
    pub color_empty: egui::Color32,
    pub nose_height_ratio: f32,
    pub fin_width_ratio: f32,
    pub fin_height_ratio: f32,
    pub porthole_radius: f32,
    pub porthole_offset_y: f32,
    pub exhaust_radius: f32,
}

pub fn draw_rocket(painter: &egui::Painter, center: egui::Pos2, config: &RocketConfig) {
    let total_height = config.height;
    let nose_height = total_height * config.nose_height_ratio;
    let fin_height = total_height * config.fin_height_ratio;
    let body_width = config.width;
    let fin_width = body_width * config.fin_width_ratio;

    let top_y = center.y - total_height * 0.5;
    let nose_base_y = top_y + nose_height;
    let body_bottom_y = center.y + total_height * 0.5;

    let left_x = center.x - body_width * 0.5;
    let right_x = center.x + body_width * 0.5;

    // 1. Body — rectangle from nose base to bottom
    painter.rect_filled(
        egui::Rect::from_min_max(egui::pos2(left_x, nose_base_y), egui::pos2(right_x, body_bottom_y)),
        0.0,
        config.color_body,
    );

    // 2. Nose cone — triangle pointing up
    let nose_points = vec![
        egui::pos2(center.x, top_y),
        egui::pos2(left_x, nose_base_y),
        egui::pos2(right_x, nose_base_y),
    ];
    painter.add(egui::Shape::convex_polygon(
        nose_points,
        config.color_nose,
        egui::Stroke::NONE,
    ));

    // 3. Left fin — thin spike extending left
    let left_fin_points = vec![
        egui::pos2(left_x, body_bottom_y),
        egui::pos2(left_x - fin_width * 1.2, body_bottom_y + fin_height * 1.5),
        egui::pos2(left_x - fin_width * 0.8, body_bottom_y),
    ];
    painter.add(egui::Shape::convex_polygon(
        left_fin_points,
        config.color_fins,
        egui::Stroke::NONE,
    ));

    // 4. Right fin — thin spike extending right
    let right_fin_points = vec![
        egui::pos2(right_x, body_bottom_y),
        egui::pos2(right_x + fin_width * 1.2, body_bottom_y + fin_height * 1.5),
        egui::pos2(right_x + fin_width * 0.8, body_bottom_y),
    ];
    painter.add(egui::Shape::convex_polygon(
        right_fin_points,
        config.color_fins,
        egui::Stroke::NONE,
    ));

    // 5. Bottom center fin — thin spike extending straight down
    let bottom_fin_points = vec![
        egui::pos2(center.x - fin_width * 0.3, body_bottom_y),
        egui::pos2(center.x, body_bottom_y + fin_height * 1.5),
        egui::pos2(center.x + fin_width * 0.3, body_bottom_y),
    ];
    painter.add(egui::Shape::convex_polygon(
        bottom_fin_points,
        config.color_fins,
        egui::Stroke::NONE,
    ));

    // 6. Porthole — circle outline on body
    let porthole_center = egui::pos2(center.x, center.y + total_height * config.porthole_offset_y);
    painter.circle_stroke(
        porthole_center,
        config.porthole_radius,
        egui::Stroke::new(1.5, multiply_color(config.color_body, 1.4)),
    );

    // 7. Exhaust port — filled circle at bottom center
    painter.circle_filled(egui::pos2(center.x, body_bottom_y), config.exhaust_radius, config.color_exhaust);
}

pub fn draw_thruster(painter: &egui::Painter, center: egui::Pos2, config: &ThrusterConfig) {
    let total_width = config.width;
    let nozzle_width = total_width * config.nozzle_width_ratio;
    let body_width = total_width * config.body_width_ratio;
    let height = total_width * 0.45;

    let left_edge = center.x - total_width * 0.5;
    let join_x = left_edge + nozzle_width;
    let right_edge = center.x + total_width * 0.5;

    let nozzle_points = vec![
        egui::pos2(left_edge, center.y - height * 0.25),
        egui::pos2(join_x, center.y - height * 0.5),
        egui::pos2(join_x, center.y + height * 0.5),
        egui::pos2(left_edge, center.y + height * 0.25),
    ];

    painter.add(egui::Shape::convex_polygon(
        nozzle_points.clone(),
        config.color_nozzle,
        egui::Stroke::NONE,
    ));

    let body_rect = egui::Rect::from_min_max(
        egui::pos2(join_x, center.y - height * 0.5),
        egui::pos2(right_edge, center.y + height * 0.5),
    );
    painter.rect_filled(body_rect, 0.0, config.color_body);

    let mut rng = StdRng::seed_from_u64(42);
    for wire_idx in 0..config.wire_count {
        rng = StdRng::seed_from_u64(wire_idx as u64);
        let start_y = center.y + rng.gen_range(-height * 0.3..height * 0.3);
        let end_y = center.y + rng.gen_range(-height * 0.3..height * 0.3);

        let start_pos = egui::pos2(join_x + body_width * 0.1, start_y);
        let end_pos = egui::pos2(right_edge, end_y);

        painter.line_segment(
            [start_pos, end_pos],
            egui::Stroke::new(1.5, config.color_wire),
        );
    }

    painter.add(egui::Shape::convex_polygon(
        nozzle_points,
        egui::Color32::TRANSPARENT,
        egui::Stroke::new(1.0, multiply_color(config.color_body, 0.7)),
    ));
}

pub fn draw_hull(painter: &egui::Painter, center: egui::Pos2, config: &HullConfig) {
    let width = config.width;
    let height = width * 0.65;

    let left_x = center.x - width * 0.5;
    let _right_x = center.x + width * 0.5;
    let top_y = center.y - height * 0.5;
    let bottom_y = center.y + height * 0.5;

    let outline_stroke = egui::Stroke::new(config.stroke_width * 1.5, config.color_outline);

    let mut top_arc_points = Vec::new();
    for i in 0..=8 {
        let t = i as f32 / 8.0;
        let x = left_x + t * width;
        let angle = t * PI;
        let y = top_y - (angle.sin() * height * 0.5);
        top_arc_points.push(egui::pos2(x, y));
    }

    for i in 0..top_arc_points.len() - 1 {
        painter.line_segment(
            [top_arc_points[i], top_arc_points[i + 1]],
            outline_stroke,
        );
    }

    let mut bottom_arc_points = Vec::new();
    for i in 0..=8 {
        let t = i as f32 / 8.0;
        let x = left_x + t * width;
        let angle = t * PI;
        let y = bottom_y + (angle.sin() * height * 0.35);
        bottom_arc_points.push(egui::pos2(x, y));
    }

    for i in 0..bottom_arc_points.len() - 1 {
        painter.line_segment(
            [bottom_arc_points[i], bottom_arc_points[i + 1]],
            outline_stroke,
        );
    }

    let rib_stroke = egui::Stroke::new(config.stroke_width, config.color_frame);
    for rib_idx in 0..config.rib_count {
        let t = (rib_idx as f32 + 1.0) / (config.rib_count as f32 + 1.0);
        let x = left_x + t * width;

        let top_y_at_x = top_y - ((t * PI).sin() * height * 0.5);
        let bottom_y_at_x = bottom_y + ((t * PI).sin() * height * 0.35);

        let mut rib_points = Vec::new();
        for seg in 0..=5 {
            let s = seg as f32 / 5.0;
            let y = top_y_at_x + s * (bottom_y_at_x - top_y_at_x);
            let bulge = (s * PI).sin() * width * 0.08;
            rib_points.push(egui::pos2(x + bulge, y));
        }

        for i in 0..rib_points.len() - 1 {
            painter.line_segment([rib_points[i], rib_points[i + 1]], rib_stroke);
        }
    }
}

pub fn draw_canister(painter: &egui::Painter, center: egui::Pos2, config: &CanisterConfig) {
    let width = config.width;
    let total_height = config.height;
    let lid_height = total_height * config.lid_height_ratio;
    let body_height = total_height - lid_height;

    let body_rect = egui::Rect::from_min_max(
        egui::pos2(center.x - width * 0.5, center.y - body_height * 0.5 + lid_height * 0.5),
        egui::pos2(center.x + width * 0.5, center.y + body_height * 0.5 + lid_height * 0.5),
    );
    painter.rect_filled(body_rect, 2.0, config.color_body);

    let lid_rect = egui::Rect::from_min_max(
        egui::pos2(center.x - width * 0.52, center.y - total_height * 0.5 + lid_height),
        egui::pos2(center.x + width * 0.52, center.y - total_height * 0.5 + lid_height + lid_height),
    );
    painter.rect_filled(lid_rect, 2.0, config.color_lid);

    let highlight_color = multiply_color(config.color_highlight, 0.35);
    let highlight_rect = egui::Rect::from_min_max(
        egui::pos2(center.x - width * 0.25, center.y - body_height * 0.4 + lid_height * 0.5),
        egui::pos2(center.x - width * 0.12, center.y + body_height * 0.3 + lid_height * 0.5),
    );
    painter.rect_filled(highlight_rect, 1.0, highlight_color);

    let handle_center = egui::pos2(center.x, center.y - total_height * 0.5 + lid_height * 0.5);
    let handle_radius = width * 0.16;

    let mut handle_points = Vec::new();
    for i in 0..=6 {
        let t = i as f32 / 6.0;
        let angle = PI + t * PI;
        let x = handle_center.x + angle.cos() * handle_radius;
        let y = handle_center.y + angle.sin() * handle_radius;
        handle_points.push(egui::pos2(x, y));
    }

    for i in 0..handle_points.len() - 1 {
        painter.line_segment(
            [handle_points[i], handle_points[i + 1]],
            egui::Stroke::new(2.5, config.color_handle),
        );
    }

    painter.rect_stroke(body_rect, 2.0, egui::Stroke::new(1.0, multiply_color(config.color_body, 0.7)), egui::StrokeKind::Outside);
    painter.rect_stroke(lid_rect, 2.0, egui::Stroke::new(1.0, multiply_color(config.color_body, 0.7)), egui::StrokeKind::Outside);
}

pub fn draw_ai_core(painter: &egui::Painter, center: egui::Pos2, config: &AICoreConfig) {
    let body_radius = config.radius;
    let fan_radius = body_radius * config.fan_radius_ratio;

    painter.circle_filled(center, body_radius, config.color_body);

    for fin_idx in 0..config.fin_count {
        let angle = (fin_idx as f32 / config.fin_count as f32) * TAU;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let inner_point = egui::pos2(center.x + cos_a * body_radius, center.y + sin_a * body_radius);
        let outer_point = egui::pos2(
            center.x + cos_a * (body_radius + config.fin_length),
            center.y + sin_a * (body_radius + config.fin_length),
        );

        let perp_x = -sin_a * config.fin_width * 0.5;
        let perp_y = cos_a * config.fin_width * 0.5;

        let p1 = egui::pos2(inner_point.x + perp_x, inner_point.y + perp_y);
        let p2 = egui::pos2(inner_point.x - perp_x, inner_point.y - perp_y);
        let p3 = egui::pos2(outer_point.x - perp_x * 0.5, outer_point.y - perp_y * 0.5);
        let p4 = egui::pos2(outer_point.x + perp_x * 0.5, outer_point.y + perp_y * 0.5);

        painter.add(egui::Shape::convex_polygon(
            vec![p1, p4, p3, p2],
            config.color_fins,
            egui::Stroke::NONE,
        ));
    }

    painter.circle_filled(center, fan_radius, config.color_fan_housing);

    for blade_idx in 0..config.fan_blade_count {
        let angle = (blade_idx as f32 / config.fan_blade_count as f32) * TAU;

        let mut blade_points = Vec::new();
        for seg in 0..=4 {
            let s = seg as f32 / 4.0;
            let r = fan_radius * (0.15 + s * 0.85);
            let x = center.x + angle.cos() * r;
            let y = center.y + angle.sin() * r;
            blade_points.push(egui::pos2(x, y));
        }

        for i in 0..blade_points.len() - 1 {
            painter.line_segment(
                [blade_points[i], blade_points[i + 1]],
                egui::Stroke::new(1.5, multiply_color(config.color_fins, 0.7)),
            );
        }
    }

    painter.circle_stroke(center, body_radius, egui::Stroke::new(1.0, multiply_color(config.color_fins, 0.4)));
}

pub fn draw_drone_bay(painter: &egui::Painter, center: egui::Pos2, config: &DroneBayConfig, is_ready: bool) {
    let base_color = if is_ready { config.color_ready } else { config.color_empty };
    let exhaust_color = multiply_color(base_color, 0.6);

    let rocket_config = RocketConfig {
        width: config.width,
        height: config.height,
        color_body: base_color,
        color_nose: base_color,
        color_fins: base_color,
        color_exhaust: exhaust_color,
        nose_height_ratio: config.nose_height_ratio,
        fin_width_ratio: config.fin_width_ratio,
        fin_height_ratio: config.fin_height_ratio,
        exhaust_radius: config.exhaust_radius,
        porthole_radius: config.porthole_radius,
        porthole_offset_y: config.porthole_offset_y,
    };

    draw_rocket(painter, center, &rocket_config);
}

fn multiply_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    let [r, g, b, a] = color.to_srgba_unmultiplied();
    let r = ((r as f32 * factor).min(255.0)) as u8;
    let g = ((g as f32 * factor).min(255.0)) as u8;
    let b = ((b as f32 * factor).min(255.0)) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}
