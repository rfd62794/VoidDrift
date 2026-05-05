use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::config::BalanceConfig;

pub fn auto_refine_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    toggles: Res<ProductionToggles>,
    cfg: Res<BalanceConfig>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        let dt = time.delta_secs();
        let ratio = cfg.refinery.ratio as f32;

        // Process Iron
        if toggles.refine_iron {
            let iron_batches = (1.0 / cfg.refinery.iron_time) * dt;
            let iron_ore_needed = iron_batches * ratio;
            let actual_iron_ore = iron_ore_needed.min(station.iron_reserves);
            station.iron_reserves -= actual_iron_ore;
            station.iron_ingots += actual_iron_ore / ratio;
        }

        // Process Tungsten
        if toggles.refine_tungsten {
            let tungsten_batches = (1.0 / cfg.refinery.tungsten_time) * dt;
            let tungsten_ore_needed = tungsten_batches * ratio;
            let actual_tungsten_ore = tungsten_ore_needed.min(station.tungsten_reserves);
            station.tungsten_reserves -= actual_tungsten_ore;
            station.tungsten_ingots += actual_tungsten_ore / ratio;
        }

        // Process Nickel
        if toggles.refine_nickel {
            let nickel_batches = (1.0 / cfg.refinery.nickel_time) * dt;
            let nickel_ore_needed = nickel_batches * ratio;
            let actual_nickel_ore = nickel_ore_needed.min(station.nickel_reserves);
            station.nickel_reserves -= actual_nickel_ore;
            station.nickel_ingots += actual_nickel_ore / ratio;
        }
    }
}

pub fn auto_forge_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    toggles: Res<ProductionToggles>,
    cfg: Res<BalanceConfig>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }
        let dt = time.delta_secs();

        // Forge Hull Plates (Requires Iron Ingots ONLY)
        if toggles.forge_hull {
            let hull_cost = cfg.forge.hull_plate_cost_iron as f32;
            let hull_batches = (1.0 / cfg.forge.hull_time) * dt;
            let max_iron = station.iron_ingots / hull_cost;
            let actual_hull_batches = hull_batches.min(max_iron);
            station.iron_ingots -= actual_hull_batches * hull_cost;
            station.hull_plate_reserves += actual_hull_batches;
        }

        // Forge Thrusters (Requires Tungsten Ingots)
        if toggles.forge_thruster {
            let thruster_cost = cfg.forge.thruster_cost_tungsten as f32;
            let thruster_batches = (1.0 / cfg.forge.thruster_time) * dt;
            let max_tungsten = station.tungsten_ingots / thruster_cost;
            let actual_thruster_batches = thruster_batches.min(max_tungsten);
            station.tungsten_ingots -= actual_thruster_batches * thruster_cost;
            station.thruster_reserves += actual_thruster_batches;
        }

        // Forge AI Cores (Requires Nickel Ingots)
        if toggles.forge_core {
            let core_cost = cfg.forge.ai_core_cost_nickel as f32;
            let core_batches = (1.0 / cfg.forge.core_time) * dt;
            let max_nickel = station.nickel_ingots / core_cost;
            let actual_core_batches = core_batches.min(max_nickel);
            station.nickel_ingots -= actual_core_batches * core_cost;
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
    cfg: Res<BalanceConfig>,
) {
    if !toggles.build_drones { return; }
    if let Ok(mut station) = station_query.get_single_mut() {
        if !station.online { return; }

        // Don't build beyond the cap
        if queue.available_count >= cfg.drone.max_queue { return; }

        // Check if we have at least partial resources to begin building
        let has_resources =
            station.hull_plate_reserves >= cfg.drone.cost_hulls
            && station.thruster_reserves >= cfg.drone.cost_thrusters
            && station.ai_cores >= cfg.drone.cost_cores;

        if !has_resources {
            // Stall — don't tick progress without materials
            return;
        }

        let dt = time.delta_secs();
        station.drone_build_progress += dt / cfg.drone.build_time;

        // Each time progress crosses a whole number, complete one drone
        if station.drone_build_progress >= 1.0 {
            let built = station.drone_build_progress.floor() as u32;
            // Clamp to what we can actually afford and to the cap
            let affordable = (
                (station.hull_plate_reserves / cfg.drone.cost_hulls).floor() as u32
            ).min(
                (station.thruster_reserves / cfg.drone.cost_thrusters).floor() as u32
            ).min(
                (station.ai_cores / cfg.drone.cost_cores).floor() as u32
            );
            let space_in_queue = cfg.drone.max_queue.saturating_sub(queue.available_count);
            let actual = built.min(affordable).min(space_in_queue);

            if actual > 0 {
                station.hull_plate_reserves -= actual as f32 * cfg.drone.cost_hulls;
                station.thruster_reserves   -= actual as f32 * cfg.drone.cost_thrusters;
                station.ai_cores            -= actual as f32 * cfg.drone.cost_cores;
                queue.available_count += actual;
                info!("[Voidrift] Drone assembly complete: {} built. Queue: {}", actual, queue.available_count);
            }

            station.drone_build_progress -= station.drone_build_progress.floor();
        }
    }
}
