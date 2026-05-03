# Voidrift — Directive: Save/Load System + Main Menu
**Status:** Approved — Ready for Execution
**Type:** Core Infrastructure + UI
**Date:** April 2026
**Branch:** dev
**Reference:** Voidrift GDD v1.0 (docs/GDD.md)
**Depends On:** Priority 4 (Opening Sequence) COMPLETE ✅

---

## 1. Objective

Implement a named save/load system and a Main Menu scene. The save system
serves two distinct audiences — players (Play Saves) and the developer
(Stage Saves). Stage Saves are a developer test harness: named snapshots
of specific game states that can be loaded directly, bypassing the opening
sequence to test any feature or progression point in isolation.

The Main Menu provides: New Game (full opening sequence), Continue (autosave),
Play Saves (named player saves), and Stage Saves (developer snapshots, hidden
behind a 7-tap unlock mirroring Android Developer Options).

---

## 2. New AppState

Add to `components.rs` or a new `state.rs`:

```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
```

Register in `lib.rs`:
```rust
.init_state::<AppState>()
```

All existing systems that run during gameplay must be gated to `InGame` state:
```rust
.add_systems(Update, some_system.run_if(in_state(AppState::InGame)))
```

All new main menu systems run in `MainMenu` state.

---

## 3. SaveData Structure

New file: `src/systems/save.rs`

```rust
use serde::{Deserialize, Serialize};

pub const SAVE_VERSION: u32 = 3;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SaveData {
    // Meta
    pub save_version: u32,
    pub save_name: String,
    pub save_category: SaveCategory,
    pub timestamp: String,
    pub description: String,          // developer note for stage saves

    // Opening state
    pub opening_complete: bool,
    pub opening_phase: String,        // serialized phase name

    // Station state
    pub station_online: bool,
    pub station_power: f32,
    pub power_cells: u32,
    pub magnetite: f32,
    pub carbon: f32,
    pub hull_plates: u32,
    pub ship_hulls: u32,
    pub ai_cores: u32,
    pub repair_progress: f32,

    // Tabs unlocked
    pub tab_power: bool,
    pub tab_cargo: bool,
    pub tab_refinery: bool,
    pub tab_foundry: bool,
    pub tab_hangar: bool,

    // Fleet state
    pub drone_count: u8,
    pub drones: Vec<DroneSaveData>,

    // World state
    pub sectors_discovered: Vec<String>,

    // UI state
    pub active_tab: String,
    pub drawer_state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SaveCategory {
    Play,   // player-facing saves
    Stage,  // developer test snapshots
    Auto,   // autosave
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DroneSaveData {
    pub assignment_sector: String,
    pub assignment_pos_x: f32,
    pub assignment_pos_y: f32,
    pub ore_type: String,
    pub state: String,
    pub cargo: f32,
    pub is_echo_primary: bool,
}
```

---

## 4. File Structure

Save files written to Android external storage — same path as debug logs,
already confirmed writable on Moto G.

```
/sdcard/Download/voidrift/
  autosave.json
  saves/
    play/
      [name].json
    stage/
      [name].json
```

On desktop (development): `./saves/play/` and `./saves/stage/`

Use `#[cfg(target_os = "android")]` to switch paths:

```rust
pub fn save_dir(category: &SaveCategory) -> PathBuf {
    #[cfg(target_os = "android")]
    let base = PathBuf::from("/sdcard/Download/voidrift/saves");
    #[cfg(not(target_os = "android"))]
    let base = PathBuf::from("./saves");

    match category {
        SaveCategory::Play => base.join("play"),
        SaveCategory::Stage => base.join("stage"),
        SaveCategory::Auto => base.join(".."),
    }
}

pub fn autosave_path() -> PathBuf {
    #[cfg(target_os = "android")]
    return PathBuf::from("/sdcard/Download/voidrift/autosave.json");
    #[cfg(not(target_os = "android"))]
    return PathBuf::from("./autosave.json");
}
```

Create directories on first save if they don't exist.

---

## 5. Save and Load Functions

