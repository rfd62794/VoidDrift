use bevy_egui::egui;

#[derive(Copy, Clone, Debug)]
pub enum HighlightKind {
    Amber,
}

#[derive(Copy, Clone, Debug)]
pub struct ButtonStyle {
    pub min_size: egui::Vec2,
    pub fill: Option<egui::Color32>,
    pub text_color: Option<egui::Color32>,
    pub stroke: egui::Stroke,
    pub corner_radius: u8,
}

impl ButtonStyle {
    pub const fn primary() -> Self {
        Self {
            min_size: egui::Vec2::new(80.0, 44.0),
            fill: None,
            text_color: None,
            stroke: egui::Stroke { width: 0.0, color: egui::Color32::TRANSPARENT },
            corner_radius: 0,
        }
    }
    pub const fn wide() -> Self {
        Self {
            min_size: egui::Vec2::new(160.0, 30.0),
            fill: None,
            text_color: None,
            stroke: egui::Stroke { width: 0.0, color: egui::Color32::TRANSPARENT },
            corner_radius: 0,
        }
    }
}
