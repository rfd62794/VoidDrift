use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::systems::persistence::save::{list_saves, load_game, autosave_path, SaveCategory, SaveData, SAVE_VERSION};

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
    pub dev_mode_signal_fired: bool,
}

pub fn setup_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<MainMenuState>,
) {
    // Spawn camera for main menu
    spawn_menu_camera(&mut commands);
    
    // Load save lists on menu entry
    menu_state.play_saves = list_saves(&SaveCategory::Play);
    menu_state.stage_saves = list_saves(&SaveCategory::Stage);
    menu_state.autosave = load_game(&autosave_path()).ok();
    menu_state.developer_mode = false;
    menu_state.dev_tap_count = 0;
    menu_state.dev_mode_signal_fired = false;
}

pub fn main_menu_system(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_rgb(4, 6, 12)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(120.0);

                // Station title - 7-tap developer unlock
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
                    &mut next_state,
                );

                // STAGE SAVES - developer mode only
                if menu_state.developer_mode {
                    ui.add_space(16.0);
                    render_save_list(
                        ui,
                        "STAGE SAVES",
                        &menu_state.stage_saves.clone(),
                        &mut menu_state,
                        btn_width,
                        true,
                        &mut next_state,
                    );
                }
            });
        });
}

fn render_save_list(
    ui: &mut egui::Ui,
    heading: &str,
    saves: &[SaveData],
    menu_state: &mut MainMenuState,
    btn_width: f32,
    is_stage: bool,
    next_state: &mut NextState<AppState>,
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
            next_state.set(AppState::InGame);
        }
    }
}

fn format_timestamp(ts: &str) -> String {
    ts.parse::<u64>().map(|secs| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let diff = now.saturating_sub(secs);
        if diff < 60 {
            format!("{}s ago", diff)
        } else if diff < 3600 {
            let mins = diff / 60;
            format!("{}m ago", mins)
        } else {
            let hours = diff / 3600;
            format!("{}h ago", hours)
        }
    }).unwrap_or_else(|_| ts.to_string())
}


