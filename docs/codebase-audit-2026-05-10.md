# Codebase Organization Audit

**Date:** 2026-05-10  
**Scope:** Full `src/` directory  
**Objective:** Identify all remaining refactor and organization opportunities across the codebase

---

## Executive Summary

This audit analyzed 68 Rust source files totaling approximately 15,000 lines of code. The audit identified 5 god classes/oversized files, 5 areas of significant code duplication, 6 locations with hardcoded values, 4 instances of misplaced logic, and 8 minor cleanup opportunities.

**Key Findings:**
- **systems/ui/hud/content.rs** (744 lines) is the largest god class with massive duplicate drawing code
- **lib.rs** duplicates ~127 lines of app setup between native and WASM entry points
- **config/content.rs** repeats identical read_yaml pattern 5 times
- Multiple files contain magic numbers and hardcoded values that belong in configuration

---

## God Classes / Oversized Files

### systems/ui/hud/content.rs (744 lines)

**Issue:** Mixed responsibilities with massive duplicate drawing code

**Details:**
- Contains near-identical 18-line blocks for each ore type (Iron, Tungsten, Nickel, Aluminum) at lines 59-130
- Each ore block differs only in color/seed values
- Component drawing (Hull, Thruster, AI Core, Canister, Drone Bay) repeats similar pattern at lines 142-226
- Hardcoded layout constants: X_ORE: 40.0, X_INGOT: 220.0, X_COMPONENT: 400.0, X_ARROW_START: 428.0, X_ARROW_END: 648.0, X_DRONE: 660.0
- Hardcoded alpha values: 51, 128, 255 repeated throughout
- Lines 366-550: `draw_symbol_text` closure duplicates the same drawing logic

**Recommendation:** Extract drawing logic to visuals module and use data-driven approach with array of (symbol_type, color_config, seed) tuples. Move layout constants to visual.toml.

**Estimated Impact:** Reduce ~170 lines of duplication to ~30 lines

---

### systems/telemetry/mod.rs (453 lines)

**Issue:** Mixed responsibilities across telemetry, loop stall tracking, and log tab tracking

**Details:**
- Lines 125-152: Telemetry event sending
- Lines 154-206: Loop stall tracking logic
- Lines 208-238: Loop stall event sending (duplicates telemetry pattern)
- Lines 260-290: Log tab tracking (UI state, not telemetry)
- Lines 292-321: Log heartbeat tracking (UI state, not telemetry)
- Lines 323-406: Log tab state management (UI state, not telemetry)
- Line 6: Hardcoded `CLIENT_VERSION: "3.3.0"`
- Line 122: Hardcoded telemetry URL

**Recommendation:** Split into focused sub-modules:
- `telemetry_core`: Event sending infrastructure
- `loop_stall_monitor`: Loop stall detection
- `log_tracker`: Move to systems/ui/ module

Move CLIENT_VERSION and URL to config.

---

### systems/setup/entity_setup.rs (410 lines)

**Issue:** Mixed responsibilities - entity spawning + mesh generation helpers

**Details:**
- Lines 49-144: Entity spawning (appropriate for this module)
- Lines 146-257: Station spawning (appropriate)
- Lines 310-410: Procedural mesh generation functions:
  - `triangle_mesh` (lines 310-328)
  - `generate_ore_mesh` (lines 330-337)
  - `generate_iron_mesh_with_radius` (lines 340-361)
  - `generate_tungsten_mesh_with_radius` (lines 364-387)
  - `generate_nickel_mesh_with_radius` (lines 389-410)

**Recommendation:** Move mesh generation functions (lines 310-410) to `systems/visuals/mesh_builder.rs`. These are visual rendering utilities, not entity setup logic.

---

### systems/ui/hud/mod.rs (432 lines)

**Issue:** Mixed responsibilities - HUD systems + panel registration + state machine

**Details:**
- Lines 20-50: Non-egui world entity systems (ship_cargo_display_system, cargo_label_system)
- Lines 74-89: Non-egui station visual system
- Lines 92-100: Non-egui sync system
- Lines 144-431: egui panel registration and UI rendering
- Lines 102-142: HudParams SystemParam with 20+ resources/queries - excessive coupling

**Recommendation:** Separate non-egui world entity systems to dedicated module (e.g., `systems/ui/hud/world_systems.rs`). Split HudParams into smaller focused param structs to reduce coupling.

---

### systems/ui/hud/prod_tree.rs (410 lines)

**Issue:** Mixed responsibilities - production tree rendering + config construction

**Details:**
- Lines 11-178: Production tree rendering (appropriate)
- Lines 227-263: Duplicate ore config construction (same pattern as content.rs)
- Lines 270-340: Duplicate component config construction (same pattern as content.rs)
- Lines 29-32: Magic numbers for grid layout (col_width, row_height, node_size, drone_bay_size)

