# VoidDrift — Sprint 5: Visual Overhaul Directive
**Date:** May 2026  
**Status:** Ready for Agent Execution  
**Owner:** Robert (rfd62794)  
**Branch:** `feature/sprint5-visual-overhaul`  
**Builds Required:** WASM + Android (Moto G)

---

## Objective

Replace all asteroid visuals and Production Tree ore/ingot node visuals with a unified procedural language. The same procedural ore polygon that represents an asteroid in the space view represents an ore node in the Production Tree — same algorithm, scaled by context. Visual continuity between both viewports is the goal.

No new mechanics. No new systems. Pure visual work.

---

## Scope

### IN SCOPE
- Procedural ore polygon — shared algorithm used in both space view (asteroids) and Production Tree (ore nodes)
- Three asteroid size tiers (Inner / Middle / Outer Ring)
- Three ore types with distinct vein colors (Metal / H3 Gas / Void Essence)
- Ingot nodes in Production Tree (3-rect 3D block with depth)
- Destruction particles on asteroid depletion (4–6 fragments, fading alpha)
- All colors and shape parameters driven by `visual.toml`

### OUT OF SCOPE
- Zoom/scroll on Production Tree (deferred — separate sprint)
- Locked ??? Layer 2/3 nodes (deferred — separate sprint)
- New asteroid mechanics or mining logic changes
- Audio changes
- Any UI layout changes

---

## Part A: visual.toml — New Config Entries

Add the following sections to `assets/visual.toml`. All procedural rendering reads from here. No hardcoded values in Rust.

```toml
[ore.metal]
color_body     = [120, 120, 130]   # grey-silver base
color_vein     = [200, 210, 255]   # pale blue-white veins
vein_count     = 4
vein_width     = 1.5

[ore.h3_gas]
color_body     = [60, 100, 80]     # dark teal base
color_vein     = [80, 255, 160]    # bright green veins
vein_count     = 5
vein_width     = 1.5

[ore.void_essence]
color_body     = [30, 20, 45]      # near-black purple base
color_vein     = [160, 80, 255]    # violet veins
vein_count     = 6
vein_width     = 2.0

[asteroid.inner_ring]
radius         = 18.0
vertex_count   = 9
jaggedness     = 0.35              # 0.0 = circle, 1.0 = very jagged

[asteroid.middle_ring]
radius         = 28.0
vertex_count   = 11
jaggedness     = 0.40

[asteroid.outer_ring]
radius         = 42.0
vertex_count   = 13
jaggedness     = 0.45

[production_tree.ore_node]
radius         = 22.0
vertex_count   = 9
jaggedness     = 0.35              # matches inner_ring — same shape, same scale

[production_tree.ingot_node]
width          = 28.0
height         = 18.0
depth_offset_x = 8.0              # depth face offset right
depth_offset_y = -6.0             # depth face offset up
color_face_light_factor = 1.3     # front face is lightest
color_face_dark_factor  = 0.6     # depth face is darkest

[particles.depletion]
count          = 5                 # fragments per depletion event (4–6)
speed_min      = 40.0
speed_max      = 110.0
lifetime_secs  = 0.7
size_min       = 3.0
size_max       = 8.0
```

---

## Part B: Procedural Ore Polygon — Shared Algorithm

### B-1: Create `src/visuals/ore_polygon.rs`

This module is the single source of truth for the ore polygon shape. Both asteroid rendering and Production Tree ore node rendering call into this module.

```rust
pub struct OrePolygonConfig {
    pub radius: f32,
    pub vertex_count: usize,
    pub jaggedness: f32,          // 0.0–1.0
    pub color_body: Color,
    pub color_vein: Color,
    pub vein_count: usize,
    pub vein_width: f32,
    pub seed: u64,                // per-entity seed for consistent shape
}

pub fn draw_ore_polygon(painter: &egui::Painter, center: egui::Pos2, config: &OrePolygonConfig)
```

**Algorithm:**

1. Generate `vertex_count` vertices around center at `radius`
2. Per vertex: apply random radial offset using `seed` — range `radius * (1.0 - jaggedness)` to `radius * (1.0 + jaggedness * 0.5)`
3. Use deterministic RNG seeded by `config.seed` so shape is stable across frames
4. Fill polygon with `color_body`
5. Draw `vein_count` line segments from near-center outward to random edge points — color `color_vein`, width `vein_width`
6. Veins use same seed offset so they're stable

**Seed assignment:**
- Asteroids: use entity ID as seed
- Production Tree ore nodes: use ore type as seed (Metal=1, H3=2, Void=3) — all Metal nodes look identical, which is correct

---

## Part C: Asteroid Visual Replacement

### C-1: Remove existing asteroid sprites/placeholder geometry

Locate current asteroid rendering — wherever asteroids are currently drawn (likely a `draw_asteroids` system or similar). Remove existing visual logic.

### C-2: Wire `draw_ore_polygon` into asteroid rendering

Per asteroid entity:
- Read ring assignment → look up `[asteroid.inner_ring]` / `[asteroid.middle_ring]` / `[asteroid.outer_ring]` from visual.toml
- Read ore type component → look up `[ore.metal]` / `[ore.h3_gas]` / `[ore.void_essence]` from visual.toml
- Build `OrePolygonConfig` combining ring size params + ore color params
- Call `draw_ore_polygon`

**Depleted state:**
- When asteroid is depleted: render same polygon at 40% opacity (greyed)
- Do NOT change shape — same vertices, just faded

### C-3: Destruction particles on depletion

When asteroid transitions to depleted state, spawn particle burst:

