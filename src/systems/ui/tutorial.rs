use bevy::prelude::*;
use crate::components::*;

pub fn tutorial_system(
    mut tutorial: ResMut<TutorialState>,
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
            *h_vis = Visibility::Hidden;
        }
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

    // ── T-101 to T-106: Phase 4a tutorial — Echo voice ───────────────────────────────────

    // T-101: Game start — tap the highlighted asteroid
    if !tutorial.shown.contains(&101) {
        tutorial.active = Some(TutorialPopup {
            id: 101,
            title: "ECHO".into(),
            body: "Mining protocols active. Tap the highlighted asteroid to dispatch a drone.".into(),
            button_label: "Understood".into(),
        });
        return;
    }

    // T-102: Drone dispatched — first navigation detected
    if !tutorial.shown.contains(&102) {
        if auto_ship_query.iter().any(|s| {
            s.state == ShipState::Navigating || s.state == ShipState::Mining
        }) {
            tutorial.active = Some(TutorialPopup {
                id: 102,
                title: "ECHO".into(),
                body: "Drone en route. It will return automatically when cargo is full.".into(),
                button_label: "Understood".into(),
            });
            return;
        }
    }

    // T-103: First dock — ore unloaded at station
    if !tutorial.shown.contains(&103) && tutorial.shown.contains(&102) {
        if let Ok((st, _)) = station_query.get_single() {
            if st.iron_reserves > 0.0
                || st.tungsten_reserves > 0.0
                || st.nickel_reserves > 0.0
                || st.aluminum_reserves > 0.0
            {
                tutorial.active = Some(TutorialPopup {
                    id: 103,
                    title: "ECHO".into(),
                    body: "Ore secured. Open the station drawer to check reserves. Tap the grey bar at the bottom of the screen.".into(),
                    button_label: "Understood".into(),
                });
                return;
            }
        }
    }

    // T-104: Drawer opened
    if !tutorial.shown.contains(&104) && tutorial.shown.contains(&103) {
        if *drawer_state == DrawerState::Expanded {
            tutorial.active = Some(TutorialPopup {
                id: 104,
                title: "ECHO".into(),
                body: "Production pipeline standing by. Switch to the FORGE tab to enable automatic processing.".into(),
                button_label: "Understood".into(),
            });
            return;
        }
    }

    // T-105: FORGE tab opened
    if !tutorial.shown.contains(&105) && tutorial.shown.contains(&104) {
        if *active_tab == ActiveStationTab::Production {
            tutorial.active = Some(TutorialPopup {
                id: 105,
                title: "ECHO".into(),
                body: "Refinery online. Materials will process automatically. Return to mining — more ore means more drones.".into(),
                button_label: "Understood".into(),
            });
            return;
        }
    }

    // T-106: Bottle visible in world
    if !tutorial.shown.contains(&106) && tutorial.shown.contains(&105) {
        if bottle_query.iter().next().is_some() {
            tutorial.active = Some(TutorialPopup {
                id: 106,
                title: "ECHO".into(),
                body: "Signal detected. Dispatch a drone to retrieve it. Check the REQUESTS tab in the drawer after collection.".into(),
                button_label: "Understood".into(),
            });
        }
    }
}
