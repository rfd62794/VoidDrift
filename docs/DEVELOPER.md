# Voidrift Developer Guide

*Monday morning clarity for the Voidrift development team*

---

## Quick Start (First 15 Minutes)

### Build & Deploy
```bash
# One-click build + deploy to connected Android device
./build_android.ps1
```

### Run Tests
```bash
cargo test           # Run all tests
cargo check          # Check compilation without running
```

### Check Current Status
- Read the "Current State" section below
- Check [ROADMAP.md](ROADMAP.md) for current sprint
- Review [CHANGELOG.md](CHANGELOG.md) for recent changes

---

## Current State (What's Working Right Now)

### ✅ Core Gameplay Loop
- **Mining:** Extract ore from asteroids with laser beams
- **Refining:** Auto-process ore into ingots (Iron, Tungsten, Nickel)
- **Production:** Build hull plates, thrusters, AI cores
- **Drone Fleet:** Dispatch autonomous ships on mining routes

### ✅ User Interface
- **Bottom Drawer:** Collapsible UI with station tabs
- **Station Tabs:** Iron, Tungsten, Nickel, Fleet, Upgrades
- **Signal Strip:** ECHO AI communication at bottom
- **Map View:** Strategic overview with pinch zoom and pan

### ✅ Narrative Frame
- **Survival Sci-Fi:** Crashed into black hole, fused with station AI
- **ECHO AI:** Helpful partner, not threatening presence
- **Mechanical Justification:** Black hole setting explains why mechanics exist
- **Not Horror:** Mystery/discovery, not dread or terror

### ✅ Technical Foundation
- **Android Stable:** Universal Disjointness prevents crashes on Mali-G57
- **Save/Load:** Fleet persistence works correctly
- **Performance:** Efficient ECS queries, proper system chaining
- **Build Pipeline:** Robust Android build system

---

## Last Changes (What Changed Since You Left)

### April 26, 2026 - Documentation Reorganization
- **Reorganized docs:** Clear hierarchy with README.md navigation
- **Added DEVELOPER.md:** This onboarding guide
- **Added ROADMAP.md:** Phase/milestone/task breakdown
- **Clean structure:** Design docs in `design/`, directives organized by domain

### April 18-25, 2026 - Major Narrative Pivot
- **Frame Change:** From "space mining sim" to "Event Horizon survival sci-fi"
- **Narrative Justification:** Black hole setting explains mechanics, not genre shift
- **ECHO AI:** Reframed as helpful partner in survival
- **Faction System:** Added design for trapped ships at event horizon boundary
- **Documentation:** Created NARRATIVE_JUSTIFICATION.md explaining mechanical rationale

### Phase 1b Complete - Unified Ship Queue
- **Ship System:** All ships identical, queue-based dispatch
- **Fleet Management:** Ships available count, auto-assemble drones
- **UI Streamlined:** Fleet status at a glance, removed redundant tabs
- **Save/Load Fixed:** Fleet persists correctly across sessions

---

## Next Phase (Where We're Going)

### Phase 1c: Asteroid Lifecycle (QUEUED)
**Goal:** Finite ore per asteroid with respawn cycle
- Asteroid inventory system (ore count)
- Depletion mechanic (asteroid disappears when mined out)
- Respawn timer (new asteroids spawn periodically)
- Natural gameplay cycle (player manages resource flow)

**Estimated:** 1 week  
**Status:** Ready to start

### Phase 2: Station Expansion (PLANNED)
**Goal:** Progression through station modules
- Module system (upgrades unlock new capabilities)
- Upgrade costs (escalating, meaningful progression)
- Unlock gates (module A enables asteroid type B)
- Visual feedback (station grows as upgraded)

### Phase 3: Faction System (PLANNED)
**Goal:** First NPC interaction and trade mechanics
- Faction appearance (unmanned drone at boundary)
- Trade board (specific resource requests)
- Reputation system (trades affect standing)
- Story hints (dialogue reveals mystery)

See [ROADMAP.md](ROADMAP.md) for complete phase breakdown.

---

## How to Contribute

### Development Workflow
1. **Read ROADMAP.md** - Understand current sprint tasks
2. **Check ADRs** - Review architectural decisions in `docs/adr/`
3. **Follow Patterns** - Use established code organization
4. **Test Changes** - Verify on device before committing
5. **Document Decisions** - Create ADR for significant choices

### Code Organization
```
src/
├── lib.rs              # App setup, system registration
├── constants.rs        # All magic numbers (single source of truth)
├── components.rs       # ECS components and resources
└── systems/            # Game logic by domain
    ├── hud/           # UI systems (bottom drawer, tabs)
    ├── mining.rs      # Ore extraction
    ├── autopilot.rs   # Ship navigation
    ├── auto_process.rs # Production automation
    └── ...            # Other systems
```

### Key Files to Know
- **`src/lib.rs`** - App entry point, system registration
- **`src/constants.rs`** - All game balance values
- **`src/components.rs`** - ECS components and global resources
- **`src/systems/hud/`** - UI implementation (egui panels)
- **`docs/NARRATIVE_JUSTIFICATION.md`** - Why each mechanic exists

