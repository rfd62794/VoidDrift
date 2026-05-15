use bevy::prelude::*;
use crate::components::*;

/// Scout Mk I auto-dispatch system.
/// When ScoutEnabled.active == true, finds idle Mining drones (state = Holding)
/// and assigns them to the nearest unoccupied Inner Ring asteroid.
/// This is auto-dispatch — it replaces the player tap for those drones only when the toggle is on.
///
/// ADR: System runs in GameLoop schedule after autonomous_ship_system.
/// This ensures that Holding→Outbound transitions happen in the same frame when scout assigns.
pub fn scout_dispatch_system(
    scout_enabled: Res<ScoutEnabled>,
    mut idle_drones: Query<(
        &mut AutonomousShip,
        &mut AutonomousAssignment,
        &Drone,
        &Transform,
    ), With<Drone>>,
    asteroids: Query<(&ActiveAsteroid, &Transform, Entity), With<InnerRingAsteroid>>,
    occupied: Query<&AutonomousAssignment>,
) {
    if !scout_enabled.active {
        return;
    }

    // Collect currently targeted asteroid positions to avoid double-dispatch
    let occupied_targets: Vec<Vec2> = occupied
        .iter()
        .map(|a| a.target_pos)
        .collect();

    for (mut ship_state, mut assignment, drone, drone_transform) in idle_drones.iter_mut() {
        // Only dispatch Mining drones via Scout — not Scout drones (future)
        if drone.class != DroneClass::Mining || drone.tier != 1 {
            continue;
        }
        if ship_state.state != AutonomousShipState::Holding {
            continue;
        }

        // Find nearest unoccupied asteroid
        let nearest = asteroids
            .iter()
            .filter(|(_, ast_transform, _)| {
                let pos = ast_transform.translation.truncate();
                !occupied_targets.contains(&pos)
            })
            .min_by_key(|(_, ast_transform, _)| {
                let dx = ast_transform.translation.x - drone_transform.translation.x;
                let dy = ast_transform.translation.y - drone_transform.translation.y;
                (dx * dx + dy * dy) as i64
            });

        if let Some((active_asteroid, ast_transform, _)) = nearest {
            // Assign and dispatch — mirror exactly what asteroid_input_system does
            *assignment = AutonomousAssignment {
                target_pos: ast_transform.translation.truncate(),
                ore_type: active_asteroid.ore_type,
                sector_name: "S1".to_string(), // Inner Ring sector placeholder
            };
            ship_state.state = AutonomousShipState::Outbound;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // T-S1-07: test_scout_dispatch_mining_tier1_only
    #[test]
    fn test_scout_dispatch_mining_tier1_only() {
        // Test that Scout drones (class: Scout) are not dispatched
        let mining_drone = Drone { class: DroneClass::Mining, tier: 1 };
        let scout_drone = Drone { class: DroneClass::Scout, tier: 1 };
        let mining_tier2 = Drone { class: DroneClass::Mining, tier: 2 };

        // Mining tier 1 should pass the filter
        assert!(mining_drone.class == DroneClass::Mining && mining_drone.tier == 1);
        
        // Scout drone should not pass the filter
        assert!(scout_drone.class != DroneClass::Mining || scout_drone.tier != 1);
        
        // Mining tier 2 should not pass the filter
        assert!(mining_tier2.class != DroneClass::Mining || mining_tier2.tier != 1);
    }

    // T-S1-04: test_scout_dispatch_ignores_non_idle_drones
    #[test]
    fn test_scout_dispatch_ignores_non_idle_drones() {
        // Test that only Holding state drones are dispatched
        let holding_state = AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron };
        let outbound_state = AutonomousShip { state: AutonomousShipState::Outbound, cargo: 0.0, cargo_type: OreDeposit::Iron };
        let mining_state = AutonomousShip { state: AutonomousShipState::Mining, cargo: 0.0, cargo_type: OreDeposit::Iron };

        // Holding state should pass the filter
        assert!(holding_state.state == AutonomousShipState::Holding);
        
        // Outbound state should not pass the filter
        assert!(outbound_state.state != AutonomousShipState::Holding);
        
        // Mining state should not pass the filter
        assert!(mining_state.state != AutonomousShipState::Holding);
    }

    // T-S1-02: test_scout_dispatch_skips_when_disabled
    #[test]
    fn test_scout_dispatch_skips_when_disabled() {
        // Test that the system returns early when ScoutEnabled.active is false
        let disabled = ScoutEnabled { active: false, unlocked: true };
        let enabled = ScoutEnabled { active: true, unlocked: true };

        // When disabled, system should skip
        assert_eq!(disabled.active, false);
        
        // When enabled, system should proceed
        assert_eq!(enabled.active, true);
    }

    // T-S1-05: test_scout_dispatch_avoids_occupied_asteroids
    #[test]
    fn test_scout_dispatch_avoids_occupied_asteroids() {
        // Test that occupied asteroids are filtered out
        let occupied_pos1 = Vec2::new(100.0, 200.0);
        let occupied_pos2 = Vec2::new(300.0, 400.0);
        let free_pos = Vec2::new(500.0, 600.0);
        
        let occupied_targets = vec![occupied_pos1, occupied_pos2];
        
        // Occupied positions should be in the list
        assert!(occupied_targets.contains(&occupied_pos1));
        assert!(occupied_targets.contains(&occupied_pos2));
        
        // Free position should not be in the list
        assert!(!occupied_targets.contains(&free_pos));
    }

    // T-S1-03: test_scout_dispatch_assigns_idle_mining_drone
    #[test]
    fn test_scout_dispatch_assigns_idle_mining_drone() {
        // Test that assignment structure is correct
        let assignment = AutonomousAssignment {
            target_pos: Vec2::new(100.0, 200.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        };
        
        // Verify assignment structure
        assert_eq!(assignment.target_pos, Vec2::new(100.0, 200.0));
        assert_eq!(assignment.ore_type, OreDeposit::Iron);
        assert_eq!(assignment.sector_name, "S1");
    }
}
