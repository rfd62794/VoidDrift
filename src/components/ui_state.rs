use bevy::prelude::*;

#[derive(Resource, Default, PartialEq, Debug, Clone, Copy)]
pub enum ActiveStationTab {
    #[default]
    Cargo,
    Production,
    Requests,
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

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct ProductionTabState {
    pub selected_ore: OreType,
}

impl Default for ProductionTabState {
    fn default() -> Self {
        Self { selected_ore: OreType::Iron }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum OreType {
    #[default]
    Iron,
    Tungsten,
    Nickel,
    Aluminum,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct RequestsTabState {
    pub selected_faction: FactionId,
    pub collected_requests: Vec<CollectedRequest>,
    pub visited_after_t106: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct CollectedRequest {
    pub id: RequestId,
    pub faction: FactionId,
    pub fulfilled: bool,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, Copy, Default)]
pub enum FactionId {
    #[default]
    Signal,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum RequestId {
    FirstLight,
}

#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[default]
    Desktop,
    Mobile,
}