**Recommendation:** Extract config construction helpers to shared module accessible by both content.rs and prod_tree.rs. Move grid layout magic numbers to visual.toml.

---

## Code Duplication

### config/content.rs - Duplicate read_yaml Pattern

**Issue:** Identical read_yaml() implementation repeated 5 times

**Details:**
- Lines 32-51: ContentConfig::load()
- Lines 74-93: TutorialConfig::load()
- Lines 115-134: QuestConfig::load()
- Lines 167-186: RequestConfig::load()
- Lines 201-220: LogsConfig::load()

All 5 implementations have identical conditional compilation pattern:
```rust
#[cfg(any(target_arch = "wasm32", target_os = "android"))]
fn read_yaml() -> &'static str {
    include_str!("../../assets/content/...")
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn read_yaml() -> &'static str {
    Box::leak(
        std::fs::read_to_string("assets/content/...")
            .expect("Failed to read assets/content/...")
            .into_boxed_str(),
    )
}
```

**Recommendation:** Extract to shared helper:
```rust
fn read_config_yaml(path: &str) -> &'static str {
    // conditional compilation logic
}
```

**Estimated Impact:** 15 minutes

---

### lib.rs - Duplicate App Setup

**Issue:** ~127 lines of near-identical app setup code duplicated between native and WASM

**Details:**
- Lines 71-227: main() for native target
- Lines 233-398: start() for WASM target

Both functions contain identical:
- Plugin registration (EguiPlugin, TelemetryPlugin)
- State initialization (GameState, AppState)
- Resource insertion (ClearColor, various resources)
- Config loading (BalanceConfig, VisualConfig, ContentConfig, etc.)
- System registration (nearly identical system sets)
- Event registration

Only differences:
- WindowPlugin configuration (fullscreen vs canvas)
- detect_device_type call in WASM version

**Recommendation:** Extract common app setup to shared function:
```rust
fn build_app(app: &mut App, platform_specific: PlatformSpecificConfig) {
    // common setup
}
```

Consider moving bootstrapping logic to `src/bootstrap.rs`.

**Estimated Impact:** 30 minutes

---

### systems/ui/hud/content.rs - Duplicate Symbol Drawing

**Issue:** Massive duplicate drawing code for ore/ingot/component symbols

**Details:**
- Lines 59-94: IronOre drawing
- Lines 95-112: TungstenOre drawing (identical structure, different color/seed)
- Lines 113-129: NickelOre drawing (identical structure, different color/seed)
- Lines 130-141: AluminumOre drawing (identical structure, different color/seed)
- Lines 142-153: HullPlate drawing
- Lines 155-169: Thruster drawing
- Lines 171-187: AICore drawing
- Lines 189-204: Canister drawing
- Lines 206-225: DroneBay drawing

Each block manually constructs config objects with identical structure.

**Recommendation:** Use data-driven approach:
```rust
const SYMBOL_CONFIGS: &[(&str, SymbolType, ColorConfig, u64)] = &[
    ("IronOre", SymbolType::IronOre, metal_color, 1),
    ("TungstenOre", SymbolType::TungstenOre, h3_gas_color, 2),
    // ...
];
```

**Estimated Impact:** Reduce ~170 lines of duplication to ~30 lines (45 minutes)

---

### systems/ui/hud/prod_tree.rs - Duplicate Config Construction

**Issue:** Ore and component config construction duplicates content.rs pattern

**Details:**
- Lines 227-263: Ore config construction matches content.rs pattern
- Lines 270-340: Component config construction matches content.rs pattern

Both files manually construct the same config objects with identical field mappings.

**Recommendation:** Extract shared config helper functions to module accessible by both files. Create functions like:
- `build_ore_config(ore_type: OreDeposit, vcfg: &VisualConfig) -> OrePolygonConfig`
- `build_component_config(component_type: &str, vcfg: &VisualConfig) -> ComponentConfig`

---

### scenes/restore.rs - Duplicate Drone Spawning

**Issue:** spawn_saved_drones() duplicates opening drone spawn from entity_setup.rs

**Details:**
- Lines 141-208: spawn_saved_drones()
- entity_setup.rs lines 49-144: spawn_opening_drone()

Both spawn identical child entities:
- ThrusterGlow
- MiningBeam
- Cargo bar (background + fill)
- MapElement
- CargoOreLabel
- CargoCountLabel

**Recommendation:** Extract to shared drone spawning function:
```rust
fn spawn_drone_with_children(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &AssetServer,
    cfg: &BalanceConfig,
    vcfg: &VisualConfig,
    parent: Entity,
    drone_type: DroneType,
)
```

**Estimated Impact:** 20 minutes

