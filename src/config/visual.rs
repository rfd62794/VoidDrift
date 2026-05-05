use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct StarfieldConfig {
    pub far_count: u32,
    pub near_count: u32,
    pub far_size: f32,
    pub near_size: f32,
    pub far_parallax: f32,
    pub near_parallax: f32,
    pub radius: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AsteroidVisualConfig {
    pub color_iron: [f32; 3],
    pub color_tungsten: [f32; 3],
    pub color_nickel: [f32; 3],
    pub color_aluminum: [f32; 3],
    pub color_depleted: [f32; 3],
}

#[derive(Deserialize, Clone, Debug)]
pub struct BottleVisualConfig {
    pub width: f32,
    pub height: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub hit_radius: f32,
    pub color: [f32; 3],
}

#[derive(Deserialize, Clone, Debug)]
pub struct StationVisualConfig {
    pub hub_radius: f32,
    pub arm_length: f32,
    pub arm_thickness: f32,
    pub berth_radius: f32,
    pub stub_length: f32,
    pub berth_initial: u8,
    pub color_hub_offline: [f32; 3],
    pub color_hub_online: [f32; 3],
    pub color_arm_active: [f32; 3],
    pub color_arm_stub: [f32; 3],
    pub color_berth_empty: [f32; 3],
    pub color_berth_player: [f32; 3],
    pub color_berth_drone: [f32; 3],
    pub dock_slowdown_distance: f32,
    pub slowdown_rate_multiplier: f32,
    pub resume_delay: f32,
    pub resume_rate_multiplier: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DroneVisualEntry {
    pub hull_w: f32,
    pub hull_h: f32,
    pub color_hull: [f32; 3],
    pub color_beam: [f32; 3],
    pub beam_alpha: f32,
    pub color_thruster: [f32; 3],
    pub color_cargo_fill: [f32; 3],
    pub color_cargo_bg: [f32; 3],
    pub cargo_bar_w: f32,
    pub cargo_bar_h: f32,
    pub map_icon_w: f32,
    pub map_icon_h: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DroneVisualConfig {
    pub opening: DroneVisualEntry,
    pub mission: DroneVisualEntry,
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct VisualConfig {
    pub starfield: StarfieldConfig,
    pub asteroid: AsteroidVisualConfig,
    pub bottle: BottleVisualConfig,
    pub station: StationVisualConfig,
    pub drone: DroneVisualConfig,
}

impl VisualConfig {
    pub fn load() -> Self {
        let src = Self::read_toml();
        toml::from_str(src).expect("Failed to parse assets/visual.toml")
    }

    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    fn read_toml() -> &'static str {
        include_str!("../../assets/visual.toml")
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn read_toml() -> &'static str {
        Box::leak(
            std::fs::read_to_string("assets/visual.toml")
                .expect("Failed to read assets/visual.toml")
                .into_boxed_str(),
        )
    }
}

pub fn rgb(c: [f32; 3]) -> bevy::prelude::Color {
    bevy::prelude::Color::srgb(c[0], c[1], c[2])
}

pub fn rgba(c: [f32; 3], a: f32) -> bevy::prelude::Color {
    bevy::prelude::Color::srgba(c[0], c[1], c[2], a)
}