```rust
pub fn save_game(data: &SaveData) -> Result<(), String> {
    let path = if data.save_category == SaveCategory::Auto {
        autosave_path()
    } else {
        let dir = save_dir(&data.save_category);
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create save dir: {e}"))?;
        let filename = sanitize_filename(&data.save_name);
        dir.join(format!("{filename}.json"))
    };

    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Serialization failed: {e}"))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("Write failed: {e}"))?;

    Ok(())
}

pub fn load_game(path: &PathBuf) -> Result<SaveData, String> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| format!("Read failed: {e}"))?;

    let data: SaveData = serde_json::from_str(&json)
        .map_err(|e| format!("Deserialization failed: {e}"))?;

    if data.save_version != SAVE_VERSION {
        // Return data anyway but caller can show version warning
        return Ok(data);
    }

    Ok(data)
}

pub fn list_saves(category: &SaveCategory) -> Vec<SaveData> {
    let dir = save_dir(category);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return vec![];
    };

    let mut saves: Vec<SaveData> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
        .filter_map(|e| load_game(&e.path()).ok())
        .collect();

    // Sort by timestamp descending — newest first
    saves.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    saves
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect()
}
```

---

## 6. Collecting Save State

Function to build `SaveData` from current world state:

```rust
pub fn collect_save_data(
    world: &World,
    name: String,
    category: SaveCategory,
    description: String,
) -> SaveData {
    let station = world.resource::<Station>();
    let tabs = world.resource::<StationTabsUnlocked>();
    let opening = world.resource::<OpeningSequence>();
    let drawer = world.resource::<DrawerState>();
    let active_tab = world.resource::<ActiveStationTab>();

    // Collect drone data
    let mut drones = vec![];
    // Query autonomous ships from world and serialize

    SaveData {
        save_version: SAVE_VERSION,
        save_name: name,
        save_category: category,
        timestamp: current_timestamp(),
        description,
        opening_complete: opening.phase == OpeningPhase::Complete,
        opening_phase: format!("{:?}", opening.phase),
        station_online: station.online,
        station_power: station.power,
        power_cells: station.power_cells,
        magnetite: station.magnetite_reserves,
        carbon: station.carbon_reserves,
        hull_plates: station.hull_plate_reserves,
        ship_hulls: station.ship_hulls,
        ai_cores: station.ai_cores,
        repair_progress: station.repair_progress,
        tab_power: tabs.power,
        tab_cargo: tabs.cargo,
        tab_refinery: tabs.refinery,
        tab_foundry: tabs.foundry,
        tab_hangar: tabs.hangar,
        drone_count: drones.len() as u8,
        drones,
        sectors_discovered: vec![], // populate from SectorDiscovered resource
        active_tab: format!("{:?}", active_tab),
        drawer_state: format!("{:?}", drawer),
    }
}

fn current_timestamp() -> String {
    // Use std::time for a basic timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}
```

---

## 7. Applying Save State

Function to restore world state from `SaveData`:

```rust
pub fn apply_save_data(data: &SaveData, world: &mut World) {
    // Station
    let mut station = world.resource_mut::<Station>();
    station.online = data.station_online;
    station.power = data.station_power;
    station.power_cells = data.power_cells;
    station.magnetite_reserves = data.magnetite;
    station.carbon_reserves = data.carbon;
    station.hull_plate_reserves = data.hull_plates;
    station.ship_hulls = data.ship_hulls;
    station.ai_cores = data.ai_cores;
    station.repair_progress = data.repair_progress;

    // Tabs
    let mut tabs = world.resource_mut::<StationTabsUnlocked>();
    tabs.power = data.tab_power;
    tabs.cargo = data.tab_cargo;
    tabs.refinery = data.tab_refinery;
    tabs.foundry = data.tab_foundry;
    tabs.hangar = data.tab_hangar;

    // Opening phase
    let mut opening = world.resource_mut::<OpeningSequence>();
    if data.opening_complete {
        opening.phase = OpeningPhase::Complete;
    }

    // UI state
    // Restore DrawerState and ActiveStationTab from saved strings

    // Drones — despawn existing, respawn from save data
    // This is the most complex part — see Section 7.1
}
```

### 7.1 Drone Restoration

Drone restoration is the most complex part of load. Approach:

1. Despawn all existing `AutonomousShip` entities
2. For each `DroneSaveData`, spawn a new `AutonomousShip` entity with
   the saved assignment and state
