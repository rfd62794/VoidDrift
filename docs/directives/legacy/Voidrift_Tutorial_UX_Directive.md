# Voidrift — Directive: Tutorial Pop-ups, UX Clarity & Map Pan
**Status:** Approved — Ready for Execution  
**Type:** UX / Tutorial / Input  
**Date:** April 2026  
**Depends On:** World Expansion Directive COMPLETE ✅

---

## 1. Objective

Four things that make Voidrift legible to a first-time player:

1. **Tutorial pop-ups** — contextual, one-time, dismissible messages at key decision points
2. **Cargo bar clarity** — ore type label and return-to-station cue
3. **Refinery chain clarity** — SMELTER tab shows full conversion chain explicitly
4. **Map swipe-to-pan** — single finger pans the strategic map freely

These changes are driven directly by observed playtesting. Every fix maps to a specific confusion a real player experienced.

---

## 2. Tutorial Pop-up System

### 2.1 Design Rules

- **One-time only** — each pop-up fires exactly once per session, never again
- **Dismissible** — tap anywhere on the pop-up or the GOT IT button to close
- **Non-blocking** — game continues running behind the pop-up
- **Terse** — maximum 3 sentences. Plain language. No jargon.
- **Matches Signal voice** — functional, not warm. Information, not hand-holding tone.

### 2.2 Data Structure

Add to `components.rs`:

```rust
#[derive(Resource, Default)]
pub struct TutorialState {
    pub shown: HashSet<u32>,     // IDs of pop-ups already shown
    pub active: Option<TutorialPopup>, // currently visible pop-up
}

#[derive(Clone)]
pub struct TutorialPopup {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub button_label: String,    // default: "GOT IT"
}
```

Register `TutorialState` as a resource in `lib.rs`.

### 2.3 Pop-up Definitions — All 6

| ID | Trigger Condition | Title | Body |
|----|------------------|-------|------|
| T-001 | First cargo fill (cargo >= 80% capacity) | CARGO HOLD FULL | Your ship is carrying Magnetite. Return to the station to unload. The station will refine it into Power Cells. |
| T-002 | First dock at station | STATION DOCKED | Open the SMELTER tab to refine your Magnetite into Power Cells. You need 25 Power Cells to repair the station. |
| T-003 | First refinery queue started | PROCESSING STARTED | The station is refining your ore. You can leave — it works while you fly. Return when the queue completes. |
| T-004 | Power Cells first reach 10 | POWER CELLS READY | You have Power Cells. Keep mining and refining until you reach 25. Then use REPAIR in the RESERVES tab. |
| T-005 | Station repaired | STATION ONLINE | The station is running. Build an AI Core in the FORGE tab to begin automating your operation. |
| T-006 | First time near Sector 4 (within 150 units) | EXTRACTION BLOCKED | This deposit requires a Tungsten Laser to extract. Upgrade your equipment to access higher-tier resources. |

### 2.4 Trigger System

Add `tutorial_system` to `narrative.rs` or new `systems/tutorial.rs`:

```rust
pub fn tutorial_system(
    mut tutorial: ResMut<TutorialState>,
    ship_query: Query<(&Ship, &Transform), Without<Station>>,
    station_query: Query<&Station, Without<Ship>>,
    sector4_query: Query<&Transform, (With<AsteroidField>, /* Tungsten */)>,
) {
    // Skip if a pop-up is already active
    if tutorial.active.is_some() {
        return;
    }

    // Check each trigger condition in priority order
    // T-001: cargo fill
    // T-002: first dock
    // T-003: first queue
    // T-004: cells >= 10
    // T-005: station.online
    // T-006: near sector 4
    
    // Fire the lowest-numbered unfired trigger
}
```

Only one pop-up active at a time. If multiple triggers are met simultaneously, fire the lowest ID first.

### 2.5 Pop-up Rendering

Render in `ui.rs` as an `egui::Window`:

```rust
if let Some(popup) = &tutorial.active {
    egui::Window::new(&popup.title)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .frame(egui::Frame::window(&ctx.style()))
        .show(ctx, |ui| {
            ui.label(&popup.body);
            ui.add_space(12.0);
            if ui.button(&popup.button_label).clicked() {
                tutorial.shown.insert(popup.id);
                tutorial.active = None;
            }
        });
    
    // Also dismiss on tap outside
    if ctx.input(|i| i.pointer.any_click()) {
        if let Some(popup) = tutorial.active.take() {
            tutorial.shown.insert(popup.id);
        }
    }
}
```

**Styling:**
- Background: dark panel, same as station UI
- Title: bold, bright white
- Body: normal weight, slightly dim white
- Button: cyan, 44px height, full width

