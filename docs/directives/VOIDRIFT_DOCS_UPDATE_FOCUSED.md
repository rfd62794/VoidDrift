# VoidDrift Documentation Update — Focused
**Priority:** Critical public-facing + contributor-facing docs
**Status:** Ready for implementation
**Date:** May 2, 2026

---

## Objective

Update the most critical documentation to bring it in line with the v2.8.15 codebase state. This directive focuses on:
1. README.md (public-facing, first thing anyone sees)
2. CHANGELOG.md (development history gaps)
3. WASM build guide (missing contributor documentation)
4. Archive legacy directives (reduce noise)

---

## Part 1: README.md Update

### File: `README.md`

#### 1.1 Update Status Line (Line 7)

**Current:**
```markdown
> **Status:** Phase 2 Complete — `v2.2.0-docs-phase2-complete` — Verified on Moto G 2025
```

**Replace with:**
```markdown
> **Status:** Phase 4a Complete — `v2.8.7-tutorial-4a` — Live on [itch.io](https://rfd627.itch.io/voidrift)
```

#### 1.2 Update Roadmap (Lines 105-112)

**Current:**
```markdown
| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ Complete | Core mining → refining → drone building loop |
| Phase 1c | ✅ Complete | Asteroid lifecycle, lifespan timers, stuck-ship safety |
| Phase 2 | ✅ Complete | UI refactor, Requests framework, bottle mechanic, random spawn |
| Phase 3 | 🚧 Next | Architectural refactor — SRP / event bus, decouple narrative from simulation |
| Phase 4 | 🔮 Planned | Narrative drops — memory fragments, faction voice differentiation |
```

**Replace with:**
```markdown
| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ Complete | Core mining → refining → drone building loop |
| Phase 1c | ✅ Complete | Asteroid lifecycle, lifespan timers, stuck-ship safety |
| Phase 2 | ✅ Complete | UI refactor, Requests framework, bottle mechanic, random spawn |
| Phase 3 | ✅ Complete | Architectural refactor — SRP / event bus, decouple narrative from simulation |
| Phase 4a | ✅ Complete | Tutorial system (T-101 to T-106) with TutorialHighlight ring |
| Phase 4b | 🔮 Planned | Narrative drops — memory fragments, faction voice differentiation |
```

#### 1.3 Add WASM Build Instructions

Insert after Android build section (after line 131):

```markdown
---

## Building for WASM

### Prerequisites
- Rust 1.85+
- wasm-pack v0.14.0 (`cargo install wasm-pack`)
- Node.js (for wasm-pack dependencies)

### Build

```powershell
.\build_wasm.ps1
```

Builds the WASM binary to `pkg/` directory and copies assets to `pkg/assets/` for font loading.

**Output:**
- `pkg/voidrift_bg.wasm` (~22.7MB)
- `pkg/voidrift.js` (~149KB)
- `pkg/index.html` (hand-maintained entry point)
- `pkg/assets/` (fonts copied from root assets/)

### Local Testing

Serve the `pkg/` directory with a local web server:
```powershell
python -m http.server 8000 --directory pkg
```

Then open `http://localhost:8000` in a browser.

---

## Publishing to itch.io

### Prerequisites
- Butler v15.26.1 in system PATH
- `.publish.env` file configured (gitignored):
  ```
  ITCHIO_TARGET=rdug627/voidrift:html5
  ```

### Deploy

```powershell
.\publish.ps1
```

Deploys the `pkg/` directory to itch.io. Use `-Build` flag to build before deploying:
```powershell
.\publish.ps1 -Build
```

Use `-DryRun` to verify without uploading:
```powershell
.\publish.ps1 -DryRun
```

---
```

---

## Part 2: CHANGELOG.md Update

### File: `docs/CHANGELOG.md`

Add entries for v2.8.4 through v2.8.15 after the last existing entry:

```markdown
## v2.8.7-tutorial-4a (April 2026)
- Implemented Phase 4a tutorial system (T-101 to T-106 Echo voice)
- Added TutorialHighlight component (cyan ring, distinct from DestinationHighlight)
- Added tutorial position driver system (highlights asteroid, then bottle)
- Implemented new game guard (tutorial resets on new game, skips on load)
- Preserved legacy T-001 to T-006 system (non-functional, requires InOpeningSequence ship)
- Files modified: markers.rs, entity_setup.rs, world_spawn.rs, tutorial.rs, main_menu.rs

## v2.8.6-balance-speed-mining (April 2026)
- Increased MINING_RATE from 18.0 to 22.0 ore/sec
- Increased SHIP_SPEED from 180.0 to 210.0 units/sec
- Asteroid depletion time: ~4.5s at current mining rate

## v2.8.5-balance-forge-mining (April 2026)
- Halved FORGE_HULL_TIME from 10.0s to 5.0s
- Increased MINING_RATE from 12.0 to 18.0 ore/sec

