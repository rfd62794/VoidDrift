# VoidDrift — Sprint 5 Visual Amendment 2
**Amends:** VoidDrift_Sprint5_Visual_Overhaul_Directive.md + Visual_Amendment_1.md  
**Date:** May 2026  
**Scope:** Complete replacement of vein algorithm with stratified banding. Applies to both Production Tree ore nodes AND asteroids — same algorithm, size only differs.

---

## What Changed and Why

Amendment 1 described wandering bezier veins radiating from center outward. This produces spoke patterns that don't resemble real ore. The correct reference is stratified banding — veins that run ACROSS the rock following a grain direction, roughly parallel, organic in width, clipped to the polygon boundary.

Target aesthetic: between Minecraft (readable silhouette, clear color contrast) and real malachite (directional banding, bands that pool and merge). Dark opaque body, colored bands crossing it.

---

## Amendment: Replace Ore Vein Algorithm Entirely

**Replaces** Part B step 5 from the base directive AND the entire Amendment 1 vein section.

### Revised Algorithm — Stratified Banding

#### Step 1: Grain Direction
Each ore type has a fixed grain angle (seeded, stable across frames). Add `grain_angle_degrees` per ore type to `visual.toml`:

```toml
[ore.metal]
color_body      = [85, 85, 95]     # dark grey, fully opaque
color_vein      = [200, 210, 255]  # pale blue-white
band_count      = 4
band_width_min  = 0.08             # fraction of polygon radius
band_width_max  = 0.18
grain_angle_deg = 25.0

[ore.h3_gas]
color_body      = [25, 45, 35]     # dark teal, fully opaque
color_vein      = [80, 255, 160]   # bright green
band_count      = 5
band_width_min  = 0.10
band_width_max  = 0.22
grain_angle_deg = 15.0

[ore.void_essence]
color_body      = [28, 18, 40]     # dark purple, fully opaque
color_vein      = [160, 80, 255]   # violet
band_count      = 3
band_width_min  = 0.12
band_width_max  = 0.28
grain_angle_deg = 50.0
```

Body colors are dark but fully opaque. No transparency on the body fill. Vein bands sit on top.

#### Step 2: Generate Band Positions

In local space (polygon centered at origin), rotated to grain angle:

1. Project the polygon extent along the grain-perpendicular axis to find min/max span
2. Divide span into `band_count + 1` segments
3. Per band: place center position with small random offset of ±15% of segment size
4. Per band: assign width by sampling between `band_width_min * radius` and `band_width_max * radius`

Use deterministic RNG seeded by ore type integer — band positions are stable across frames.

#### Step 3: Draw Each Band

Each band is a filled polygon strip, not a line. Per band:

1. In grain-rotated space, the band runs the full width of the polygon at its center position
2. Generate the band as a quad: four corner points at `(±half_span, center ± half_width)`
3. Irregularize the edges: subdivide each long edge into 5 segments, offset each midpoint perpendicular by `random(-band_width * 0.4, band_width * 0.4)` — gives organic, non-straight band edges
4. Rotate quad back to world space
5. Fill with `color_vein`

#### Step 4: Clip to Polygon

Clip each band to the ore polygon boundary:

**Preferred:** For each band quad, intersect with the ore polygon using convex clip. Render only the interior portion.

**Acceptable fallback:** Set egui `clip_rect` to the polygon's axis-aligned bounding box before drawing bands, then draw the polygon outline on top at 2px stroke in `color_body * 0.7` to cover edge bleed. Acceptable at this render scale.

---

## Asteroid Visual — Same Algorithm, Scaled Down

Asteroids use the identical banding algorithm. Size is the only difference. Add `ore_type` field to asteroid ring config:

```toml
[asteroid.inner_ring]
radius         = 14.0
vertex_count   = 9
jaggedness     = 0.35
ore_type       = "metal"

[asteroid.middle_ring]
radius         = 22.0
vertex_count   = 11
jaggedness     = 0.40
ore_type       = "h3_gas"

[asteroid.outer_ring]
radius         = 34.0
vertex_count   = 13
jaggedness     = 0.45
ore_type       = "void_essence"
```

Each asteroid reads `ore_type` → looks up `[ore.{type}]` → uses that ore's body color, band config, and grain angle. Same code path as Production Tree ore nodes. One function, two call sites.

**Depleted state:** body at 50% opacity, bands at 20% opacity. Shape and position unchanged.

---

## Visual Target

- Dark solid polygon body — readable against black space
- 3–5 colored bands crossing the rock at a consistent diagonal angle
- Band edges are organic and irregular — not ruler-straight
- Bands are filled color strips, not lines
- All banding contained within the polygon silhouette
- Production Tree ore nodes: same rock, smaller radius
- Asteroids: same rock, asteroid-scale radius
- Ore type identifiable at a glance by body + band color combination

---

## Updated Acceptance Criteria (Replaces F-1 visual description)

- [ ] Each ore type has a dark, fully opaque body
- [ ] Bands cross the polygon directionally — not radiating from center
- [ ] Band edges are irregular, not straight lines
- [ ] No bands extend outside the polygon boundary
- [ ] Three ore types visually distinct at a glance
- [ ] Same visual reads at both Production Tree scale and asteroid scale
- [ ] Depleted asteroids are visibly faded, same shape

**Screenshot required:** All three ore types side by side in Production Tree. All three asteroid types visible in space view.

---

## Notes for Agent

- Grain angle is in degrees — convert to radians for trig
- All RNG must use deterministic seed by ore type integer: Metal=1, H3=2, Void=3. Never thread_rng()
- Band generation happens in grain-rotated local space, then rotated back to world space
- Band width values in toml are fractions of polygon radius — multiply by radius to get pixel width
- Do not modify mining logic, drone systems, or UI layout
- Amendment 1 vein algorithm is superseded entirely by this document

---

*VoidDrift Sprint 5 Visual Amendment 2*  
*May 2026 — RFD IT Services Ltd.*
