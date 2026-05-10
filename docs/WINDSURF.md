# Windsurf Context

**Date:** May 2026
**Purpose:** Windsurf-specific context for every coding session

---

## Project Root

**Location:** `C:\Github\VoidDrift`

All file paths are relative to this directory. When using file tools, use absolute paths starting with `C:\Github\VoidDrift\`.

---

## Primary Scripts

| Script | Purpose | Usage |
| :--- | :--- | :--- |
| `run.ps1` | Desktop build with instant feedback | Day-to-day development, config tuning, UI work |
| `verify.ps1` | Compilation verification | Run before every commit |
| `build_wasm.ps1` | WASM build for web deployment | Before publishing to itch.io |
| `publish.ps1` | Build and push to itch.io | Manual publish to itch.io |
| `build_android.ps1` | Android build and deploy | Android testing on connected device |

**Usage pattern:**
```powershell
# Start desktop dev
.\run.ps1

# Verify before commit
.\verify.ps1

# Build WASM for testing
.\build_wasm.ps1

# Publish to itch.io
.\publish.ps1 -Build
```

---

## GitHub Tools

**Script:** `gh_tools.ps1`

**Purpose:** PowerShell wrapper for GitHub CLI (gh) for issue management

**Available commands:**
```powershell
# List all open issues
List-Issues

# Add a new issue
Add-Issue -Title "Issue title" -Body "Issue description"

