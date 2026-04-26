use bevy::prelude::*;
use crate::components::{OreDeposit, LaserTier};

pub fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Iron     => "IRON",
        OreDeposit::Tungsten => "TUNGSTEN",
        OreDeposit::Nickel   => "NICKEL",
    }
}

pub fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Iron     => LaserTier::Basic,
        OreDeposit::Tungsten => LaserTier::Basic,
        OreDeposit::Nickel   => LaserTier::Basic,
    }
}

pub fn berth_world_pos(
    station_pos: Vec2,
    station_rotation: f32,
    arm_index: u8,
) -> Vec2 {
    let arm_angle = station_rotation + (arm_index as f32 * std::f32::consts::TAU / 6.0);
    station_pos + Vec2::new(
        arm_angle.cos() * crate::constants::STATION_ARM_LENGTH,
        arm_angle.sin() * crate::constants::STATION_ARM_LENGTH,
    )
}
