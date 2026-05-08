# VoidDrift — Sprint 5 Visual Amendment 3
**Amends:** VoidDrift_Sprint5_Visual_Overhaul_Directive.md  
**Date:** May 2026  
**Scope:** Four component node visual specs for Production Tree — Thruster, Hull, Canister, AI Core. All pure math, no sprites. Adds to Part E (node visuals), does not replace ore or ingot specs.

---

## Context

The Production Tree has three node tiers:
- **Tier 1 — Ore nodes** (raw material, irregular polygon + banding — covered in base directive + Amendment 2)
- **Tier 2 — Ingot nodes** (processed material, isometric block — covered in base directive + Amendment 1)
- **Tier 3 — Component nodes** (fabricated outputs — this document)

Component nodes are the downstream outputs: what the raw materials become. They give the player visual context for what the production chain is building toward. Four types: Thruster, Hull, Canister, AI Core.

All rendering uses existing egui painter primitives only — `convex_polygon`, `line_segment`, `circle_filled`, `rect_filled`, `arc`. No new rendering infrastructure required.

All sizes, colors, and detail counts are driven by `visual.toml` entries defined below.

---

## visual.toml — Component Node Entries

Add to `assets/visual.toml`:

```toml
[component.thruster]
width               = 38.0          # total node width
color_nozzle        = [210, 215, 225]  # silver-white
color_body          = [140, 145, 155]  # darker metallic grey
color_wire          = [200, 60, 60]    # red wiring accent
wire_count          = 3
nozzle_width_ratio  = 0.55          # nozzle takes 55% of total width
body_width_ratio    = 0.45

[component.hull]
width               = 40.0
rib_count           = 7
color_frame         = [160, 155, 140]  # warm structural grey
color_outline       = [200, 195, 185]  # lighter outline
stroke_width        = 1.2

[component.canister]
width               = 28.0
height              = 34.0
lid_height_ratio    = 0.18          # lid is 18% of total height
color_body          = [148, 130, 50]   # olive-amber glaze
color_lid           = [130, 115, 42]   # slightly darker lid
color_highlight     = [190, 175, 90]   # glaze sheen
color_handle        = [120, 105, 35]

[component.ai_core]
radius              = 20.0
fin_count           = 14
fin_length          = 8.0
fin_width           = 2.5
color_body          = [28, 28, 32]     # near-black
color_fins          = [190, 200, 215]  # silver-white
color_fan_housing   = [45, 45, 52]     # dark grey
fan_radius_ratio    = 0.45            # fan circle is 45% of body radius
fan_blade_count     = 3
```

---

## Part E-3: Component Node Rendering

Create `src/visuals/component_nodes.rs` with one public function per component type.

All functions share the same signature pattern:
```rust
pub fn draw_[component](painter: &egui::Painter, center: egui::Pos2, config: &ComponentConfig)
```

---

### E-3a: Thruster

**Reference:** Rocket thruster — cone nozzle + cylindrical body + wiring.

**Structure:** Two joined shapes, nozzle left, body right, wires overlaid.

```
Total width = config.width
Nozzle width = total_width * nozzle_width_ratio
Body width   = total_width * body_width_ratio
Height       = total_width * 0.45   (fixed ratio, not configurable)

Left edge of nozzle  = center.x - total_width * 0.5
Nozzle/body join     = center.x - total_width * 0.5 + nozzle_width
Right edge of body   = center.x + total_width * 0.5
```

**Draw order:**

1. **Nozzle** — trapezoid (convex_polygon, 4 points):
   ```
   top-left:     (left_edge,    center.y - height * 0.25)
   top-right:    (join_x,       center.y - height * 0.5)
   bottom-right: (join_x,       center.y + height * 0.5)
   bottom-left:  (left_edge,    center.y + height * 0.25)
   ```
   Color: `color_nozzle`

2. **Body** — rectangle (rect_filled):
   ```
   top-left:     (join_x,       center.y - height * 0.5)
   bottom-right: (right_edge,   center.y + height * 0.5)
   ```
   Color: `color_body`

3. **Wires** — `wire_count` bezier curves over the body section:
   - Each wire: starts at `(join_x + body_width * 0.1, center.y + random_y_offset)`
   - Ends at `(right_edge, center.y + random_y_end_offset)`
   - One control point with perpendicular drift of `±height * 0.3`
   - Use seeded RNG (seed = wire index) for stable offsets
   - Stroke: `color_wire`, width 1.5px

4. **Outline** — stroke the nozzle trapezoid boundary at 1px, `color_body * 0.7`

---

### E-3b: Hull

**Reference:** Ship hull cross-section — outer curved boundary with interior rib arcs.

**Structure:** Outer hull curve + internal structural ribs. No fill — line drawing only. Reads as a technical diagram/schematic.

```
Width  = config.width
Height = config.width * 0.65
```

**Draw order:**

1. **Outer hull boundary** — two arcs forming a lens/hull shape:
   - Top arc: from `(center.x - width*0.5, center.y)` to `(center.x + width*0.5, center.y)`, bulging upward by `height * 0.5`
   - Bottom arc: same endpoints, bulging downward by `height * 0.35` (asymmetric — flatter bottom like a real hull)
   - Approximate each arc with 8 line segments
   - Stroke: `color_outline`, width `stroke_width * 1.5`

