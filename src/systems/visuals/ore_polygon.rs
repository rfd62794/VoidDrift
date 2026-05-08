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
        let start_radius = config.radius * rng.gen_range(0.1..0.4);
        let start_x = start_angle.cos() * start_radius;
        let start_y = start_angle.sin() * start_radius;
        let start_pos = egui::Pos2::new(center.x + start_x, center.y + start_y);

        let end_angle = rng.gen_range(0.0..TAU);
        let end_radius = config.radius * rng.gen_range(0.7..1.0);
        let end_x = end_angle.cos() * end_radius;
        let end_y = end_angle.sin() * end_radius;
        let end_pos = egui::Pos2::new(center.x + end_x, center.y + end_y);

        painter.line_segment(
            [start_pos, end_pos],
            egui::Stroke::new(config.vein_width, config.color_vein),
        );
    }
}
