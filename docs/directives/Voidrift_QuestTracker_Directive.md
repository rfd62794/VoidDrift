# Voidrift — Directive: Quest Tracker & UI Alignment
**Status:** Approved — Ready for Execution  
**Type:** UI Feature + Polish  
**Date:** April 2026  
**Depends On:** Station Phase B COMPLETE ✅ | Directive A Stabilization COMPLETE ✅

---

## 1. Objective

Two things in one directive:

1. **Quest Tracker** — persistent QUEST button below MAP button in left panel, always visible, shows current objectives, completed objectives, and locked future objectives
2. **UI Alignment pass** — fix station department tab layout, Signal strip sizing, and general left panel alignment issues

---

## 2. Scope

**In scope:**
- `QuestObjective` and `QuestLog` data structures
- QUEST button in left panel — always visible
- Quest panel — opens when QUEST tapped, shows objectives in three sections
- Quest state updates wired to existing Signal trigger IDs
- Station tab layout polish — spacing, sizing, readability
- Signal strip height and text sizing tuning
- Left panel general alignment

**Explicitly out of scope:**
- New Signal lines or narrative content
- New gameplay systems
- Multi-device egui scaling (separate directive)
- Directive B component decomposition
- Any economy or power logic changes

---

## 3. Quest Data Structures

Add to `components.rs`:

```rust
#[derive(PartialEq, Clone)]
pub enum ObjectiveState {
    Locked,    // not yet revealed — shown as dim placeholder
    Active,    // current objective — highlighted
    Complete,  // done — greyed with checkmark
}

#[derive(Clone)]
pub struct QuestObjective {
    pub id: u32,
    pub description: String,
    pub progress_current: Option<u32>,  // None if no progress bar
    pub progress_target: Option<u32>,
    pub state: ObjectiveState,
}

#[derive(Resource)]
pub struct QuestLog {
    pub objectives: Vec<QuestObjective>,
    pub panel_open: bool,
}
```

Initialize in `setup_world` with all 7 objectives. Q-001 starts as `Active`, Q-002 through Q-007 start as `Locked`.

### 3.1 Objective Definitions

```rust
// Initialize QuestLog in setup_world
QuestLog {
    panel_open: false,
    objectives: vec![
        QuestObjective {
            id: 1,
            description: "Locate the signal source".to_string(),
            progress_current: None,
            progress_target: None,
            state: ObjectiveState::Active,
        },
        QuestObjective {
            id: 2,
            description: "Dock at the derelict station".to_string(),
            progress_current: None,
            progress_target: None,
            state: ObjectiveState::Locked,
        },
        QuestObjective {
            id: 3,
            description: "Repair the station".to_string(),
            progress_current: Some(0),
            progress_target: Some(25),
            state: ObjectiveState::Locked,
        },
        QuestObjective {
            id: 4,
            description: "Build an AI Core".to_string(),
            progress_current: None,
            progress_target: None,
            state: ObjectiveState::Locked,
        },
        QuestObjective {
            id: 5,
            description: "Discover Sector 7".to_string(),
            progress_current: None,
            progress_target: None,
            state: ObjectiveState::Locked,
        },
        QuestObjective {
            id: 6,
            description: "Mine Carbon".to_string(),
            progress_current: None,
            progress_target: None,
            state: ObjectiveState::Locked,
        },
        QuestObjective {
            id: 7,
            description: "Assemble autonomous ship".to_string(),
            progress_current: None,
            progress_target: None,
            state: ObjectiveState::Locked,
        },
    ],
}
```

---

## 4. Quest State Trigger Wiring

Wire objective state changes into `signal_system` in `narrative.rs`. When a Signal trigger fires, update the corresponding Quest objective simultaneously.

| Signal ID | Quest Effect |
|-----------|-------------|
| S-003 (AutoPiloting) | Q-001 → Complete, Q-002 → Active |
| S-005 (Docked) | Q-002 → Complete, Q-003 → Active |
| S-009 (First Power Cells) | Q-003 progress_current updates each refine |
| S-010 (25 cells threshold) | Q-003 stays Active — repair not done yet |
| S-011 (Station online) | Q-003 → Complete, Q-004 → Active |
| S-013 (AI Core fabricated) | Q-004 → Complete, Q-005 → Active |
| S-014 (Sector 7 detected) | Q-005 → Complete, Q-006 → Active |
| S-017 (First autonomous ship) | Q-007 → Active |
| First Carbon unloaded | Q-006 → Complete, Q-007 → Active (if not already) |

Q-003 progress updates: whenever Power Cells count changes while Q-003 is Active, update `progress_current` to `min(power_cells, 25)`.

---

## 5. Left Panel Layout

### 5.1 Permanent Elements (always visible)

```
[MAP]          ← existing, unchanged behavior
[QUEST]        ← NEW — always visible, toggles quest panel
```

Separator line between permanent buttons and station tabs:
- Thin horizontal line, dim grey, only visible when docked
- Hides when not docked — clean separation

### 5.2 Station Tabs (docked only, below separator)

```
[RESERVES]
[POWER]        ← greyed until online
[SMELTER]      ← always available when docked
[FORGE]        ← always available when docked
[SHIP PORT]    ← greyed until first autonomous ship
```

### 5.3 Button Styling

All left panel buttons:
- Full width of panel
- Consistent height — `ui.available_width()` for width, fixed height 40.0 logical
- Active/selected: bright text, slight background tint
- Locked: 40% opacity text, no background
- Hover: subtle highlight (egui default is fine)

---

## 6. Quest Panel

Opens when QUEST button tapped. Closes when tapped again or when MAP is tapped.

