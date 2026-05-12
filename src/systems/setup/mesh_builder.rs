use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use crate::components::OreDeposit;
use crate::config::BalanceConfig;

/// Generates a triangle mesh pointing up (+Y is forward in 2D).
pub fn triangle_mesh(w: f32, h: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    
    let vertices = vec![
        [0.0, h / 2.0, 0.0],
        [-w / 2.0, -h / 2.0, 0.0],
        [w / 2.0, -h / 2.0, 0.0],
    ];
    let normals = vec![[0.0, 0.0, 1.0]; 3];
    let uvs = vec![[0.5, 1.0], [0.0, 0.0], [1.0, 0.0]];
    let indices = vec![0, 1, 2];

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

/// Generates ore mesh based on ore type and seed.
pub fn generate_ore_mesh(ore: &OreDeposit, seed: u64, cfg: &BalanceConfig) -> Mesh {
    match ore {
        OreDeposit::Iron     => generate_iron_mesh_with_radius(seed, cfg.asteroid.radius_iron),
        OreDeposit::Tungsten => generate_tungsten_mesh_with_radius(seed, cfg.asteroid.radius_tungsten),
        OreDeposit::Nickel   => generate_nickel_mesh_with_radius(seed, cfg.asteroid.radius_nickel),
        OreDeposit::Aluminum => generate_iron_mesh_with_radius(seed, cfg.asteroid.radius_aluminum),
    }
}

/// Generates iron ore mesh with jagged edges.
pub fn generate_iron_mesh_with_radius(seed: u64, base_radius: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let vertex_count = rng.gen_range(8..=10);
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..vertex_count {
        let angle = (i as f32 / vertex_count as f32) * TAU;
        let radius = base_radius + rng.gen_range(-base_radius * 0.25..base_radius * 0.25);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / (base_radius * 2.0)) + 0.5, (y / (base_radius * 2.0)) + 0.5]);
    }
    let mut indices = Vec::new();
    for i in 1..vertex_count { indices.extend_from_slice(&[0, i, i + 1]); }
    indices.extend_from_slice(&[0, vertex_count, 1]);
    Mesh::new(PrimitiveTopology::TriangleList, Default::default()).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs).with_inserted_indices(Indices::U32(indices))
}

/// Generates tungsten ore mesh with blocky edges.
pub fn generate_tungsten_mesh_with_radius(seed: u64, base_radius: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let vertex_count = rng.gen_range(6..=8);
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..vertex_count {
        let angle = (i as f32 / vertex_count as f32) * TAU;
        let radius = base_radius + rng.gen_range(-base_radius * 0.08..base_radius * 0.08);
        // Blocky: modify angle to align more to grid
        let snappy_angle = (angle * (vertex_count as f32 / TAU)).round() * (TAU / vertex_count as f32);
        let x = snappy_angle.cos() * radius;
        let y = snappy_angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / (base_radius * 2.0)) + 0.5, (y / (base_radius * 2.0)) + 0.5]);
    }
    let mut indices = Vec::new();
    for i in 1..vertex_count { indices.extend_from_slice(&[0, i, i + 1]); }
    indices.extend_from_slice(&[0, vertex_count, 1]);
    Mesh::new(PrimitiveTopology::TriangleList, Default::default()).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs).with_inserted_indices(Indices::U32(indices))
}

/// Generates nickel ore mesh with many vertices.
pub fn generate_nickel_mesh_with_radius(seed: u64, base_radius: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let vertex_count = rng.gen_range(10..=12);
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..vertex_count {
        let angle = (i as f32 / vertex_count as f32) * TAU;
        let radius = base_radius + rng.gen_range(-base_radius * 0.15..base_radius * 0.15);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / (base_radius * 2.0)) + 0.5, (y / (base_radius * 2.0)) + 0.5]);
    }
    let mut indices = Vec::new();
    for i in 1..vertex_count { indices.extend_from_slice(&[0, i, i + 1]); }
    indices.extend_from_slice(&[0, vertex_count, 1]);
    Mesh::new(PrimitiveTopology::TriangleList, Default::default()).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs).with_inserted_indices(Indices::U32(indices))
}