## v2.8.4-balance-drone-spawn-weights (April 2026)
- Reduced DRONE_BUILD_TIME from 30.0s to 18.0s per drone
- Changed Aluminum spawn weight from 25% to 10%
- Iron/Tungsten/Nickel each at 30% spawn weight

## v2.8.3-eventbus-complete (April 2026)
- Completed Phase 3b event bus refactor
- Implemented 8 Bevy events for decoupled systems
- Refactored autopilot.rs to fire events instead of direct mutations
- Created economy.rs and narrative_events.rs systems
- Wired OpeningCompleteEvent, ShipDockedWithCargo, ShipDockedWithBottle, FulfillRequestEvent, RepairStationEvent, DroneDispatched, InsufficientLaserEvent, SignalFired

## v2.8.2-ui-refactor-v2 (April 2026)
- Completed Phase 2 UI Refactor v2
- Collapsed ore pipeline tabs into single PRODUCTION tab with ComboBox
- Replaced UPGRADES placeholder with REQUESTS tab
- Implemented Aluminum ore type (10% spawn weight)
- Implemented Bottle collection mechanic (spawn, drift, tap, dual output)
- Added Faction ComboBox to REQUESTS tab
- Implemented Request cards with fulfillment logic

## v2.8.1-cleanup-complete (April 2026)
- Completed Phase 3a pre-refactor cleanup
- Replaced despawn() with despawn_recursive() in cleanup_world_entities
- Removed duplicate station_visual_system registration in lib.rs
- Added warn!() log lines to silent fallback paths in autopilot.rs and mining.rs

## v2.8.0-power-multiplier (April 2026)
- Completed Phase 2 closeout
- Wired Station.power_multiplier to base mining rate
- Effective mining rate: BASE_MINING_RATE * station.power_multiplier
- power_multiplier increases by 0.25 after First Light request fulfillment
```

---

## Part 3: Create WASM Build Guide

### New File: `docs/WASM_BUILD.md`

```markdown
# WASM Build Guide

VoidDrift builds to WebAssembly for browser deployment via itch.io.

---

## Prerequisites

- Rust 1.85+
- wasm-pack v0.14.0 (`cargo install wasm-pack`)
- Node.js (for wasm-pack dependencies)

---

## Build Process

### 1. Build WASM Binary

```powershell
wasm-pack build --target web --out-dir pkg
```

This compiles the Rust code to WASM and generates JavaScript bindings.

