# VoidDrift Codebase Analysis Report
**Date:** 2026-05-17  
**Author:** Cascade (read-only audit)  
**Scope:** Full source tree — no modifications made.

---

## §1 System Registration Audit

All `pub fn` system functions found in `src/systems/` and their registration status in `lib.rs`.

| System | File | Registered | State Guard | Flag |
|---|---|---|---|---|
| `setup_debug_log_system` | `visuals/debug_log.rs` | Yes — **twice** | — | DOUBLE-REG: `Startup` + `OnEnter(InGame)` |
| `flush_debug_log_system` | `visuals/debug_log.rs` | No | — | **DEAD — never registered** |
| `cleanup_world_entities` | `systems/setup` | Yes — **twice** | — | `OnExit(InGame)` + `OnEnter(InGame)` chain |
| `scout_spawn_system` | `game_loop/scout_dispatch.rs` | Yes | **NONE** | **⚠ RUNS IN ALL STATES** |
| `scout_orbit_system` | `game_loop/scout_dispatch.rs` | Yes | **NONE** | **⚠ RUNS IN ALL STATES** |
| `scout_paint_cleanup_system` | `game_loop/scout_dispatch.rs` | Yes | **NONE** | **⚠ RUNS IN ALL STATES** |
| `reset_loop_stall_timer_on_upgrade` | `telemetry/mod.rs` | No | — | **DEAD** (`#[allow(dead_code)]`) |
| `ui_layout_system` | `visuals/viewport.rs` | Yes | `InGame` | No-op body — registered stub |
| All other 50+ systems | various | Yes | `InGame` (correct) | CLEAN |

Source: `src/lib.rs:200–202` — scout systems have no `.run_if(...)` call; all surrounding blocks do.

---

## §2 Duplicate & Dead File Audit

| File | Status | Detail |
|---|---|---|
| `systems/setup/mesh_builder.rs` | LIVE — name collision only | Ore-specific mesh gen (`triangle_mesh`, `generate_iron_mesh_with_radius` etc.) |
| `systems/visuals/mesh_builder.rs` | LIVE — different content | General polygon builder (`build_mesh_from_polygon`, `generate_rocket_points`) |
| `scenes/restore.rs` | **NOT IN MODULE TREE** | `scenes/mod.rs` declares only `main_menu` + `menu_starfield`. Likely dead. |
| `scenes/save_overlay.rs` | **NOT IN MODULE TREE** | Same. `save_overlay_system` lives in `main_menu.rs`, confirmed by `lib.rs:141`. |
| `flush_debug_log_system` | **Dead function** | Never scheduled. Reads global Mutex, prints to stdout. |
| `reset_loop_stall_timer_on_upgrade` | **Dead function** | `#[allow(dead_code)]`, comment says "future task." |

---

## §3 Opening Sequence Diagnosis

**File:** `src/systems/narrative/opening_sequence.rs`

### Phase transitions

| Phase | Transition Trigger | Stall Risk |
|---|---|---|
| `Adrift` | `timer >= 3.0` | None — timer only |
| `SignalIdentified` | `timer >= 4.0` | None — timer only |
| `AutoPiloting` | `dist_to_station < 300.0` | **HIGH** — ship must move |
| `InRange` | `dist_to_station < 5.0` | **HIGH** — tight threshold, no timeout |
| `Docked` | `beat_timer >= 10.5` | None — timer only |

### Confirmed stall conditions

1. **Lines 34–35:** Both `ship_query.get_single_mut()` and `station_query.get_single_mut()` use `let Ok(...) else { return; }`. Any entity configuration failure silently aborts every frame — no log, no timeout, no recovery.
2. **`InRange` threshold of 5.0:** Movement driven by `opening_drone_move_system` which only runs in `AutoPiloting`/`InRange`. If `ship.speed == 0.0`, distance never decreases. No maximum timer fallback exists on any distance-gated phase.
3. **`pan_state.is_focused` written every frame** in `Complete` branch — minor waste, not a stall.

