use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn auto_refine_system(time: Res<Time>, mut station_query: Query<&mut Station>) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        let dt = time.delta_secs();

        // Process Iron
        let iron_batches = (1.0 / REFINERY_IRON_TIME) * dt;
        let iron_ore_needed = iron_batches * REFINERY_RATIO as f32;
        let actual_iron_ore = iron_ore_needed.min(station.iron_reserves);
        station.iron_reserves -= actual_iron_ore;
        station.iron_ingots += actual_iron_ore / REFINERY_RATIO as f32;

        // Process Tungsten
        let tungsten_batches = (1.0 / REFINERY_TUNGSTEN_TIME) * dt;
        let tungsten_ore_needed = tungsten_batches * REFINERY_RATIO as f32;
        let actual_tungsten_ore = tungsten_ore_needed.min(station.tungsten_reserves);
        station.tungsten_reserves -= actual_tungsten_ore;
        station.tungsten_ingots += actual_tungsten_ore / REFINERY_RATIO as f32;

        // Process Nickel
        let nickel_batches = (1.0 / REFINERY_NICKEL_TIME) * dt;
        let nickel_ore_needed = nickel_batches * REFINERY_RATIO as f32;
        let actual_nickel_ore = nickel_ore_needed.min(station.nickel_reserves);
        station.nickel_reserves -= actual_nickel_ore;
        station.nickel_ingots += actual_nickel_ore / REFINERY_RATIO as f32;
    }
}

pub fn auto_forge_system(time: Res<Time>, mut station_query: Query<&mut Station>) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        let dt = time.delta_secs();

        // Forge Hull Plates (Requires Iron + Tungsten Ingots)
        let hull_batches = (1.0 / FORGE_HULL_TIME) * dt;
        let max_iron = station.iron_ingots / HULL_PLATE_COST_IRON as f32;
        let max_tungsten = station.tungsten_ingots / HULL_PLATE_COST_TUNGSTEN as f32;
        let actual_hull_batches = hull_batches.min(max_iron).min(max_tungsten);

        station.iron_ingots -= actual_hull_batches * HULL_PLATE_COST_IRON as f32;
        station.tungsten_ingots -= actual_hull_batches * HULL_PLATE_COST_TUNGSTEN as f32;
        station.hull_plate_reserves += actual_hull_batches;

        // Forge AI Cores (Requires Nickel Ingots)
        let core_batches = (1.0 / FORGE_CORE_TIME) * dt;
        let max_nickel = station.nickel_ingots / AI_CORE_COST_NICKEL as f32;
        let actual_core_batches = core_batches.min(max_nickel);

        station.nickel_ingots -= actual_core_batches * AI_CORE_COST_NICKEL as f32;
        station.ai_cores += actual_core_batches;
    }
}

pub fn auto_build_drones_system(time: Res<Time>, mut station_query: Query<&mut Station>) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        
        let dt = time.delta_secs();
        let drone_batches = (1.0 / 10.0) * dt; // 10s base build time
        let max_plates = station.hull_plate_reserves / SHIP_HULL_COST_PLATES as f32;
        let max_cores = station.ai_cores / 1.0; 
        let actual_drone_batches = drone_batches.min(max_plates).min(max_cores);

        station.hull_plate_reserves -= actual_drone_batches * SHIP_HULL_COST_PLATES as f32;
        station.ai_cores -= actual_drone_batches * 1.0;
        station.ship_hulls += actual_drone_batches;
    }
}
