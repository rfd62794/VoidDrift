# Changelog

All notable changes to VoidDrift will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.8.0] - 2026-05-01
### Added
- WASM build target — playable in browser via itch.io
- localStorage persistence for browser save/load
- Mouse click support for asteroid and bottle dispatch
- Scroll wheel zoom (desktop)
- Click-drag pan (desktop)
- Device detection (mobile/desktop) with adaptive zoom sensitivity
- Portrait canvas constraint with 9:16 aspect ratio
- Fullscreen button for touch devices
- Dynamic UiLayout from actual window dimensions
- Cargo.toml metadata (description, repository, license)

### Fixed
- Canvas dimensions swapped (landscape vs portrait) on WASM
- Page scroll prevented when canvas focused
- std::time panic replaced with Bevy Time::elapsed for WASM compatibility

## [2.7.0] - Phase 3 Complete
### Added
- Phase 3 narrative systems complete
- Event bus implementation
- Save/load system improvements

## [2.6.1] - Menu Starfield
### Added
- Starfield background in main menu

## [2.6.0] - Phase 3D Narrative Bundling
### Added
- Narrative bundling system
- Event-driven narrative progression

## [2.5.0] - Phase 3C Event Bus Gaps
### Added
- Event bus gap filling
- Additional event types for narrative

## [2.4.0] - Phase 3B Event Bus
### Added
- Event bus architecture (ADR-011)
- ShipDockedWithCargo, ShipDockedWithBottle events
- FulfillRequestEvent, RepairStationEvent
- OpeningCompleteEvent
- One-frame deferral for economy events

## [2.3.0] - Phase 3A Cleanup Complete
### Added
- Phase 3A cleanup complete
- Code organization improvements

## [2.2.0] - Docs Phase 2 Complete
### Added
- Documentation for Phase 2 features
- Architecture documentation

## [2.1.0] - Starmap Parallax Fix
### Fixed
- Starmap parallax rendering issues

## [2.0.0] - Phase 2 Complete
### Added
- Phase 2 gameplay systems complete
- Station interactions
- Resource management

## [1.2.2] - Stuck Ship Safety Complete
### Fixed
- Stuck ship safety mechanisms
- Edge case handling for ship movement

## [1.1] - Phase 1C Final
### Added
- Phase 1C features complete
- Core gameplay loop

## [0.5.21] - Singularity Grind
### Added
- Singularity grind mechanics

## [0.5.20] - Queue Working
### Fixed
- Queue system functionality

## [0.5.18] - Vertical Pipeline
### Added
- Vertical pipeline systems

## [0.5.17] - Full Auto
### Added
- Full autopilot functionality

## [0.5.16] - Feel Tuned
### Fixed
- Gameplay feel tuning
- Control sensitivity adjustments

## [0.5.15] - Phase 0 Checkpoint
### Added
- Phase 0 checkpoint features

## [0.5.14] - Power Deleted
### Fixed
- Power system cleanup

## [0.5.13] - Save Load Functional
### Added
- Save/load system functional
- Persistence system

## [0.5.12] - HUD Refactor
### Fixed
- HUD system refactoring
- UI cleanup

## [0.5.11] - Wireless Viewport
### Added
- Wireless viewport system

## [0.5.10] - Viewport Drawer Fix
### Fixed
- Viewport drawer issues

## [0.5.9] - Starfield Drawer Fixes
### Fixed
- Starfield drawer rendering

## [0.5.8] - Bottom Drawer
### Added
- Bottom drawer UI component

## [0.5.7] - Directive B
### Added
- Directive B implementation

## [0.5.6] - Timing Fixed
### Fixed
- Timing system issues

## [0.5.5] - Main Menu GDD
### Added
- Main menu GDD features

## [0.5.4] - Save System
### Added
- Save system implementation

## [0.5.3] - Focus Tab Starfield
### Added
- Starfield focus tab

## [0.5.2] - Infinite Starfield
### Added
- Infinite starfield generation

## [0.5.1] - Balanced Production
### Fixed
- Production balancing

