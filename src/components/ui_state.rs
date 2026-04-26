use bevy::prelude::*;

#[derive(Resource, Default, PartialEq, Debug, Clone, Copy)]
pub enum ActiveStationTab {
    Station,
    Fleet,
    #[default]
    Cargo,
    Iron,
    Tungsten,
    Nickel,
    Upgrades,
}

#[derive(Resource, PartialEq, Debug, Clone, Copy, Default)]
pub enum DrawerState {
    #[default]
    Collapsed,
    Expanded,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct UiLayout {
    pub screen_width: f32,
    pub screen_height: f32,
    pub handle_height: f32,
    pub signal_height: f32,
    pub primary_tab_height: f32,
    pub secondary_tab_height: f32,
    pub content_height: f32,
}

impl Default for UiLayout {
    fn default() -> Self {
        Self {
            screen_width: 720.0,
            screen_height: 1604.0,
            handle_height: 32.0,
            signal_height: 64.0,
            primary_tab_height: 48.0,
            secondary_tab_height: 48.0,
            content_height: 358.0,
        }
    }
}