2. **Ribs** — `rib_count` curved arcs inside the hull, evenly spaced along the horizontal axis:
   - Each rib: vertical arc from top hull boundary to bottom hull boundary at its x position
   - Arc bulges outward (toward the hull edge) by `width * 0.08`
   - Approximate each rib with 5 line segments
   - Stroke: `color_frame`, width `stroke_width`

3. **Longitudinal members** — 2 diagonal sweep lines running the length of the hull:
   - Upper member: follows the upper arc contour inset by `height * 0.15`
   - Lower member: follows lower arc contour inset by `height * 0.12`
   - 6 line segments each
   - Stroke: `color_frame`, width `stroke_width * 0.8`

**Result:** Clean schematic line drawing — looks like a blueprint cross-section. No fills.

---

### E-3c: Canister

**Reference:** Glazed ceramic storage jar — cylinder body, lid, loop handle.

```
Width        = config.width
Total height = config.height
Lid height   = total_height * lid_height_ratio
Body height  = total_height - lid_height
```

**Draw order:**

1. **Body** — rectangle (rect_filled):
   ```
   top-left:     (center.x - width*0.5,  center.y - body_height*0.5 + lid_height*0.5)
   bottom-right: (center.x + width*0.5,  center.y + body_height*0.5 + lid_height*0.5)
   ```
   Color: `color_body`

2. **Lid** — slightly wider rectangle (rect_filled):
   ```
   top-left:     (center.x - width*0.52, center.y - total_height*0.5 + lid_height)
   bottom-right: (center.x + width*0.52, center.y - total_height*0.5 + lid_height + lid_height)
   ```
   The lid is 4% wider than the body (overhangs slightly)
   Color: `color_lid`

3. **Glaze highlight** — thin rectangle on body, left-of-center:
   ```
   top-left:     (center.x - width*0.25, center.y - body_height*0.4 + lid_height*0.5)
   bottom-right: (center.x - width*0.12, center.y + body_height*0.3 + lid_height*0.5)
   ```
   Color: `color_highlight` at 35% opacity — gives the glazed ceramic sheen

4. **Handle** — semicircle arc on top of lid:
   - Center: `(center.x, center.y - total_height*0.5 + lid_height*0.5)`
   - Radius: `width * 0.16`
   - Arc from 180° to 0° (top half only)
   - Approximate with 6 line segments
   - Stroke: `color_handle`, width 2.5px

5. **Outline** — stroke body and lid rectangles at 1px, `color_body * 0.7`

---

### E-3d: AI Core

**Reference:** CPU cooler — circular body with radiating heat fins, fan housing on top.

```
Body radius    = config.radius
Fin length     = config.fin_length
Fan radius     = body_radius * fan_radius_ratio
```

**Draw order:**

1. **Body** — filled circle:
   - Center: `center`
   - Radius: `body_radius`
   - Color: `color_body`

2. **Heat fins** — `fin_count` thin rectangles radiating outward from body perimeter:
   - Per fin at angle `i * (360 / fin_count)` degrees:
     - Inner point: `center + direction * body_radius`
     - Outer point: `center + direction * (body_radius + fin_length)`
     - Draw as a rotated rectangle of width `fin_width`
     - Alternatively: two parallel line_segments at ±`fin_width*0.5` offset from the radial line
   - Color: `color_fins`
   - Fins taper: wider at inner end (`fin_width`), narrower at outer end (`fin_width * 0.5`)

3. **Fan housing** — filled circle on top of body:
   - Center: `center` (same center, sits on top in draw order)
   - Radius: `fan_radius`
   - Color: `color_fan_housing`

4. **Fan blades** — `fan_blade_count` curved arcs inside fan housing:
   - Evenly spaced at `360 / fan_blade_count` degree intervals
   - Each blade: arc from fan center offset by `fan_radius * 0.15` curving to near perimeter
   - Approximate with 4 line segments, slight curve
   - Stroke: `color_fins * 0.7`, width 1.5px

5. **Body outline** — stroke outer circle at 1px, `color_fins * 0.4`

---

## Acceptance Criteria

- [ ] All four component nodes render without sprites — pure painter calls only
- [ ] Thruster: nozzle trapezoid + body rectangle + red wire curves visible
- [ ] Hull: schematic line drawing with outer boundary + ribs + longitudinal members — no fill
- [ ] Canister: body + lid + highlight + handle arc visible and proportioned correctly
- [ ] AI Core: body circle + radiating fins + fan housing + blade arcs visible
- [ ] All colors read from `visual.toml` — no hardcoded color values in Rust
- [ ] All four nodes visually distinct from each other and from ore/ingot nodes at a glance
- [ ] Nodes render correctly at Production Tree node size — not too dense, not too sparse

**Screenshot required:** Production Tree showing all tier types — ore node, ingot node, and all four component nodes visible simultaneously.

---

## Notes for Agent

- `component_nodes.rs` is a new file under `src/visuals/`
- Add `mod component_nodes;` and `pub use component_nodes::*;` to `src/visuals/mod.rs`
- All arc approximations: 5-8 line segments is sufficient at this render scale
- Seeded RNG for wire offsets: use wire index as seed, not thread_rng()
- Hull node has no fill — it is entirely a line drawing. This is intentional.
- Do not modify ore node, ingot node, asteroid, or any existing rendering code

---

*VoidDrift Sprint 5 Visual Amendment 3*  
*May 2026 — RFD IT Services Ltd.*