### Constants Pattern
All magic numbers live in `constants.rs`. Never hardcode values in systems.

```rust
// GOOD
use crate::constants::SHIP_SPEED;
ship.speed = SHIP_SPEED;

// BAD
ship.speed = 180.0;  // Don't do this
```

### System Pattern
Systems are organized by domain in `src/systems/`. Follow existing patterns for query structure and Universal Disjointness.

```rust
// Always use Without<T> filters for Transform queries
pub fn my_system(
    mut ship_query: Query<&mut Transform, (With<Ship>, Without<Station>, Without<AsteroidField>)>,
) {
    // System logic
}
```

---

## Build & Deploy Details

### Full Development Pipeline
For the complete end-to-end development workflow from local edit to live on itch.io, see **[DEVELOPMENT_PIPELINE.md](DEVELOPMENT_PIPELINE.md)**. This document covers:
- Local development loop (desktop and WASM testing)
- Commit and tag loop (branching, versioning, publishing)
- CI/CD automation (GitHub Actions for itch.io and telemetry deployment)
- Verification checklists at each stage

### Prerequisites
- Rust 1.95+
- Android SDK/NDK r26+
- `cargo-ndk` (`cargo install cargo-ndk`)
- Connected Android device (Moto G 2025 for testing)

### Build Pipeline
The `build_android.ps1` script handles:
- Compilation for Android target
- APK packaging
- Installation to connected device
- Logcat monitoring

### Manual Build (if needed)
```bash
cargo ndk --target aarch64-linux-android --platform 35 build --release
cd android
./gradlew assembleDebug
./gradlew installDebug
```

---

## Testing Strategy

### Unit Tests
```bash
cargo test                    # Run all tests
cargo test --lib             # Library tests only
cargo test --bin voidrift    # Binary tests only
```

### Device Testing
- Always test on physical Android hardware
- Moto G 2025 (720×1604) is primary target
- Check performance on Mali-G57 GPU
- Verify touch interactions work correctly

### Common Issues
- **B0001 Panics:** Check Transform query filters (Universal Disjointness)
- **UI Drift:** Verify egui scale constants
- **Performance:** Monitor frame rate during intensive operations

---

## Emergency Contacts & Resources

### When You're Stuck
1. **Check ADRs** - `docs/adr/` explains why we made technical choices
2. **Review Architecture** - `docs/ARCHITECTURE.md` for system design
3. **Read Narrative Justification** - `docs/NARRATIVE_JUSTIFICATION.md` for mechanical rationale
4. **Check ROADMAP** - `docs/ROADMAP.md` for current priorities

### Key Documentation
- **[GDD.md](GDD.md)** - Complete game design vision
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical architecture
- **[NARRATIVE_JUSTIFICATION.md](NARRATIVE_JUSTIFICATION.md)** - Why mechanics exist
- **[CHANGELOG.md](CHANGELOG.md)** - Project history

### Architecture Decisions (ADRs)
Key ADRs to understand:
- **ADR-008:** Universal Disjointness (critical for Android stability)
- **ADR-003:** bevy_egui HUD framework
- **ADR-010:** Survival narrative as mechanical justification

---

## Development Environment Setup

### IDE Configuration
- Use VS Code with rust-analyzer
- Install Bevy extension for ECS support
- Configure for Android development

### Useful Commands
```bash
cargo check                    # Quick compilation check
cargo clippy                   # Lint checking
cargo fmt                      # Code formatting
cargo doc --open              # Open documentation
```

### Debugging
- Use `info!()` macros for debugging output
- Check Android logcat with: `adb logcat | grep voidrift`
- Use Bevy's built-in inspector for entity debugging

---

## Code Quality Standards

### Rust Patterns
- Use `#[derive(Default)]` for components with sensible defaults
- Prefer `Option<T>` over nullable values
- Use `Result<T, E>` for error handling
- Follow rustfmt formatting standards

### ECS Patterns
- Components for data, systems for logic
- Resources for global state
- Queries for component access
- Events for one-way communication

### Performance Considerations
- Use `Without<T>` filters for Transform queries
- Batch similar operations
- Avoid excessive allocations in hot paths
- Profile on target hardware

---

## Frequently Asked Questions

### Q: Why does the UI use egui instead of Bevy UI?
**A:** ADR-003 locked egui for stability on Mali-G57 GPU. Bevy UI migration is planned but not prioritized.

### Q: What's with all the `Without<T>` filters?
**A:** ADR-008 (Universal Disjointness) prevents runtime crashes on Android by ensuring Transform queries don't conflict.

### Q: Why is the narrative about a black hole?
**A:** ADR-010 established survival sci-fi as mechanical justification. The black hole setting explains why mining is necessary and why the player is isolated.

### Q: Can I add new features?
**A:** Check ROADMAP.md for current priorities. Major features should be discussed and documented as ADRs.

---

*This guide is updated with each phase completion. Last updated: April 26, 2026*
