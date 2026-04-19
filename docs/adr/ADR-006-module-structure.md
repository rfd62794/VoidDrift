# ADR-006: Module Structure — lib.rs App Setup Only
**Date:** April 2026  
**Status:** Accepted

## Context
lib.rs grew to 1000+ lines across Phases 0-9. The agent began hitting 
structural friction — difficulty locating systems, import conflicts, and 
unclear ownership of functions. Single-file architecture was no longer 
sustainable for mobile development on Bevy 0.15.

## Decision
Split lib.rs into a module hierarchy:
- `constants.rs`: all constants (single source of truth)
- `components.rs`: all Component and Resource structs  
- `systems/`: modular files organized by concern (9 logic files)
- `lib.rs`: application entry point, plugin registration, and system scheduling only.

## Rationale
- Each system file has single responsibility.
- Constants and components are importable from any system file without circular dependencies.
- lib.rs as pure app setup is readable and allows for clear system ordering (.chain()).
- Progressive one-file migration with compile verification prevented regressions.
- Module boundaries make future agent sessions faster to orient.

## Consequences
- All new systems go in `systems/` — never in `lib.rs`.
- All new constants go in `constants.rs` — never hardcoded inline.
- All new components go in `components.rs`.
- Import pattern: `use crate::constants::*`; `use crate::components::*`.
- lib.rs changes are rare — only for new plugin registration or system scheduling.
