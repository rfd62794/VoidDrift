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
    pub band_count: usize,
    pub band_width_min: f32,
    pub band_width_max: f32,
    pub grain_angle_deg: f32,
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

    let segment_size = (config.radius * 2.0) / (config.band_count + 1) as f32;

    for band_idx in 0..config.band_count {
        let band_center_y = -config.radius + (band_idx as f32 + 1.0) * segment_size;
        let band_center_offset = rng.gen_range(-segment_size * 0.2..segment_size * 0.2);
        let band_center = band_center_y + band_center_offset;

        let band_width = rng.gen_range(config.band_width_min..=config.band_width_max) * config.radius;

        for i in 0..32 {
            let angle = (i as f32 / 32.0) * TAU;
            let x = angle.cos() * config.radius * 1.2;
            let y = angle.sin() * config.radius * 1.2;

            let next_angle = ((i + 1) as f32 / 32.0) * TAU;
            let next_x = next_angle.cos() * config.radius * 1.2;
            let next_y = next_angle.sin() * config.radius * 1.2;

            let dist_to_band = (y - band_center).abs();
            let next_dist_to_band = (next_y - band_center).abs();

            if dist_to_band < band_width || next_dist_to_band < band_width {
                let p1 = egui::pos2(center.x + x, center.y + y);
                let p2 = egui::pos2(center.x + next_x, center.y + next_y);
                painter.line_segment(
                    [p1, p2],
                    egui::Stroke::new(band_width, config.color_vein),
                );
            }
        }
    }
}