Quest panel and Map view are mutually exclusive — opening one closes the other.

### 6.1 Panel Location

Right side of screen when open, or overlay — whichever fits the existing egui layout without conflicting with the station tab detail panel. If conflict exists, quest panel replaces the tab detail area when open.

**Simplest approach:** Quest panel opens as a `egui::Window` anchored to the right edge, full height. Non-modal — player can still navigate. Closes on second QUEST tap.

### 6.2 Panel Content

```
╔══════════════════╗
║ OBJECTIVES       ║
╠══════════════════╣
║ ACTIVE           ║
║ ▶ Repair station ║
║   12 / 25 cells  ║
╠══════════════════╣
║ COMPLETED        ║
║ ✓ Locate signal  ║
║ ✓ Dock at station║
╠══════════════════╣
║ UPCOMING         ║
║ ◦ Build AI Core  ║
║ ◦ ...            ║
╚══════════════════╝
```

**Colors:**
- Active objective: bright white text, `>` prefix
- Progress bar under active: cyan fill, grey background, shows X/Y
- Completed: dim grey text, `✓` prefix
- Locked/Upcoming: very dim, `◦` prefix, no detail shown

**Font:** FiraSans-Bold, same as Signal strip. Consistent terminal aesthetic.

### 6.3 Progress Bar for Q-003

When Q-003 is Active, show a simple progress bar below the description:

```
Repair the station
[████████░░░░░░░░] 12/25
```

Implemented as egui `ProgressBar` or as two rectangles — whichever is simpler.

---

## 7. Signal Strip UI Alignment

The Signal strip has sizing and positioning issues noted from device testing. Apply these fixes:

- **Height:** Increase to 3 lines minimum when collapsed. Current 2-line height clips on some content.
- **Font size:** Verify at EGUI_SCALE 3.0 the text is readable without being enormous. Target: comfortable reading at arm's length on phone.
- **Padding:** Add 4px top/bottom internal padding to prevent text clipping at strip edges.
- **Expanded state:** When expanded (tapped), show last 20 lines in `ScrollArea`. Ensure `stick_to_bottom` is working — newest entry should always be at bottom on open.

---

## 8. Station Tab Detail Panel Alignment

Current issues: cramped layout, inconsistent spacing, buttons too close together.

Apply these fixes to all tab detail panels:

- **Section spacing:** 8px gap between sections (resource row, button row)
- **Button height:** Minimum 44px for all action buttons (thumb-friendly)
- **Resource display:** Use egui `Grid` for aligned columns rather than horizontal label concatenation
- **Greyed buttons:** Consistent opacity — use `ui.set_enabled(false)` rather than manual color manipulation where possible

---

## 9. New System: quest_update_system

Add to `narrative.rs` or new `systems/quest.rs`:

```rust
pub fn quest_update_system(
    mut quest_log: ResMut<QuestLog>,
    station_query: Query<&Station>,
    ship_query: Query<&Ship, Without<Station>>,
) {
    // Update Q-003 progress if active
    if let Some(q3) = quest_log.objectives.iter_mut()
        .find(|o| o.id == 3 && o.state == ObjectiveState::Active) 
    {
        if let Ok(station) = station_query.get_single() {
            q3.progress_current = Some(station.power_cells.min(25));
        }
    }
}
```

Runs every tick. Lightweight — only updates progress values, no state transitions. State transitions happen in `signal_system` only.

---

## 10. File Scope

| File | Change |
|------|--------|
| `src/components.rs` | Add QuestObjective, ObjectiveState, QuestLog |
| `src/lib.rs` | Register QuestLog resource, register quest_update_system |
| `src/systems/setup.rs` | Initialize QuestLog with 7 objectives |
| `src/systems/narrative.rs` | Wire quest state changes to Signal triggers |
| `src/systems/ui.rs` | Add QUEST button to left panel, quest panel rendering, Signal strip fixes, tab alignment fixes |
| `src/systems/quest.rs` (new) | quest_update_system for progress tracking |
| `src/systems/mod.rs` | Add pub mod quest if new file created |
| `Cargo.toml` | READ-ONLY |

---

## 11. Implementation Sequence

1. Add data structures to `components.rs`, initialize in `setup_world` — verify compile
2. Add QUEST button to left panel — deploy, verify button visible on device
3. Implement quest panel rendering — deploy, verify panel opens and shows objectives
4. Wire quest state to Signal triggers in `narrative.rs` — deploy, verify Q-001 completes on approach
5. Add `quest_update_system` for Q-003 progress — deploy, verify progress bar updates
6. Signal strip alignment fixes — deploy, verify readability
7. Station tab detail panel alignment fixes — deploy, verify spacing

Deploy after each step. Do not combine steps.

---

## 12. Completion Criteria

- [ ] QUEST button visible below MAP button always
- [ ] Quest panel opens and closes on tap
- [ ] Three sections visible: Active, Completed, Upcoming
- [ ] Q-001 starts Active on game launch
- [ ] Q-002 activates when ship begins approach to station
- [ ] Q-003 activates on first dock, progress updates with Power Cells
- [ ] Q-003 completes on repair, Q-004 activates
- [ ] Signal strip shows minimum 3 lines, text readable on device
- [ ] Station tab buttons minimum 44px height
- [ ] All tab detail panels use consistent spacing
- [ ] Quest panel mutual exclusivity with Map view
- [ ] Gate screenshot: quest panel open showing active objective with progress bar

---

*Voidrift Quest Tracker & UI Alignment Directive | April 2026 | RFD IT Services Ltd.*  
*The player always knows where they are. The Signal tells them what happened. The Quest tells them where they're going.*
