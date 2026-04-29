use bevy::prelude::*;
use crate::components::OreDeposit;
use crate::components::RequestId;
use crate::components::FactionId;

/// Fired by autopilot when a ship arrives at its berth carrying ore cargo.
#[derive(Event)]
pub struct ShipDockedWithCargo {
    pub ship_entity: Entity,
    pub ore_type: OreDeposit,
    pub amount: f32,
    /// If true, economy.rs will despawn the entity after unloading.
    /// Mission ships (autopilot): true. Autonomous ships (cycle): false.
    pub despawn: bool,
}

/// Fired by autopilot when a ship arrives at its berth carrying a bottle.
#[derive(Event)]
pub struct ShipDockedWithBottle {
    pub ship_entity: Entity,
}

/// Fired by content.rs when the player clicks FULFILL on a request card.
#[derive(Event)]
pub struct FulfillRequestEvent {
    pub request_id: RequestId,
    pub faction_id: FactionId,
}

/// Fired by content.rs when the player clicks REPAIR STATION.
#[derive(Event)]
pub struct RepairStationEvent;

/// Fired by opening_sequence_system when the cinematic ends and gameplay begins.
#[derive(Event)]
pub struct OpeningCompleteEvent;

/// Fired by input systems (asteroid_input, bottle_input) when a drone is dispatched.
/// economy.rs decrements queue.available_count on receive.
#[derive(Event)]
pub struct DroneDispatched;
