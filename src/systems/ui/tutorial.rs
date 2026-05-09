use bevy::prelude::*;
use crate::components::*;
use crate::config::TutorialConfig;

// ── PART A: TUTORIAL BEAT AUDIT ─────────────────────────────────────────────────────────────
// T-101 (game_start, highlight: asteroid_nearest): ✓ Highlight renders via show_for_asteroid logic
// T-102 (drone_navigating_or_mining, highlight: asteroid_nearest): ✓ Highlight renders via show_for_asteroid logic
// T-103 (ore_reserves_positive, highlight: none): ✗ BROKEN - Drawer button highlight missing (body mentions "Tap the grey bar")
// T-104 (drawer_expanded, highlight: none): ✗ BROKEN - Drawer button highlight missing (should highlight drawer button)
// T-105 (forge_tab_active, highlight: none): No highlight expected (UI tab, no spatial element)
// T-106 (bottle_exists, highlight: bottle): ✓ Highlight renders via show_for_bottle logic
// UNDERSTOOD button dismissal: ✓ Works correctly (tutorial.active cleared on click, id added to shown set)
// Sequence advancement: ✓ Works correctly (requires array checked, triggers evaluated in order)
// ─────────────────────────────────────────────────────────────────────────────────────────────

pub fn tutorial_system(
    mut tutorial: ResMut<TutorialState>,
    tutorial_cfg: Res<TutorialConfig>,
    opening: Res<OpeningSequence>,
    // T-001 to T-006: require opening ship (despawned at Complete — legacy, preserved)
    opening_ship_query: Query<
        (&Ship, &Transform),
        (With<InOpeningSequence>, Without<Station>, Without<ActiveAsteroid>, Without<TutorialHighlight>),
    >,
    station_query: Query<(&Station, &StationQueues), (Without<Ship>, Without<AutonomousShipTag>)>,
    ast_query: Query<
        (&ActiveAsteroid, &Transform),
        (Without<Ship>, Without<Station>, Without<TutorialHighlight>),
    >,
    // T-101 to T-106: Phase 4a Echo tutorial
    auto_ship_query: Query<&Ship, (With<AutonomousShipTag>, Without<InOpeningSequence>)>,
    drawer_state: Res<DrawerState>,
    active_tab: Res<ActiveStationTab>,
    bottle_query: Query<&Transform, (With<ActiveBottle>, Without<TutorialHighlight>)>,
    mut highlight_query: Query<
        (&mut Transform, &mut Visibility),
        (With<TutorialHighlight>, Without<ActiveAsteroid>, Without<ActiveBottle>, Without<InOpeningSequence>),
    >,
) {
    // ── TutorialHighlight position update — runs every frame regardless of popup state ──
    if let Ok((mut h_transform, mut h_vis)) = highlight_query.get_single_mut() {
        let show_for_asteroid = opening.phase == OpeningPhase::Complete
            && !tutorial.shown.contains(&102);
        let show_for_bottle = tutorial.shown.contains(&105)
            && !tutorial.shown.contains(&106);

        if show_for_asteroid {
            if let Some((_, ast_t)) = ast_query.iter().min_by(|(_, a), (_, b)| {
                a.translation.length_squared()
                    .partial_cmp(&b.translation.length_squared())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }) {
                h_transform.translation.x = ast_t.translation.x;
                h_transform.translation.y = ast_t.translation.y;
                *h_vis = Visibility::Inherited;
            } else {
                *h_vis = Visibility::Hidden;
            }
        } else if show_for_bottle {
            if let Some(bottle_t) = bottle_query.iter().next() {
                h_transform.translation.x = bottle_t.translation.x;
                h_transform.translation.y = bottle_t.translation.y;
                *h_vis = Visibility::Inherited;
            } else {
                *h_vis = Visibility::Hidden;
            }
        } else {
            // Hide world-space highlight when showing drawer button highlight (rendered in HUD)
            *h_vis = Visibility::Hidden;
        }

        // Set drawer highlight flag for HUD rendering (show during T-103 popup)
        tutorial.show_drawer_highlight = tutorial.shown.contains(&102)
            && !tutorial.shown.contains(&104);

        // Part C: Set pipeline highlight flag for HUD rendering (simplified - no save data check needed)
        tutorial.show_pipeline_highlight = tutorial.shown.contains(&105)
            && !tutorial.shown.contains(&107);
    }

    // Skip popup triggers while opening is incomplete or a popup is already visible
    if opening.phase != OpeningPhase::Complete || tutorial.active.is_some() {
        return;
    }

    // ── T-001 to T-006: legacy triggers (opening ship despawned at Complete — never fires) ──
    if let Ok((ship, ship_transform)) = opening_ship_query.get_single() {
        if !tutorial.shown.contains(&1) && ship.cargo >= 80.0 {
            tutorial.active = Some(TutorialPopup {
                id: 1,
                title: "CARGO HOLD FULL".to_string(),
                body: "Your ship is nearly at capacity. Return to the station and DOCK to unload cargo and begin processing.".to_string(),
                button_label: "UNDERSTOOD".to_string(),
            });
            return;
        }
        if !tutorial.shown.contains(&2) && ship.state == ShipState::Docked {
            tutorial.active = Some(TutorialPopup {
                id: 2,
                title: "STATION DOCKED".to_string(),
                body: "Ship cargo is automatically transferred to station reserves. Open the SMELTER tab to refine raw ore into usable components.".to_string(),
                button_label: "OPEN SMELTER".to_string(),
            });
            return;
        }
        if let Ok((st, queues)) = station_query.get_single() {
            let processing_active = queues.iron_refinery.is_some()
                || queues.tungsten_refinery.is_some()
                || queues.nickel_refinery.is_some()
                || queues.hull_forge.is_some()
                || queues.core_fabricator.is_some();
            if !tutorial.shown.contains(&3) && processing_active {
                tutorial.active = Some(TutorialPopup {
                    id: 3,
                    title: "PROCESSING STARTED".to_string(),
                    body: "Queues convert raw ore over time. Stay docked to monitor progress, or return to space; the station processes materials even while you're away.".to_string(),
                    button_label: "CONTINUE".to_string(),
                });
                return;
            }
            if !tutorial.shown.contains(&4) && st.iron_reserves >= 10.0 {
                tutorial.active = Some(TutorialPopup {
                    id: 4,
                    title: "MATERIALS READY".to_string(),
                    body: "You've gathered enough materials to begin station repairs. Go to the CARGO tab and click REPAIR to restore station functionality.".to_string(),
                    button_label: "ONWARD".to_string(),
                });
                return;
            }
            if !tutorial.shown.contains(&5) && st.online {
                tutorial.active = Some(TutorialPopup {
                    id: 5,
                    title: "STATION ONLINE".to_string(),
                    body: "Power is restored. Automation is now available for drone manufacturing and advanced equipment upgrades. Keep the juice flowing!".to_string(),
                    button_label: "EXCELLENT".to_string(),
                });
                return;
            }
        }
        if !tutorial.shown.contains(&6) {
            if let Some((_, ast_transform)) = ast_query.iter().find(|(a, _)| a.ore_type == OreDeposit::Tungsten) {
                if ship_transform.translation.truncate().distance(ast_transform.translation.truncate()) < 150.0 {
                    tutorial.active = Some(TutorialPopup {
                        id: 6,
                        title: "EXTRACTION BLOCKED".to_string(),
                        body: "This asteroid is too dense for your current mining laser. You need a TUNGSTEN LASER, available in the EQUIPMENT tab once you have the parts.".to_string(),
                        button_label: "COPY THAT".to_string(),
                    });
                    return;
                }
            }
        }
    }

    // ── T-101 to T-106: Phase 4a tutorial — Echo voice (YAML-driven) ───────────────
    for step in &tutorial_cfg.steps {
        if tutorial.shown.contains(&step.id) {
            continue;
        }

        // Check prerequisites
        if !step.requires.iter().all(|req_id| tutorial.shown.contains(req_id)) {
            continue;
        }

        // Evaluate trigger condition
        if !evaluate_trigger(&step.trigger, &auto_ship_query, &station_query, &drawer_state, &active_tab, &bottle_query, &tutorial) {
            continue;
        }

        // Fire popup
        tutorial.active = Some(TutorialPopup {
            id: step.id,
            title: step.popup.title.clone(),
            body: step.popup.body.clone(),
            button_label: step.popup.button.clone(),
        });
        return;
    }
}

