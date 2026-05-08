# VoidDrift — Sprint 5 Visual Amendment
**Amends:** VoidDrift_Sprint5_Visual_Overhaul_Directive.md  
**Date:** May 2026  
**Scope:** Ore vein algorithm + ingot geometry only. All other directive sections unchanged.

---

## Amendment 1: Ore Vein Algorithm

**Replace** the vein description in Part B (draw_ore_polygon algorithm, step 5) with the following:

### Revised Vein Rendering

Veins are wandering curves, not straight radial lines. Each vein:

1. **Start point:** Random position within 20% of center (small cluster near core)
2. **End point:** Random point on the polygon edge (one of the polygon vertices or midpoint between two)
3. **Control point 1:** Midpoint between start and end, offset perpendicular by `random(-radius * 0.25, radius * 0.25)` — this is what creates the wander
4. **Control point 2:** 75% along the start→end line, offset perpendicular by `random(-radius * 0.15, radius * 0.15)` — secondary drift near the tip

Render as a quadratic bezier approximated by 6 line segments (sufficient for this scale). Width tapers from `vein_width` at start to `vein_width * 0.4` at tip.

Use same deterministic seed as polygon shape so veins are stable across frames.

**Result:** Veins that originate from a core cluster and drift organically toward the polygon edge — branching appearance without actual branching logic.

---

## Amendment 2: Ingot Node Geometry

**Replace** the entire ingot algorithm in Part E-1 with the following:

### Revised Ingot Rendering — Isometric Parallelogram Faces

An isometric block requires three distinct face shapes. Rectangles produce a flat stacked look. Use `painter.add(Shape::convex_polygon(...))` for each face with the correct 4 corner points.

Given:
- `center` — node center position
- `w` — face width (from `production_tree.ingot_node.width`)
- `h` — face height (from `production_tree.ingot_node.height`)
- `dx` — depth x-offset (from `depth_offset_x`, positive = right)
- `dy` — depth y-offset (from `depth_offset_y`, negative = up)

**Three faces, drawn back-to-front:**

**1. Top face** (parallelogram — drawn first, darkest):
```
top-left:     center + (-w/2 + dx,  -h/2 + dy)
top-right:    center + ( w/2 + dx,  -h/2 + dy)
bottom-right: center + ( w/2,       -h/2)
bottom-left:  center + (-w/2,       -h/2)
```
Color: `base_color * color_face_dark_factor`

**2. Right side face** (parallelogram — drawn second, mid tone):
```
top-left:     center + (w/2,       -h/2)
top-right:    center + (w/2 + dx,  -h/2 + dy)
bottom-right: center + (w/2 + dx,   h/2 + dy)
bottom-left:  center + (w/2,        h/2)
```
Color: `base_color * 0.85`

**3. Front face** (rectangle — drawn last, lightest):
```
top-left:     center + (-w/2, -h/2)
top-right:    center + ( w/2, -h/2)
bottom-right: center + ( w/2,  h/2)
bottom-left:  center + (-w/2,  h/2)
```
Color: `base_color * color_face_light_factor`

**Result:** A solid 3D block sitting in space. The top face angles back-right. The right side angles back-right and down. The front face is flat toward the viewer. Depth is immediately readable.

### Suggested visual.toml values (replace existing ingot_node block):
```toml
[production_tree.ingot_node]
width                  = 30.0
height                 = 20.0
depth_offset_x         = 10.0
depth_offset_y         = -7.0
color_face_light_factor = 1.25
color_face_dark_factor  = 0.55
```

---

## Summary

| Section | Change |
|---|---|
| Part B step 5 | Straight radial veins → wandering bezier curves with two control points |
| Part E-1 | 3 offset rectangles → 3 parallelogram/rectangle faces with correct isometric corner points |
| All other sections | Unchanged |

---

*VoidDrift Sprint 5 Visual Amendment*  
*May 2026 — RFD IT Services Ltd.*
