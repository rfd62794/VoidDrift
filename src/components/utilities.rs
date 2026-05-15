use bevy::prelude::*;
use crate::components::{OreDeposit, LaserTier};
use crate::config::VisualConfig;

// Ore type to config key mapping (used for visual config lookup)
pub fn ore_config_key(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Iron => "metal",
        OreDeposit::Tungsten => "h3_gas",
        OreDeposit::Nickel => "void_essence",
        OreDeposit::Aluminum => "metal",
    }
}

pub fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Iron     => "IRON",
        OreDeposit::Tungsten => "TUNGSTEN",
        OreDeposit::Nickel   => "NICKEL",
        OreDeposit::Aluminum => "ALUMINUM",
    }
}

pub fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Iron     => LaserTier::Basic,
        OreDeposit::Tungsten => LaserTier::Basic,
        OreDeposit::Nickel   => LaserTier::Basic,
        OreDeposit::Aluminum => LaserTier::Basic,
    }
}

pub fn berth_world_pos(
    station_pos: Vec2,
    station_rotation: f32,
    arm_index: u8,
    vcfg: &VisualConfig,
) -> Vec2 {
    let arm_angle = station_rotation + (arm_index as f32 * std::f32::consts::TAU / 6.0);
    station_pos + Vec2::new(
        arm_angle.cos() * vcfg.station.arm_length,
        arm_angle.sin() * vcfg.station.arm_length,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test anchor for Issue #57: test_ore_deposit_lookup_consistent
    #[test]
    fn test_ore_deposit_lookup_consistent() {
        // Test ore_config_key
        assert_eq!(ore_config_key(&OreDeposit::Iron), "metal");
        assert_eq!(ore_config_key(&OreDeposit::Tungsten), "h3_gas");
        assert_eq!(ore_config_key(&OreDeposit::Nickel), "void_essence");
        assert_eq!(ore_config_key(&OreDeposit::Aluminum), "metal");

        // Test ore_name
        assert_eq!(ore_name(&OreDeposit::Iron), "IRON");
        assert_eq!(ore_name(&OreDeposit::Tungsten), "TUNGSTEN");
        assert_eq!(ore_name(&OreDeposit::Nickel), "NICKEL");
        assert_eq!(ore_name(&OreDeposit::Aluminum), "ALUMINUM");

        // Test ore_laser_required
        assert_eq!(ore_laser_required(&OreDeposit::Iron), LaserTier::Basic);
        assert_eq!(ore_laser_required(&OreDeposit::Tungsten), LaserTier::Basic);
        assert_eq!(ore_laser_required(&OreDeposit::Nickel), LaserTier::Basic);
        assert_eq!(ore_laser_required(&OreDeposit::Aluminum), LaserTier::Basic);
    }
}
