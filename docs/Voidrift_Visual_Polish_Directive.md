# Voidrift — Visual Polish Directive: Classic Asteroids Feel
**Status:** Approved — Ready for Execution  
**Type:** Visual Polish — No gameplay changes  
**Date:** April 2026  
**Depends On:** Phase 9 Gate PASSED ✅

---

## 1. Objective

Make Voidrift feel like a game you want to play, not just test. No new systems, no economy changes, no ECS components that affect gameplay. Pure visual addition — the minimum polish that transforms colored rectangles into a space game.

Target reference: Classic Asteroids aesthetic. Simple, clean, immediately readable. Not Vanguard Galaxy's full visual fidelity — just enough that the world feels alive.

---

## 2. Scope Boundaries

> ⚠️ HARD LIMIT: Visual changes only. Zero gameplay impact.

**In scope:**
- Scrolling parallax starfield
- Ship visual — directional triangle with thruster glow
- Mining beam — line from ship to asteroid while mining
- Asteroid visual — irregular polygon, depleted state distinct
- Map polish — sector circles, connection lines, distance indicators
- Autonomous ship visual distinction maintained — orange triangle

**Explicitly out of scope:**
- Sound of any kind
- Particle systems or animations
- New ECS components that affect gameplay
- Any economy, power, or resource changes
- Sprite assets — Mesh2d only, no external textures required
- Camera changes beyond what already exists

---

## 3. Visual Specifications

### 3.1 Parallax Starfield

Two layers of point entities at different depths:

| Layer | Star Count | Scroll Speed | Size | Color |
|-------|-----------|-------------|------|-------|
| Far | 150 | 0.05x camera | 1.0 | White, 40% opacity |
| Near | 50 | 0.15x camera | 2.0 | White, 70% opacity |

Stars wrap at screen edges — when a star scrolls off one edge it reappears on the opposite. No star ever disappears permanently.

Stars are static `Mesh2d` points spawned at startup. Position updates each tick based on camera delta movement multiplied by layer scroll speed.

> ⚠️ Stars must not affect gameplay hitboxes or input detection. Z position: -50.0 (far behind all game entities).

### 3.2 Ship Visual

Replace cyan rectangle with a directional triangle:

```
Player ship:
- Shape: isoceles triangle, pointing in direction of travel
- Size: 20w × 28h (slightly larger than current rectangle)
- Color: Cyan (unchanged)
- Rotation: matches ship movement direction

Thruster glow:
- Shape: small rectangle, 6w × 8h
- Position: rear of ship opposite travel direction
- Color: Orange when Navigating or Mining, hidden when Idle/Docked
- Z: ship Z - 0.1 (behind ship body)
```

Autonomous ships follow identical pattern in orange with cyan thruster glow — inverted colors for visual distinction.

### 3.3 Mining Beam

A thin line drawn from ship center to asteroid center while `ShipState::Mining`:

```
- Width: 2.0
- Color: Cyan, 60% opacity
- Length: dynamic — ship to asteroid distance
- Appears: on Mining state entry
- Disappears: on Mining state exit
- Z: between ship and asteroid
```

Implementation: single `Mesh2d` rectangle, rotated and scaled each tick to connect ship position to asteroid position. Not a particle system — just a rotated rectangle.

Autonomous ships show orange mining beam when mining.

### 3.4 Asteroid Visual

Replace grey rectangle with an irregular polygon:

```
Active asteroid:
- Shape: 8-vertex irregular polygon (slightly randomised at spawn)
- Size: roughly 48×48 bounding box (unchanged from current)
- Color: Medium grey #888888
- Outline feel: slightly lighter vertices

Depleted asteroid:
- Same shape
- Color: Dark grey #333333
- Distinct enough to read at a glance
```

Each asteroid gets a random rotation seed at spawn — no two look identical. Rotation is static, not animated.

### 3.5 Map Polish

Current map has basic markers. Upgrade to:

```
Sector markers:
- Shape: circle (not rectangle)
- Size: 12px radius
- Color: matches ore type
  - Magnetite: blue-white
  - Iron: rust orange  
  - Carbon: dark grey
  - Tungsten: yellow-grey (when unlocked)
  - Titanite: silver (when unlocked)

Connection lines:
- Thin lines between adjacent sectors
- Color: #333333 dark, subtle
- Always visible — shows the network

Distance indicator:
- Small text label showing sector name below marker
- Current sector highlighted with brighter ring

Player ship marker:
- Small triangle on map matching ship color
- Updates position in real time

Autonomous ship markers:
- Small orange triangles
- Update position in real time
```

---

## 4. Implementation Notes

### 4.1 Starfield Performance

150 + 50 = 200 star entities. At 60 FPS on Moto G 2025 this should be well within budget but verify in logcat after first deploy. If frame rate drops below 55 FPS, reduce far layer to 80 stars.

### 4.2 Ship Rotation

The ship triangle must rotate to face the direction of travel. Use `Autopilot` target position to calculate heading angle. Apply to ship `Transform` rotation each tick while `ShipState::Navigating`. Hold last heading when `Idle` or `Docked` — ship doesn't snap back to default.

### 4.3 Mining Beam Geometry

A rotated rectangle connecting two points:

```rust
// Pseudocode
let direction = asteroid_pos - ship_pos;
let distance = direction.length();
let angle = direction.y.atan2(direction.x);
// Rectangle: width=2.0, height=distance
// Position: midpoint between ship and asteroid
// Rotation: angle
```

### 4.4 Irregular Asteroid Shape

Generate 8 vertices around a circle with slight random radius variation per vertex:

```rust
// Pseudocode — run once at spawn
let base_radius = 24.0;
let vertices: Vec<Vec2> = (0..8).map(|i| {
    let angle = (i as f32 / 8.0) * TAU;
    let radius = base_radius + random_range(-6.0, 6.0);
    Vec2::new(angle.cos() * radius, angle.sin() * radius)
}).collect();
```

Seed the random from asteroid entity ID so the same asteroid always looks the same across frames.

---

## 5. File Scope

Only these files may be modified:

| File | Change |
|------|--------|
| `src/lib.rs` | Starfield spawn, ship triangle, thruster glow, mining beam, asteroid polygon, map marker polish |
| `Cargo.toml` | Only if new dependency required — justify before adding |

**All other files are read-only.**

---

## 6. Verification

No formal gate with logcat evidence required. One screenshot showing all five visual changes simultaneously on the Moto G 2025:

- [ ] Starfield visible in background
- [ ] Ship is a directional triangle with thruster glow
- [ ] Asteroid is an irregular polygon
- [ ] Mining beam visible when mining
- [ ] Map markers are circles with labels and connection lines

Single screenshot. If it looks like a space game, it passes.

**One additional check:** frame rate must remain at or above 55 FPS with all visual additions active. Check logcat `queueBuffer fps` line after 30 seconds of play.

---

## 7. Sequencing

Implement in this order — each is independently verifiable:

1. Starfield — deploy, confirm no frame rate impact
2. Asteroid polygon — deploy, confirm depleted state readable
3. Ship triangle + rotation — deploy, confirm direction tracking
4. Thruster glow — deploy, confirm state-based visibility
5. Mining beam — deploy, confirm appears and disappears correctly
6. Map polish — deploy, confirm all sectors readable

Do not combine steps. Each visual should be confirmed working before the next is added.

---

*Voidrift Visual Polish Directive | April 2026 | RFD IT Services Ltd.*  
*Goal: make it feel like a game. Minimum viable polish. No scope creep.*
