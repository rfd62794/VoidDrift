use bevy_egui::egui;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f32::consts::TAU;

pub struct OrePolygonConfig {
    pub radius: f32,
    pub vertex_count: usize,
    pub jaggedness: f32,
    pub color_body: egui::Color32,
    pub color_vein: egui::Color32,
    pub vein_count: usize,
    pub vein_width: f32,
    pub seed: u64,
}

pub fn draw_ore_polygon(painter: &egui::Painter, center: egui::Pos2, config: &OrePolygonConfig) {
    let mut rng = StdRng::seed_from_u64(config.seed);

    let mut vertices = Vec::new();
    for i in 0..config.vertex_count {
        let angle = (i as f32 / config.vertex_count as f32) * TAU;
        let radial_offset = rng.gen_range(
            (config.radius * (1.0 - config.jaggedness))
                ..=(config.radius * (1.0 + config.jaggedness * 0.5)),
        );
        let x = angle.cos() * radial_offset;
        let y = angle.sin() * radial_offset;
        vertices.push(egui::Pos2::new(center.x + x, center.y + y));
    }

    painter.add(egui::Shape::convex_polygon(
        vertices.clone(),
        config.color_body,
        egui::Stroke::NONE,
    ));

    rng = StdRng::seed_from_u64(config.seed);
    for _ in 0..config.vein_count {
        let start_angle = rng.gen_range(0.0..TAU);
        let start_radius = config.radius * rng.gen_range(0.05..0.2);
        let start_x = start_angle.cos() * start_radius;
        let start_y = start_angle.sin() * start_radius;
        let start_pos = egui::Pos2::new(center.x + start_x, center.y + start_y);

        let end_angle = rng.gen_range(0.0..TAU);
        let end_vertex_idx = rng.gen_range(0..vertices.len());
        let end_pos = vertices[end_vertex_idx];

        let mid_x = (start_pos.x + end_pos.x) / 2.0;
        let mid_y = (start_pos.y + end_pos.y) / 2.0;
        let perp_angle = end_angle + std::f32::consts::PI / 2.0;
        let ctrl1_offset = rng.gen_range(-config.radius * 0.25..config.radius * 0.25);
        let ctrl1_x = mid_x + perp_angle.cos() * ctrl1_offset;
        let ctrl1_y = mid_y + perp_angle.sin() * ctrl1_offset;
        let ctrl1 = egui::Pos2::new(ctrl1_x, ctrl1_y);

        let t75_x = start_pos.x + (end_pos.x - start_pos.x) * 0.75;
        let t75_y = start_pos.y + (end_pos.y - start_pos.y) * 0.75;
        let ctrl2_offset = rng.gen_range(-config.radius * 0.15..config.radius * 0.15);
        let ctrl2_x = t75_x + perp_angle.cos() * ctrl2_offset;
        let ctrl2_y = t75_y + perp_angle.sin() * ctrl2_offset;
        let ctrl2 = egui::Pos2::new(ctrl2_x, ctrl2_y);

        let segment_count = 6;
        for i in 0..segment_count {
            let t0 = i as f32 / segment_count as f32;
            let t1 = (i + 1) as f32 / segment_count as f32;

            let p0 = quadratic_bezier(start_pos, ctrl1, ctrl2, end_pos, t0);
            let p1 = quadratic_bezier(start_pos, ctrl1, ctrl2, end_pos, t1);

            let width0 = config.vein_width * (1.0 - t0 * 0.6);
            let width1 = config.vein_width * (1.0 - t1 * 0.6);

            painter.line_segment(
                [p0, p1],
                egui::Stroke::new((width0 + width1) / 2.0, config.color_vein),
            );
        }
    }
}

fn quadratic_bezier(p0: egui::Pos2, cp1: egui::Pos2, cp2: egui::Pos2, p3: egui::Pos2, t: f32) -> egui::Pos2 {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;

    let x = mt2 * mt * p0.x + 3.0 * mt2 * t * cp1.x + 3.0 * mt * t2 * cp2.x + t2 * t * p3.x;
    let y = mt2 * mt * p0.y + 3.0 * mt2 * t * cp1.y + 3.0 * mt * t2 * cp2.y + t2 * t * p3.y;

    egui::Pos2::new(x, y)
}
