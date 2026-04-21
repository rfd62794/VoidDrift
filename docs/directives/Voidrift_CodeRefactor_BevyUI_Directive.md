# Voidrift — Combined Directive: Code Refactor + Bevy UI Migration
**Status:** Approved — Ready for Execution  
**Type:** Structural Refactor + UI Framework Migration  
**Date:** April 2026  
**Depends On:** Documentation Refactor v2 COMPLETE ✅  
**Blocks:** Economy Redesign Directive

---

## 1. Objective

Two phases, one directive, one hard gate between them.

**Phase A — Code Refactor:**
Split oversized files, fix two registration bugs, remove dead component.
Zero logic changes. Zero UI changes. Pure structural work.

**Phase B — Bevy UI Migration:**
Replace egui panels with Bevy UI nodes, panel by panel.
Only begins after Phase A passes the device gate.

---

## 2. The Hard Gate

> [!IMPORTANT]
> **Phase B cannot begin until Phase A passes the device gate.**
> The gate is: app launches, opening sequence completes, ship docks,
> Signal strip visible, RESERVES tab accessible, 60 seconds of play
> with no crash.
> If the gate fails, stop. Fix Phase A. Do not touch Phase B.
> A failure after Phase B begins is a UI regression. A failure during
> Phase A is a refactor regression. The gate is the isolation mechanism.

---

## 3. Phase A — Code Refactor

### 3.1 Bug Fixes (do these first, before any splits)

**Bug 1: `quest_update_system` not registered**
In `lib.rs`, `quest_update_system` is defined in `quest.rs` but never
added to the Update schedule. Add it to Group 2 (Station/Narrative/UI).

```rust
// In Group 2 system tuple, add:
systems::quest::quest_update_system,
```

Effect: Q-003 progress bar now updates at runtime.

**Bug 2: `autopilot_system` double-registration**
In `lib.rs`, `autopilot_system` appears twice in the Update schedule.
A system registered twice runs twice per frame — all movement calculations,
docking transitions, and DockedAt insertions execute twice.
Remove the duplicate. Keep only one registration.

Verify: search `lib.rs` for `autopilot_system` — exactly one occurrence
must remain after the fix.

### 3.2 Dead Component Removal

**`CargoBarFill` duplicate:**
`components.rs` contains both `CargoBarFill` and `ShipCargoBarFill`.
Audit which one is actually queried in system files.
Remove the unused one. Update any references if needed.

Do not remove both — one is actively used. Identify which before deleting.

### 3.3 Module Splits

**`setup.rs` (641 lines) — split into focused functions:**

Do not create new files. Extract into clearly named private functions
within `setup.rs`:

```rust
fn setup_world(/* ... */) {
    spawn_starfield(&mut commands, &mut meshes, &mut materials);
    spawn_station(&mut commands, &mut meshes, &mut materials, &asset_server);
    spawn_asteroids(&mut commands, &mut meshes, &mut materials, &asset_server);
    spawn_ships(&mut commands, &mut meshes, &mut materials);
    spawn_berths(&mut commands);
    spawn_map_markers(&mut commands, &mut meshes, &mut materials);
}
```

Each sub-function is private. `setup_world` becomes an orchestrator.
No new modules, no new files — just function extraction within the
same file. This reduces the 487-line `setup_world` to ~30 lines.

**`narrative.rs` (407 lines) — split into three files:**

Create:
- `src/systems/opening_sequence.rs` — `opening_sequence_system` only
- `src/systems/signal.rs` — `signal_system` only  
- `src/systems/tutorial.rs` — `tutorial_system` only

Keep `narrative.rs` as a re-export module:
```rust
// narrative.rs
pub mod opening_sequence;
pub mod signal;
pub mod tutorial;
```

Update `lib.rs` system registrations to use new paths:
```rust
systems::opening_sequence::opening_sequence_system,
systems::signal::signal_system,
systems::tutorial::tutorial_system,
```

