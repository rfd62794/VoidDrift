use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn auto_refine_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    toggles: Res<ProductionToggles>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        let dt = time.delta_secs();

        // Process Iron
        if toggles.refine_iron {
            let iron_batches = (1.0 / REFINERY_IRON_TIME) * dt;
            let iron_ore_needed = iron_batches * REFINERY_RATIO as f32;
            let actual_iron_ore = iron_ore_needed.min(station.iron_reserves);
            station.iron_reserves -= actual_iron_ore;
            station.iron_ingots += actual_iron_ore / REFINERY_RATIO as f32;
        }

        // Process Tungsten
        if toggles.refine_tungsten {
            let tungsten_batches = (1.0 / REFINERY_TUNGSTEN_TIME) * dt;
            let tungsten_ore_needed = tungsten_batches * REFINERY_RATIO as f32;
            let actual_tungsten_ore = tungsten_ore_needed.min(station.tungsten_reserves);
            station.tungsten_reserves -= actual_tungsten_ore;
            station.tungsten_ingots += actual_tungsten_ore / REFINERY_RATIO as f32;
        }

        // Process Nickel
        if toggles.refine_nickel {
            let nickel_batches = (1.0 / REFINERY_NICKEL_TIME) * dt;
            let nickel_ore_needed = nickel_batches * REFINERY_RATIO as f32;
            let actual_nickel_ore = nickel_ore_needed.min(station.nickel_reserves);
            station.nickel_reserves -= actual_nickel_ore;
            station.nickel_ingots += actual_nickel_ore / REFINERY_RATIO as f32;
        }
    }
}

pub fn auto_forge_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    toggles: Res<ProductionToggles>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        let dt = time.delta_secs();

        // Forge Hull Plates (Requires Iron Ingots ONLY)
        if toggles.forge_hull {
            let hull_batches = (1.0 / FORGE_HULL_TIME) * dt;
            let max_iron = station.iron_ingots / HULL_PLATE_COST_IRON as f32;
            let actual_hull_batches = hull_batches.min(max_iron);
            station.iron_ingots -= actual_hull_batches * HULL_PLATE_COST_IRON as f32;
            station.hull_plate_reserves += actual_hull_batches;
        }

        // Forge Thrusters (Requires Tungsten Ingots)
        if toggles.forge_thruster {
            let thruster_batches = (1.0 / FORGE_THRUSTER_TIME) * dt;
            let max_tungsten = station.tungsten_ingots / THRUSTER_COST_TUNGSTEN as f32;
            let actual_thruster_batches = thruster_batches.min(max_tungsten);
            station.tungsten_ingots -= actual_thruster_batches * THRUSTER_COST_TUNGSTEN as f32;
            station.thruster_reserves += actual_thruster_batches;
        }

        // Forge AI Cores (Requires Nickel Ingots)
        if toggles.forge_core {
            let core_batches = (1.0 / FORGE_CORE_TIME) * dt;
            let max_nickel = station.nickel_ingots / AI_CORE_COST_NICKEL as f32;
            let actual_core_batches = core_batches.min(max_nickel);
            station.nickel_ingots -= actual_core_batches * AI_CORE_COST_NICKEL as f32;
            station.ai_cores += actual_core_batches;
        }
    }
}

/// Continuously assembles drones from hull plates, thrusters, and AI cores.
/// Uses a fractional accumulator — progress ticks each frame, whole drones
/// are completed and added to the ShipQueue pool when progress >= 1.0.
pub fn auto_build_drones_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    toggles: Res<ProductionToggles>,
    mut queue: ResMut<ShipQueue>,
) {
    if !toggles.build_drones { return; }
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }

        // Don't build beyond the cap
        if queue.available_count >= MAX_DRONE_QUEUE { return; }

        // Check if we have at least partial resources to begin building
        let has_resources =
            station.hull_plate_reserves >= DRONE_BUILD_COST_HULLS
            && station.thruster_reserves >= DRONE_BUILD_COST_THRUSTERS
            && station.ai_cores >= DRONE_BUILD_COST_CORES;

        if !has_resources {
            // Stall — don't tick progress without materials
            return;
        }

        let dt = time.delta_secs();
        station.drone_build_progress += dt / DRONE_BUILD_TIME;

        // Each time progress crosses a whole number, complete one drone
        if station.drone_build_progress >= 1.0 {
            let built = station.drone_build_progress.floor() as u32;
            // Clamp to what we can actually afford and to the cap
            let affordable = (
                (station.hull_plate_reserves / DRONE_BUILD_COST_HULLS).floor() as u32
            ).min(
                (station.thruster_reserves / DRONE_BUILD_COST_THRUSTERS).floor() as u32
            ).min(
                (station.ai_cores / DRONE_BUILD_COST_CORES).floor() as u32
            );
            let space_in_queue = MAX_DRONE_QUEUE.saturating_sub(queue.available_count);
            let actual = built.min(affordable).min(space_in_queue);

            if actual > 0 {
                station.hull_plate_reserves -= actual as f32 * DRONE_BUILD_COST_HULLS;
                station.thruster_reserves   -= actual as f32 * DRONE_BUILD_COST_THRUSTERS;
                station.ai_cores            -= actual as f32 * DRONE_BUILD_COST_CORES;
                queue.available_count += actual;
                // Bump max_drones if queue exceeds it
                station.max_drones = station.max_drones.max(queue.available_count);
                info!("[Voidrift] Drone assembly complete: {} built. Queue: {}", actual, queue.available_count);
            }

            station.drone_build_progress -= station.drone_build_progress.floor();
        }
    }
}