---

## 3. Cargo Bar Clarity

### 3.1 Ore Type Label on Cargo Bar

The cargo bar currently shows fill percentage with no context. Add:

- **Ore type label** above the bar: "MAGNETITE" in small text, matches ore color
- **Count display** below the bar: "47 / 100"
- **Full indicator** — when cargo >= 95%, bar pulses or changes to bright cyan to signal "return now"

### 3.2 Implementation

The cargo bar is a world-space `Mesh2d` child of the ship entity. Add two `Text2d` children:

```rust
// Ore type label — above bar
parent.spawn((
    Text2d::new(""),  // updated each tick by cargo_label_system
    TextFont { font_size: 8.0, ..default() },
    TextColor(Color::srgba(0.8, 0.8, 0.8, 0.7)),
    Transform::from_xyz(0.0, CARGO_BAR_OFFSET_Y + 10.0, Z_HUD),
    CargoOreLabel,  // marker component
));

// Count label — below bar
parent.spawn((
    Text2d::new(""),  // updated each tick
    TextFont { font_size: 7.0, ..default() },
    TextColor(Color::srgba(0.7, 0.7, 0.7, 0.6)),
    Transform::from_xyz(0.0, CARGO_BAR_OFFSET_Y - 10.0, Z_HUD),
    CargoCountLabel,  // marker component
));
```

New system `cargo_label_system` updates these Text2d components each tick:

```rust
fn cargo_label_system(
    ship_query: Query<(&Ship, &Children), Without<Station>>,
    mut ore_label_query: Query<&mut Text2d, (With<CargoOreLabel>, Without<CargoCountLabel>)>,
    mut count_label_query: Query<&mut Text2d, (With<CargoCountLabel>, Without<CargoOreLabel>)>,
) {
    for (ship, children) in &ship_query {
        let ore_text = match ship.cargo_type {
            OreType::Empty => "".to_string(),
            OreType::Magnetite => "MAGNETITE".to_string(),
            OreType::Carbon => "CARBON".to_string(),
        };
        let count_text = if ship.cargo > 0.0 {
            format!("{}/{}", ship.cargo as u32, CARGO_CAPACITY)
        } else {
            "".to_string()
        };
        
        // Update child text entities
    }
}
```

### 3.3 Cargo Full Pulse

When `ship.cargo >= CARGO_CAPACITY * 0.95`, change cargo bar fill color to bright pulsing cyan:

```rust
// In cargo_display_system
if ship.cargo >= CARGO_CAPACITY as f32 * 0.95 {
    let pulse = (time.elapsed_secs() * 3.0).sin() * 0.3 + 0.7;
    fill_material.color = Color::srgba(0.0, pulse, pulse, 1.0);
} else {
    fill_material.color = Color::srgb(0.0, 0.8, 1.0); // normal cyan
}
```

---

## 4. Smelter Tab Clarity

### 4.1 Conversion Chain Display

Each operation card in the SMELTER and FORGE tabs must show the full chain explicitly. No player should wonder what goes in or what comes out.

**Current card header:**
```
MAGNETITE REFINERY
```

**New card header:**
```
MAGNETITE  →  POWER CELLS
10 ore per batch  |  20 sec per batch
Stock: 240 Mag  |  Makes: 24 cells
```

Three lines of context before any buttons appear. The player sees:
- What goes in
- What comes out
- The ratio
- The time
- How many they can make right now

### 4.2 Apply to All Four Operations

| Operation | Header Line 1 | Header Line 2 |
|-----------|-------------|--------------|
| Magnetite Refinery | `MAGNETITE → POWER CELLS` | `10 ore per batch · 20 sec` |
| Carbon Refinery | `CARBON → HULL PLATES` | `5 ore per batch · 30 sec` |
| Hull Forge | `HULL PLATES → SHIP HULL` | `3 plates per batch · 45 sec` |
| Core Fabricator | `POWER CELLS → AI CORE` | `55 cells per batch · 60 sec` |

### 4.3 "Makes X" Dynamic Display

Below the ratio line, show what the player can make right now:

```
You can make: 24 Power Cells (24 batches)
```

If insufficient resources:
```
Need: 10 Magnetite  (have: 3)
```

This answers "why aren't my resources processing" before the player has to ask.

---

## 5. Map Swipe-to-Pan

### 5.1 Behavior

In map view, single-finger drag pans the camera freely. The camera follows the finger movement in world space.

In space view, single-finger tap navigates the ship as before. No pan in space view — pan only in map view.

### 5.2 Implementation