# Close an issue with comment
Close-Issue -Number 42 -Comment "Superseded by #45"
```

**Usage:** Use these commands instead of raw `gh` commands for consistency.

---

## Test Device

**Device:** Moto G 2025
**Screen:** 720×1604 physical pixels
**Android:** API 35
**GPU:** Mali-G57

**Importance:** Primary target for Android builds. All Android verification should be performed on this device.

**Known issues:**
- Mali-G57 requires Universal Disjointness (INV-004) to prevent B0001 panics
- PresentMode::Fifo required (ADR-001) to prevent screen flicker
- Specific to this device class — desktop behavior differs

---

## Three-Layer Model

**Director (Claude)**
- Strategic direction
- High-level decisions
- User communication
- Task prioritization

**Pipeline (Directives)**
- `.windsurf/workflows/*.md` files define workflows
- Each workflow is a specific task with steps
- Director instructs to execute a workflow
- Pipeline provides structured steps

**Agent (Windsurf)**
- Executes workflows
- Implements code changes
- Reports findings
- Follows AGENT_CONTRACT.md rules

**Workflow execution:**
1. Director: "Execute workflow /sprint-kickoff"
2. Agent: Reads `.windsurf/workflows/sprint-kickoff.md`
3. Agent: Executes steps in order
4. Agent: Reports completion

---

## Directive Reading Rule

**Always read the relevant directive fully before touching any file.**

When Director instructs to execute a workflow or work on a specific issue:
1. Read the directive/workflow file completely
2. Understand the full scope and requirements
3. Report findings before implementing
4. Wait for confirmation before making changes

**Rationale:** Prevents implementing the wrong thing or missing requirements due to partial understanding.

---

## Report Findings Before Implementing

**Investigate first, report findings, then implement.**

When given a task:
1. Investigate the current state (read relevant files)
2. Report what you found
3. Wait for confirmation
4. Then implement changes

**Rationale:** Ensures alignment on the problem before making changes.

---

## Branch Strategy

**Current branch:** `dev`

**Branch structure:**
- `main` — Production branch, always deployable
- `dev` — Active development branch
- Feature branches — Created from `dev` for specific work

**Workflow:**
1. Feature work on `dev` or feature branch
2. Test and verify
3. Merge to `main` when ready
4. Tag version on `main`
5. Publish to itch.io

**Never force-push `dev` without explicit confirmation.**

---

## Verification Pattern

**Verify before every commit:**

```powershell
.\verify.ps1
```

This runs:
- `cargo check` — compilation check
- `cargo test` — run tests
- Fails if compilation errors exist

**Do not commit if verify fails.**

---

## WASM Testing Pattern

**Before publishing to itch.io, test locally:**

1. Build WASM: `.\build_wasm.ps1`
2. Serve locally: `cd pkg && python -m http.server 8080`
3. Open itch preview: `scripts/local_itch_preview.html`
4. Select preset size (Landscape 1280×640 for current itch.io setting)

**This replicates exact itch.io iframe constraints.**

See `docs/WASM_BUILD.md` for details.

---

## Local Itch Preview Tool

**Location:** `scripts/local_itch_preview.html`

**Purpose:** Emulate itch.io browser page locally without publishing

**Usage:**
1. Build WASM: `.\build_wasm.ps1`
2. Serve pkg/: `cd pkg && python -m http.server 8080`
3. Open preview in browser
4. Select preset size

**Preset sizes:**
- Landscape 1280×640 — current itch.io embed
- Portrait 720×640 — portrait embed
- Full Portrait 720×1280 — tall portrait
- Fullscreen Sim — approximates browser fullscreen

**Known values at Landscape 1280×640:**
- `ui.available_height()` in cargo tab = 162.1875px
- Canvas constrained by `max-width: 720px, max-height: 100vh`

---

## Universal Disjointness (INV-004)

**Critical for Android stability on Mali-G57.**

Every system that queries `&mut Transform` MUST include `Without<T>` filters for all major entity types.

**Canonical filter sets:**
```rust
// Ship query
Query<&mut Transform, (With<Ship>, Without<Station>, Without<AsteroidField>, ...)>

// Station query
Query<&mut Transform, (With<Station>, Without<Ship>, Without<AsteroidField>, ...)>
```

**Violation causes:** B0001 runtime panics on Android (not caught by cargo check).

See ADR-008 for details.

---

## egui Pattern

**Use painter + ui.interact() for buttons.**

Do not use `egui::Window` for click detection.

**Pattern:**
```rust
let btn_rect = ui.painter().add(rect);
if ui.interact(btn_rect, id, Sense::click()).clicked() {
    // handle click
}
```

**Rationale:** bevy_egui 0.33.0 doesn't properly handle click events on egui::Window buttons.

See AGENT_CONTRACT.md for details.

---

## EGUI_SCALE Usage

**EGUI_SCALE = 3.0 applies to world coordinates only.**

- Multiply egui logical coordinates by EGUI_SCALE for world space
- Do not apply EGUI_SCALE to egui panel dimensions (ui.available_height(), etc.)
- EGUI_SCALE is a Bevy world coordinate multiplier

---

## Layer 1/2/3 Architecture

**Codebase organized into three layers:**

- **Layer 1 (Engine):** Infrastructure — app setup, config, components, persistence, spawning
- **Layer 2 (Game):** Mechanics — mining, refining, autonomous ships, narrative, quest progression
- **Layer 3 (Presentation):** Rendering and interface — HUD, menus, visual effects, camera

**Dependency rule:** Layer N can only depend on Layer < N.

See ADR-016 and docs/ARCHITECTURE.md for details.

---

## Current State

**Build:** v3.1.0-sprint5-visual-overhaul
**Branch:** dev
**Tag:** Latest on main is v3.1.0-sprint5-visual-overhaul

**Recent work:**
- Sprint 5: Visual Overhaul (ore nodes, ingot nodes, component nodes, drone bay)
- 7 new ADRs (ADR-012 through ADR-018)
- Documentation updates (ARCHITECTURE.md, current.md, DEVELOPMENT_PIPELINE.md)
- Local itch preview tool

See docs/state/current.md for complete current state.

---

## Related Documentation

- `docs/AGENT_CONTRACT.md` — Universal agent rules
- `docs/DEVELOPMENT_PIPELINE.md` — Development workflow
- `docs/ARCHITECTURE.md` — Codebase architecture
- `docs/DEVELOPER.md` — Developer onboarding
- `docs/state/current.md` — Current project state
- `docs/WASM_BUILD.md` — WASM build details
- `docs/WASM_ITCH_SIZING.md` — Itch.io sizing reference

---

## Quick Reference

| Command | Purpose |
| :--- | :--- |
| `.\run.ps1` | Start desktop build |
| `.\verify.ps1` | Verify compilation |
| `.\build_wasm.ps1` | Build WASM |
| `.\publish.ps1 -Build` | Publish to itch.io |
| `cd pkg && python -m http.server 8080` | Serve WASM locally |
| `scripts/local_itch_preview.html` | Itch.io preview |
| `.\gh_tools.ps1` | GitHub issue management |
