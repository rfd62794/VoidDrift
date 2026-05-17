pub mod fleet;
pub mod fsm;
pub mod dispatch;
pub mod economy;
pub mod invariants;

use bevy::prelude::*;
use crate::components::*;
use crate::config::BalanceConfig;

pub fn minimal_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_resource::<FleetCount>();
    app
}

pub fn test_balance_config() -> BalanceConfig {
    BalanceConfig::load()
}

pub fn test_station() -> Station {
    Station {
        repair_progress: 0.0,
        online: true,
        iron_reserves: 0.0,
        iron_ingots: 0.0,
        tungsten_reserves: 0.0,
        tungsten_ingots: 0.0,
        nickel_reserves: 0.0,
        nickel_ingots: 0.0,
        aluminum_reserves: 0.0,
        aluminum_ingots: 0.0,
        aluminum_canisters: 0.0,
        hull_plate_reserves: 0.0,
        thruster_reserves: 0.0,
        ai_cores: 0.0,
        drone_build_progress: 0.0,
        drone_count: 0,
        log: std::collections::VecDeque::new(),
        rotation: 0.0,
        rotation_speed: 0.0,
        dock_state: StationDockState::Rotating,
        resume_timer: 0.0,
        cargo_capacity_multiplier: 1.0,
        ship_speed_multiplier: 1.0,
        power_multiplier: 1.0,
        max_dispatch: 5,
        max_active_asteroids: 6,
    }
}