---

## Hardcoded Values

### systems/telemetry/mod.rs

**Locations:**
- Line 6: `const CLIENT_VERSION: &str = "3.3.0"`
- Line 122: `get_telemetry_url()` returns hardcoded `"https://rfditservices.com/api/telemetry/v1/event"`

**Recommendation:** Move to config file (balance.toml or dedicated telemetry config section)

---

### systems/narrative/signal.rs

**Locations:**
- Lines 19-185: 33 hardcoded signal IDs (1-33) with hardcoded message strings
- Magic timing values: 2.0, 3.0, 5.0, 120.0 seconds
- Example: `"> SIGNAL RECEIVED."`, `"> SOURCE IDENTIFIED. BEARING 047."`, etc.

**Recommendation:** Move signal definitions to content config YAML for data-driven narrative authoring. Structure:
```yaml
signals:
  - id: 1
    message: "> SIGNAL RECEIVED."
    timing: 0.0
  - id: 2
    message: "> SOURCE IDENTIFIED. BEARING 047."
    timing: 2.0
```

---

### systems/ui/hud/content.rs

**Locations:**
- Lines 317-323: Layout positions
  ```rust
  const X_ORE: f32 = 40.0;
  const X_INGOT: f32 = 220.0;
  const X_COMPONENT: f32 = 400.0;
  const X_ARROW_START: f32 = 428.0;
  const X_ARROW_END: f32 = 648.0;
  const X_DRONE: f32 = 660.0;
  const CONTENT_TOP: f32 = 0.0;
  ```
- Lines 47, 48, 49: Alpha values (51, 128, 255)
- Lines 326-329: Dynamic sizing magic numbers
  ```rust
  let row_height = available_height / 3.8;
  let symbol_size = (row_height * 0.38).clamp(13.0, 17.0);
  let drone_size = (row_height * 0.6).clamp(32.0, 56.0);
  ```

**Recommendation:** Move to visual.toml config under production_tree section

---

### systems/asteroid/spawn.rs

**Locations:**
- Lines 9-16: Hardcoded ore type to config key mapping
  ```rust
  fn ore_config_key(ore_type: &OreDeposit) -> &'static str {
      match ore_type {
          OreDeposit::Iron => "metal",
          OreDeposit::Tungsten => "h3_gas",
          OreDeposit::Nickel => "void_essence",
          OreDeposit::Aluminum => "metal",
      }
  }
  ```
- Lines 154-159: Hardcoded sector IDs
  ```rust
  let sector_id = match ore_type {
      OreDeposit::Iron => "S1",
      OreDeposit::Tungsten => "S2",
      OreDeposit::Nickel => "S3",
      OreDeposit::Aluminum => "S4",
  };
  ```
- Line 57: Spawn distance range `200.0..500.0`
- Line 54: Retry attempt count `10`

**Recommendation:** Move to balance.toml or content config

---

### scenes/restore.rs

**Location:**
- Lines 47-49: Hardcoded tutorial ID array
  ```rust
  for id in [101u32, 102, 103, 104, 105, 106] {
      tutorial.shown.insert(id);
  }
  ```

**Recommendation:** Move to tutorial config YAML

---

### constants.rs

**Locations:**
- Lines 12-14: Hardcoded sector positions
  ```rust
  pub const SECTOR_1_POS: Vec2 = Vec2::new(320.0, 140.0);   // Iron
  pub const SECTOR_2_POS: Vec2 = Vec2::new(-220.0, 340.0);  // Tungsten
  pub const SECTOR_3_POS: Vec2 = Vec2::new(380.0, -280.0);  // Nickel
  ```

**Recommendation:** Move to visual.toml or balance.toml. Note: File already has comment indicating most constants moved to config, these are the remaining stragglers.

---

### systems/ui/hud/prod_tree.rs

**Locations:**
- Lines 29-32: Grid layout magic numbers
  ```rust
  let col_width = rect.width() / 4.0;
  let row_height = rect.height() / 5.0;
  let node_size = egui::vec2(100.0, 40.0);
  let drone_bay_size = egui::vec2(200.0, 40.0);
  ```

**Recommendation:** Move to visual.toml under production_tree section

---

## Misplaced Logic

### systems/setup/entity_setup.rs

**Misplaced:** Mesh generation functions (lines 310-410)

**Details:**
- `triangle_mesh`, `generate_ore_mesh`, `generate_iron_mesh_with_radius`, `generate_tungsten_mesh_with_radius`, `generate_nickel_mesh_with_radius`
- These are procedural mesh generation utilities for visual rendering
- Not entity setup logic

**Correct Location:** `systems/visuals/mesh_builder.rs`

---

### systems/telemetry/mod.rs

