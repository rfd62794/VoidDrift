// ui.rs - re-export module. Logic lives in hud.rs and station_tabs.rs.
pub use crate::systems::hud::{
    hud_ui_system,
    camera_viewport_system,
    ship_cargo_display_system,
    autonomous_ship_cargo_display_system,
    cargo_label_system,
    station_visual_system,
};
pub use crate::systems::station_tabs::add_log_entry;