**Output Location:** `C:\Github\VoidDrift\pkg\`

**Artifacts:**
- `voidrift_bg.wasm` (~22.7MB) - The compiled WASM binary
- `voidrift.js` (~149KB) - JavaScript bindings
- `package.json` - NPM package metadata

### 2. Copy Assets

The build script automatically copies assets:

```powershell
Copy-Item -Path "assets\fonts\FiraSans-Bold.ttf" -Destination "pkg\assets\fonts\FiraSans-Bold.ttf" -Force
```

This is required because WASM cannot access files outside its build output directory.

### 3. One-Click Build

Use the provided PowerShell script:

```powershell
.\build_wasm.ps1
```

This runs steps 1 and 2 in sequence.

---

## Entry Point

The WASM entry point is in `src/lib.rs`:

```rust
#[cfg(target_arch = "wasm32")]
pub fn start() {
    // WASM initialization
}
```

This is gated to only compile for the wasm32 architecture.

---

## HTML Entry Point

`pkg/index.html` is hand-maintained. wasm-pack does NOT own this file.

Key points:
- Loads `voidrift.js` and `voidrift_bg.wasm`
- Calls `start()` to initialize the game
- Sets up the canvas for rendering

Do not delete or overwrite this file.

---

## Asset Pipeline

### Font Loading

Fonts live at `assets/fonts/FiraSans-Bold.ttf` in the repository root.

The build script copies them to `pkg/assets/fonts/FiraSans-Bold.ttf` so the WASM build can load them.

### Git Tracking

`pkg/.gitignore` has exceptions:
```
!assets/
!assets/**
```

This ensures assets are tracked in git and can be pushed by Butler to itch.io.

---

## wasm-opt

wasm-opt is DISABLED in `Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

Do not remove this setting. It was disabled due to build issues.

---

## Local Testing

### Serve the Build

Use Python's built-in HTTP server:

```powershell
python -m http.server 8000 --directory pkg
```

Then open `http://localhost:8000` in a browser.

### Alternative: Use any local server

- VS Code Live Server extension
- Node.js http-server
- Any static file server

---

## Browser Compatibility

Tested on modern browsers:
- Chrome/Edge (Chromium)
- Firefox
- Safari

Requires WebAssembly support.

---

## Troubleshooting

### Build Fails

Ensure wasm-pack is installed:
```powershell
cargo install wasm-pack
```

Check Rust version:
```powershell
rustc --version
```

### Fonts Not Loading

Ensure assets were copied:
```powershell
Test-Path pkg\assets\fonts\FiraSans-Bold.ttf
```

If false, run the build script again.

### wasm-opt Errors

If you see wasm-opt errors, ensure it's disabled in `Cargo.toml` (it should be by default).

---

## See Also

- `build_wasm.ps1` - Build script
- `publish.ps1` - Deployment script
- `docs/directives/VoidDrift_WASM_Polish_Sprint_Directive.md` - WASM polish directive
```

---

## Part 4: Archive Legacy Directives

### Action: Move `docs/directives/legacy/` to Archive

Create archive directory and move:

```powershell
New-Item -ItemType Directory -Path "docs/archive/legacy_directives" -Force
Move-Item -Path "docs/directives/legacy/*" -Destination "docs/archive/legacy_directives/" -Force
Remove-Item -Path "docs/directives/legacy" -Force
```

**Files being archived:**
- Voidrift_Tutorial_UX_Directive.md (superseded by Phase 4a)
- Voidrift_SDD_v0_2_Addendum.md (outdated SDD version)
- Voidrift_Documentation_Refactor_v2.md (superseded)
- Voidrift_Documentation_Refactor_Directive.md (superseded)
- Voidrift_CodeRefactor_BevyUI_Directive.md (superseded)

### Action: Archive Superseded Directive

Move superseded Phase 2 UI Refactor v1:

```powershell
Move-Item -Path "docs/directives/VoidDrift_Phase2_UI_Refactor_Requests_Directive.md" -Destination "docs/archive/" -Force
```

### Action: Archive Analysis Reports

Move analysis reports (may be obsolete):

```powershell
New-Item -ItemType Directory -Path "docs/archive/analysis_reports" -Force
Move-Item -Path "docs/directives/analysis/*" -Destination "docs/archive/analysis_reports/" -Force
Remove-Item -Path "docs/directives/analysis" -Force
```

### Action: Mark phase-10-tutorial-ux.md as Legacy

Add header to `docs/phases/phase-10-tutorial-ux.md`:

```markdown
# Phase 10 Summary: Tutorial & UX (LEGACY)

**⚠️ LEGACY:** This document describes the T-001 to T-006 tutorial system, which has been superseded by the Phase 4a T-101 to T-106 Echo tutorial system. The legacy system is preserved in code but non-functional (requires InOpeningSequence ship which is despawned at OpeningPhase::Complete).

See `docs/directives/VoidDrift_Phase4a_Tutorial_Directive.md` for the current tutorial implementation.

---

[Rest of document unchanged]
```

---

## Verification Checklist

- [ ] README.md line 7 updated with v2.8.7-tutorial-4a and itch.io URL
- [ ] README.md roadmap updated to reflect Phase 3 and 4a complete
- [ ] README.md WASM build instructions added
- [ ] README.md publishing section added
- [ ] CHANGELOG.md entries added for v2.8.0 through v2.8.7
- [ ] docs/WASM_BUILD.md created
- [ ] docs/directives/legacy/ moved to docs/archive/legacy_directives/
- [ ] docs/directives/VoidDrift_Phase2_UI_Refactor_Requests_Directive.md (v1) moved to archive
- [ ] docs/directives/analysis/ moved to docs/archive/analysis_reports/
- [ ] docs/phases/phase-10-tutorial-ux.md marked as legacy

---

## Commit Message

```bash
git add README.md docs/CHANGELOG.md docs/WASM_BUILD.md docs/directives/ docs/phases/phase-10-tutorial-ux.md
git commit -m "docs: update critical documentation to v2.8.7 state

- Update README.md status from v2.2.0 to v2.8.7-tutorial-4a with itch.io URL
- Update README.md roadmap to reflect Phase 3 and 4a completion
- Add WASM build instructions to README.md
- Add publishing workflow (Butler + itch.io) to README.md
- Fill CHANGELOG.md gaps for v2.8.0 through v2.8.7
- Create docs/WASM_BUILD.md with comprehensive WASM build guide
- Archive docs/directives/legacy/ to docs/archive/legacy_directives/
- Archive superseded Phase 2 UI Refactor v1 directive
- Archive analysis reports to docs/archive/analysis_reports/
- Mark phase-10-tutorial-ux.md as legacy (T-001 to T-006 superseded by T-101 to T-106)

This brings public-facing and contributor documentation in line with current codebase state."
```

---

## Deferred Items

The following items are intentionally deferred for a dedicated docs sprint:

- Tutorial system (T-101 to T-106) dedicated documentation
- Faction system documentation
- Narrative system implementation details
- Balance constants rationale documentation
- ECONOMY.md, STARGATE.md, UI_VISION.md updates (design docs with mixed accuracy)

---

**Ready for implementation.**
