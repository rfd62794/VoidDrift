# Voidrift HUD System Refactor Directive
**Phase: Architecture Reorganization**  
**Target: Split monolithic hud_ui_system into modular components**  
**Status: Ready for implementation**  
**Date: 2026-04-23**

---

## Objective

Restructure `src/systems/hud.rs` (450+ lines, one function) into a modular subsystem while preserving all current behavior. This unblocks the drawer-open-when-not-docked feature and makes future UI changes surgical rather than risky.

**Key fix:** Remove the line that force-closes the drawer every frame when not docked, allowing manual toggle to stick.

---

## Current State

- `hud_ui_system` in `src/systems/hud.rs` handles:
  - Drawer state machine (auto-open on dock, auto-close on undock)
  - Signal strip rendering (always visible)
  - Handle bar click logic (toggle drawer)
  - Content area (6 tabs: Station, Fleet, Cargo, Power, Refinery, Foundry, Hangar)
  - Secondary tabs (Power/Cargo/Refinery/Foundry/Hangar)
  - Primary tabs (Station/Fleet)
  - Quest overlay window
  - Central panel (world view rect calc)
  - Viewport rect tracking

- **Problem:** Drawer state is forced-closed at line 136 when `!is_docked && drawer == Expanded`
  - This prevents manual opening when flying
  - The fix is one-line, but the system is too tangled to touch safely

---

## Proposed Structure

Create a new `src/systems/hud/` module:

```
src/systems/hud/
├── mod.rs                    (module definition + HudParams re-export)
├── state_machine.rs          (drawer state logic only)
├── panels.rs                 (panel registration order, handle bar, signal strip)
├── content.rs                (all 6 tab content render functions)
└── helpers.rs                (shared render helpers, cargo bars, etc.)
```

Keep in `src/systems/hud.rs`:
- `ship_cargo_display_system` (unchanged)
- `autonomous_ship_cargo_display_system` (unchanged)
- `cargo_label_system` (unchanged)
- `station_visual_system` (unchanged)

---

## Implementation Steps

### Step 1: Create `src/systems/hud/mod.rs`

**Purpose:** Module root, re-exports, shared types.

```rust
// src/systems/hud/mod.rs

pub mod state_machine;
pub mod panels;
pub mod content;
pub mod helpers;

pub use state_machine::update_drawer_state;
pub use panels::register_hud_panels;
pub use content::render_tab_content;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::EguiContexts;
use crate::components::*;

// Re-export HudParams (from current hud.rs line 123-145)
#[derive(SystemParam)]
pub struct HudParams<'w, 's> {
    pub contexts: EguiContexts<'w, 's>,
    pub ship_query: Query<'w, 's, &'static mut Ship, (With<PlayerShip>, Without<AutonomousShipTag>, Without<Station>)>,
    pub station_query: Query<'w, 's, (Entity, &'static mut Station, &'static mut StationQueues), (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    pub state: Res<'w, State<GameState>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
    pub signal_log: Res<'w, SignalLog>,
    pub opening: Res<'w, OpeningSequence>,
    pub active_tab: ResMut<'w, ActiveStationTab>,
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub expanded: ResMut<'w, SignalStripExpanded>,
    pub quest_log: ResMut<'w, QuestLog>,
    pub forge_settings: Res<'w, ForgeSettings>,
    pub auto_dock_settings: ResMut<'w, AutoDockSettings>,
    pub tutorial: ResMut<'w, TutorialState>,
    pub pan_state: ResMut<'w, MapPanState>,
    pub cam_query: Query<'w, 's, &'static mut OrthographicProjection, With<MainCamera>>,
    pub menu_state: ResMut<'w, MainMenuState>,
    pub drawer: ResMut<'w, DrawerState>,
    pub ui_layout: Res<'w, UiLayout>,
    pub world_view_rect: ResMut<'w, WorldViewRect>,
}
```

---

### Step 2: Create `src/systems/hud/state_machine.rs`

