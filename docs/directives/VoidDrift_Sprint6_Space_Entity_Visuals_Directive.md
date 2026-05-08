# VoidDrift — Sprint 6: Space View Entity Visuals
**Date:** May 2026  
**Status:** Ready for Agent Execution  
**Owner:** Robert (rfd62794)  
**Branch:** `feature/sprint6-space-entity-visuals`  
**Builds Required:** WASM + Android (Moto G)

---

## Objective

Replace all remaining placeholder sprites and entity visuals in the space view with procedural math rendering. Extends the visual language established in Sprint 5 into the game world itself. No new mechanics. Pure visual work.

---

## Scope

### IN SCOPE
- Drone ships in space view → green rocket silhouette
- Opening sequence ship → orange rocket silhouette  
- Drone Bay node orientation fix → upright rocket, not horizontal
- All colors and dimensions driven by `visual.toml`

### OUT OF SCOPE
- Asteroid visual replacement (completed Sprint 5)
- Production Tree node changes
- Any mechanic, mining logic, or UI layout changes
- Audio

---

## visual.toml — New Entries

Add to `assets/visual.toml`:

```toml
[ship.drone]
width              = 14.0
height             = 28.0
color_body         = [60, 180, 100]     # green — deployed drone
color_nose         = [80, 210, 120]     # slightly lighter green nose
color_fins         = [40, 140, 75]      # darker green fins
color_exhaust      = [30, 100, 55]      # dark exhaust port
nose_height_ratio  = 0.28
fin_width_ratio    = 0.25
fin_height_ratio   = 0.20
exhaust_radius     = 2.5
porthole_radius    = 2.0
porthole_offset_y  = -0.15

[ship.opening]
width              = 18.0
height             = 36.0
color_body         = [210, 95, 30]      # orange — player ship / Drone-1
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

Also update the existing Drone Bay entry in `visual.toml` — the width/height ratio was wrong causing horizontal rendering. Replace with:

```toml
[component.drone_bay]
width               = 28.0             # narrower — forces upright in node box
height              = 52.0
color_ready         = [210, 95, 30]
color_empty         = [80, 40, 15]
nose_height_ratio   = 0.28
fin_width_ratio     = 0.28
fin_height_ratio    = 0.22
porthole_radius     = 3.5
porthole_offset_y   = -0.15
exhaust_radius      = 4.0
```

---

## Part A: Shared Rocket Draw Function

The drone ship and opening ship use the same rocket silhouette as the Drone Bay node — same shape, different colors and scale. Rather than duplicating code, extract the rocket drawing logic into a shared function in `src/systems/visuals/component_nodes.rs`:

```rust
pub struct RocketConfig {
    pub width: f32,
    pub height: f32,
    pub color_body: Color32,
    pub color_nose: Color32,
    pub color_fins: Color32,
    pub color_exhaust: Color32,
    pub nose_height_ratio: f32,
    pub fin_width_ratio: f32,
    pub fin_height_ratio: f32,
    pub exhaust_radius: f32,
    pub porthole_radius: f32,
    pub porthole_offset_y: f32,
}

pub fn draw_rocket(painter: &egui::Painter, center: egui::Pos2, config: &RocketConfig)
```

**Refactor `draw_drone_bay`** to call `draw_rocket` internally with the bay's color selection logic on top. This removes duplicate geometry code.

**Rocket always renders upright** — nose at top, fins at bottom, regardless of containing rect dimensions.

### Rocket Draw Order (same as Amendment 4, now shared):

```
nose_height   = config.height * config.nose_height_ratio
body_height   = config.height * (1.0 - config.nose_height_ratio)
fin_height    = config.height * config.fin_height_ratio
fin_width     = config.width  * config.fin_width_ratio

