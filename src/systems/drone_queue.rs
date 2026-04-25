use bevy::prelude::*;
use crate::components::*;

pub fn ship_queue_system(
    mut queue: ResMut<ShipQueue>,
    ships: Query<&Ship>,
) {
    // If currently assigned ships are done mining, return them to queue
    let mut to_make_available = Vec::new();
    queue.active_ships.retain(|&ship_entity| {
        if let Ok(ship) = ships.get(ship_entity) {
            if ship.state == ShipState::Docked {
                // Ship docked, return to queue
                to_make_available.push(ship_entity);
                return false; // remove from active_ships
            }
            true // keep in active_ships
        } else {
            false // ship despawned
        }
    });
    
    for entity in to_make_available {
        queue.available_ships.push(entity);
    }
}
