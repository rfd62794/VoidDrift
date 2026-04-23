use bevy::prelude::*;
use bevy::render::camera::Viewport;
use crate::components::*;
use crate::constants::EGUI_SCALE;

/// Sets the camera viewport each frame to exactly match the CentralPanel rect
/// written by hud_ui_system. egui handles all panel layout — we just mirror it.
///
/// egui logical pts × EGUI_SCALE = physical px.
/// Do NOT multiply by window.scale_factor() — that double-scales and crashes.
pub fn drawer_viewport_system(
    world_view: Res<WorldViewRect>,
    windows: Query<&Window>,
    mut cam_query: Query<&mut Camera, With<MainCamera>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Ok(mut camera) = cam_query.get_single_mut() else { return; };

    let win_w = window.physical_width();
    let win_h = window.physical_height();
    if win_w == 0 || win_h == 0 { return; }

    // egui logical pts → physical px
    let phys_x = (world_view.x * EGUI_SCALE).round() as u32;
    let phys_y = (world_view.y * EGUI_SCALE).round() as u32;
    let phys_w = (world_view.w * EGUI_SCALE).round() as u32;
    let phys_h = (world_view.h * EGUI_SCALE).round() as u32;

    // Clamp to window bounds — never let viewport exceed render target
    let phys_x = phys_x.min(win_w.saturating_sub(1));
    let phys_y = phys_y.min(win_h.saturating_sub(1));
    let phys_w = phys_w.min(win_w - phys_x);
    let phys_h = phys_h.min(win_h - phys_y);

    if phys_w == 0 || phys_h == 0 { return; }

    let new_viewport = Viewport {
        physical_position: UVec2::new(phys_x, phys_y),
        physical_size: UVec2::new(phys_w, phys_h),
        depth: 0.0..1.0,
    };

    eprintln!("VP: egui=({:.0},{:.0},{:.0},{:.0}) phys=({},{},{},{}) win={}x{}",
        world_view.x, world_view.y, world_view.w, world_view.h,
        phys_x, phys_y, phys_w, phys_h, win_w, win_h);
    camera.viewport = Some(new_viewport);
}

/// No-op — kept for lib.rs registration compatibility.
/// Content height is now fixed in UiLayout::default().
pub fn ui_layout_system() {}