3. Use the existing `spawn_autonomous_ship` helper if one exists,
   or spawn directly with the correct component bundle

This is acceptable complexity for MVP — the drone count is low (1-2 drones).

---

## 8. Autosave Trigger

In `autopilot.rs`, when a drone transitions to `Unloading` state
(ship has returned and docked), trigger autosave:

```rust
// In autonomous_ship_system, when state transitions to Unloading:
if ship.state == AutonomousShipState::Unloading {
    ev_autosave.send(AutosaveEvent);
}
```

New event:
```rust
#[derive(Event)]
pub struct AutosaveEvent;
```

New system in `save.rs`:
```rust
pub fn autosave_system(
    mut events: EventReader<AutosaveEvent>,
    world: &World,  // or use individual resource params
) {
    for _ in events.read() {
        let data = collect_save_data(
            world,
            "autosave".to_string(),
            SaveCategory::Auto,
            String::new(),
        );
        if let Err(e) = save_game(&data) {
            warn!("Autosave failed: {e}");
        }
    }
}
```

---

## 9. Main Menu Scene

New file: `src/scenes/main_menu.rs`

### 9.1 Resources for Main Menu State

```rust
#[derive(Resource, Default)]
pub struct MainMenuState {
    pub play_saves: Vec<SaveData>,
    pub stage_saves: Vec<SaveData>,
    pub autosave: Option<SaveData>,
    pub developer_mode: bool,
    pub dev_tap_count: u8,           // counts taps on title
    pub save_name_input: String,     // for new save name entry
    pub show_save_overlay: bool,     // in-game save overlay
    pub pending_load: Option<SaveData>, // save selected for loading
    pub version_mismatch_warning: Option<String>,
}
```

### 9.2 Main Menu Setup System

```rust
pub fn setup_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<MainMenuState>,
) {
    // Load save lists on menu entry
    menu_state.play_saves = list_saves(&SaveCategory::Play);
    menu_state.stage_saves = list_saves(&SaveCategory::Stage);
    menu_state.autosave = load_game(&autosave_path()).ok();
    menu_state.developer_mode = false;
    menu_state.dev_tap_count = 0;
}
```

### 9.3 Main Menu Render System