fn evaluate_trigger(
    trigger: &str,
    auto_ship_query: &Query<&Ship, (With<AutonomousShipTag>, Without<InOpeningSequence>)>,
    station_query: &Query<(&Station, &StationQueues), (Without<Ship>, Without<AutonomousShipTag>)>,
    drawer_state: &Res<DrawerState>,
    active_tab: &Res<ActiveStationTab>,
    bottle_query: &Query<&Transform, (With<ActiveBottle>, Without<TutorialHighlight>)>,
    tutorial_state: &TutorialState,
) -> bool {
    match trigger {
        "game_start" => true, // Always fires when opening Complete (guard at line 60)
        "drone_navigating_or_mining" => auto_ship_query.iter().any(|s| {
            s.state == ShipState::Navigating || s.state == ShipState::Mining
        }),
        "ore_reserves_positive" => {
            if let Ok((st, _)) = station_query.get_single() {
                st.iron_reserves > 0.0
                    || st.tungsten_reserves > 0.0
                    || st.nickel_reserves > 0.0
                    || st.aluminum_reserves > 0.0
            } else {
                false
            }
        }
        "drawer_expanded" => **drawer_state == DrawerState::Expanded,
        "forge_tab_active" => **active_tab == ActiveStationTab::Production,
        "bottle_exists" => bottle_query.iter().next().is_some(),
        // Part C: Pipeline discovery nudge trigger
        "drone_built_and_pipeline_never_opened" => {
            // Check if at least one drone has been built
            if let Ok((st, queues)) = station_query.get_single() {
                let drone_built = queues.hull_forge.is_some()
                    || queues.core_fabricator.is_some();
                // Check if pipeline nudge has never been shown
                let nudge_not_shown = !tutorial_state.shown.contains(&107);
                drone_built && nudge_not_shown
            } else {
                false
            }
        }
        _ => false,
    }
}