top_y         = center.y - config.height * 0.5
nose_base_y   = top_y + nose_height
body_bottom_y = center.y + config.height * 0.5
```

1. **Body** — rect_filled, `color_body`
2. **Nose cone** — convex_polygon (3 pts), `color_nose`
3. **Left fin** — convex_polygon (3 pts), `color_fins`
4. **Right fin** — convex_polygon (3 pts), `color_fins`
5. **Porthole** — circle_stroke, `color_body * 1.4`, width 1.5px
6. **Exhaust port** — circle_filled, `color_exhaust`

---

## Part B: Drone Ship in Space View

### B-1: Locate current drone rendering

Find where drones/ships are currently drawn in the space view — likely a sprite draw call or a placeholder shape in the world rendering system.

### B-2: Replace with `draw_rocket`

For each drone entity in the space view:
- Read position from entity transform
- Build `RocketConfig` from `visual_config.ship.drone`
- Call `draw_rocket(painter, world_to_screen(position), &config)`
- Rotate the painter context to match the drone's current heading — the rocket nose should point in the direction of travel

**Rotation:** Wrap the `draw_rocket` call in a painter transform that rotates around the entity center by the drone's heading angle. In egui this is done via `painter.with_clip_rect` + transform, or by rotating all points manually before passing to draw calls.

**Scale:** Drone ships are small — `[ship.drone]` width/height values are set for space view scale. Do not use Production Tree node sizing.

---

## Part C: Opening Sequence Ship

### C-1: Locate opening sequence ship rendering

Find where the opening sequence renders the initial ship — the vessel that arrives, gets taken by ECHO, becomes Drone-1.

### C-2: Replace with `draw_rocket`

- Build `RocketConfig` from `visual_config.ship.opening`
- Call `draw_rocket` at the ship's position
- Apply heading rotation same as drone ships
- The opening ship is larger than deployed drones (`[ship.opening]` has larger dimensions)

The orange color distinguishes it visually as the player's original vessel before it becomes part of the fleet.

---

## Part D: Drone Bay Orientation Fix

The Drone Bay node in the Production Tree renders horizontally because the node box width exceeds height, and the rocket was being scaled to fit the box. Fix:

- `draw_drone_bay` must use `RocketConfig` dimensions directly — do not scale to fit the containing rect
- Center the rocket within the node rect
- Rocket is always upright — the node box clips anything outside its bounds naturally
- The updated `visual.toml` values (width=28, height=52) are sized to fit correctly within the node

---

## Acceptance Criteria

### D-1: Drone ships — WASM
- [ ] Drones in space view render as small green upright rockets
- [ ] Rockets rotate to face direction of travel
- [ ] No sprite or placeholder shape visible for drones

**Screenshot required:** Space view with at least one drone visible in transit

### D-2: Drone ships — Android (Moto G)
- [ ] Same as D-1 confirmed on device
- [ ] No performance regression — 60 FPS maintained

**Screenshot required:** Device screenshot with drone visible

### D-3: Opening sequence ship — WASM
- [ ] Opening ship renders as orange rocket
- [ ] Larger than drone ships, same silhouette

**Screenshot required:** Opening sequence showing orange rocket

### D-4: Drone Bay orientation — WASM
- [ ] Drone Bay node renders rocket upright — nose at top, fins at bottom
- [ ] Rocket centered within node box
- [ ] Both empty (dim) and ready (bright) states confirmed

**Screenshot required:** Production Tree with Drone Bay node showing upright rocket

### D-5: No regressions
- [ ] Tutorial T-101 → T-106 completes without errors
- [ ] Mining loop functional
- [ ] Production Tree toggles functional
- [ ] Save/load unaffected

---

## Commit & Tag

```
git add -A
git commit -m "feat: Sprint 6 space entity visuals — green drone ships, orange opening ship, drone bay orientation fix"
git tag v3.2.0-sprint6-entity-visuals
git push origin feature/sprint6-space-entity-visuals --tags
```

Then run `.\publish.ps1 -Build` and confirm build number changed on itch.io.

---

## Notes for Agent

- `draw_rocket` is a shared function — refactor `draw_drone_bay` to use it before adding drone/opening ship calls
- Rotation in egui painter: rotate the point array before drawing, using `center + Vec2::angled(heading) * offset` per point
- `[ship.drone]` dimensions are space-view scale — smaller than Production Tree node dimensions
- Do not modify ore node, ingot node, or component node rendering
- Do not modify asteroid rendering

---

*VoidDrift Sprint 6 Directive*  
*May 2026 — RFD IT Services Ltd.*