- Read `[particles.depletion]` from visual.toml
- Spawn `count` fragments at asteroid center position
- Each fragment: random velocity direction, speed between `speed_min` and `speed_max`
- Fragment shape: small irregular triangle (3 vertices, random rotation)
- Fragment color: `color_body` from the ore type
- Alpha fades from 1.0 to 0.0 over `lifetime_secs`
- Fragments despawn after lifetime expires

Particles are visual only — no collision, no interaction, no gameplay effect.

---

## Part D: Production Tree Ore Node Visual

### D-1: Replace existing ore node rendering

In Production Tree rendering code, locate ore node draw calls. Replace with `draw_ore_polygon`.

Per ore node:
- Read ore type → look up `[ore.metal]` / `[ore.h3_gas]` / `[ore.void_essence]`
- Use `[production_tree.ore_node]` for size/shape params
- Use ore type integer (1/2/3) as seed
- Call `draw_ore_polygon`

Result: Metal ore nodes in the Production Tree look like small Inner Ring Metal asteroids. H3 nodes look like small Middle Ring H3 asteroids. Visual language is shared.

---

## Part E: Production Tree Ingot Node Visual

### E-1: Create ingot rendering function

```rust
pub fn draw_ingot_node(painter: &egui::Painter, center: egui::Pos2, config: &IngotNodeConfig, base_color: Color)
```

**Algorithm — 3-rect isometric block:**

Three rectangles drawn in order (back-to-front):

1. **Depth face** (top-right): offset by `(depth_offset_x, depth_offset_y)` from center. Color = `base_color * color_face_dark_factor`
2. **Side face** (right): offset by `(depth_offset_x * 0.5, depth_offset_y * 0.5)`. Color = `base_color * 0.85`
3. **Front face** (center): no offset. Color = `base_color * color_face_light_factor`

All three rects: width = `width`, height = `height`

**Base color per ingot type:**
- Metal ingot: `[180, 185, 200]` (polished grey)
- Crystal ingot: `[140, 120, 220]` (purple-blue)
- Void ingot: `[80, 40, 120]` (deep purple)

Add ingot color entries to `visual.toml`:

```toml
[ingot.metal]
color = [180, 185, 200]

[ingot.crystal]
color = [140, 120, 220]

[ingot.void]
color = [80, 40, 120]
```

### E-2: Wire into Production Tree

Replace existing ingot node draw calls with `draw_ingot_node`. Read ingot type from node data, look up color from visual.toml.

---

## Part F: Acceptance Criteria

All criteria must be verified before this sprint is considered complete. Agent provides terminal output + screenshots for each.

### F-1: Asteroid visual — WASM
- [ ] Inner Ring asteroids render as small irregular polygons with blue-white veins
- [ ] Middle Ring asteroids render as medium polygons with green veins
- [ ] Outer Ring asteroids render as large polygons with violet veins
- [ ] Depleted asteroids render at 40% opacity, same shape
- [ ] Depletion triggers particle burst (visually confirm ≥3 fragments)
- [ ] No asteroid renders as a sprite or rectangle placeholder

**Screenshot required:** Space view showing all three ring asteroid types simultaneously

### F-2: Asteroid visual — Android (Moto G)
- [ ] Same as F-1, confirmed on device
- [ ] No performance regression — 60 FPS maintained during particle burst

**Screenshot required:** Device screenshot showing asteroid field

### F-3: Production Tree ore nodes — WASM
- [ ] Ore nodes render as procedural polygons matching asteroid visual language
- [ ] Metal, H3, Void nodes are visually distinct by vein color
- [ ] Ore node shape matches `[production_tree.ore_node]` config (not asteroid ring sizes)

**Screenshot required:** Production Tree open, ore nodes visible

### F-4: Production Tree ingot nodes — WASM
- [ ] Ingot nodes render as 3-rect isometric blocks
- [ ] Three faces visible with light/mid/dark color variation
- [ ] Metal, Crystal, Void ingots are visually distinct

**Screenshot required:** Production Tree open, ingot nodes visible

### F-5: Visual.toml drives everything
- [ ] Changing `color_vein` in visual.toml for any ore type changes both asteroid veins AND Production Tree ore node veins
- [ ] Agent confirms this by making one config change, rebuilding, and verifying both views updated

### F-6: No regressions
- [ ] Tutorial still completes T-101 → T-106 without errors
- [ ] Mining loop functional — drones dispatch, return, unload
- [ ] Production Tree toggles still function
- [ ] Save/load unaffected

---

## Part G: Commit & Tag

On sprint completion:

```
git add -A
git commit -m "feat: Sprint 5 visual overhaul — unified procedural ore polygon, ingot nodes, depletion particles"
git tag v3.1.0-sprint5-visual-overhaul
git push origin feature/sprint5-visual-overhaul --tags
```

Then run:
```
.\publish.ps1 -Build
```

Confirm itch.io build number changed post-publish.

---

## Notes for Agent

- `ore_polygon.rs` is a new file. Create it under `src/visuals/`. If `src/visuals/` does not exist, create the directory and add `mod visuals;` to `main.rs` or `lib.rs`.
- All color values in visual.toml are `[R, G, B]` u8 arrays. Convert to `egui::Color32` on read.
- Deterministic RNG: use `rand::rngs::SmallRng` seeded with the entity seed. Do not use `thread_rng()` — shape must be stable across frames.
- Desktop runtime reads visual.toml at startup. WASM reads the `include_str!` baked version. Both paths must work.
- Do not modify game logic, mining systems, or UI layout. Visual layer only.

---

*VoidDrift Sprint 5 Directive*  
*May 2026 — RFD IT Services Ltd.*
