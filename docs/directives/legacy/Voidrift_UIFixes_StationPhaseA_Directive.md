# Voidrift — Directive: UI Fixes, Terminal Log & Station Visual Phase A
**Status:** Approved — Ready for Execution  
**Type:** UI Fix + Visual Implementation  
**Date:** April 2026  
**Depends On:** UI Overhaul Directive COMPLETE ✅ | Station Architecture Design LOCKED ✅

---

## 1. Objective

Three things in one directive:

1. **Remove SMELTER and FORGE unlock gates** — both available immediately on dock
2. **Terminal log expansion** — more lines visible, expandable on tap for history
3. **Station Visual Phase A** — rotating hub-and-spoke Main Station replacing the square

These are independent enough to implement sequentially but related enough to ship together.

---

## 2. Part 1 — Unlock Gate Removal

### 2.1 Change

Remove unlock conditions from SMELTER and FORGE tabs. Both are available immediately when docked, regardless of station online state.

**Before:**
```rust
// SMELTER — only if station.online
// FORGE — only if ai_cores > 0 || ship_hulls > 0
```

**After:**
```rust
// SMELTER — always when docked
// FORGE — always when docked
```

POWER and SHIP PORT retain their unlock conditions:
- POWER: `station.online == true`
- SHIP PORT: autonomous ship count > 0

### 2.2 New Signal Lines

Add to `signal_system` — fire once on first dock while station is offline:

| ID | Trigger | Line |
|----|---------|------|
| S-025 | First dock, station offline | `> SMELTER OPERATIONAL. MANUAL MODE.` |
| S-026 | 1.0s after S-025 | `> FORGE OPERATIONAL. MANUAL MODE.` |
| S-027 | Station comes online | `> AUTOMATED SYSTEMS ONLINE.` |

### 2.3 File Scope
- `src/systems/ui.rs` — remove unlock conditions from tab render logic
- `src/systems/narrative.rs` — add S-025, S-026, S-027 triggers

---

## 3. Part 2 — Terminal Log Expansion

### 3.1 Default State

Signal strip at bottom — always visible, full width.

**Default:** 3 lines visible. Slim height. Non-interactive except for tap-to-expand.

### 3.2 Expanded State

Tap anywhere on the Signal strip to expand. Tap again to collapse.

**Expanded:**
- Full-width panel, taller — shows last 20 lines of Signal history
- Scrollable — player can scroll up through older entries
- Same terminal green aesthetic, monospaced, `>` prefix on each line
- Newest line at bottom
- Tap anywhere outside to collapse, or tap strip again

### 3.3 Implementation

Add to `components.rs`:

```rust
#[derive(Resource, Default)]
pub struct SignalStripExpanded(pub bool);
```

In `hud_ui_system`, detect tap on Signal strip area:

```rust
// If strip is tapped — toggle SignalStripExpanded
// If expanded: show ScrollArea with last 20 entries
// If collapsed: show last 3 entries, fixed height
```

egui `ScrollArea::vertical()` for the expanded view. Auto-scroll to bottom on new entry arrival.

### 3.4 File Scope
- `src/components.rs` — add `SignalStripExpanded` resource
- `src/lib.rs` — register resource
- `src/systems/ui.rs` — toggle logic and expanded render

---

## 4. Part 3 — Station Visual Phase A

### 4.1 Scope

Phase A is visual only. No berth gameplay logic. No NPC docking. No autopilot changes.

**What gets built:**
- Hub circle replaces square station sprite
- 6 arms rendered (3 active, 3 stub)
- Berth circles at active arm ends
- Slow rotation — hub + arms + berths rotate as unit
- Hub color reflects online/offline state
- Existing `STATION_POS = (0, 0)` unchanged

**What is deferred to Phase B:**
- Berth entities with `Berth` component
- Autopilot navigating to berth positions
- Berth occupancy display
- Player/drone berth assignment

### 4.2 New Constants

Add to `constants.rs`:

```rust
pub const STATION_HUB_RADIUS: f32     = 40.0;
pub const STATION_ARM_LENGTH: f32     = 120.0;
pub const STATION_ARM_THICKNESS: f32  = 6.0;
pub const STATION_BERTH_RADIUS: f32   = 22.0;
pub const STATION_STUB_LENGTH: f32    = 60.0;
pub const STATION_STUB_ALPHA: f32     = 0.3;
pub const STATION_ROTATION_SPEED: f32 = std::f32::consts::TAU / 90.0;
pub const STATION_BERTHS_INITIAL: u8  = 3;
```

### 4.3 Station Component Extension

Add to `Station` in `components.rs`:

```rust
pub rotation: f32,   // current rotation angle, increments each tick
```

Initialize at `0.0` in `setup_world`.

### 4.4 Station Rotation System

New system in `visuals.rs`:

```rust
pub fn station_rotation_system(
    time: Res<Time>,
    mut station_query: Query<(&mut Station, &Children)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (mut station, children) in &mut station_query {
        station.rotation += STATION_ROTATION_SPEED * time.delta_seconds();
        // Update Transform rotation on station visual entity
    }
}
```