**Purpose:** Drawer state transitions only. No UI rendering.

```rust
// src/systems/hud/state_machine.rs

use crate::components::*;

/// Update drawer state based on dock/undock and opening phase.
/// Returns whether viewport needs recalc (typically always true when drawer state changes).
pub fn update_drawer_state(
    is_docked: bool,
    opening_complete: bool,
    was_docked: &mut bool,
    drawer: &mut DrawerState,
) -> bool {
    // If opening sequence isn't done, drawer stays collapsed
    if !opening_complete {
        *drawer = DrawerState::Collapsed;
        return false;
    }

    // Auto-open drawer once when docking
    if is_docked && !*was_docked {
        *drawer = DrawerState::Expanded;
    }

    // Track dock state for next frame
    *was_docked = is_docked;

    // If drawer state changed, viewport needs recalc
    true
}
```

**Key point:** No auto-close logic. Drawer stays in whatever state it's set to.

---

### Step 3: Create `src/systems/hud/panels.rs`

**Purpose:** Panel registration in canonical order. Handle bar, signal strip.

```rust
// src/systems/hud/panels.rs

use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::constants::*;

/// Register all HUD panels in bottom-up order.
/// Panels are registered in this order:
/// 1. Signal strip (bottommost)
/// 2. Content area (if Expanded)
/// 3. Secondary tabs (if Expanded && is_docked)
/// 4. Primary tabs (if Expanded)
/// 5. Handle bar (always when opening_complete)
/// 6. Central panel (topmost, world view)
pub fn register_hud_panels(
    ctx: &mut egui::Context,
    contexts: &mut EguiContexts,
    drawer: DrawerState,
    is_docked: bool,
    opening_complete: bool,
    layout: UiLayout,
    signal_log: &SignalLog,
    expanded: &mut SignalStripExpanded,
    active_tab: &mut ActiveStationTab,
    world_view_rect: &mut WorldViewRect,
    quest_log: &QuestLog,
    pan_state: &mut MapPanState,
    tutorial: &mut TutorialState,
    cam_query: &mut bevy::prelude::Query<&mut bevy::prelude::OrthographicProjection, bevy::prelude::With<crate::components::MainCamera>>,
) {
    // Record canvas size for viewport calculation
    let screen = ctx.screen_rect();
    world_view_rect.canvas_w = screen.width();
    world_view_rect.canvas_h = screen.height();

    // ── 1. SIGNAL STRIP (always visible, bottommost) ─────────────────────────
    let signal_height = if expanded.0 { layout.signal_height * 3.0 } else { layout.signal_height };
    egui::TopBottomPanel::bottom("signal_strip")
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_rgb(5, 8, 12))
            .inner_margin(4.0))
        .exact_height(signal_height)
        .show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let response = ui.interact(rect, ui.id().with("strip_click"), egui::Sense::click());
            if response.clicked() {
                expanded.0 = !expanded.0;
            }
            let display_count = if expanded.0 { 8 } else { 3 };
            let entries: Vec<&String> = signal_log.entries.iter().rev().take(display_count).collect();
            ui.vertical(|ui| {
                for line in entries.iter().rev() {
                    ui.label(egui::RichText::new(*line)
                        .monospace()
                        .size(11.0)
                        .color(egui::Color32::from_rgb(0, 204, 102)));
                }
            });
        });

    if !opening_complete {
        // Register central panel for viewport rect, then early return
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let r = ui.max_rect();
                world_view_rect.x = r.min.x;
                world_view_rect.y = r.min.y;
                world_view_rect.w = r.width();
                world_view_rect.h = r.height();
            });
        return;
    }

    // ── 2. HANDLE BAR (always visible when game running, toggles drawer) ─────
    egui::TopBottomPanel::bottom("handle_bar")
        .resizable(false)
        .exact_height(layout.handle_height)
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_rgb(8, 10, 14)))
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let response = ui.interact(rect, ui.id(), egui::Sense::click());

            // Handle tap: toggle drawer state
            if response.clicked() {
                // THIS IS THE FIX: just toggle, don't force anything
                // The state_machine runs first, so drawer has a defined state
                // Tapping here changes it, and it STAYS changed
                // (No frame-by-frame force-reset)
            }

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                egui::FontId::monospace(10.0),
                egui::Color32::from_gray(60),
            );
        });

    // ── 3. CONTENT AREA (if Expanded) ───────────────────────────────────────
    if drawer == DrawerState::Expanded {
        egui::TopBottomPanel::bottom("content_area")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(8, 10, 16))
                .inner_margin(egui::Margin::symmetric(8, 8)))
            .exact_height(layout.content_height)
            .show(ctx, |ui| {
                if !is_docked {
                    // When flying, show placeholder
                    ui.heading("APPROACHING STATION");
                    ui.label("Dock to access station systems.");
                } else {
                    // When docked, content will be rendered by the main system
                    // (we'll call render_tab_content here in the next step)
                    ui.label("(content renders here)");
                }
            });
    }

    // ── 4. SECONDARY TABS (if Expanded && is_docked) ────────────────────────
    if drawer == DrawerState::Expanded && is_docked {
        egui::TopBottomPanel::bottom("secondary_tabs")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(15, 15, 20))
                .inner_margin(egui::Margin::symmetric(4, 4)))
            .exact_height(layout.secondary_tab_height)
            .show(ctx, |ui| {
                let available = ui.available_width();
                let tab_w = (available / 5.0 - 4.0).max(60.0);
                let tab_size = egui::vec2(tab_w, layout.secondary_tab_height - 8.0);
                ui.horizontal(|ui| {
                    for (tab, label) in [
                        (ActiveStationTab::Power, "POWER"),
                        (ActiveStationTab::Cargo, "CARGO"),
                        (ActiveStationTab::Refinery, "REFINERY"),
                        (ActiveStationTab::Foundry, "FOUNDRY"),
                        (ActiveStationTab::Hangar, "HANGAR"),
                    ] {
                        if ui.add_sized(tab_size, egui::SelectableLabel::new(*active_tab == tab, label)).clicked() {
                            *active_tab = tab;
                        }
                        ui.add_space(2.0);
                    }
                });
            });
    }

    // ── 5. PRIMARY TABS (if Expanded) ───────────────────────────────────────
    if drawer == DrawerState::Expanded {
        egui::TopBottomPanel::bottom("primary_tabs")
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgb(10, 10, 18))
                .inner_margin(egui::Margin::symmetric(4, 4)))
            .exact_height(layout.primary_tab_height)
            .show(ctx, |ui| {
                let available = ui.available_width();
                let tab_w = (available / 2.0 - 6.0).max(80.0);
                let tab_size = egui::vec2(tab_w, layout.primary_tab_height - 8.0);
                ui.horizontal(|ui| {
                    for (tab, label) in [
                        (ActiveStationTab::Station, "STATION"),
                        (ActiveStationTab::Fleet, "FLEET"),
                    ] {
                        if ui.add_sized(tab_size, egui::SelectableLabel::new(*active_tab == tab, label)).clicked() {
                            *active_tab = tab;
                        }
                        ui.add_space(4.0);
                    }
                });
            });
    }

    // ── 6. QUEST OVERLAY (window, above world) ──────────────────────────────
    if quest_log.panel_open {
        egui::Window::new("OBJECTIVES")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.set_min_height(300.0);
                ui.heading(egui::RichText::new("ACTIVE").color(egui::Color32::WHITE));
                ui.separator();
                for obj in quest_log.objectives.iter().filter(|o| o.state == ObjectiveState::Active) {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("▶").color(egui::Color32::CYAN));
                        ui.label(egui::RichText::new(&obj.description).strong());
                    });
                    if let (Some(curr), Some(target)) = (obj.progress_current, obj.progress_target) {
                        ui.add(egui::ProgressBar::new(curr as f32 / target as f32).text(format!("{}/{}", curr, target)));
                    }
                    ui.add_space(8.0);
                }
                ui.add_space(12.0);
                ui.heading(egui::RichText::new("COMPLETED").color(egui::Color32::GRAY));
                ui.separator();
                for obj in quest_log.objectives.iter().filter(|o| o.state == ObjectiveState::Complete) {
                    ui.label(egui::RichText::new(format!("✓ {}", obj.description)).color(egui::Color32::from_gray(140)));
                }
            });
    }

    // ── 7. TUTORIAL POP-UP ──────────────────────────────────────────────────
    if let Some(popup) = tutorial.active.clone() {
        egui::Window::new(egui::RichText::new(&popup.title).strong().color(egui::Color32::CYAN))
            .id(egui::Id::new("tutorial_popup"))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([300.0, 180.0])
            .frame(egui::Frame::window(&ctx.style())
                .fill(egui::Color32::from_rgb(5, 5, 10))
                .stroke(egui::Stroke::new(2.0, egui::Color32::CYAN))
                .inner_margin(16.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(&popup.body).size(13.0).color(egui::Color32::WHITE));
                    ui.add_space(20.0);
                    if ui.add(egui::Button::new(egui::RichText::new(&popup.button_label).strong()).min_size(egui::vec2(120.0, 40.0))).clicked() {
                        tutorial.shown.insert(popup.id);
                        tutorial.active = None;
                    }
                });
            });

        // Dismiss on tap anywhere else if not consumed by the window
        if ctx.input(|i| i.pointer.any_click()) && !ctx.is_pointer_over_area() {
            tutorial.shown.insert(popup.id);
            tutorial.active = None;
        }
    }

    // ── 8. CENTRAL PANEL (world view — MUST be last) ───────────────────────
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let r = ui.max_rect();
            world_view_rect.x = r.min.x;
            world_view_rect.y = r.min.y;
            world_view_rect.w = r.width();
            world_view_rect.h = r.height();

            if ui.add(egui::Button::new("FOCUS").min_size(egui::vec2(80.0, 44.0))).clicked() {
                pan_state.is_focused = true;
                pan_state.cumulative_offset = bevy::prelude::Vec2::ZERO;
                if let Ok(mut proj) = cam_query.get_single_mut() {
                    proj.scale = 1.0;
                }
            }
        });
}
```

