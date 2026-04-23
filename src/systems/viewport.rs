use bevy::prelude::*;
use bevy::render::camera::Viewport;
use crate::components::*;
use crate::constants::EGUI_SCALE;

/// Sets the camera viewport each frame based purely on DrawerState + UiLayout constants.
/// No egui rect involved — constants are reliable on frame 0, egui rect is not.
pub fn drawer_viewport_system(
    drawer: Res<DrawerState>,
    layout: Res<UiLayout>,
    windows: Query<&Window>,
    mut cam_query: Query<&mut Camera, With<MainCamera>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Ok(mut camera) = cam_query.get_single_mut() else { return; };

    let win_w = window.physical_width();
    let win_h = window.physical_height();

    if win_w == 0 || win_h == 0 { return; }

    // Compute drawer height in egui logical pts from constants
    let drawer_egui_h = match *drawer {
        DrawerState::Collapsed => layout.handle_height + layout.signal_height,
        DrawerState::TabsOnly  => layout.handle_height + layout.primary_tab_height
                                + layout.signal_height,
        DrawerState::Expanded  => layout.handle_height + layout.primary_tab_height
                                + layout.secondary_tab_height + layout.content_height
                                + layout.signal_height,
    };

    // egui logical pts → physical px: multiply by EGUI_SCALE only
    let drawer_physical = (drawer_egui_h * EGUI_SCALE).round() as u32;
    let world_h = win_h.saturating_sub(drawer_physical).max(1);

    let new_viewport = Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(win_w, world_h),
        depth: 0.0..1.0,
    };

    // Only write if changed to avoid unnecessary GPU state changes
    let needs_update = camera.viewport.as_ref().map_or(true, |v| {
        v.physical_position != new_viewport.physical_position
            || v.physical_size != new_viewport.physical_size
    });

    if needs_update {
        eprintln!("VIEWPORT: world_h={} drawer_h={} win={}x{}",
            world_h, drawer_physical, win_w, win_h);
        camera.viewport = Some(new_viewport);
    }
}