```rust
pub fn main_menu_system(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::none()
            .fill(egui::Color32::from_rgb(4, 6, 12)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(120.0);

                // Station title — 7-tap developer unlock
                let title_response = ui.add(
                    egui::Label::new(
                        egui::RichText::new("VOIDRIFT STATION")
                            .size(28.0)
                            .color(egui::Color32::from_rgb(0, 204, 102))
                            .strong()
                    )
                    .sense(egui::Sense::click())
                );

                if title_response.clicked() {
                    menu_state.dev_tap_count += 1;
                    if menu_state.dev_tap_count >= 7 {
                        menu_state.developer_mode = true;
                        // Signal entry added when InGame — note for now
                    }
                }

                ui.label(
                    egui::RichText::new("COMMAND INTERFACE")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(60, 80, 60))
                );

                if menu_state.developer_mode {
                    ui.label(
                        egui::RichText::new("[ DEVELOPER MODE ]")
                            .size(11.0)
                            .color(egui::Color32::from_rgb(180, 120, 0))
                    );
                }

                ui.add_space(48.0);

                // ECHO ambient line
                ui.label(
                    egui::RichText::new("> ECHO: AWAITING AUTHORIZATION.")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(0, 140, 70))
                );

                ui.add_space(32.0);

                let btn_width = 320.0;
                let btn_height = 52.0;
                let btn_size = egui::vec2(btn_width, btn_height);

                // NEW GAME
                if ui.add_sized(btn_size, egui::Button::new(
                    egui::RichText::new("NEW GAME")
                        .size(16.0)
                )).clicked() {
                    menu_state.pending_load = None;
                    next_state.set(AppState::InGame);
                    // InGame setup will detect no pending_load and run opening
                }

                ui.add_space(8.0);

                // CONTINUE
                let continue_label = if menu_state.autosave.is_some() {
                    "CONTINUE"
                } else {
                    "CONTINUE  (no autosave)"
                };
                let continue_btn = ui.add_sized(btn_size, egui::Button::new(
                    egui::RichText::new(continue_label).size(16.0)
                ));
                if continue_btn.clicked() {
                    if let Some(save) = &menu_state.autosave {
                        menu_state.pending_load = Some(save.clone());
                        next_state.set(AppState::InGame);
                    }
                }

                ui.add_space(24.0);

                // PLAY SAVES
                render_save_list(
                    ui,
                    "PLAY SAVES",
                    &menu_state.play_saves.clone(),
                    &mut menu_state,
                    btn_width,
                    false,
                );

                // STAGE SAVES — developer mode only
                if menu_state.developer_mode {
                    ui.add_space(16.0);
                    render_save_list(
                        ui,
                        "STAGE SAVES",
                        &menu_state.stage_saves.clone(),
                        &mut menu_state,
                        btn_width,
                        true,
                    );
                }
            });
        });

    // Handle pending load transition
    if let Some(_save) = &menu_state.pending_load {
        if matches!(next_state.0, Some(AppState::InGame)) {
            // State will transition — InGame startup will read pending_load
        }
    }
}

fn render_save_list(
    ui: &mut egui::Ui,
    heading: &str,
    saves: &[SaveData],
    menu_state: &mut MainMenuState,
    btn_width: f32,
    is_stage: bool,
) {
    ui.label(
        egui::RichText::new(heading)
            .size(12.0)
            .color(egui::Color32::from_rgb(80, 100, 80))
    );

    ui.add_space(4.0);

    if saves.is_empty() {
        ui.label(
            egui::RichText::new("  no saves found")
                .size(11.0)
                .color(egui::Color32::from_rgb(50, 60, 50))
        );
    }

    for save in saves.iter().take(20) {
        let version_ok = save.save_version == SAVE_VERSION;
        let label = if version_ok {
            format!(
                "{}    {}",
                save.save_name,
                format_timestamp(&save.timestamp),
            )
        } else {
            format!("{}  [VERSION MISMATCH]", save.save_name)
        };

        let color = if !version_ok {
            egui::Color32::from_rgb(180, 60, 60)
        } else if is_stage {
            egui::Color32::from_rgb(180, 140, 0)
        } else {
            egui::Color32::from_rgb(0, 180, 90)
        };

        let btn = ui.add_sized(
            egui::vec2(btn_width, 40.0),
            egui::Button::new(
                egui::RichText::new(&label).size(13.0).color(color)
            )
        );

        if btn.clicked() {
            menu_state.pending_load = Some(save.clone());
            // next_state set by caller
        }
    }
}

fn format_timestamp(ts: &str) -> String {
    // Basic unix timestamp to readable
    ts.parse::<u64>().map(|secs| {
        let mins = secs / 60;
        let hours = mins / 60;
        let days = hours / 24;
        if days > 0 {
            format!("{days}d ago")
        } else if hours > 0 {
            format!("{hours}h ago")
        } else {
            format!("{mins}m ago")
        }
    }).unwrap_or_else(|_| ts.to_string())
}
```

---

## 10. In-Game Save Overlay

A small gear icon in the world view top-right triggers the save overlay.
The game continues running — this is not a pause screen.

### 10.1 Gear Button

Add to `hud.rs` alongside the FOCUS button:

```rust
// Gear button — top-right of world view
egui::Window::new("save_control")
    .fixed_pos([layout.screen_width - 56.0, 8.0])
    .fixed_size([44.0, 44.0])
    .frame(egui::Frame::none())
    .title_bar(false)
    .show(ctx, |ui| {
        if ui.add_sized([44.0, 44.0],
            egui::Button::new("⚙")).clicked() {
            menu_state.show_save_overlay = !menu_state.show_save_overlay;
        }
    });
```

### 10.2 Save Overlay

