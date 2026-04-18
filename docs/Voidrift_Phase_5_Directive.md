# Voidrift — Phase 5 Directive: Station Repair & Slice Complete
**Status:** Approved — Ready for Execution  
**Gate Phase:** 5 — Final Slice Gate  
**Date:** April 2026  
**Depends On:** Phase 4 Gate PASSED ✅

---

## 1. Objective

Complete the MVP slice. The player uses accumulated power cells to repair the derelict station. When repair is complete, the station visual changes to its online state and the slice is done.

This is the emotional payoff of everything built across Phases 0–4. The station coming online is the moment the game earns. Everything in this phase serves that moment.

---

## 2. Scope Boundaries

> ⚠️ HARD LIMIT: Phase 5 is repair action, station visual state change, and slice completion screen only.

**In scope:**
- Repair button added to the docking egui panel
- Repair action consumes `REPAIR_COST = 25` power cells
- Repair progress tracked on a `Station` component (`repair_progress: f32`, range 0.0–1.0)
- Station Mesh2d color changes when `repair_progress >= 1.0` (derelict → online)
- Slice completion state — simple egui overlay: "Station Online. Slice Complete."
- Repair button disabled/greyed when insufficient power cells

**Explicitly out of scope — do not implement:**
- Animated repair sequence
- Sound effects
- Save/load or persistence
- Any crew or fleet system
- New sectors or navigation destinations
- Any content beyond the slice completion screen

---

## 3. Economic Constants — Unchanged

All constants remain locked. Do not modify:

| Constant | Value |
|----------|-------|
| `REPAIR_COST` | 25 |
| `REFINERY_RATIO` | 10 |
| `CARGO_CAPACITY` | 100 |
| `MINING_RATE` | 8.0 |
| `SHIP_SPEED` | 120.0 |
| `EGUI_SCALE` | 3.0 |

At `REPAIR_COST = 25` and `REFINERY_RATIO = 10`, the player needs 250 ore minimum across approximately 3 full mining runs. This pacing is intentional.

---

## 4. Technical Specification

### 4.1 Station Component Extension

Add repair tracking to the existing Station component:

```rust
#[derive(Component)]
struct Station {
    repair_progress: f32,  // 0.0 = derelict, 1.0 = online
    online: bool,
}
```

Initialize at spawn: `repair_progress: 0.0, online: false`

### 4.2 Repair Action

Triggered when player taps Repair button while `ShipState::Docked` and `ship.power_cells >= REPAIR_COST`:

```
// Pseudocode
fn repair_action(ship, station) {
    if ship.power_cells < REPAIR_COST { return; }
    ship.power_cells -= REPAIR_COST;
    station.repair_progress = 1.0;  // Instant repair — no animation in slice
    station.online = true;
    log("[Voidrift Phase 5] Station repair complete. Slice done.");
}
```

Repair is instant in the slice — no incremental progress bar. One payment, one result.

### 4.3 Docking UI Extension

Extend the existing `hud_ui_system` egui bottom panel to include the Repair button alongside the existing Refine button:

| Element | Condition | Behaviour |
|---------|-----------|-----------|
| REPAIR button (active) | `power_cells >= 25` | Tappable, cyan, triggers repair action |
| REPAIR button (disabled) | `power_cells < 25` | Greyed out, not tappable, shows cost: "REPAIR (25 cells)" |
| REPAIR button (hidden) | `station.online == true` | Not shown — repair already complete |

Display current power cell count prominently so the player always knows how close they are to the repair threshold.

### 4.4 Station Visual State Change

When `station.online` transitions to `true`, update the Station Mesh2d color:

| State | Color |
|-------|-------|
| Derelict (default) | Yellow (existing Phase 1 color) |
| Online | Bright White or Light Blue — clearly distinct from derelict |

This is a direct component mutation on the existing Station Mesh2d entity. No despawn/respawn. Change the `MeshMaterial2d` color handle in place.

Log: `[Voidrift Phase 5] Station visual: online state activated.`

### 4.5 Slice Completion Screen

When `station.online == true`, display a persistent egui overlay:

```
// egui — centred modal-style panel
egui::Window::new("slice_complete")
    .title_bar(false)
    .resizable(false)
    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
    .show(ctx, |ui| {
        ui.heading("STATION ONLINE");
        ui.label("Slice Complete.");
    });
```

This overlay is persistent — it does not disappear. The slice is done. No restart, no menu, no further interaction required for the gate.

---

## 5. File Scope

Only these files may be modified in Phase 5:

| File | Change |
|------|--------|
| `src/lib.rs` | Station component extension, repair action, docking UI repair button, station color change, slice completion overlay |
| `Cargo.toml` | Only if a new dependency is required — justify before adding |

**All other files are read-only for this phase.**

---

## 6. Test Anchors

All 6 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P5-01 | Repair button visible when docked | Visual: button present in bottom bar |
| TB-P5-02 | Repair button greyed when cells < 25 | Visual: button inactive with cost shown |
| TB-P5-03 | Repair button active when cells >= 25 | Visual: button tappable, cyan |
| TB-P5-04 | Repair action deducts 25 power cells | Logcat: cell count decreases by 25 |
| TB-P5-05 | Station color changes to online state | Visual: station rectangle changes color on device |
| TB-P5-06 | Slice completion overlay appears | Visual: "STATION ONLINE / Slice Complete." visible on device |

---

## 7. Gate 5 Completion Criteria

All of the following must be true before Phase 5 — and the MVP slice — is marked complete:

- [ ] App launches on Moto G 2025 without crash
- [ ] Repair button visible in docking UI
- [ ] Repair button correctly disabled below 25 cells
- [ ] Repair consumes exactly 25 power cells — logcat confirms
- [ ] Station Mesh2d color visibly changes on device
- [ ] Slice completion overlay appears and persists
- [ ] Full loop playable: mine → refine → repair across ~3 runs
- [ ] All 6 test anchors TB-P5-01 through TB-P5-06 verified
- [ ] Gate screenshot (TB-P5-GATE) shows slice completion overlay on device

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — "STATION ONLINE / Slice Complete." visible on device, raw binary PNG
3. Logcat showing repair action and station online log lines

---

## 8. Known Risks

| Risk | Mitigation |
|------|-----------|
| MeshMaterial2d color change may require handle swap not direct mutation | Research correct Bevy 0.15 pattern for runtime color change on existing Mesh2d before implementing |
| egui Window overlay may conflict with bottom panel z-ordering | Use `egui::Window` not `egui::TopBottomPanel` for the completion screen — windows float above panels |
| Repair button tap registering while greyed | Check cell count in the repair system, not just in the UI — double-gate the condition |

---

## 9. Slice Completion Note

When Gate 5 passes, the Voidrift MVP slice is complete. What has been built and proven:

- Bevy 0.15 on Android (API 35, Moto G 2025) — stable pipeline established
- Touch-based selection navigation — proven
- Autopilot movement system — proven
- Mining resource accumulation loop — proven
- Refinery conversion economy — proven
- egui HUD on Mali GPU — proven stable
- Station repair as narrative payoff — proven

The next session after Gate 5 will scope Phase 6 — the first post-slice feature. That conversation happens after the slice is complete, not before.

---

*Voidrift Phase 5 Directive | April 2026 | RFD IT Services Ltd.*  
*This is the final MVP slice gate. The slice is not complete until Gate 5 passes on device.*
