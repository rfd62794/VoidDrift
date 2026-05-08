# VoidDrift — Sprint 7: Bevy Mesh Generation
**Date:** May 2026  
**Status:** Ready for Agent Execution  
**Owner:** Robert (rfd62794)  
**Branch:** `feature/sprint7-bevy-mesh-generation`  
**Builds Required:** WASM + Android (Moto G)

---

## Objective

Bring the procedural visual language from the Production Tree into the space view. The same shapes that represent ore and rockets in egui are now generated as Bevy meshes and applied to world entities — asteroids and drone ships.

The core deliverable is a shared mesh utility that converts polygon point arrays into Bevy-renderable geometry. Everything else in this sprint calls into that utility.

---

## Architecture

### Why This Sprint Exists

The Production Tree renders via egui painter (UI layer). The space view renders via Bevy mesh (world layer). These are separate rendering systems — egui painter calls cannot be used on Bevy world entities. Sprint 7 bridges them by generating Bevy meshes from the same point arrays that drive the egui visuals.

### The Core Pattern

```
Point array (Vec2[])
       ↓
build_mesh_from_polygon()
       ↓
Bevy Mesh (triangle list)
       ↓
Mesh2d + MeshMaterial2d on entity
```

One utility function. All procedural world visuals call into it.

---

## Scope

### IN SCOPE
- `build_mesh_from_polygon()` shared utility
- Rocket mesh for drone ships (green) and opening sequence ship (orange)
- Asteroid mesh replacing current triangle mesh — body polygon + ore band quads
- All colors driven by `visual.toml` via Bevy materials
- Rotation support for drone ships (nose points direction of travel)

### OUT OF SCOPE
- Production Tree changes (egui layer, untouched)
- Audio
- New mechanics or game logic
- Any UI layout changes
- Bevy PBR or 3D rendering — 2D meshes only

---

## Part A: Mesh Utility Module

### A-1: Create `src/systems/visuals/mesh_builder.rs`

```rust
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

/// Build a mesh from a quad (4 points, two triangles).
/// Used for ore band strips.
pub fn build_mesh_from_quad(points: &[Vec2; 4]) -> Mesh {
    build_mesh_from_polygon(points)
}
```

Add `mod mesh_builder;` and `pub use mesh_builder::*;` to `src/systems/visuals/mod.rs`.

---

## Part B: Rocket Mesh Generation

### B-1: Create `generate_rocket_points()` in `src/systems/visuals/mesh_builder.rs`

Mirrors the `draw_rocket` egui geometry but outputs Vec2 arrays instead of painter calls. Takes the same `RocketConfig` struct already defined in `component_nodes.rs`.

```rust
pub struct RocketMeshParts {
    pub body: Vec<Vec2>,       // rectangle polygon (4 points)
    pub nose: Vec<Vec2>,       // triangle (3 points)
    pub fin_left: Vec<Vec2>,   // triangle (3 points)
    pub fin_right: Vec<Vec2>,  // triangle (3 points)
    // porthole and exhaust handled as separate Circle meshes
}

pub fn generate_rocket_points(config: &RocketConfig) -> RocketMeshParts {
    let half_w      = config.width * 0.5;
    let half_h      = config.height * 0.5;
    let nose_h      = config.height * config.nose_height_ratio;
    let body_top_y  = -half_h + nose_h;
    let fin_h       = config.height * config.fin_height_ratio;
    let fin_w       = config.width  * config.fin_width_ratio;

    RocketMeshParts {
        body: vec![
            Vec2::new(-half_w, body_top_y),
            Vec2::new( half_w, body_top_y),
            Vec2::new( half_w, half_h),
            Vec2::new(-half_w, half_h),
        ],
        nose: vec![
            Vec2::new(0.0,    -half_h),
            Vec2::new(-half_w, body_top_y),
            Vec2::new( half_w, body_top_y),
        ],
        fin_left: vec![
            Vec2::new(-half_w,         half_h - fin_h),
            Vec2::new(-half_w,         half_h),
            Vec2::new(-half_w - fin_w, half_h),
        ],
        fin_right: vec![
            Vec2::new(half_w,         half_h - fin_h),
            Vec2::new(half_w,         half_h),
            Vec2::new(half_w + fin_w, half_h),
        ],
    }
}
```

### B-2: Spawn rocket meshes on drone entities

In `ship_spawn.rs`, replace the existing triangle mesh with rocket mesh parts. Each rocket part (body, nose, fin_left, fin_right) becomes a child entity with its own mesh and material — this allows per-part coloring.