```rust
if menu_state.show_save_overlay {
    egui::Window::new("save_overlay")
        .fixed_pos([
            layout.screen_width / 2.0 - 160.0,
            layout.screen_height / 2.0 - 200.0,
        ])
        .fixed_size([320.0, 400.0])
        .title_bar(false)
        .frame(egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(4, 8, 12, 240))
            .stroke(egui::Stroke::new(1.0,
                egui::Color32::from_rgb(0, 100, 50))))
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("SAVE / LOAD")
                .size(14.0)
                .color(egui::Color32::from_rgb(0, 200, 100)));

            ui.separator();

            // Save name input
            ui.label(egui::RichText::new("SAVE NAME:")
                .size(11.0)
                .color(egui::Color32::from_rgb(80, 120, 80)));

            ui.text_edit_singleline(&mut menu_state.save_name_input);

            ui.add_space(8.0);

            // Save as Play Save
            if ui.add_sized([300.0, 44.0],
                egui::Button::new("SAVE — PLAY")).clicked() {
                if !menu_state.save_name_input.is_empty() {
                    ev_save.send(SaveRequestEvent {
                        name: menu_state.save_name_input.clone(),
                        category: SaveCategory::Play,
                        description: String::new(),
                    });
                    menu_state.show_save_overlay = false;
                }
            }

            // Save as Stage Save — developer mode only
            if menu_state.developer_mode {
                if ui.add_sized([300.0, 44.0],
                    egui::Button::new(
                        egui::RichText::new("SAVE — STAGE")
                            .color(egui::Color32::from_rgb(200, 160, 0))
                    )).clicked() {
                    if !menu_state.save_name_input.is_empty() {
                        ev_save.send(SaveRequestEvent {
                            name: menu_state.save_name_input.clone(),
                            category: SaveCategory::Stage,
                            description: menu_state.save_name_input.clone(),
                        });
                        menu_state.show_save_overlay = false;
                    }
                }
            }

            ui.separator();

            // Return to main menu
            if ui.add_sized([300.0, 44.0],
                egui::Button::new("MAIN MENU")).clicked() {
                next_state.set(AppState::MainMenu);
                menu_state.show_save_overlay = false;
            }

            ui.add_space(4.0);

            // Close overlay
            if ui.add_sized([300.0, 36.0],
                egui::Button::new(
                    egui::RichText::new("CLOSE")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(80, 80, 80))
                )).clicked() {
                menu_state.show_save_overlay = false;
            }
        });
}
```

---

## 11. Save Request Event

```rust
#[derive(Event)]
pub struct SaveRequestEvent {
    pub name: String,
    pub category: SaveCategory,
    pub description: String,
}

pub fn handle_save_requests(
    mut events: EventReader<SaveRequestEvent>,
    station: Res<Station>,
    tabs: Res<StationTabsUnlocked>,
    opening: Res<OpeningSequence>,
    // ... other resources
    mut signal_log: ResMut<SignalLog>,
) {
    for req in events.read() {
        let data = SaveData {
            save_name: req.name.clone(),
            save_category: req.category.clone(),
            description: req.description.clone(),
            // ... collect from resources
        };

        match save_game(&data) {
            Ok(_) => {
                let msg = format!(
                    "ECHO: SAVE COMPLETE — {}.",
                    req.name.to_uppercase()
                );
                signal_log.entries.push_back(msg);
            }
            Err(e) => {
                let msg = format!("ECHO: SAVE FAILED — {e}");
                signal_log.entries.push_back(msg);
            }
        }
    }
}
```

---

## 12. InGame Startup — New Game vs Load

When `AppState` transitions to `InGame`, check `MainMenuState.pending_load`:

```rust
pub fn ingame_startup_system(
    mut commands: Commands,
    menu_state: Res<MainMenuState>,
    // ... other params
) {
    if let Some(save_data) = &menu_state.pending_load {
        // LOAD PATH — apply save data, skip opening sequence
        apply_save_data(save_data, &mut world);
        // Set opening phase to Complete so sequence doesn't run
    } else {
        // NEW GAME PATH — normal setup, opening sequence runs
        // Station spawns dark: online=false, power=0, power_cells=0
        // Opening sequence begins at Adrift phase
    }
}
```

---

## 13. Cargo.toml Dependencies

Add if not already present:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Verify these aren't already in Cargo.toml before adding.

---

## 14. Developer Mode — ECHO Signal on Unlock

When developer mode unlocks (7th tap), the tap count is in the main menu
context where ECHO's Signal strip doesn't exist. Store a flag and fire the
Signal entry on first InGame frame:

```rust
// In ingame_startup_system or a one-shot system:
if menu_state.developer_mode && !menu_state.dev_mode_signal_fired {
    signal_log.entries.push_back(
        "ECHO: DEVELOPER MODE ENABLED.".to_string()
    );
    signal_log.entries.push_back(
        "ECHO: STAGE SAVES NOW ACCESSIBLE.".to_string()
    );
    menu_state.dev_mode_signal_fired = true;
}
```