---

### Step 4: Create `src/systems/hud/content.rs`

**Purpose:** All 6 tab content render functions. Move from current hud.rs lines 220-381.

```rust
// src/systems/hud/content.rs

use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;
use crate::systems::station_tabs::render_queue_card;

/// Render the active tab content.
pub fn render_tab_content(
    ui: &mut egui::Ui,
    active_tab: ActiveStationTab,
    station: &mut Station,
    queues: &mut StationQueues,
    ship: &mut Ship,
    auto_dock_settings: &mut AutoDockSettings,
    meshes: &mut bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::Mesh>>,
    materials: &mut bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::ColorMaterial>>,
    commands: &mut bevy::prelude::Commands,
) {
    match active_tab {
        ActiveStationTab::Station => {
            ui.heading("VOIDRIFT STATION");
            ui.add_space(8.0);
            ui.label(egui::RichText::new("ECHO: STATION AI - OPERATIONAL")
                .color(egui::Color32::from_rgb(0, 204, 102)));
        }
        ActiveStationTab::Fleet => {
            ui.heading("FLEET");
            ui.add_space(8.0);
            ui.label("DRONE MANAGEMENT - COMING ONLINE");
        }
        ActiveStationTab::Cargo => {
            ui.vertical(|ui| {
                ui.heading("CARGO BAY");
                ui.add_space(8.0);
                egui::Grid::new("res_grid").spacing([20.0, 8.0]).show(ui, |ui| {
                    ui.label("MAGNETITE:"); ui.label(egui::RichText::new(format!("{:.1}", station.magnetite_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("CARBON:"); ui.label(egui::RichText::new(format!("{:.1}", station.carbon_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("HULL PLATES:"); ui.label(egui::RichText::new(format!("{}", station.hull_plate_reserves)).color(egui::Color32::WHITE)); ui.end_row();
                    ui.label("POWER CELLS:"); ui.label(egui::RichText::new(format!("{}", station.power_cells)).color(egui::Color32::GREEN)); ui.end_row();
                    ui.label("AI CORES:"); ui.label(egui::RichText::new(format!("{}", station.ai_cores)).color(egui::Color32::CYAN)); ui.end_row();
                    ui.label("SHIP HULLS:"); ui.label(egui::RichText::new(format!("{}", station.ship_hulls)).color(egui::Color32::GOLD)); ui.end_row();
                });
                ui.add_space(16.0);
                ui.separator();
                ui.heading("AUTO-DOCK SETTINGS");
                ui.checkbox(&mut auto_dock_settings.auto_unload, "Auto-Unload Cargo");
                ui.checkbox(&mut auto_dock_settings.auto_smelt_magnetite, "Auto-Smelt Magnetite");
                ui.checkbox(&mut auto_dock_settings.auto_smelt_carbon, "Auto-Smelt Carbon");
            });
            if !station.online {
                if ui.button(format!("REPAIR STATION [{} CELLS]", REPAIR_COST)).clicked() && station.power_cells >= REPAIR_COST {
                    station.power_cells -= REPAIR_COST; station.repair_progress = 1.0; station.online = true;
                }
            }
        }
        ActiveStationTab::Power => {
            ui.heading("POWER");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(format!("STATION: {:.1}/{:.0}", station.power, STATION_POWER_MAX));
                ui.add(egui::ProgressBar::new(station.power / STATION_POWER_MAX).desired_width(150.0));
            });
            ui.horizontal(|ui| {
                ui.label(format!("SHIP:    {:.1}/{:.0}", ship.power, SHIP_POWER_MAX));
                ui.add(egui::ProgressBar::new(ship.power / SHIP_POWER_MAX).desired_width(150.0));
            });
        }
        ActiveStationTab::Refinery => {
            ui.horizontal(|ui| {
                render_queue_card(ui, station, &mut queues.magnetite_refinery, ProcessingOperation::MagnetiteRefinery, REFINERY_RATIO as f32, POWER_COST_REFINERY as f32, REFINERY_MAGNETITE_TIME);
                ui.add_space(16.0);
                render_queue_card(ui, station, &mut queues.carbon_refinery, ProcessingOperation::CarbonRefinery, HULL_PLATE_COST_CARBON as f32, POWER_COST_HULL_FORGE as f32, REFINERY_CARBON_TIME);
            });
        }
        ActiveStationTab::Foundry => {
            ui.horizontal(|ui| {
                render_queue_card(ui, station, &mut queues.hull_forge, ProcessingOperation::HullForge, SHIP_HULL_COST_PLATES as f32, POWER_COST_SHIP_FORGE as f32, FORGE_HULL_TIME);
                ui.add_space(16.0);
                render_queue_card(ui, station, &mut queues.core_fabricator, ProcessingOperation::CoreFabricator, AI_CORE_COST_CELLS as f32, POWER_COST_AI_FABRICATE as f32, FORGE_CORE_TIME);
            });
        }
        ActiveStationTab::Hangar => {
            ui.horizontal(|ui| {
                if ui.button("ASSEMBLE & DEPLOY AUTONOMOUS SHIP").clicked() && station.ship_hulls >= 1 && station.ai_cores >= 1 {
                    station.ship_hulls -= 1; station.ai_cores -= 1;
                    let (target_pos, ore, name) = if station.ai_cores >= 1 { (SECTOR_3_POS, OreType::Carbon, "Sector 3") } else { (SECTOR_1_POS, OreType::Magnetite, "Sector 1") };
                    commands.spawn((AutonomousShipTag, LastHeading(0.0), AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore, power: SHIP_POWER_MAX }, AutonomousAssignment { target_pos, ore_type: ore, sector_name: name.to_string() }, Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))), MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))), Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_SHIP)))
                    .with_children(|parent| {
                        parent.spawn((ThrusterGlow, Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))), MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), Transform::from_xyz(0.0, -18.0, 0.1), Visibility::Hidden));
                        parent.spawn((MiningBeam, Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))), MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))), Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP), Visibility::Hidden));
                        parent.spawn((Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))), MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))), Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP)));
                        parent.spawn((ShipCargoBarFill, Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))), MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))), Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05)));
                        parent.spawn((MapElement, Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(12.0, 16.0))), MeshMaterial2d(materials.add(ColorMaterial { color: Color::srgb(1.0, 0.5, 0.0), alpha_mode: bevy::sprite::AlphaMode2d::Opaque, ..default() })), Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)), Visibility::Hidden));
                    });
                }
                ui.separator();
                if ui.button("TOP UP SHIP [3 CELLS]").clicked() && station.power_cells >= 3 && ship.power_cells < 5 {
                    station.power_cells -= 3; ship.power_cells = (ship.power_cells + 3).min(5);
                }
            });
        }
    }
}
```