**Misplaced:** Log tab tracking logic (lines 323-406)

**Details:**
- `track_log_tab_open`, `track_log_heartbeat`, `reset_log_heartbeat_timer`
- These track UI state (which tab is open, heartbeat timing)
- Not telemetry data collection

**Correct Location:** `systems/ui/` module (new file or existing)

---

### systems/ui/hud/content.rs

**Misplaced:** Procedural visual drawing (lines 35-252)

**Details:**
- Ore polygon drawing, ingot node drawing, component node drawing
- These are visual rendering operations using egui painters
- Not UI content/logic

**Correct Location:** `systems/visuals/` module (egui rendering utilities)

---

### lib.rs

**Misplaced:** Entire app setup logic (lines 71-398)

**Details:**
- Plugin registration, resource initialization, system registration
- This is application bootstrapping
- Not library code (file is named lib.rs but contains main())

**Correct Location:** Create `src/bootstrap.rs` or `src/app.rs` for app setup

---

## Minor Cleanups

### config/content.rs - Extract read_yaml Helper

**Task:** Extract common read_yaml pattern to shared helper function

**Impact:** Reduces 5 identical implementations to 1 shared function

**Estimate:** 15 minutes

---

### lib.rs - Extract App Setup

**Task:** Extract common app setup to shared function called by both main() and start()

**Impact:** Eliminates ~127 lines of duplication

**Estimate:** 30 minutes

---

### systems/ui/hud/content.rs - Extract Symbol Drawing Helper

**Task:** Extract duplicate symbol drawing to data-driven helper function

**Impact:** Reduce ~170 lines of duplication to ~30 lines

**Estimate:** 45 minutes

---

### systems/asteroid/spawn.rs - Extract Ore Type Matching

**Task:** Extract ore type matching logic to helper function, move sector IDs to config

**Impact:** Cleaner code, configurable values

**Estimate:** 20 minutes

---

### systems/ui/hud/prod_tree.rs - Extract Node Rendering Helper

**Task:** Extract node rendering helper, extract config construction helpers to shared module

**Impact:** Reduced duplication with content.rs

**Estimate:** 30 minutes

---

### constants.rs - Move to Config

**Task:** Move remaining sector positions to visual.toml or balance.toml

**Impact:** Complete migration to config-driven approach

**Estimate:** 10 minutes

---

### systems/telemetry/mod.rs - Split Module

**Task:** Split into smaller modules (telemetry_core, loop_stall, log_tracker)

**Impact:** Focused responsibilities, easier to maintain

**Estimate:** 1 hour

---

### systems/setup/entity_setup.rs - Move Mesh Generation

**Task:** Move mesh generation functions to visuals module

**Impact:** Correct module placement

**Estimate:** 30 minutes

---

### systems/ui/hud/mod.rs - Separate Non-Egui Systems

**Task:** Separate non-egui world entity systems to dedicated module, split HudParams

**Impact:** Clearer separation of concerns, reduced coupling

**Estimate:** 45 minutes

---

## Summary Statistics

- **Total files analyzed:** 68
- **Total lines of code:** ~15,000
- **God classes identified:** 5
- **Code duplication instances:** 5
- **Hardcoded value locations:** 6
- **Misplaced logic instances:** 4
- **Minor cleanup opportunities:** 8
- **Estimated total refactor time:** ~5.5 hours

---

## Prioritized Recommendations

### High Priority (God Classes)
1. Extract duplicate symbol drawing from content.rs
2. Split telemetry module into focused sub-modules
3. Move mesh generation from entity_setup to visuals
4. Separate non-egui systems from hud/mod.rs
5. Extract duplicate app setup from lib.rs

### Medium Priority (Duplication)
6. Extract read_yaml helper from config/content.rs
7. Extract config construction helpers shared between content.rs and prod_tree.rs
8. Deduplicate drone spawning between restore.rs and entity_setup.rs

### Low Priority (Cleanups)
9. Move hardcoded values to config (sweep across all files)
10. Move signal definitions to content YAML

---

## Appendix: File Size Distribution

**Largest files (>300 lines):**
1. systems/ui/hud/content.rs: 744 lines
2. systems/visuals/component_nodes.rs: 465 lines
3. systems/telemetry/mod.rs: 453 lines
4. systems/ui/hud/mod.rs: 432 lines
5. systems/ui/hud/prod_tree.rs: 410 lines
6. systems/setup/entity_setup.rs: 410 lines
7. lib.rs: 398 lines
8. config/visual.rs: 348 lines
9. scenes/main_menu.rs: 273 lines
10. systems/asteroid/spawn.rs: 256 lines

**Note:** component_nodes.rs (465 lines) was reviewed but deemed acceptable - it contains focused visual rendering for 5 component types with appropriate separation of concerns.
