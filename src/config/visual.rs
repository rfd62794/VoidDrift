use serde::Deserialize;
use bevy_egui::egui;

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
pub struct OreTypeConfig {
    pub color_body: [u8; 3],
    pub color_vein: [u8; 3],
    pub band_count: usize,
    pub band_width_min: f32,
    pub band_width_max: f32,
    pub grain_angle_deg: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OreConfig {
    pub metal: OreTypeConfig,
    pub h3_gas: OreTypeConfig,
    pub void_essence: OreTypeConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AsteroidRingConfig {
    pub radius: f32,
    pub vertex_count: usize,
    pub jaggedness: f32,
    pub ore_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AsteroidRingsConfig {
    pub inner_ring: AsteroidRingConfig,
    pub middle_ring: AsteroidRingConfig,
    pub outer_ring: AsteroidRingConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OreNodeConfig {
    pub radius: f32,
    pub vertex_count: usize,
    pub jaggedness: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IngotNodeConfig {
    pub width: f32,
    pub height: f32,
    pub depth_offset_x: f32,
    pub depth_offset_y: f32,
    pub color_face_light_factor: f32,
    pub color_face_dark_factor: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ProductionTreeConfig {
    pub ore_node: OreNodeConfig,
    pub ingot_node: IngotNodeConfig,
    pub node_width: f32,
    pub node_height: f32,
    pub drone_bay_width: f32,
    pub drone_bay_height: f32,
    pub zoom_min: f32,
    pub zoom_max: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DepletionParticlesConfig {
    pub count: u32,
    pub speed_min: f32,
    pub speed_max: f32,
    pub lifetime_secs: f32,
    pub size_min: f32,
    pub size_max: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ParticlesConfig {
    pub depletion: DepletionParticlesConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IngotTypeConfig {
    pub color: [u8; 3],
}

#[derive(Deserialize, Clone, Debug)]
pub struct IngotsConfig {
    pub metal: IngotTypeConfig,
    pub crystal: IngotTypeConfig,
    pub void: IngotTypeConfig,
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
    pub berth_1_arm_index: u8,
    pub berth_2_arm_index: u8,
    pub berth_3_arm_index: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ZLayerConfig {
    pub z_stars_far: f32,
    pub z_stars_near: f32,
    pub z_connectors: f32,
    pub z_environment: f32,
    pub z_map_markers: f32,
    pub z_ship: f32,
    pub z_beam: f32,
    pub z_cargo_bar: f32,
    pub z_hud: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MapColorsConfig {
    pub color_map_station: [f32; 3],
    pub color_map_s1: [f32; 3],
    pub color_map_s7: [f32; 3],
    pub color_map_s3: [f32; 3],
    pub color_map_line: [f32; 4],
    pub color_map_highlight: [f32; 3],
}

#[derive(Deserialize, Clone, Debug)]
pub struct EguiConfig {
    pub egui_scale: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DroneVisualEntry {
    pub hull_width: f32,
    pub hull_height: f32,
    pub color_hull: [f32; 3],
    pub color_beam: [f32; 3],
    pub beam_alpha: f32,
    pub color_thruster: [f32; 3],
    pub color_cargo_fill: [f32; 3],
    pub color_cargo_bg: [f32; 3],
    pub cargo_bar_width: f32,
    pub cargo_bar_height: f32,
    pub map_icon_width: f32,
    pub map_icon_height: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DroneVisualConfig {
    pub opening: DroneVisualEntry,
    pub mission: DroneVisualEntry,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentThrusterConfig {
    pub width: f32,
    pub color_nozzle: [u8; 3],
    pub color_body: [u8; 3],
    pub color_wire: [u8; 3],
    pub wire_count: usize,
    pub nozzle_width_ratio: f32,
    pub body_width_ratio: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentHullConfig {
    pub width: f32,
    pub rib_count: usize,
    pub color_frame: [u8; 3],
    pub color_outline: [u8; 3],
    pub stroke_width: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentCanisterConfig {
    pub width: f32,
    pub height: f32,
    pub lid_height_ratio: f32,
    pub color_body: [u8; 3],
    pub color_lid: [u8; 3],
    pub color_highlight: [u8; 3],
    pub color_handle: [u8; 3],
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentAICoreConfig {
    pub radius: f32,
    pub fin_count: usize,
    pub fin_length: f32,
    pub fin_width: f32,
    pub color_body: [u8; 3],
    pub color_fins: [u8; 3],
    pub color_fan_housing: [u8; 3],
    pub fan_radius_ratio: f32,
    pub fan_blade_count: usize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentDroneBayConfig {
    pub width: f32,
    pub height: f32,
    pub color_ready: [u8; 3],
    pub color_empty: [u8; 3],
    pub nose_height_ratio: f32,
    pub fin_width_ratio: f32,
    pub fin_height_ratio: f32,
    pub porthole_radius: f32,
    pub porthole_offset_y: f32,
    pub exhaust_radius: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipDroneConfig {
    pub width: f32,
    pub height: f32,
    pub color_body: [u8; 3],
    pub color_nose: [u8; 3],
    pub color_fins: [u8; 3],
    pub color_exhaust: [u8; 3],
    pub nose_height_ratio: f32,
    pub fin_width_ratio: f32,
    pub fin_height_ratio: f32,
    pub exhaust_radius: f32,
    pub porthole_radius: f32,
    pub porthole_offset_y: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipOpeningConfig {
    pub width: f32,
    pub height: f32,
    pub color_body: [u8; 3],
    pub color_nose: [u8; 3],
    pub color_fins: [u8; 3],
    pub color_exhaust: [u8; 3],
    pub nose_height_ratio: f32,
    pub fin_width_ratio: f32,
    pub fin_height_ratio: f32,
    pub exhaust_radius: f32,
    pub porthole_radius: f32,
    pub porthole_offset_y: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipConfig {
    pub drone: ShipDroneConfig,
    pub opening: ShipOpeningConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentConfig {
    pub thruster: ComponentThrusterConfig,
    pub hull: ComponentHullConfig,
    pub canister: ComponentCanisterConfig,
    pub ai_core: ComponentAICoreConfig,
    pub drone_bay: ComponentDroneBayConfig,
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct VisualConfig {
    pub starfield: StarfieldConfig,
    pub asteroid: AsteroidVisualConfig,
    pub bottle: BottleVisualConfig,
    pub station: StationVisualConfig,
    pub drone: DroneVisualConfig,
    pub ore: OreConfig,
    pub production_tree: ProductionTreeConfig,
    pub particles: ParticlesConfig,
    pub ingot: IngotsConfig,
    pub component: ComponentConfig,
    pub ship: ShipConfig,
    pub z_layer: ZLayerConfig,
    pub map_colors: MapColorsConfig,
    pub egui: EguiConfig,
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

pub fn rgb_u8(c: [u8; 3]) -> bevy::prelude::Color {
    bevy::prelude::Color::srgb(c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0)
}

pub fn rgb_u8_to_egui(c: [u8; 3]) -> egui::Color32 {
    egui::Color32::from_rgb(c[0], c[1], c[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test anchor for Issue #58: test_visual_config_loads_without_error
    #[test]
    fn test_visual_config_loads_without_error() {
        // Load visual.toml via the existing config load path
        let config = VisualConfig::load();
        
        // Verify no missing field errors occurred (config loaded successfully)
        // Verify key fields are present with expected types
        assert!(config.starfield.far_count > 0);
        assert!(config.asteroid.color_iron.len() == 3);
        assert!(config.drone.opening.hull_width > 0.0);
        assert!(config.drone.opening.hull_height > 0.0);
        assert!(config.drone.opening.cargo_bar_width > 0.0);
        assert!(config.drone.opening.cargo_bar_height > 0.0);
        assert!(config.drone.opening.map_icon_width > 0.0);
        assert!(config.drone.opening.map_icon_height > 0.0);
        
        // Verify drone.mission also has the renamed fields
        assert!(config.drone.mission.hull_width > 0.0);
        assert!(config.drone.mission.hull_height > 0.0);
        assert!(config.drone.mission.cargo_bar_width > 0.0);
        assert!(config.drone.mission.cargo_bar_height > 0.0);
        assert!(config.drone.mission.map_icon_width > 0.0);
        assert!(config.drone.mission.map_icon_height > 0.0);
    }
}