**`ui.rs` (405 lines) — split into three files:**

Create:
- `src/systems/hud.rs` — Signal strip rendering, cargo labels, tutorial popup
- `src/systems/station_tabs.rs` — All station department tab content
- `src/systems/quest_ui.rs` — Quest panel rendering

Keep `ui.rs` as a re-export module:
```rust
// ui.rs
pub mod hud;
pub mod station_tabs;
pub mod quest_ui;
```

Update `lib.rs` system registrations.

**`mod.rs` updates:**

After all splits, update `src/systems/mod.rs` to declare all new modules:
```rust
pub mod opening_sequence;
pub mod signal;
pub mod tutorial;
pub mod hud;
pub mod station_tabs;
pub mod quest_ui;
// ... existing modules unchanged
```

### 3.4 Phase A Verification

1. `cargo check` — zero errors, zero warnings
2. `cargo build --release` — clean build
3. `.\build_android.ps1` — APK produced
4. App launches on Moto G 2025 — no B0001, no crash
5. Opening sequence completes — ship docks, Signal fires through S-007
6. RESERVES tab accessible — Q-003 progress bar updates (bug fix verification)
7. 60 seconds of play — mine, return, dock, no crash

**All 7 must pass. If any fail, stop and fix before Phase B.**

---

## 4. Phase B — Bevy UI Migration

Only begins after Phase A device gate passes.

### 4.1 Feature Flags

**First change in Phase B. Verify compile before anything else.**

Update `Cargo.toml` features:
```toml
bevy = { version = "0.15.3", default-features = false, features = [
    "bevy_winit",
    "bevy_window",
    "bevy_render",
    "bevy_sprite",
    "bevy_text",
    "bevy_state",
    "bevy_asset",
    "bevy_core_pipeline",
    "bevy_ui",                      # NEW
    "bevy_picking",                 # NEW
    "bevy_ui_picking_backend",      # NEW
    "default_font",                 # NEW
    "android_shared_stdcxx",
    "android-game-activity",
] }
```

Run `cargo check` after this change. Fix any compilation issues before
proceeding. Do not touch any system files until this compiles cleanly.

### 4.2 UI Architecture

**Portrait layout (primary — 720×1604):**

```
┌─────────────────────────────┐  ← top
│                             │
│       WORLD VIEW            │
│    (camera renders here)    │
│    flex: 1, fills space     │
│                             │
├───────────┬─────────────────┤  ← ~55% from bottom
│ LEFT NAV  │ CONTEXT PANEL   │
│ 30% width │ 70% width       │
│           │                 │
│ [MAP]     │ Tab content     │
│ [QUEST]   │ or ship status  │
│ ────────  │                 │
│ [RESERVES]│                 │
│ [POWER]   │                 │
│ [SMELTER] │                 │
│ [FORGE]   │                 │
│ [SHIPPORT]│                 │
├───────────┴─────────────────┤  ← 64px from bottom
│     SIGNAL STRIP            │
│     always visible          │
└─────────────────────────────┘  ← bottom
```

**Landscape layout (secondary — 1200×2000 onn tablet):**

```
┌────────┬──────────────────┬─────────┐
│        │                  │         │
│  LEFT  │   WORLD VIEW     │ CONTEXT │
│  NAV   │                  │  PANEL  │
│ 20%    │      60%         │   20%   │
│        │                  │         │
├────────┴──────────────────┴─────────┤
│         SIGNAL STRIP (64px)         │
└─────────────────────────────────────┘
```

**Orientation detection:**
```rust
fn is_landscape(window: &Window) -> bool {
    window.physical_width() > window.physical_height()
}
```

**Context panel states:**
- Not docked: minimal ship status (engine tier, cargo, power)
- Docked + tab selected: tab content
- QUEST tapped: quest panel overlays context
- MAP tapped: map view overlays world

### 4.3 Migration Sequence — Panel by Panel

