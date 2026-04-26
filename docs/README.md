# Voidrift Documentation

This directory contains all project documentation, organized for clarity and maintainability.

---

## Quick Navigation

### 🎯 Core Documentation (Start Here)
- **[GDD.md](GDD.md)** - Game Design Document - The complete game vision
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical architecture and system design
- **[CHANGELOG.md](CHANGELOG.md)** - Project history and changes
- **[NARRATIVE_JUSTIFICATION.md](NARRATIVE_JUSTIFICATION.md)** - Why the survival sci-fi frame exists

### 👥 Developer Resources
- **[DEVELOPER.md](DEVELOPER.md)** - Developer onboarding guide (Monday morning clarity)
- **[ROADMAP.md](ROADMAP.md)** - Phases, milestones, and current tasks

### 📁 Subdirectories

#### **[adr/](adr/)**
Architectural Decision Records - Why we made technical choices
- ADR-001 through ADR-010 covering present mode, UI framework, system architecture, narrative scope

#### **[design/](design/)**
Design deep-dives and vision documents
- **[economy.md](design/economy.md)** - Three-track resource economy design
- **[stargate.md](design/stargate.md)** - Stargate and galaxy expansion design
- **[ui-vision.md](design/ui-vision.md)** - UI architecture and migration plan
- **[vision.md](design/vision.md)** - High-level design vision

#### **[phases/](phases/)**
Historical phase documentation - What we've built and learned
- Phase 0 through Phase 10 summaries
- Reference for understanding development progression

#### **[directives/](directives/)**
Implementation directives and analysis (archival)
- **[analysis/](directives/analysis/)** - Codebase analysis and audit reports
- **[narrative/](directives/narrative/)** - Narrative system implementation
- **[systems/](directives/systems/)** - System-specific implementation directives
- **[legacy/](directives/legacy/)** - Historical implementation directives

---

## Project Overview

**Voidrift** is an arcade mining and production game where you've crashed into a black hole and merged with a dying station AI. Build a drone army, discover faction secrets, and determine your fate.

### Key Features
- **Mining Loop:** Extract ore → Refine resources → Build drones
- **Survival Narrative:** Black hole setting justifies mechanics (not horror genre)
- **Faction Discovery:** Trade with trapped ships at the event horizon boundary
- **Station Progression:** Unlock capabilities through station upgrades

### Technical Stack
- **Engine:** Bevy 0.15 (pinned for stability)
- **Language:** Rust
- **UI:** bevy_egui 0.33 (planned migration to Bevy UI)
- **Platform:** Android (Moto G 2025 primary target)
- **Build:** cargo-ndk r29

---

## Development Status

### Current State ✅
- Core gameplay loop (mining → refining → drone building)
- Narrative frame locked (survival sci-fi, not horror)
- Unified ship queue system
- Bottom drawer UI with station tabs
- Save/load persistence

### Next Phase 🚧
**Phase 1c: Asteroid Lifecycle**
- Finite ore per asteroid
- Depletion and respawn mechanics
- Natural gameplay cycles

See [ROADMAP.md](ROADMAP.md) for complete phase breakdown.

---

## How to Contribute

1. **Read DEVELOPER.md** - Quick start guide and current state
2. **Check ROADMAP.md** - Current sprint tasks and priorities
3. **Review ADRs** - Understand architectural decisions
4. **Follow patterns** - Constants in `constants.rs`, systems in `/src/systems/`

### Build & Deploy
```bash
./build_android.ps1  # One-click build + deploy
```

### Testing
```bash
cargo test           # Run tests
cargo check          # Check compilation
```

---

## Documentation Philosophy

This documentation follows a consistent pattern:

1. **Core docs in root** - Essential reference materials
2. **Specialized in subdirs** - Deep dives by domain
3. **Clear naming** - Lowercase with hyphens
4. **Navigate easily** - This README explains everything
5. **Archive old work** - Legacy directives organized by type

---

## Recent Changes

### April 26, 2026 - Documentation Reorganization
- Reorganized docs into clear hierarchy
- Added DEVELOPER.md and ROADMAP.md
- Moved design docs to `design/` subdirectory
- Organized directives by domain (analysis, narrative, systems, legacy)

### April 18-25, 2026 - Narrative Pivot
- Shifted from mining sim to survival sci-fi frame
- Event Horizon-inspired narrative (mechanical justification, not horror)
- Added faction system design
- Created NARRATIVE_JUSTIFICATION.md

See [CHANGELOG.md](CHANGELOG.md) for complete history.

---

## Questions?

- **Current development:** Check ROADMAP.md
- **Technical decisions:** Review ADRs in `adr/`
- **Design vision:** Read GDD.md and design/ docs
- **How to start:** Read DEVELOPER.md

---

*Last updated: April 26, 2026*