fn spawn_menu_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: -1, // Below MainCamera (order 0) — no ambiguity when transitioning
            ..default()
        },
        OrthographicProjection {
            far: 1200.0,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
        MenuCamera,
    ));
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuCamera>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn ingame_startup_system(
    mut menu_state: ResMut<MainMenuState>,
    mut opening: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
    mut station_query: Query<&mut Station, (With<Station>, Without<Ship>)>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut queue: ResMut<ShipQueue>,
    opening_drone_query: Query<Entity, With<InOpeningSequence>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if let Some(save_data) = menu_state.pending_load.take() {
        // LOAD PATH — apply save data, skip opening sequence
        // Restore opening phase
        opening.phase = match save_data.opening_phase.as_str() {
            "Adrift"           => OpeningPhase::Adrift,
            "SignalIdentified" => OpeningPhase::SignalIdentified,
            "AutoPiloting"     => OpeningPhase::AutoPiloting,
            "InRange"          => OpeningPhase::InRange,
            "Docked"           => OpeningPhase::Docked,
            "Complete"         => OpeningPhase::Complete,
            _                  => OpeningPhase::Complete,
        };
        opening.timer = 0.0;

        // Only despawn the opening drone if the intro is actually finished
        if opening.phase == OpeningPhase::Complete {
            for ent in opening_drone_query.iter() {
                commands.entity(ent).despawn_recursive();
            }
        }

        // Restore the queue count from save
        queue.available_count = save_data.ship_hulls as u32;

        // Emergency sanity check: if intro is skipped/finished but fleet is 0, gift 1 drone
        if opening.phase == OpeningPhase::Complete && queue.available_count == 0 {
            queue.available_count = 1;
            info!("[Voidrift] Load sanity check: Gifting emergency drone to empty fleet.");
        }

        // Restore station state to the just-spawned station entity
        if let Ok(mut station) = station_query.get_single_mut() {
            station.online            = save_data.station_online;
            station.iron_reserves       = save_data.iron;
            station.iron_ingots         = save_data.iron_ingots;
            station.tungsten_reserves   = save_data.tungsten;
            station.tungsten_ingots     = save_data.tungsten_ingots;
            station.nickel_reserves     = save_data.nickel;
            station.nickel_ingots       = save_data.nickel_ingots;
            station.hull_plate_reserves = save_data.hull_plates;
            station.thruster_reserves   = save_data.thruster_reserves;
            // ship_hulls restored to queue above
            station.ai_cores            = save_data.ai_cores;
            station.repair_progress     = save_data.repair_progress;
            station.drone_build_progress = save_data.drone_build_progress;
        }

        // Restore active tab
        *active_tab = match save_data.active_tab.as_str() {
            "Station"  => ActiveStationTab::Station,
            "Fleet"    => ActiveStationTab::Fleet,
            "Cargo"    => ActiveStationTab::Cargo,
            "Iron"     => ActiveStationTab::Iron,
            "Tungsten" => ActiveStationTab::Tungsten,
            "Nickel"   => ActiveStationTab::Nickel,
            "Upgrades" => ActiveStationTab::Upgrades,
            _          => ActiveStationTab::Cargo,
        };

        signal_log.entries.push_back("ECHO: SAVE LOADED SUCCESSFULLY.".to_string());
        signal_log.entries.push_back(format!("ECHO: {} RESTORED.", save_data.save_name.to_uppercase()));

        // ── RESTORE ACTIVE DRONES ──
        for d in save_data.drones.iter() {
            let state = match d.state.as_str() {
                "Idle"       => ShipState::Idle,
                "Navigating" => ShipState::Navigating,
                "Mining"     => ShipState::Mining,
                "Docked"     => ShipState::Docked,
                _            => ShipState::Navigating,
            };
            let ore_type = match d.ore_type.as_str() {
                "Iron"     => OreDeposit::Iron,
                "Tungsten" => OreDeposit::Tungsten,
                "Nickel"   => OreDeposit::Nickel,
                _          => OreDeposit::Iron,
            };

            let ship_ent = commands.spawn((
                Ship {
                    state,
                    speed: crate::constants::SHIP_SPEED,
                    cargo: d.cargo,
                    cargo_type: ore_type,
                    cargo_capacity: crate::constants::CARGO_CAPACITY,
                    laser_tier: LaserTier::Basic,
                    current_mining_target: None,
                },
                AutonomousShipTag,
                LastHeading(d.heading),
                Transform::from_xyz(d.pos_x, d.pos_y, crate::constants::Z_SHIP),
                Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.0, 0.6, 1.0))),
            )).id();

            // If it had a destination, restore Autopilot
            if d.assignment_pos_x != 0.0 || d.assignment_pos_y != 0.0 {
                commands.entity(ship_ent).insert(AutopilotTarget {
                    destination: Vec2::new(d.assignment_pos_x, d.assignment_pos_y),
                    target_entity: None, // Asteroid entity link is lost, but position is enough
                });
            }

            commands.entity(ship_ent).with_children(|parent| {
                parent.spawn((
                    ThrusterGlow,
                    Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.0))),
                    Transform::from_xyz(0.0, -18.0, 0.1),
                    Visibility::Hidden,
                ));
                parent.spawn((
                    MiningBeam,
                    Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
                    MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))),
                    Transform::from_xyz(0.0, 0.0, crate::constants::Z_BEAM - crate::constants::Z_SHIP),
                    Visibility::Hidden,
                ));
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                    MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
                    Transform::from_xyz(0.0, 24.0, crate::constants::Z_CARGO_BAR - crate::constants::Z_SHIP),
                ));
                parent.spawn((
                    ShipCargoBarFill,
                    Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                    MeshMaterial2d(materials.add(Color::srgb(0.0, 0.6, 1.0))),
                    Transform::from_xyz(0.0, 24.0, (crate::constants::Z_CARGO_BAR - crate::constants::Z_SHIP) + 0.05),
                ));
            });
        }
    } else {
        // NEW GAME PATH — opening sequence runs normally
    }

    // Developer mode signal (only once per session)
    if menu_state.developer_mode && !menu_state.dev_mode_signal_fired {
        signal_log.entries.push_back(
            "ECHO: DEVELOPER MODE ENABLED.".to_string()
        );
        signal_log.entries.push_back(
            "ECHO: STAGE SAVES NOW ACCESSIBLE.".to_string()
        );
        menu_state.dev_mode_signal_fired = true;
    }
}

pub fn save_overlay_system(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut save_events: EventWriter<crate::systems::persistence::save::SaveRequestEvent>,
) {
    let ctx = contexts.ctx_mut();

    if menu_state.show_save_overlay {
        egui::Window::new("save_overlay")
            .fixed_pos([
                ctx.screen_rect().width() / 2.0 - 160.0,
                ctx.screen_rect().height() / 2.0 - 200.0,
            ])
            .fixed_size([320.0, 400.0])
            .title_bar(false)
            .frame(egui::Frame::NONE
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
                    egui::Button::new("SAVE - PLAY")).clicked() {
                    let name = if menu_state.save_name_input.is_empty() {
                        "quicksave".to_string()
                    } else {
                        menu_state.save_name_input.clone()
                    };
                    save_events.send(crate::systems::persistence::save::SaveRequestEvent {
                        name,
                        category: crate::systems::persistence::save::SaveCategory::Play,
                        description: String::new(),
                    });
                    menu_state.show_save_overlay = false;
                }

                // Save as Stage Save - developer mode only
                if menu_state.developer_mode {
                    if ui.add_sized([300.0, 44.0],
                        egui::Button::new(
                            egui::RichText::new("SAVE - STAGE")
                                .color(egui::Color32::from_rgb(200, 160, 0))
                        )).clicked() {
                        if !menu_state.save_name_input.is_empty() {
                            save_events.send(crate::systems::persistence::save::SaveRequestEvent {
                                name: menu_state.save_name_input.clone(),
                                category: crate::systems::persistence::save::SaveCategory::Stage,
                                description: format!("Stage save - {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
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
}