`opening_drone_move_system` correctly uses `station_t.translation.truncate()` (Z ignored). No Z-plane mismatch risk.

---

## §4 Config-Driven Compliance Audit

| File | Line | Hardcoded Value | Should Be In |
|---|---|---|---|
| `game_loop/economy.rs` | 51 | `25.0` — request fulfillment iron cost | `requests.yaml` requirement amount |
| `game_loop/economy.rs` | 52 | `0.25` — power multiplier reward | `requests.yaml` reward value |
| `game_loop/mining.rs` | 28, 80, 126 | `80.0`, `100.0`, `80.0` — mining/retarget ranges | `balance.toml [mining]` |
| `game_loop/mining.rs` | 66 | `Color::srgba(0.18, 0.18, 0.18, 0.5)` — depleted color | `visual.toml [asteroid]` |
| `game_loop/scout_dispatch.rs` | 37 | `Color::srgb(0.0, 1.0, 1.0)` — Scout cyan | `visual.toml [drone.scout]` |
| `game_loop/scout_dispatch.rs` | 116–119 | `Annulus::new(38.0, 40.0)`, green `(0,1,0,0.6)` | `visual.toml [scout]` |
| `narrative/content_router.rs` | 6–8 | `90.0`, `120.0`, `10` — ambient timers, log max | `balance.toml [narrative]` |
| `narrative/signal.rs` | 130–133 | `5.0`, `15.0` — reserve alert thresholds | `balance.toml [narrative]` |
| `ui/hud/overlays.rs` | 32–88 | Popup dims `480×220`, all RGBA colors | `visual.toml [ui.tutorial_popup]` |
| `ui/hud/content.rs` | 97–103 | `X_ORE=40.0`, `X_INGOT=220.0`, `X_COMPONENT=400.0` etc. | `visual.toml [ui.cargo_tab]` |
| `ui/hud/content.rs` | 164–220 | Inline ore rgba values duplicating `visual.toml` | Use `vcfg.ore.*` exclusively |
| `telemetry/mod.rs` | 6 | `CLIENT_VERSION = "3.3.0"` | `env!("CARGO_PKG_VERSION")` |
| `narrative/bottle.rs` | 57 | `== 45.0` magic default timer check | Remove guard; use config unconditionally |

---

## §5 ADR-016 Layer Compliance Audit

**Layer map:** L1=Engine (`config/`, `components/`, `persistence/`, `setup/`), L2=Game (`game_loop/`, `ship_control/`, `asteroid/`, `narrative/`), L3=Presentation (`ui/`, `visuals/`, `scenes/`).

| File (Layer) | Import | Violation | Severity |
|---|---|---|---|
| `narrative/bottle.rs` (L2) | `use bevy_egui::EguiContexts` | **L2 → L3** upward dependency | HIGH |
| `narrative/bottle.rs` (L2) | `p.contexts.ctx_mut().wants_pointer_input()` | Presentation gating logic in game mechanic layer | HIGH |
| `systems/setup/entity_setup.rs` (L1) | `use bevy_egui::egui::Color32` | **L1 → L3** upward dependency | MEDIUM |

All other cross-layer references checked are permitted (L3→L1, L2→L1, intra-L3).

---

## §6 Fork Readiness Classification