Add to `map.rs`:

```rust
#[derive(Resource, Default)]
pub struct MapPanState {
    pub last_position: Option<Vec2>,
}

pub fn map_pan_system(
    touches: Res<Touches>,
    game_state: Res<State<GameState>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut pan_state: ResMut<MapPanState>,
) {
    // Only active in MapView
    if *game_state.get() != GameState::MapView {
        pan_state.last_position = None;
        return;
    }
    
    // Single touch only — two touches are pinch zoom
    let touch_count = touches.iter().count();
    if touch_count != 1 {
        pan_state.last_position = None;
        return;
    }
    
    if let Some(touch) = touches.iter().next() {
        let current_pos = touch.position();
        
        if let Some(last_pos) = pan_state.last_position {
            let delta = current_pos - last_pos;
            
            if let Ok(mut cam_transform) = camera_query.get_single_mut() {
                // Invert delta — drag right moves world left (camera right)
                cam_transform.translation.x -= delta.x * MAP_PAN_SPEED;
                cam_transform.translation.y += delta.y * MAP_PAN_SPEED; // Y inverted in screen space
            }
        }
        
        pan_state.last_position = Some(current_pos);
    } else {
        pan_state.last_position = None;
    }
}
```

### 5.3 Pan Constants

Add to `constants.rs`:

```rust
pub const MAP_PAN_SPEED: f32 = 1.5;  // world units per screen pixel
```

Tune on device — if too fast the map flies past, if too slow it feels sticky.

### 5.4 Pan Reset on Map Close

When player closes the map (EXIT MAP), reset camera to follow ship:

```rust
// In exit_map_view or map toggle:
pan_state.last_position = None;
// camera_follow_system resumes naturally
```

### 5.5 Register New Systems

Add to `lib.rs`:

```rust
.insert_resource(MapPanState::default())
.add_systems(Update, (
    systems::map::map_pan_system,
    // ... existing systems
))
```

---

## 6. New Marker Components

Add to `components.rs`:

```rust
#[derive(Component)] pub struct CargoOreLabel;
#[derive(Component)] pub struct CargoCountLabel;
```

---

## 7. File Scope

| File | Change |
|------|--------|
| `src/components.rs` | Add TutorialState, TutorialPopup, CargoOreLabel, CargoCountLabel, MapPanState |
| `src/lib.rs` | Register TutorialState, MapPanState resources, register new systems |
| `src/systems/narrative.rs` | Add tutorial_system with 6 trigger conditions |
| `src/systems/ui.rs` | Pop-up rendering, cargo label updates, smelter card clarity improvements |
| `src/systems/map.rs` | Add map_pan_system |
| `Cargo.toml` | READ-ONLY |

---

## 8. Implementation Sequence

1. Add data structures — verify compile
2. Add `tutorial_system` with T-001 only — deploy, verify cargo full pop-up appears on device
3. Add remaining tutorial triggers T-002 through T-006 — deploy, verify each fires correctly
4. Add cargo bar ore label and count — deploy, verify readable on device
5. Add cargo full pulse — deploy, verify visual cue works
6. Update smelter card headers with conversion chain — deploy, verify clarity
7. Add `map_pan_system` — deploy, verify swipe pans map correctly
8. Tune `MAP_PAN_SPEED` on device

---

## 9. Completion Criteria

- [ ] T-001 through T-006 all fire at correct moments — once each
- [ ] Pop-ups dismiss on GOT IT tap or tap outside
- [ ] Cargo bar shows ore type above and count below
- [ ] Cargo bar pulses when full
- [ ] Smelter cards show full conversion chain before buttons
- [ ] "You can make X" dynamic display correct
- [ ] "Need X more" display correct when insufficient resources
- [ ] Map swipe-to-pan works in map view
- [ ] No pan in space view
- [ ] Pinch zoom still works alongside pan (two-finger = zoom, one-finger = pan)
- [ ] No B0001 crashes — all new queries follow Universal Disjointness

**Gate:** Give device to someone unfamiliar with the game. They should be able to complete T-001 through T-004 (mine, dock, refine, repair) without asking what to do.

---

## 10. Design Note

Every pop-up in this directive exists because a real person — your grandfather, playing for the first time — was confused at that exact moment. These aren't hypothetical UX improvements. They're direct responses to observed confusion.

The goal is not to make the game easier. It is to make the game legible. The depth is already there. The player just needs to be able to see it.

---

*Voidrift Tutorial & UX Clarity Directive | April 2026 | RFD IT Services Ltd.*  
*A confused player isn't a bad player. They're a player who deserved better information.*
