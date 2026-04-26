# Voidrift — Phase 2 Directive: Map & Navigation
**Status:** Approved — Ready for Execution  
**Gate Phase:** 2  
**Date:** April 2026  
**Depends On:** Phase 1 Gate PASSED ✅

---

## 0. Pre-Flight: Fix capture_gate_evidence.ps1

**Complete this before any Phase 2 implementation.**

The Phase 1 gate screenshot was written as UTF-16 LE by PowerShell, making it unreadable outside of the shell environment. Fix the screenshot capture to write raw binary.

**The problem:** PowerShell's `Out-File` and `Set-Content` default to UTF-16 LE encoding. ADB pulls binary data — writing it through these cmdlets corrupts the file.

**The fix:** Replace any ADB screenshot pull that pipes through `Out-File` or `Set-Content` with `[System.IO.File]::WriteAllBytes()`.

**Pattern to replace:**
```powershell
# WRONG — corrupts binary output
adb exec-out screencap -p | Out-File -FilePath $outputPath -Encoding Byte
# or
adb exec-out screencap -p | Set-Content -Path $outputPath -Encoding Byte
```

**Correct pattern:**
```powershell
# CORRECT — raw binary write
$bytes = adb exec-out screencap -p
[System.IO.File]::WriteAllBytes($outputPath, $bytes)
```

**Completion criteria for this fix:**
- Run the fixed script after Phase 2 gate build
- Open the resulting PNG in any image viewer outside PowerShell
- If it renders correctly, the fix is verified

---

## 1. Objective

Implement selection-based navigation. The player taps a destination marker on a map view and the ship moves to that destination in world space. The camera follows the ship during transit.

This phase proves the core interaction model: **tap to command, watch to confirm.**

No mining. No combat. No UI panels beyond the map overlay. Movement and camera only.

---

## 2. Scope Boundaries

> ⚠️ HARD LIMIT: Phase 2 is strictly Map View + Ship Movement + Camera Follow.

**In scope:**
- Map view rendering (zoomed-out overlay showing markers)
- Tap input on map markers sets ship autopilot target
- Ship moves toward target in world space each tick
- Camera follows ship during transit
- Ship arrives and becomes Idle

**Explicitly out of scope — do not implement:**
- Asteroid targeting or mining
- Station docking UI
- Any resource or inventory system
- Touch input in space view (map tap only)
- Multiple ships
- Sector transitions or gate network
- Any UI panels, HUD elements, or overlays beyond map markers

---

## 3. Technical Specification

### 3.1 Map View

The map view is a toggled overlay — not a separate Bevy scene. When active:

- Camera zooms out to show all three world markers simultaneously
- Each marker is a small colored circle with a label
- Tapping a marker sets it as the autopilot target and closes the map view
- Map toggle: single tap on a fixed corner button (simple colored rectangle, no art required)

| Marker | World Position | Color | Label |
|--------|---------------|-------|-------|
| Station | (-150, -200) | Yellow | "Station" |
| Asteroid Field | (150, 100) | Grey | "Asteroid Field" |

The ship marker shows current ship position on the map — moves as the ship moves.

### 3.2 Autopilot System

Extend `src/lib.rs` with the following components and system:

**New components:**
```rust
#[derive(Component)]
struct Ship {
    state: ShipState,
    speed: f32,
}

#[derive(PartialEq)]
enum ShipState {
    Idle,
    Navigating,
}

#[derive(Component)]
struct AutopilotTarget {
    destination: Vec2,
}
```

**AutopilotSystem behaviour:**
- Runs every tick when `ShipState == Navigating`
- Moves ship Transform toward `AutopilotTarget.destination` at `Ship.speed` units/second
- On arrival (distance < arrival_threshold of 8.0 units): set state to `Idle`, remove `AutopilotTarget` component
- Speed constant: `SHIP_SPEED = 120.0` (world units per second) — lock as a named constant, do not hardcode inline

### 3.3 Camera Follow System

- Camera tracks ship Transform position every tick
- Smoothing optional — instant follow is acceptable for Phase 2
- In map view: camera snaps to a fixed overview position showing all markers
- On map close: camera returns to following ship

### 3.4 Input Handling

Touch input routes differently depending on current view state:

| View State | Tap Target | Result |
|------------|-----------|--------|
| Map View | Station marker | Set autopilot target to (-150, -200), close map |
| Map View | Asteroid marker | Set autopilot target to (150, 100), close map |
| Map View | Anywhere else | No action |
| Space View | Map toggle button | Open map view |
| Space View | Anywhere else | No action (Phase 2 only) |

### 3.5 File Scope

Only these files may be modified in Phase 2:

| File | Change |
|------|--------|
| `src/lib.rs` | Add Ship component, AutopilotSystem, CameraFollowSystem, MapView state, input routing |
| `capture_gate_evidence.ps1` | Binary write fix (pre-flight) |
| `Cargo.toml` | Only if a new dependency is required — justify before adding |

**All other files are read-only for this phase.**

---

## 4. Test Anchors

Minimum 5 named behaviours must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P2-01 | Map view opens on tap of toggle button | Tap button, map overlay appears |
| TB-P2-02 | Tapping Station marker sets ship destination | Ship begins moving toward (-150, -200) |
| TB-P2-03 | Tapping Asteroid marker sets ship destination | Ship begins moving toward (150, 100) |
| TB-P2-04 | Ship arrives and stops at destination | Ship reaches target, stops, state = Idle |
| TB-P2-05 | Camera follows ship during transit | Camera tracks ship position while moving |
| TB-P2-06 | Map view closes after marker tap | Space view returns after destination selected |

---

## 5. Gate 2 Completion Criteria

All of the following must be true before Phase 2 is marked complete:

- [ ] `capture_gate_evidence.ps1` fix verified — screenshot opens correctly in an image viewer
- [ ] App launches on Moto G 2025 without crash
- [ ] Map view opens and displays both destination markers
- [ ] Tapping Station marker causes ship to navigate to station position
- [ ] Tapping Asteroid marker causes ship to navigate to asteroid position
- [ ] Ship stops on arrival
- [ ] Camera follows ship throughout transit
- [ ] All 6 test anchors TB-P2-01 through TB-P2-06 verified
- [ ] Gate screenshot (TB-P2-GATE) shows ship mid-transit in space view — raw binary PNG, opens in image viewer

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — ship visibly mid-transit on device
3. Logcat excerpt showing autopilot state transitions (Idle → Navigating → Idle)

---

## 6. Known Risks

| Risk | Mitigation |
|------|-----------|
| Bevy touch coordinate space may not match world space directly | Use `Camera::viewport_to_world_2d` for tap-to-world transform |
| Map overlay z-ordering may conflict with world sprites | Assign explicit z values: world entities z=0, map overlay z=10 |
| Camera snap between map/space views may feel jarring | Instant snap is acceptable for Phase 2 — smooth transition is post-slice polish |
| SHIP_SPEED may feel too fast or slow on device | Tune after first device run — constant is named for easy adjustment |

---

*Voidrift Phase 2 Directive | April 2026 | RFD IT Services Ltd.*  
*Each phase produces a directive. No phase begins without the prior gate passing.*
