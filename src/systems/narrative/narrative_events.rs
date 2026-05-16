use bevy::prelude::*;
use crate::components::*;
use crate::config::VisualConfig;

pub fn narrative_event_system(
    mut bottle_events: EventReader<ShipDockedWithBottle>,
    mut opening_events: EventReader<OpeningCompleteEvent>,
    mut laser_events: EventReader<InsufficientLaserEvent>,
    mut signal_log: ResMut<SignalLog>,
    mut requests_tab: ResMut<RequestsTabState>,
    mut queue: ResMut<ShipQueue>,
    mut station_query: Query<&mut Station>,
    mut ship_query: Query<(&mut Transform, &mut Ship)>,
    mut commands: Commands,
    vcfg: Res<VisualConfig>,
) {
    for _event in laser_events.read() {
        signal_log.entries.push_front("> INSUFFICIENT LASER RATING. UPGRADE REQUIRED.".to_string());
        if signal_log.entries.len() > 10 {
            signal_log.entries.pop_front();
        }
    }

    for _event in bottle_events.read() {
        info!("CarryingBottle unload branch reached");
        signal_log.entries.push_back("SIGNAL RECEIVED — ORIGIN UNKNOWN\nFrequency matched. You were expected.\nWe have observed your work. It is... acceptable.\nA proposal follows.".to_string());
        if signal_log.entries.len() > 10 {
            signal_log.entries.pop_front();
        }
        let already_collected = requests_tab.collected_requests.iter().any(|r| r.id == RequestId::FirstLight);
        if !already_collected {
            requests_tab.collected_requests.push(CollectedRequest {
                id: RequestId::FirstLight,
                faction: FactionId::Signal,
                fulfilled: false,
            });
        }
        // CarryingBottle component is removed automatically when the ship entity
        // is despawned by ship_docked_economy_system — no explicit remove needed.
    }

    for event in opening_events.read() {
        queue.available_count += 1;
        if let Ok(mut station) = station_query.get_single_mut() {
            station.dock_state = StationDockState::Resuming;
            station.resume_timer = vcfg.station.resume_delay;
        }

        // Transform the opening ship into a Mining drone
        let ship_entity = event.ship_entity;
        let station_pos = if let Ok((ship_transform, _ship)) = ship_query.get(ship_entity) {
            ship_transform.translation.truncate()
        } else {
            Vec2::ZERO
        };

        commands.entity(ship_entity).insert((
            Drone { class: DroneClass::Mining, tier: 1 },
            AutonomousShip {
                state: AutonomousShipState::Holding,
                cargo: 0.0,
                cargo_type: OreDeposit::Iron,
            },
            AutonomousAssignment {
                target_pos: station_pos,
                ore_type: OreDeposit::Iron,
                sector_name: "S1".to_string(),
            },
        ));

        commands.entity(ship_entity).remove::<InOpeningSequence>();

        // Explicitly remove DroneTarget if present (shouldn't be, but kills it at source)
        commands.entity(ship_entity).remove::<DroneTarget>();
        info!("[Voidrift] DroneTarget remove called on opening ship entity");

        info!("[Voidrift] OpeningCompleteEvent received. Ship transformed into Mining drone. Queue: {}", queue.available_count);
    }
}