| Classification | Files |
|---|---|
| **CHASSIS** (port unchanged) | `autopilot.rs`, `game_loop/autonomous.rs`, `viewport.rs`, `visuals/mesh_builder.rs`, `persistence/io.rs`, `persistence/save.rs`, `telemetry/mod.rs`, `visuals/debug_log.rs`, `scenes/menu_starfield.rs`, `ui_kit/primitives.rs`, `ui_kit/styles.rs`, `config/mod.rs` |
| **RESKIN** (structure kept, content replaced) | `lib.rs`, `constants.rs`, `components/markers.rs`, `components/ui_state.rs`, `components/utilities.rs`, `config/balance.rs`, `config/visual.rs`, `ship_control/ship_spawn.rs`, `narrative/content_router.rs`, `ui/hud/mod.rs`, `ui/hud/buttons.rs`, `ui/hud/overlays.rs`, `ui/hud/state_machine.rs`, `ui/tutorial.rs`, `visuals/visuals.rs`, `visuals/map.rs`, `persistence/systems.rs`, `scenes/main_menu.rs` |
| **VOIDRIFT-SPECIFIC** (rebuild for fork) | All `assets/`, `components/events.rs`, `components/game_state.rs`, `components/resources.rs`, `components/queries.rs`, `config/content.rs`, `setup/entity_setup.rs`, `setup/mesh_builder.rs`, `setup/world_spawn.rs`, `setup/quest_init.rs`, `game_loop/mining.rs`, `game_loop/auto_process.rs`, `game_loop/economy.rs`, `game_loop/scout_dispatch.rs`, `asteroid/lifecycle.rs`, `asteroid/spawn.rs`, `narrative/opening_sequence.rs`, `narrative/signal.rs`, `narrative/quest.rs`, `narrative/bottle.rs`, `narrative/logs.rs`, `narrative/narrative_events.rs`, `ui/hud/content.rs`, `ui/hud/prod_tree.rs`, `ui/station_tabs.rs`, `visuals/ore_polygon.rs`, `visuals/ingot_node.rs`, `visuals/component_nodes.rs`, `persistence/schema.rs` |
| **UNKNOWN** (not in module tree) | `scenes/restore.rs`, `scenes/save_overlay.rs` |

---

## §7 Critical Path to v4.0

### BLOCKERS

| # | Item | Location | Risk |
|---|---|---|---|
| 1 | **Scout systems run in all states** — missing `run_if(in_state(AppState::InGame))` | `lib.rs:200–202` | HIGH |
| 2 | **Opening sequence silent stall** — `get_single_mut()` failure returns silently; no timeout on distance-gated phases | `opening_sequence.rs:34–35` | HIGH |
| 3 | **`economy.rs` fulfillment cost hardcoded** — `25.0` iron not read from `requests.yaml` | `economy.rs:51` | HIGH |
| 4 | **`bottle.rs` L2→L3 violation** — `EguiContexts` in narrative layer | `bottle.rs:4` | HIGH |

### CLEANUP (pre-ship)

| # | Item | Location | Risk |
|---|---|---|---|
| 5 | `CLIENT_VERSION = "3.3.0"` will be wrong at v4.0 | `telemetry/mod.rs:6` | MEDIUM |
| 6 | `entity_setup.rs` imports `egui::Color32` — L1→L3 violation | `entity_setup.rs:8` | MEDIUM |
| 7 | `scenes/restore.rs` and `scenes/save_overlay.rs` not in module tree | `scenes/mod.rs` | MEDIUM |
| 8 | `flush_debug_log_system` and `reset_loop_stall_timer_on_upgrade` are dead | `debug_log.rs`, `telemetry/mod.rs` | LOW |
| 9 | `setup_debug_log_system` double-registered | `lib.rs:120,161` | LOW |
| 10 | `mining.rs` range values not in config | `mining.rs:28` | LOW |
| 11 | Scout paint ring geometry hardcoded | `scout_dispatch.rs:116` | LOW |

### DEFERRED (post-v4.0)

| # | Item | Risk |
|---|---|---|
| 12 | `content.rs` cargo tab X positions not config-driven | LOW |
| 13 | `content.rs` inline rgba values duplicate `visual.toml` | LOW |
| 14 | `logs.rs` `outer_ring_unlocked` always returns `false` | LOW |
| 15 | Opening sequence: add maximum timer fallback for distance-gated phases | MEDIUM |
| 16 | Rename `setup/mesh_builder.rs` → `setup/ore_mesh.rs` to eliminate name collision | LOW |