---

### Step 5: Create `src/systems/hud/helpers.rs`

**Purpose:** Shared rendering utilities (placeholder for future shared code).

```rust
// src/systems/hud/helpers.rs

// Placeholder for shared helpers (e.g., progress bar rendering, grid layouts)
// Currently empty — can grow as more UI code is extracted
```

---

### Step 6: Update `src/systems/hud.rs`

**New hud.rs:** Keep only the non-egui systems.

```rust
// src/systems/hud.rs

mod hud_panels;

pub use hud_panels::*;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::components::*;
use crate::constants::*;

// [Keep all existing cargo display systems unchanged]
// pub fn ship_cargo_display_system(...)
// pub fn autonomous_ship_cargo_display_system(...)
// pub fn cargo_label_system(...)
// pub fn station_visual_system(...)

// NEW MAIN HUD SYSTEM
pub fn hud_ui_system(mut params: hud_panels::HudParams, mut was_docked: Local<bool>) {
    let Ok(mut ship) = params.ship_query.get_single_mut() else { return; };
    let ctx = params.contexts.ctx_mut();
    let is_docked = ship.state == ShipState::Docked;
    let opening_complete = params.opening.phase == OpeningPhase::Complete;

    // Step 1: Update drawer state (state machine)
    hud_panels::state_machine::update_drawer_state(
        is_docked,
        opening_complete,
        &mut was_docked,
        &mut params.drawer,
    );

    let drawer = *params.drawer;

    // Step 2: Register all panels
    hud_panels::panels::register_hud_panels(
        ctx,
        &mut params.contexts,
        drawer,
        is_docked,
        opening_complete,
        *params.ui_layout,
        &params.signal_log,
        &mut params.expanded,
        &mut params.active_tab,
        &mut params.world_view_rect,
        &params.quest_log,
        &mut params.pan_state,
        &mut params.tutorial,
        &mut params.cam_query,
    );

    // Step 3: If docked and drawer open, render content
    if is_docked && drawer == DrawerState::Expanded {
        if let Ok((_station_ent, mut station, mut queues)) = params.station_query.get_single_mut() {
            // Render content in the content_area panel
            // NOTE: We need to hook this into the egui context somehow
            // For now, this is a placeholder — the content will be rendered
            // in register_hud_panels by passing station data through
        }
    }
}
```

