use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

/// Build a filled 2D mesh from an ordered polygon point array.
/// Points must be convex or simple (no self-intersections).
/// Uses fan triangulation from point[0].
pub fn build_mesh_from_polygon(points: &[Vec2]) -> Mesh {
    assert!(points.len() >= 3, "Polygon requires at least 3 points");

    let vertices: Vec<[f32; 3]> = points
        .iter()
        .map(|p| [p.x, p.y, 0.0])
        .collect();

    // Fan triangulation: triangle (0, i, i+1) for i in 1..n-1
    let mut indices: Vec<u32> = Vec::new();
    for i in 1..(points.len() as u32 - 1) {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; points.len()];
    let uvs: Vec<[f32; 2]> = points
        .iter()
        .map(|p| [p.x, p.y])
        .collect();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Build a filled 2D mesh from an ordered polygon point array with vertex colors.
/// Colors array must match points length. Alternates between body and vein colors to create banding effect.
pub fn build_mesh_from_polygon_with_colors(points: &[Vec2], colors: &[Color]) -> Mesh {
    assert!(points.len() >= 3, "Polygon requires at least 3 points");
    assert_eq!(points.len(), colors.len(), "Points and colors must have same length");

    let vertices: Vec<[f32; 3]> = points
        .iter()
        .map(|p| [p.x, p.y, 0.0])
        .collect();

    // Fan triangulation: triangle (0, i, i+1) for i in 1..n-1
    let mut indices: Vec<u32> = Vec::new();
    for i in 1..(points.len() as u32 - 1) {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; points.len()];
    let uvs: Vec<[f32; 2]> = points
        .iter()
        .map(|p| [p.x, p.y])
        .collect();

    let vertex_colors: Vec<[f32; 4]> = colors
        .iter()
        .map(|c| [c.r(), c.g(), c.b(), c.a()])
        .collect();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Build a mesh from a quad (4 points, two triangles).
/// Used for ore band strips.
pub fn build_mesh_from_quad(points: &[Vec2; 4]) -> Mesh {
    build_mesh_from_polygon(points)
}

/// Generate procedural ore polygon points using jagged radial distribution.
/// Deterministic based on seed for stable shapes across frames.
pub fn generate_ore_polygon_points(
    radius: f32,
    vertex_count: usize,
    jaggedness: f32,
    seed: u64,
) -> Vec<Vec2> {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;
    let mut rng = StdRng::seed_from_u64(seed);

    (0..vertex_count)
        .map(|i| {
            let angle = (i as f32 / vertex_count as f32) * std::f32::consts::TAU;
            let min_r = radius * (1.0 - jaggedness);
            let max_r = radius * (1.0 + jaggedness * 0.5);
            let r = rng.gen_range(min_r..max_r);
            Vec2::new(r * angle.cos(), r * angle.sin())
        })
        .collect()
}

/// Configuration for ore band generation.
pub struct OreBandConfig {
    pub radius: f32,
    pub band_count: usize,
    pub band_width_min: f32,
    pub band_width_max: f32,
    pub grain_angle_deg: f32,
    pub seed: u64,
}

/// Generate ore band quads with grain direction and organic edges.
/// Returns array of 4-point quads (rectangles) that render as bands across the ore polygon.
pub fn generate_ore_band_quads(config: &OreBandConfig) -> Vec<[Vec2; 4]> {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;
    let mut rng = StdRng::seed_from_u64(config.seed + 100);

    let angle = config.grain_angle_deg.to_radians();
    let span = config.radius * 2.2; // slightly wider than polygon to ensure full coverage
    let step = span / (config.band_count + 1) as f32;

    (0..config.band_count)
        .map(|i| {
            let center = -span * 0.5 + step * (i + 1) as f32;
            let center = center + rng.gen_range(-step * 0.15..step * 0.15);
            let half_w = config.radius * rng.gen_range(config.band_width_min..config.band_width_max);

            // Quad in grain-rotated space, then rotate back
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            let rotate = |p: Vec2| Vec2::new(p.x * cos_a - p.y * sin_a, p.x * sin_a + p.y * cos_a);

            [
                rotate(Vec2::new(-span * 0.5, center - half_w)),
                rotate(Vec2::new(span * 0.5, center - half_w)),
                rotate(Vec2::new(span * 0.5, center + half_w)),
                rotate(Vec2::new(-span * 0.5, center + half_w)),
            ]
        })
        .collect()
}

/// Rocket mesh parts — body, nose, and three fins as separate polygons.
pub struct RocketMeshParts {
    pub body: Vec<Vec2>,
    pub nose: Vec<Vec2>,
    pub fin_left: Vec<Vec2>,
    pub fin_right: Vec<Vec2>,
    pub fin_center: Vec<Vec2>,
}

/// Generate rocket mesh point arrays from RocketConfig.
/// Mirrors the egui draw_rocket geometry but outputs Vec2 arrays.
/// Rocket nose points in positive Y direction (forward).
pub fn generate_rocket_points(config: &crate::systems::visuals::component_nodes::RocketConfig) -> RocketMeshParts {
    let half_w = config.width * 0.5;
    let half_h = config.height * 0.5;
    let nose_h = config.height * config.nose_height_ratio;
    let body_bottom_y = half_h - nose_h;
    let fin_h = config.height * config.fin_height_ratio;
    let fin_w = config.width * config.fin_width_ratio;

    RocketMeshParts {
        body: vec![
            Vec2::new(-half_w, body_bottom_y),
            Vec2::new(half_w, body_bottom_y),
            Vec2::new(half_w, -half_h),
            Vec2::new(-half_w, -half_h),
        ],
        nose: vec![
            Vec2::new(0.0, half_h),
            Vec2::new(-half_w, body_bottom_y),
            Vec2::new(half_w, body_bottom_y),
        ],
        fin_left: vec![
            Vec2::new(-half_w, -half_h + fin_h),
            Vec2::new(-half_w, -half_h),
            Vec2::new(-half_w - fin_w * 1.2, -half_h - fin_h * 1.5),
        ],
        fin_right: vec![
            Vec2::new(half_w, -half_h + fin_h),
            Vec2::new(half_w, -half_h),
            Vec2::new(half_w + fin_w * 1.2, -half_h - fin_h * 1.5),
        ],
        fin_center: vec![
            Vec2::new(-fin_w * 0.3, -half_h),
            Vec2::new(fin_w * 0.3, -half_h),
            Vec2::new(0.0, -half_h - fin_h * 1.5),
        ],
    }
}