```rust
pub fn spawn_drone_ship(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    visual_config: &VisualConfig,
    position: Vec2,
    heading: f32,
) {
    let config = RocketConfig::from(&visual_config.ship.drone);
    let parts = generate_rocket_points(&config);

    let parent = commands.spawn((
        Transform::from_translation(position.extend(0.0))
            .with_rotation(Quat::from_rotation_z(heading)),
        Visibility::default(),
        // existing drone components...
    )).id();

    // Spawn each part as child
    spawn_rocket_part(commands, meshes, materials, parent,
        &parts.body,     config.color_body,  1.0);
    spawn_rocket_part(commands, meshes, materials, parent,
        &parts.nose,     config.color_nose,  1.5);
    spawn_rocket_part(commands, meshes, materials, parent,
        &parts.fin_left,  config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, parent,
        &parts.fin_right, config.color_fins, 0.5);
}

fn spawn_rocket_part(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    parent: Entity,
    points: &[Vec2],
    color: Color32,
    z_offset: f32,
) {
    let mesh = build_mesh_from_polygon(points);
    let material = ColorMaterial::from(Color::srgba_u8(
        color.r(), color.g(), color.b(), color.a()
    ));
    
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(material)),
        Transform::from_translation(Vec3::Z * z_offset),
    )).set_parent(parent);
}
```

Apply same pattern to `spawn_bottle_drone` and the opening sequence ship — use `visual_config.ship.opening` for the orange ship.

### B-3: Heading rotation

Drone heading is already tracked. Apply it as `Quat::from_rotation_z(heading)` on the parent entity transform. Child mesh parts inherit the rotation automatically. Update heading each frame in the drone movement system where it already exists.

---

## Part C: Asteroid Mesh Generation

### C-1: Create `generate_ore_polygon_points()` in `mesh_builder.rs`

Mirrors the egui ore polygon algorithm but outputs Vec2 arrays. Same config, same seed, same shape.

```rust
pub fn generate_ore_polygon_points(
    radius: f32,
    vertex_count: usize,
    jaggedness: f32,
    seed: u64,
) -> Vec<Vec2> {
    use rand::{SeedableRng, Rng};
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

    (0..vertex_count).map(|i| {
        let angle = (i as f32 / vertex_count as f32) * std::f32::consts::TAU;
        let min_r = radius * (1.0 - jaggedness);
        let max_r = radius * (1.0 + jaggedness * 0.5);
        let r = rng.gen_range(min_r..max_r);
        Vec2::new(r * angle.cos(), r * angle.sin())
    }).collect()
}
```

### C-2: Generate ore band quads

```rust
pub struct OreBandConfig {
    pub radius: f32,
    pub band_count: usize,
    pub band_width_min: f32,   // fraction of radius
    pub band_width_max: f32,
    pub grain_angle_deg: f32,
    pub seed: u64,
}

pub fn generate_ore_band_quads(config: &OreBandConfig) -> Vec<[Vec2; 4]> {
    use rand::{SeedableRng, Rng};
    let mut rng = rand::rngs::SmallRng::seed_from_u64(config.seed + 100);

    let angle = config.grain_angle_deg.to_radians();
    let span = config.radius * 2.2;  // slightly wider than polygon to ensure full coverage
    let step = span / (config.band_count + 1) as f32;

    (0..config.band_count).map(|i| {
        let center = -span * 0.5 + step * (i + 1) as f32;
        let center = center + rng.gen_range(-step * 0.15..step * 0.15);
        let half_w = config.radius * rng.gen_range(config.band_width_min..config.band_width_max);

        // Quad in grain-rotated space, then rotate back
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let rotate = |p: Vec2| Vec2::new(
            p.x * cos_a - p.y * sin_a,
            p.x * sin_a + p.y * cos_a,
        );

        [
            rotate(Vec2::new(-span * 0.5, center - half_w)),
            rotate(Vec2::new( span * 0.5, center - half_w)),
            rotate(Vec2::new( span * 0.5, center + half_w)),
            rotate(Vec2::new(-span * 0.5, center + half_w)),
        ]
    }).collect()
}
```

### C-3: Replace asteroid mesh at spawn

In the asteroid spawn system, replace `generate_ore_mesh` with procedural mesh generation:

```rust
pub fn spawn_asteroid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    visual_config: &VisualConfig,
    ore_type: OreType,
    ring: AsteroidRing,
    position: Vec2,
    entity_id: u64,  // used as polygon seed
) {
    let ring_config = visual_config.asteroid_ring(ring);
    let ore_config  = visual_config.ore(ore_type);

    // Body polygon
    let body_points = generate_ore_polygon_points(
        ring_config.radius,
        ring_config.vertex_count,
        ring_config.jaggedness,
        entity_id,
    );
    let body_mesh = build_mesh_from_polygon(&body_points);
    let body_color = Color::srgb_u8(
        ore_config.color_body[0],
        ore_config.color_body[1],
        ore_config.color_body[2],
    );

    // Band quads
    let band_config = OreBandConfig {
        radius: ring_config.radius,
        band_count: ore_config.band_count,
        band_width_min: ore_config.band_width_min,
        band_width_max: ore_config.band_width_max,
        grain_angle_deg: ore_config.grain_angle_deg,
        seed: entity_id,
    };
    let band_quads = generate_ore_band_quads(&band_config);
    let vein_color = Color::srgb_u8(
        ore_config.color_vein[0],
        ore_config.color_vein[1],
        ore_config.color_vein[2],
    );

    let parent = commands.spawn((
        Transform::from_translation(position.extend(0.0)),
        Visibility::default(),
        // existing asteroid components...
    )).id();

    // Body mesh (z = 0.0)
    commands.spawn((
        Mesh2d(meshes.add(body_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(body_color))),
        Transform::from_translation(Vec3::Z * 0.0),
    )).set_parent(parent);

    // Band meshes (z = 1.0, renders on top of body)
    for quad in &band_quads {
        let band_mesh = build_mesh_from_quad(quad);
        commands.spawn((
            Mesh2d(meshes.add(band_mesh)),
            MeshMaterial2d(materials.add(ColorMaterial::from(vein_color))),
            Transform::from_translation(Vec3::Z * 1.0),
        )).set_parent(parent);
    }
}
```

### C-4: Depleted state

When asteroid depletes, update body and band materials to use body color at 50% alpha and vein color at 20% alpha. Do not regenerate meshes — just update the material handles on existing child entities.

Tag asteroid child entities with `AsteroidBody` and `AsteroidBand` marker components so depletion system can query them.

---

## Part D: visual.toml Verification

Confirm these entries exist and are correct before spawning:

```toml
[ship.drone]
width              = 14.0
height             = 28.0
color_body         = [60, 180, 100]
color_nose         = [80, 210, 120]
color_fins         = [40, 140, 75]
color_exhaust      = [30, 100, 55]
nose_height_ratio  = 0.28
fin_width_ratio    = 0.25
fin_height_ratio   = 0.20
exhaust_radius     = 2.5
porthole_radius    = 2.0
porthole_offset_y  = -0.15

[ship.opening]
width              = 18.0
height             = 36.0
color_body         = [210, 95, 30]
color_nose         = [230, 115, 45]
color_fins         = [160, 70, 20]
color_exhaust      = [120, 50, 15]
nose_height_ratio  = 0.28
fin_width_ratio    = 0.25
fin_height_ratio   = 0.20
exhaust_radius     = 3.0
porthole_radius    = 2.5
porthole_offset_y  = -0.15
```

---

## Acceptance Criteria

### E-1: Asteroid meshes — WASM
- [ ] Asteroids render as procedural polygon shapes with colored ore banding
- [ ] Three ore types visually distinct at a glance
- [ ] Depleted asteroids fade to low opacity — same shape
- [ ] No previous mesh/sprite visible

**Screenshot required:** Space view showing all three asteroid types

### E-2: Asteroid meshes — Android
- [ ] Same as E-1 confirmed on device
- [ ] 60 FPS maintained with multiple asteroids visible

**Screenshot required:** Device screenshot

### E-3: Drone ships — WASM
- [ ] Deployed drones render as small green upright rockets
- [ ] Nose rotates to face direction of travel
- [ ] No triangle placeholder visible

**Screenshot required:** Space view with drone in transit

### E-4: Opening sequence ship — WASM
- [ ] Opening ship renders as orange rocket, larger than drones
- [ ] Correct heading during opening cinematic

**Screenshot required:** Opening sequence with orange rocket visible

### E-5: No regressions
- [ ] Tutorial T-101 → T-106 completes
- [ ] Mining loop functional
- [ ] Production Tree unchanged
- [ ] Save/load unaffected

---

## Commit & Tag

```
git add -A
git commit -m "feat: Sprint 7 Bevy mesh generation — procedural asteroid and rocket meshes in space view"
git tag v3.3.0-sprint7-bevy-mesh
git push origin feature/sprint7-bevy-mesh-generation --tags
.\publish.ps1 -Build
```

Confirm itch.io build number changed post-publish.

---

## Notes for Agent

- `build_mesh_from_polygon` uses fan triangulation — valid for convex polygons only. Ore polygons are generated as convex shapes so this is safe.
- Band quads are always convex (4-point rectangles rotated) — fan triangulation works.
- Child entity z-offsets control draw order: body at 0.0, bands at 1.0, rocket nose at 1.5
- Seed for asteroids: use the Bevy entity ID bits as u64. This is stable for the entity's lifetime.
- Do not regenerate meshes on depletion — only update material colors/alpha
- `AsteroidBody` and `AsteroidBand` are marker components (empty structs) for query targeting
- Do not modify egui Production Tree rendering under any circumstances

---

*VoidDrift Sprint 7 Directive*  
*May 2026 — RFD IT Services Ltd.*