**Wait** — there's a Bevy borrowing issue. The content needs to be rendered inside the egui `show()` closure in `panels.rs`, but we need station data from the query. Let me revise:

---

### Step 6b: Revised approach — keep content render in register_hud_panels

Actually, the cleanest approach: pass the station query result as an Option into `register_hud_panels`:

```rust
pub fn hud_ui_system(mut params: hud_panels::HudParams, mut was_docked: Local<bool>) {
    let Ok(mut ship) = params.ship_query.get_single_mut() else { return; };
    let ctx = params.contexts.ctx_mut();
    let is_docked = ship.state == ShipState::Docked;
    let opening_complete = params.opening.phase == OpeningPhase::Complete;

    hud_panels::state_machine::update_drawer_state(
        is_docked,
        opening_complete,
        &mut was_docked,
        &mut params.drawer,
    );

    let drawer = *params.drawer;

    // Get station data if available
    let station_data = params.station_query.get_single_mut().ok();

    hud_panels::panels::register_hud_panels(
        ctx,
        drawer,
        is_docked,
        opening_complete,
        *params.ui_layout,
        &params.signal_log,
        &mut params.expanded,
        &mut params.active_tab,
        &mut params.world_view_rect,
        &params.quest_log,
        &mut params.pan_state,
        &mut params.tutorial,
        &mut params.cam_query,
        station_data,  // ← Pass Option<(_, &mut Station, &mut StationQueues)>
        &mut params.commands,
        &mut params.meshes,
        &mut params.materials,
        &mut params.auto_dock_settings,
    );
}
```