Migrate in this exact order. Deploy and verify on device after each panel.
Do not proceed to next panel until current panel is verified.

**Step 1: Signal Strip**

Replace `egui::TopBottomPanel::bottom` signal strip with Bevy UI.
Keep egui for everything else — the strip is the lowest risk, highest
visibility panel. Verify it appears on device, text is readable,
tap-to-expand works.

```rust
// Bevy UI node structure
Node { // root — full width, 64px, anchored bottom
    Node { // scrollable content area
        Text::new(signal_line) // each line
    }
    // tap interaction via Pointer<Click> observer
}
```

All interactions use `Pointer<Click>` observers, not `Interaction`.
Spawn in a state-enter system, not `Startup`, to avoid text scale bug.

**Step 2: Left Navigation Panel**

Replace `egui::SidePanel::left` with Bevy UI.
MAP button, QUEST button, separator, station tabs.
Percentage-based width: `Val::Percent(30.0)` portrait,
`Val::Percent(20.0)` landscape.

All tab buttons: `Val::Px(44.0)` minimum height.
Locked tabs: `BackgroundColor` dimmed, `Pickable` disabled.

**Step 3: Context Panel — Minimal Ship Status**

The context panel when not docked shows:
- Engine tier (Mk I currently)
- Cargo: [ore type] [count]/[capacity]
- Power: [current]/[max]

This is read-only. No buttons. Updates each tick via a query system.

**Step 4: Station Tab Content — RESERVES**

Replace RESERVES egui content with Bevy UI.
Resource counts in a Grid layout.
REPAIR button (pre-online only) as a Bevy UI Button with Pointer<Click>.
Auto-dock toggles as Bevy UI checkboxes.

**Step 5: Station Tab Content — POWER, SMELTER, FORGE, SHIP PORT**

Migrate remaining tabs one at a time.
Processing queue cards with progress bars.
Queue buttons (+1, +10, MAX, CLEAR) with Pointer<Click>.

**Step 6: Quest Panel**

Replace quest egui Window with Bevy UI overlay.
Three sections: ACTIVE, COMPLETED, UPCOMING.
Progress bar for Q-003 using Bevy UI node with percentage width.

**Step 7: Tutorial Popup**

Replace tutorial egui Window with Bevy UI overlay.
Centered, non-blocking, GOT IT button with Pointer<Click>.

**Step 8: Remove egui**

Only after all 7 panels verified on device:
- Remove `bevy_egui` from `Cargo.toml`
- Remove `EguiPlugin` from `lib.rs`
- Remove any remaining egui imports
- Final device gate

### 4.4 Bevy UI Technical Requirements

**All UI spawned in state-enter systems, not Startup.**
Avoids text scale bug on high-DPI Android.

**All interactive elements use Pointer<Click> observers.**
`Interaction` component is deprecated — do not use.

**All widths percentage-based at root level.**
`Val::Percent(100.0)` on root nodes = full screen width/height.

**All touch targets minimum 44px height.**
`Val::Px(44.0)` on all buttons and tab items.

**Two-finger touch suppresses single-finger navigation.**
Existing `pinch_zoom_system` already handles this.
Verify it still functions after migration.

**Universal Disjointness maintained.**
Any new system that queries Transform must follow INV-004.

### 4.5 Phase B Verification — Per Panel

After each panel migration, verify on device:
- Panel renders correctly in portrait orientation
- All buttons/interactions respond to touch
- No layout drift or misalignment
- No B0001 crash
- Existing gameplay functionality preserved

### 4.6 Final Phase B Gate

After Step 8 (egui removed):
1. `cargo check` — zero errors
2. `.\build_android.ps1` — clean APK
3. Full opening sequence on device — no egui artifacts
4. All 5 station tabs functional
5. Quest panel opens and closes
6. Tutorial popup appears and dismisses
7. Signal strip expands and collapses
8. 5 minutes of gameplay — mine, dock, queue processing, no crash

