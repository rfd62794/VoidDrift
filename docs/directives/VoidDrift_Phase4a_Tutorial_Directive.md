# VoidDrift — Phase 4a: Tutorial System
**Directive Version:** 1.0  
**Date:** May 2, 2026  
**Branch:** `dev`  
**Prerequisite:** v2.8.6-balance-speed-mining tagged and live

---

## AGENT CONTRACT

You are implementing a tutorial system that guides cold players through VoidDrift's core loop using Echo's voice. The tutorial uses a combination of a new `TutorialHighlight` spatial ring and the existing popup system.

**You are NOT allowed to:**
- Modify the existing `DestinationHighlight` system
- Change any balance constants
- Touch save/load system
- Modify any system not listed in the File Touch Map

**You ARE responsible for:**
- Adding `TutorialHighlight` component and entity
- Driving `TutorialHighlight` position from `tutorial_system`
- Writing six tutorial popup entries in Echo's voice
- Wiring popup triggers to correct game state conditions
- Ensuring tutorial only fires on new game, not on load

**Definition of Done:**
- `TutorialHighlight` ring appears on nearest asteroid at game start
- Six popups fire in sequence, each triggered by correct condition
- Popups are dismissable by tap/click or condition advance
- Tutorial does not fire when loading an existing save
- `cargo check` clean, zero warnings
- Verified on device — cold new game walks through all six steps

---

## Context: What Already Exists

### TutorialState (`src/components/resources.rs:165-177`)
```rust
pub struct TutorialState {
    pub shown: HashSet<u32>,      // IDs of popups already shown
    pub active: Option<TutorialPopup>, // Currently displayed popup
}
```

### TutorialPopup
```rust
pub struct TutorialPopup {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub button_label: String,
}
```

### tutorial_system (`src/systems/ui/tutorial.rs`)
Existing trigger logic with 6 popups T-001 through T-006. New tutorial steps added here as additional `if` blocks.

### Popup Rendering (`src/systems/ui/hud/mod.rs:289-316`)
egui Window, CENTER_CENTER anchored, cyan title, body text, dismiss button. No changes needed.

### DestinationHighlight (`src/systems/visuals/map.rs:56-71`)
Single Annulus ring tracking active `AutopilotTarget.destination`. Visible during `ShipState::Navigating`. Do not modify.

---

## Phase 4a-1: TutorialHighlight Entity

### New Component
Add to `src/components/markers.rs`:
```rust
pub struct TutorialHighlight;
```

### Entity Spawn
Add to `src/systems/setup/entity_setup.rs` alongside existing `DestinationHighlight` spawn:

```rust
// Tutorial spatial highlight — driven by tutorial_system
commands.spawn((
    MapElement,
    TutorialHighlight,
    Mesh2d(meshes.add(Annulus::new(38.0, 40.0))),
    MeshMaterial2d(materials.add(ColorMaterial {
        color: Color::srgba(0.0, 1.0, 1.0, 0.6), // Cyan — distinct from white DestinationHighlight
        alpha_mode: AlphaMode2d::Opaque,
        ..default()
    })),
    Transform::from_xyz(0.0, 0.0, Z_HUD - 0.05),
    Visibility::Hidden,
));
```

Cyan color distinguishes it from the white `DestinationHighlight`.

### Position Driver
In `tutorial_system`, when Tutorial step T-NEW-001 is active:
- Query all active asteroids
- Find nearest asteroid to station (or any asteroid if none closer)
- Set `TutorialHighlight` Transform to that asteroid's world position
- Set Visibility to Visible
- Hide when popup is dismissed or player taps the asteroid

---

## Phase 4a-2: Tutorial Popup Content

Six popups in sequence. All titles are "ECHO" — her voice, her station, her guidance.

### T-101: Game Start — Point at asteroid
**Trigger:** `OpeningPhase::Complete` AND no existing save (new game only)  
**Condition check:** `tutorial.shown.contains(&101)` is false  
**TutorialHighlight:** Visible, positioned on nearest asteroid

