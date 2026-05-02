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