## [0.5.0] - Phase A Stable
### Added
- Phase A stable features

## [0.4.0] - Station Phase B
### Added
- Station phase B features

## [0.1.0] - Phase 1C
### Added
- Initial Phase 1C implementation

## Arcade_UI_Loop_Final_v3
### Added
- Arcade UI loop final iteration v3

## Arcade_UI_Loop_Final_v2
### Added
- Arcade UI loop final iteration v2

## Arcade_UI_Loop_Final
### Added
- Arcade UI loop final

[2.8.0]: https://github.com/rfd62794/VoidDrift/compare/v2.7.0...v2.8.0
[2.7.0]: https://github.com/rfd62794/VoidDrift/compare/v2.6.1...v2.7.0
[2.6.1]: https://github.com/rfd62794/VoidDrift/compare/v2.6.0...v2.6.1
[2.6.0]: https://github.com/rfd62794/VoidDrift/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/rfd62794/VoidDrift/compare/v2.4.0...v2.5.0
[2.4.0]: https://github.com/rfd62794/VoidDrift/compare/v2.3.0...v2.4.0
[2.3.0]: https://github.com/rfd62794/VoidDrift/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/rfd62794/VoidDrift/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/rfd62794/VoidDrift/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/rfd62794/VoidDrift/compare/v1.2.2...v2.0.0
[1.2.2]: https://github.com/rfd62794/VoidDrift/compare/v1.1...v1.2.2
[1.1]: https://github.com/rfd62794/VoidDrift/compare/v0.5.21...v1.1
[0.5.21]: https://github.com/rfd62794/VoidDrift/compare/v0.5.20...v0.5.21
[0.5.20]: https://github.com/rfd62794/VoidDrift/compare/v0.5.18...v0.5.20
[0.5.18]: https://github.com/rfd62794/VoidDrift/compare/v0.5.17...v0.5.18
[0.5.17]: https://github.com/rfd62794/VoidDrift/compare/v0.5.16...v0.5.17
[0.5.16]: https://github.com/rfd62794/VoidDrift/compare/v0.5.15...v0.5.16
[0.5.15]: https://github.com/rfd62794/VoidDrift/compare/v0.5.14...v0.5.15
[0.5.14]: https://github.com/rfd62794/VoidDrift/compare/v0.5.13...v0.5.14
[0.5.13]: https://github.com/rfd62794/VoidDrift/compare/v0.5.12...v0.5.13
[0.5.12]: https://github.com/rfd62794/VoidDrift/compare/v0.5.11...v0.5.12
[0.5.11]: https://github.com/rfd62794/VoidDrift/compare/v0.5.10...v0.5.11
[0.5.10]: https://github.com/rfd62794/VoidDrift/compare/v0.5.9...v0.5.10
[0.5.9]: https://github.com/rfd62794/VoidDrift/compare/v0.5.8...v0.5.9
[0.5.8]: https://github.com/rfd62794/VoidDrift/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/rfd62794/VoidDrift/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/rfd62794/VoidDrift/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/rfd62794/VoidDrift/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/rfd62794/VoidDrift/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/rfd62794/VoidDrift/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/rfd62794/VoidDrift/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/rfd62794/VoidDrift/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/rfd62794/VoidDrift/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/rfd62794/VoidDrift/compare/v0.1.0...v0.4.0
[0.1.0]: https://github.com/rfd62794/VoidDrift/compare/Arcade_UI_Loop_Final...v0.1.0
[Arcade_UI_Loop_Final_v3]: https://github.com/rfd62794/VoidDrift/compare/Arcade_UI_Loop_Final_v2...Arcade_UI_Loop_Final_v3
[Arcade_UI_Loop_Final_v2]: https://github.com/rfd62794/VoidDrift/compare/Arcade_UI_Loop_Final...Arcade_UI_Loop_Final_v2
[Arcade_UI_Loop_Final]: https://github.com/rfd62794/VoidDrift/releases/tag/Arcade_UI_Loop_Final