Then in `register_hud_panels`, inside the content_area show closure:

```rust
if drawer == DrawerState::Expanded {
    egui::TopBottomPanel::bottom("content_area")
        .show(ctx, |ui| {
            if let Some((_, station, queues)) = &mut station_data {
                render_tab_content(ui, *active_tab, station, queues, ...);
            } else {
                ui.heading("APPROACHING STATION");
                ui.label("Dock to access station systems.");
            }
        });
}
```

---

## Final File Structure

After refactor:

```
src/systems/hud.rs
├── [Keep: cargo display systems, station visual system]
├── [Add: new hud_ui_system that orchestrates the modules below]
└── src/systems/hud/
    ├── mod.rs                    [HudParams, re-exports]
    ├── state_machine.rs          [update_drawer_state]
    ├── panels.rs                 [register_hud_panels]
    ├── content.rs                [render_tab_content]
    └── helpers.rs                [shared utilities]
```

---

## Expected Outcome

**Before:** 450-line monolithic function with intertwined state/render logic.  
**After:** Clear separation:
- State machine runs once, updates `DrawerState`
- Panels register in canonical order
- Content renders only when it should (docked + expanded + station exists)
- Handle bar tap toggles drawer without being overridden

**The fix:** Remove lines like `} else if !is_docked && drawer == Expanded { drawer = Collapsed; }` because the state machine stops fighting the user's input.

