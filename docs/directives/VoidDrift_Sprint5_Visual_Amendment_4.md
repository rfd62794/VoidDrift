# VoidDrift — Sprint 5 Visual Amendment 4
**Amends:** VoidDrift_Sprint5_Visual_Overhaul_Directive.md  
**Date:** May 2026  
**Scope:** Drone Bay node — replaces empty rectangle with procedural rocket silhouette. Two states: empty (dim) and ready (full color).

---

## Context

The Drone Bay is the terminal node of the Production Pipeline — the destination all four component streams build toward. It should read immediately as "rocket" at node size. No texture, no detail beyond silhouette — orange solid for now, two states for empty vs ready.

---

## visual.toml — Drone Bay Entry

Add to `assets/visual.toml`:

```toml
[component.drone_bay]
width               = 32.0
height              = 52.0
color_ready         = [210, 95, 30]     # solid orange — full build state
color_empty         = [80, 40, 15]      # dim orange — waiting state
nose_height_ratio   = 0.28             # nose cone is 28% of total height
fin_width_ratio     = 0.28             # each fin extends 28% of body width outward
fin_height_ratio    = 0.22             # fins cover bottom 22% of total height
porthole_radius     = 3.5
porthole_offset_y   = -0.15            # porthole sits 15% above center
exhaust_radius      = 4.0
```

---

## Part E-4: Drone Bay Node Rendering

Add `draw_drone_bay()` to `src/systems/visuals/component_nodes.rs`.

```rust
pub fn draw_drone_bay(
    painter: &egui::Painter,
    center: egui::Pos2,
    config: &DroneBayConfig,
    is_ready: bool,
)
```

`is_ready` is true when the station has at least one drone built. False otherwise. Pass from render_node based on drone count > 0.

**Color selection:**
```rust
let base_color = if is_ready { config.color_ready } else { config.color_empty };
```

All shapes use `base_color`. No secondary colors for now — whole rocket is one orange.

---

### Draw Order

Given:
```
total_height  = config.height
body_height   = total_height * (1.0 - config.nose_height_ratio)
nose_height   = total_height * config.nose_height_ratio
fin_height    = total_height * config.fin_height_ratio
body_width    = config.width
fin_width     = body_width * config.fin_width_ratio

top_y         = center.y - total_height * 0.5
nose_base_y   = top_y + nose_height
body_bottom_y = center.y + total_height * 0.5
```

**1. Body** — rectangle (rect_filled):
```
top-left:     (center.x - body_width*0.5, nose_base_y)
bottom-right: (center.x + body_width*0.5, body_bottom_y)
```
Color: `base_color`

**2. Nose cone** — triangle (convex_polygon, 3 points):
```
tip:          (center.x,                top_y)
left:         (center.x - body_width*0.5, nose_base_y)
right:        (center.x + body_width*0.5, nose_base_y)
```
Color: `base_color`

**3. Left fin** — triangle (convex_polygon, 3 points):
```
top:          (center.x - body_width*0.5, body_bottom_y - fin_height)
inner-bottom: (center.x - body_width*0.5, body_bottom_y)
outer-bottom: (center.x - body_width*0.5 - fin_width, body_bottom_y)
```
Color: `base_color`

**4. Right fin** — triangle (convex_polygon, 3 points):
```
top:          (center.x + body_width*0.5, body_bottom_y - fin_height)
inner-bottom: (center.x + body_width*0.5, body_bottom_y)
outer-bottom: (center.x + body_width*0.5 + fin_width, body_bottom_y)
```
Color: `base_color`

**5. Porthole** — circle outline (circle_stroke, not filled):
```
center:  (center.x - body_width*0.15, center.y + total_height * config.porthole_offset_y)
radius:  config.porthole_radius
```
Color: `base_color * 1.4` (brighter than body — glass catch light)
Stroke width: 1.5px

**6. Exhaust port** — filled circle at base:
```
center: (center.x, body_bottom_y)
radius: config.exhaust_radius
```
Color: `base_color * 0.6` (darker than body)

---

## Integration in hud/mod.rs

The Drone Bay node is already rendered via `render_node`. Extend the component type match to include it:

```rust
"drone_bay" => component_nodes::draw_drone_bay(
    &painter,
    node_center,
    &visual_config.component.drone_bay,
    st.drone_count > 0,   // or whatever the drone ready field is called
),
```

Update the Drone Bay render_node call site to pass `Some("drone_bay")`.

The Drone Bay node rectangle may be wider than component nodes — if so, pass the correct dimensions or let the draw function use its own config dimensions independent of the node rect size.

---

## Two States at a Glance

**Empty (dim orange):** Dark burnt orange silhouette — rocket is present but unbuilt. The shape tells the player what they're building toward.

**Ready (full orange):** Bright orange silhouette — a drone is built and waiting in the bay.

No other visual difference for now. Color is the entire signal.

---

## Acceptance Criteria

- [ ] Drone Bay renders as upright rocket silhouette — body, nose cone, two fins, porthole, exhaust port
- [ ] Empty state: dim orange
- [ ] Ready state: bright orange  
- [ ] Silhouette readable at node size without labels
- [ ] No empty rectangle visible — rocket fills the node bounds

**Screenshot required:** Production Tree with Drone Bay visible. Both states if testable, otherwise empty state confirmed.

---

## Notes for Agent

- Add `DroneBayConfig` struct to `visual.rs` alongside the other component config structs
- The porthole uses `circle_stroke` not `circle_filled` — it's a window outline, not a solid dot
- Fin triangles use `convex_polygon` with 3 points — same as nose cone
- Do not modify any other node types

---

*VoidDrift Sprint 5 Visual Amendment 4*  
*May 2026 — RFD IT Services Ltd.*
