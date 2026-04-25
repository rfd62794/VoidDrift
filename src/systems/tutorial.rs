use bevy::prelude::*;
use crate::components::*;

pub fn tutorial_system(
    mut tutorial: ResMut<TutorialState>,
    opening: Res<OpeningSequence>,
    ship_query: Query<(&Ship, &Transform), (With<PlayerShip>, Without<Station>, Without<AsteroidField>)>,
    station_query: Query<(&Station, &StationQueues), (Without<Ship>, Without<AutonomousShipTag>)>,
    ast_query: Query<(&AsteroidField, &Transform), (Without<Ship>, Without<Station>)>, 
) {
    // Skip all triggers during opening sequence or if a pop-up is currently active
    if opening.phase != OpeningPhase::Complete || tutorial.active.is_some() {
        return;
    }

    let Ok((ship, ship_transform)) = ship_query.get_single() else { return; };
    let Ok((st, queues)) = station_query.get_single() else { return; };

    // T-001: Cargo Hold Full (80% threshold)
    if !tutorial.shown.contains(&1) && ship.cargo >= 80.0 {
        tutorial.active = Some(TutorialPopup {
            id: 1,
            title: "CARGO HOLD FULL".to_string(),
            body: "Your ship is nearly at capacity. Return to the station and DOCK to unload cargo and begin processing.".to_string(),
            button_label: "UNDERSTOOD".to_string(),
        });
        return;
    }

    // T-002: Station Docked (First time)
    if !tutorial.shown.contains(&2) && ship.state == ShipState::Docked {
        tutorial.active = Some(TutorialPopup {
            id: 2,
            title: "STATION DOCKED".to_string(),
            body: "Ship cargo is automatically transferred to station reserves. Open the SMELTER tab to refine raw ore into usable components.".to_string(),
            button_label: "OPEN SMELTER".to_string(),
        });
        return;
    }

    // T-003: Processing Started (First queue activity)
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

    // T-004: Materials Ready (REPAIR threshold)
    if !tutorial.shown.contains(&4) && st.iron_reserves >= 10.0 {
        tutorial.active = Some(TutorialPopup {
            id: 4,
            title: "MATERIALS READY".to_string(),
            body: "You've gathered enough materials to begin station repairs. Go to the CARGO tab and click REPAIR to restore station functionality.".to_string(),
            button_label: "ONWARD".to_string(),
        });
        return;
    }

    // T-005: Station Online
    if !tutorial.shown.contains(&5) && st.online {
        tutorial.active = Some(TutorialPopup {
            id: 5,
            title: "STATION ONLINE".to_string(),
            body: "Power is restored. Automation is now available for drone manufacturing and advanced equipment upgrades. Keep the juice flowing!".to_string(),
            button_label: "EXCELLENT".to_string(),
        });
        return;
    }

    // T-006: Extraction Blocked (Proximity to gated asteroid)
    if !tutorial.shown.contains(&6) {
        if let Some((_, ast_transform)) = ast_query.iter().find(|(a, _)| a.ore_deposit == OreDeposit::Tungsten) {
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