---

## Testing Checklist

- [ ] Game starts, opening sequence unchanged
- [ ] Docking auto-opens drawer
- [ ] Tapping handle bar while docked closes drawer
- [ ] Tapping handle bar while flying opens drawer (shows "APPROACHING STATION")
- [ ] Tapping handle bar again while flying closes drawer
- [ ] Viewport resizes when drawer toggles
- [ ] All 6 tabs render content correctly when docked + expanded
- [ ] Signal strip expands/collapses on click
- [ ] Quest overlay still appears when active
- [ ] Compile: `cargo check 2>&1 | grep -i error` should be empty

---

## Notes for Implementation

- Borrow checker: Be careful passing mutable references into egui closures
- If Rust complains about lifetime issues, consider using `Option<&mut T>` instead of direct references
- The handle bar click handler is currently empty (line shown in panels.rs) — you'll need to wire it up to actually toggle the drawer state. For now, the state_machine runs first so the drawer has a valid state, and future clicks just need to toggle it.

---

## Deferred Decisions

- [ ] Should flying state show a "APPROACHING STATION" message, or be completely empty?
- [ ] Should secondary tabs (Power/Cargo/etc) hide when flying, or stay visible as disabled?
- [ ] Quest overlay — keep in register_hud_panels or extract to separate window system?

---

**Ready for Antigravity to implement. Report back with compile errors if any arise.**