### 4.5 Station Visual Spawn

Replace existing square station spawn in `setup.rs` with hub-and-spoke structure.

All station visual entities are children of the station entity. Rotation applied to parent — children inherit.

**Spawn structure:**
```
Station entity (Transform at STATION_POS)
  ├── Hub circle (Mesh2d Circle, radius=40, Z=Z_ENVIRONMENT)
  ├── Arm 0 (Mesh2d Rectangle, rotated 0°, length=120)
  │     └── Berth 0 circle (Mesh2d Circle, radius=22)
  ├── Arm 1 (Mesh2d Rectangle, rotated 60°, length=120)
  │     └── Berth 1 circle (Mesh2d Circle, radius=22)
  ├── Arm 2 (Mesh2d Rectangle, rotated 120°, length=120)
  │     └── Berth 2 circle (Mesh2d Circle, radius=22)
  ├── Stub 3 (Mesh2d Rectangle, rotated 180°, length=60, alpha=0.3)
  ├── Stub 4 (Mesh2d Rectangle, rotated 240°, length=60, alpha=0.3)
  └── Stub 5 (Mesh2d Rectangle, rotated 300°, length=60, alpha=0.3)
```

**Arm geometry:**
- Rectangle: width=`STATION_ARM_THICKNESS`, height=`STATION_ARM_LENGTH`
- Position: offset from hub center by half arm length in arm direction
- Rotation: arm angle

**Berth position:**
- Offset from hub: `STATION_ARM_LENGTH` in arm direction
- Attached as child of arm entity — inherits arm rotation

### 4.6 Hub Colors

Apply via `Assets<ColorMaterial>` mutation in `station_visual_system` (existing):

```rust
// Hub color based on station.online
hub_material.color = if station.online {
    Color::srgb(1.0, 0.84, 0.0)   // #FFD700 — powered yellow
} else {
    Color::srgb(0.33, 0.27, 0.0)  // #554400 — dark amber derelict
};
```

### 4.7 Map View Update

The station map marker should reflect the new visual identity. Replace the current station circle marker with a smaller version of the hub-and-spoke — hub dot with 3 short lines radiating out. Scale down to map marker size.

This is a visual-only map change. Same tap target behavior as current.

---

## 5. File Scope

| File | Change |
|------|--------|
| `src/constants.rs` | Add station visual constants |
| `src/components.rs` | Add `rotation` to Station, add `SignalStripExpanded` |
| `src/lib.rs` | Register `SignalStripExpanded` |
| `src/systems/setup.rs` | Replace square station spawn with hub-and-spoke |
| `src/systems/visuals.rs` | Add `station_rotation_system` |
| `src/systems/ui.rs` | Remove tab unlock gates, expand Signal strip with tap-to-expand |
| `src/systems/narrative.rs` | Add S-025, S-026, S-027 triggers |
| `src/systems/map.rs` | Update station map marker visual |
| `Cargo.toml` | READ-ONLY |

---

## 6. Implementation Sequence

1. **Part 1 — Unlock gates** — one file change in `ui.rs`, add Signal lines. Deploy, verify SMELTER and FORGE available immediately on dock.
2. **Part 2 — Terminal log** — add resource, add toggle logic, add expanded view. Deploy, verify tap expands correctly on device.
3. **Part 3 — Station visual** — constants, component extension, spawn rewrite, rotation system. Deploy, verify station rotates on device.

Do not combine steps. Each part deployed and verified before the next.

---

## 7. Pre-Implementation Research

Before writing any code:

1. Does Bevy 0.15 support applying rotation to a parent entity and having all children inherit it automatically via the transform hierarchy? Confirm this is the correct approach for station rotation.
2. What is the current entity structure of the station spawn in `setup.rs`? List all child entities currently attached to the station entity.
3. Confirm `egui::ScrollArea::vertical()` is available in egui 0.31.

Report findings before implementation begins.

---

## 8. Completion Criteria

**Part 1:**
- [ ] SMELTER available immediately on first dock
- [ ] FORGE available immediately on first dock
- [ ] S-025 and S-026 fire on first dock pre-online
- [ ] S-027 fires when station comes online

**Part 2:**
- [ ] Signal strip shows 3 lines by default
- [ ] Tap expands to 20-line scrollable history
- [ ] Tap again collapses
- [ ] Auto-scrolls to newest line on expand

**Part 3:**
- [ ] Hub circle visible at station position
- [ ] 3 active arms with berth circles
- [ ] 3 dim stub arms
- [ ] Entire structure rotates slowly — one rotation per 90 seconds
- [ ] Hub color changes between derelict and online states
- [ ] Map marker updated to reflect hub-and-spoke identity
- [ ] No frame rate impact — rotation is smooth at 60 FPS on Moto G 2025

**Gate screenshot:** Station rotating with hub-and-spoke visible, Signal strip expanded showing history, SMELTER tab accessible before repair.

---

*Voidrift UI Fixes + Station Visual Phase A Directive | April 2026 | RFD IT Services Ltd.*  
*The station rotates. The signal is always on. The smelter works without power.*
