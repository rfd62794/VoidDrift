use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct MiningConfig {
    pub ship_speed: f32,
    pub cargo_capacity: u32,
    pub mining_rate: f32,
    pub arrival_threshold: f32,
    pub arrival_threshold_mining: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RefineryConfig {
    pub ratio: u32,
    pub iron_time: f32,
    pub tungsten_time: f32,
    pub nickel_time: f32,
    pub aluminum_time: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ForgeConfig {
    pub hull_time: f32,
    pub thruster_time: f32,
    pub core_time: f32,
    pub aluminum_canister_time: f32,
    pub hull_plate_cost_iron: u32,
    pub thruster_cost_tungsten: u32,
    pub ai_core_cost_nickel: u32,
    pub aluminum_canister_cost_aluminum: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DroneConfig {
    pub build_time: f32,
    pub cost_hulls: f32,
    pub cost_thrusters: f32,
    pub cost_cores: f32,
    pub max_queue: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AsteroidConfig {
    pub base_ore: f32,
    pub max_lifespan_secs: f32,
    pub respawn_timer_secs: f32,
    pub min_spawn_distance: f32,
    pub radius_iron: f32,
    pub radius_tungsten: f32,
    pub radius_nickel: f32,
    pub radius_aluminum: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct StationConfig {
    pub repair_cost: u32,
    pub rotation_speed_divisor: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NarrativeConfig {
    pub bottle_spawn_delay: f32,
    pub signal_pause_s2: f32,
    pub signal_pause_dock_report: f32,
    pub signal_pause_complete: f32,
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct BalanceConfig {
    pub mining: MiningConfig,
    pub refinery: RefineryConfig,
    pub forge: ForgeConfig,
    pub drone: DroneConfig,
    pub asteroid: AsteroidConfig,
    pub station: StationConfig,
    pub narrative: NarrativeConfig,
}

impl BalanceConfig {
    pub fn load() -> Self {
        let src = include_str!("../../assets/balance.toml");
        toml::from_str(src).expect("Failed to parse assets/balance.toml")
    }
}