```rust
TutorialPopup {
    id: 101,
    title: "ECHO".into(),
    body: "Mining protocols active. Tap the highlighted asteroid to dispatch a drone.".into(),
    button_label: "Understood".into(),
}
```

### T-102: Drone Dispatched
**Trigger:** First `ShipState::Navigating` detected after T-101 shown  
**TutorialHighlight:** Hidden (DestinationHighlight takes over naturally)

```rust
TutorialPopup {
    id: 102,
    title: "ECHO".into(),
    body: "Drone en route. It will return automatically when cargo is full.".into(),
    button_label: "Understood".into(),
}
```

### T-103: First Dock — Point at drawer
**Trigger:** First drone docks and unloads after T-102 shown  
**TutorialHighlight:** Hidden (drawer is UI, not world space)

```rust
TutorialPopup {
    id: 103,
    title: "ECHO".into(),
    body: "Ore secured. Open the station drawer to check reserves. Tap the grey bar at the bottom of the screen.".into(),
    button_label: "Understood".into(),
}
```

### T-104: Drawer Opened
**Trigger:** `DrawerState::Expanded` detected after T-103 shown

```rust
TutorialPopup {
    id: 104,
    title: "ECHO".into(),
    body: "Production pipeline standing by. Switch to the FORGE tab to enable automatic processing.".into(),
    button_label: "Understood".into(),
}
```

### T-105: FORGE Tab Opened
**Trigger:** `ActiveStationTab::Production` detected after T-104 shown

```rust
TutorialPopup {
    id: 105,
    title: "ECHO".into(),
    body: "Refinery online. Materials will process automatically. Return to mining — more ore means more drones.".into(),
    button_label: "Understood".into(),
}
```

### T-106: Bottle Visible
**Trigger:** `ActiveBottle` entity exists in world after T-105 shown  
**TutorialHighlight:** Visible, positioned on the bottle entity

```rust
TutorialPopup {
    id: 106,
    title: "ECHO".into(),
    body: "Signal detected. Dispatch a drone to retrieve it.".into(),
    button_label: "Understood".into(),
}
```

---

## Phase 4a-3: New Game Guard

Tutorial must not fire when loading an existing save.

In `restore_save_state` or `ingame_startup_system` load path:
- If save exists and is restored: insert `TutorialState` with all IDs 101-106 pre-populated in `shown`
- This prevents any tutorial popup from firing on load

New game path: `TutorialState` starts empty as normal.

---

## Verification Checklist

### New Game Flow
- [ ] T-101 fires after opening sequence, TutorialHighlight visible on asteroid
- [ ] Tapping asteroid dismisses highlight, T-102 fires
- [ ] Drone returns, T-103 fires
- [ ] Opening drawer triggers T-104
- [ ] Switching to FORGE tab triggers T-105
- [ ] Bottle spawning at 45s triggers T-106, highlight on bottle
- [ ] All six popups dismissable by button tap
- [ ] Tutorial does not repeat after dismissal

### Load Game Flow
- [ ] Loading existing save — zero tutorial popups fire
- [ ] TutorialHighlight hidden throughout loaded session

### Regression
- [ ] Existing T-001 through T-006 popups unaffected
- [ ] DestinationHighlight still tracks navigation normally
- [ ] `cargo check` clean, zero warnings

---

## File Touch Map

**Modified:**
- `src/components/markers.rs` — add TutorialHighlight component
- `src/systems/setup/entity_setup.rs` — spawn TutorialHighlight entity
- `src/systems/ui/tutorial.rs` — add T-101 through T-106 trigger blocks, TutorialHighlight position driver
- `src/systems/persistence/save.rs` or `src/scenes/main_menu.rs` — new game guard, pre-populate shown IDs on load

**Created:**
- None

---

## Out of Scope

- Modifying DestinationHighlight
- Any balance constant changes
- LOGS tab or FORGE rename (Phase 4b)
- Narrative drops (Phase 4b)
- Any new gameplay features
- Save format changes beyond the new game guard
