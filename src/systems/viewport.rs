use bevy::prelude::*;
use bevy::render::camera::Viewport;
use crate::components::*;


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

    // Derive exact egui→physical scale from true canvas size (written by hud_ui_system).
    // scale_x = physical_width / egui_canvas_width (exact, no guessing)
    if world_view.canvas_w <= 0.0 || world_view.canvas_h <= 0.0 { return; }
    let scale_x = win_w as f32 / world_view.canvas_w;
    let scale_y = win_h as f32 / world_view.canvas_h;

    let phys_x = (world_view.x * scale_x).round() as u32;
    let phys_y = (world_view.y * scale_y).round() as u32;
    let phys_w = (world_view.w * scale_x).round() as u32;
    let phys_h = (world_view.h * scale_y).round() as u32;

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

    camera.viewport = Some(new_viewport);
}

/// No-op — kept for lib.rs registration compatibility.
/// Content height is now fixed in UiLayout::default().
pub fn ui_layout_system() {}