Add `dev_mode_signal_fired: bool` to `MainMenuState`.

---

## 15. Predefined Stage Saves to Create Immediately

After the system is working, create these stage saves manually by loading
the game, reaching each state, and saving with the gear button in developer
mode:

| Name | Description | How to reach |
|------|-------------|--------------|
| `OPENING_DEMO` | Fresh cinematic start | New Game, save at first frame |
| `POST_OPENING` | After reactor, before fleet | Play through opening, save at Activate button |
| `TIER1_LOOP` | Station running, 1 drone | Play until ECHO mining loop established |
| `TIER2_ENTRY` | Hull repaired, all tabs open | Play until hull repair complete |
| `FLEET_TEST` | 2 drones, S1-S4 discovered | Play until second drone active |
| `FOUNDRY_TEST` | Full production queue | Play until Foundry producing |

These become the test harness for every future feature directive.

---

## 16. File Scope

| File | Change |
|------|--------|
| `src/systems/save.rs` | CREATE — full save system |
| `src/scenes/main_menu.rs` | CREATE — main menu scene |
| `src/components.rs` | Add AppState, MainMenuState, AutosaveEvent, SaveRequestEvent |
| `src/lib.rs` | Register AppState, all new systems, scene transitions |
| `src/systems/hud.rs` | Add gear button, save overlay, developer mode flag check |
| `src/systems/autopilot.rs` | Add autosave trigger on drone Unloading transition |
| `Cargo.toml` | Add serde + serde_json if not present |

---

## 17. Implementation Sequence

**Do not combine steps. Compile-check after each.**

1. Add `AppState` enum and register — verify compile
2. Gate all existing InGame systems to `in_state(AppState::InGame)` — verify compile
3. Create `save.rs` with `SaveData` struct and file functions only — verify compile
4. Create `main_menu.rs` with setup system only (no save list yet) — verify compile, verify main menu renders on launch
5. Add New Game → InGame transition — verify compile, verify game starts from menu
6. Add Continue — verify autosave loads correctly
7. Add save collection (`collect_save_data`) — verify compile
8. Add save overlay (gear button + overlay) — deploy, verify on device
9. Add autosave trigger in autopilot — deploy, verify autosave file appears on device
10. Add Play Save list to main menu — deploy, verify saved games appear
11. Add developer mode tap counter — deploy, verify 7 taps shows Stage Saves
12. Create predefined Stage Saves (Section 15) — verify each loads cleanly
13. Tag: `git tag v0.5.4-save-system`

---

## 18. Completion Criteria

- [ ] Main Menu renders on launch with ECHO ambient line
- [ ] New Game starts fresh opening sequence
- [ ] Continue loads autosave, skips opening sequence
- [ ] Gear button visible in world view top-right
- [ ] Save overlay opens without pausing game
- [ ] Play Save created and appears in main menu save list
- [ ] Autosave fires on every drone dock
- [ ] Autosave file exists at correct Android path after first dock
- [ ] 7 taps on title enables developer mode
- [ ] ECHO signals developer mode on next InGame frame
- [ ] Stage Save section visible only in developer mode
- [ ] Stage Save created and appears in Stage Saves list
- [ ] Version mismatch flagged visually in save list
- [ ] All predefined Stage Saves load to correct game state
- [ ] Gate: load FLEET_TEST save — 2 drones active, S1-S4 discovered, all tabs open

---

## 19. ADR Update

> **ADR-018:** Save system uses named JSON files in two categories —
> Play (player-facing) and Stage (developer test snapshots). Files stored
> at `/sdcard/Download/voidrift/saves/` on Android. Autosave fires on
> every drone dock. SAVE_VERSION=3. Version mismatches shown visually,
> not silently failed. Stage Saves accessible only in developer mode
> (7-tap unlock on station title, mirrors Android Developer Options).
> Developer mode is session-only — not persisted to disk.

---

*Voidrift Save/Load + Main Menu Directive*
*April 2026 — RFD IT Services*
*Reference: Voidrift GDD v1.0 — docs/GDD.md*
*This system is the test harness for all future development.*