---

## 5. File Scope

**Phase A — files touched:**

| File | Change |
|------|--------|
| `src/lib.rs` | Fix quest_update_system registration, fix autopilot double-registration, update module paths |
| `src/components.rs` | Remove unused CargoBarFill or ShipCargoBarFill |
| `src/systems/setup.rs` | Extract sub-functions within file |
| `src/systems/narrative.rs` | Convert to re-export module |
| `src/systems/opening_sequence.rs` | CREATE — extracted from narrative.rs |
| `src/systems/signal.rs` | CREATE — extracted from narrative.rs |
| `src/systems/tutorial.rs` | CREATE — extracted from narrative.rs |
| `src/systems/ui.rs` | Convert to re-export module |
| `src/systems/hud.rs` | CREATE — extracted from ui.rs |
| `src/systems/station_tabs.rs` | CREATE — extracted from ui.rs |
| `src/systems/quest_ui.rs` | CREATE — extracted from ui.rs |
| `src/systems/mod.rs` | Declare all new modules |
| `Cargo.toml` | READ-ONLY in Phase A |

**Phase B — files touched:**

| File | Change |
|------|--------|
| `Cargo.toml` | Add bevy_ui, bevy_picking, bevy_ui_picking_backend, default_font features; remove bevy_egui (Step 8 only) |
| `src/lib.rs` | Remove EguiPlugin (Step 8), add UI node spawning systems |
| `src/systems/hud.rs` | Replace egui signal strip with Bevy UI |
| `src/systems/station_tabs.rs` | Replace egui tab content with Bevy UI |
| `src/systems/quest_ui.rs` | Replace egui quest panel with Bevy UI |
| `src/systems/setup.rs` | Add persistent UI node tree spawn |
| `src/components.rs` | Add new UI marker components |

---

## 6. What Does Not Change

- Zero gameplay logic changes in either phase
- Zero economy changes (that is the next directive)
- Zero Signal content changes
- Zero quest objective changes
- All game constants unchanged
- All ECS invariants (INV-004 through INV-008) maintained

---

## 7. Completion Criteria

**Phase A:**
- [ ] `quest_update_system` registered — Q-003 progress updates on device
- [ ] `autopilot_system` appears exactly once in lib.rs
- [ ] One of CargoBarFill/ShipCargoBarFill removed
- [ ] `setup_world` reduced to orchestrator function (~30 lines)
- [ ] `narrative.rs` split into 3 files
- [ ] `ui.rs` split into 3 files
- [ ] `cargo check` passes
- [ ] Device gate: all 7 criteria pass

**Phase B:**
- [ ] Feature flags added, cargo check passes
- [ ] Signal strip: Bevy UI, verified on device
- [ ] Left nav panel: Bevy UI, tabs work, correct widths
- [ ] Context panel: minimal ship status visible when not docked
- [ ] RESERVES tab: Bevy UI, resource counts correct
- [ ] POWER, SMELTER, FORGE, SHIP PORT tabs: Bevy UI
- [ ] Quest panel: opens, three sections, progress bar
- [ ] Tutorial popup: appears, dismisses
- [ ] egui removed entirely
- [ ] Final gate: 5 minutes gameplay, no crash

---

## 8. Note to Agent

Phase A has no creative decisions. Every change is mechanical extraction.
If you encounter something that requires a design decision during Phase A,
stop and report it rather than deciding independently.

Phase B has layout decisions per panel. Follow the specs in §4.2 and §4.3.
If a Bevy UI API behaves differently than specified, report the discrepancy
before implementing a workaround.

The hard gate between phases is not a suggestion. It is a requirement.
A device that crashes after Phase A means Phase A is not done.

---

*Voidrift Combined Refactor + UI Migration Directive | April 2026 | RFD IT Services Ltd.*  
*Phase A cleans the house. Phase B rebuilds the windows. The gate ensures the house stands first.*
