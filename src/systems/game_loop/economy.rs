use bevy::prelude::*;
use crate::components::*;
use crate::config::VisualConfig;
use crate::systems::persistence::save::AutosaveEvent;

pub fn ship_docked_economy_system(
    mut cargo_events: EventReader<ShipDockedWithCargo>,
    mut bottle_events: EventReader<ShipDockedWithBottle>,
    mut fulfill_events: EventReader<FulfillRequestEvent>,
    mut repair_events: EventReader<RepairStationEvent>,
    mut dispatch_events: EventReader<DroneDispatched>,
    mut station_query: Query<&mut Station>,
    mut requests_tab: ResMut<RequestsTabState>,
    mut queue: ResMut<ShipQueue>,
    mut autosave_events: EventWriter<AutosaveEvent>,
    mut commands: Commands,
    vcfg: Res<VisualConfig>,
) {
    for event in cargo_events.read() {
        if let Ok(mut station) = station_query.get_single_mut() {
            match event.ore_type {
                OreDeposit::Iron     => station.iron_reserves     += event.amount,
                OreDeposit::Tungsten => station.tungsten_reserves += event.amount,
                OreDeposit::Nickel   => station.nickel_reserves   += event.amount,
                OreDeposit::Aluminum => station.aluminum_reserves += event.amount,
            }
            station.dock_state = StationDockState::Resuming;
            station.resume_timer = vcfg.station.resume_delay;
        }
        if event.despawn {
            queue.available_count += 1;
            info!("[Voidrift] Ship docked & unloaded. Queue: {}", queue.available_count);
            autosave_events.send(AutosaveEvent);
            commands.entity(event.ship_entity).despawn_recursive();
        } else {
            info!("[Voidrift] Autonomous ship unloaded. Returning to cycle.");
            autosave_events.send(AutosaveEvent);
        }
    }

    for event in bottle_events.read() {
        queue.available_count += 1;
        info!("[Voidrift] Bottle carrier docked. Queue: {}", queue.available_count);
        autosave_events.send(AutosaveEvent);
        commands.entity(event.ship_entity).despawn_recursive();
    }

    for event in fulfill_events.read() {
        if let Ok(mut station) = station_query.get_single_mut() {
            if let Some(req) = requests_tab.collected_requests.iter_mut().find(|r| r.id == event.request_id && r.faction == event.faction_id && !r.fulfilled) {
                station.iron_ingots -= 25.0;
                station.power_multiplier += 0.25;
                req.fulfilled = true;
            }
        }
    }

    for _event in repair_events.read() {
        if let Ok(mut station) = station_query.get_single_mut() {
            station.repair_progress = 1.0;
            station.online = true;
        }
    }

    for _ in dispatch_events.read() {
        if queue.available_count > 0 {
            queue.available_count -= 1;
            info!("[Voidrift] Drone dispatched. Queue: {}", queue.available_count);
        }
    }
}
