use crate::components::*;

/// Drive drawer state from game state transitions.
/// - Auto-opens on dock (once, on transition frame)
/// - Does NOT auto-close on undock — user controls drawer manually
/// - Forces collapsed during opening sequence
pub fn update_drawer_state(
    is_docked: bool,
    opening_complete: bool,
    was_docked: &mut bool,
    drawer: &mut DrawerState,
) {
    if !opening_complete {
        *drawer = DrawerState::Collapsed;
    } else if is_docked && !*was_docked {
        *drawer = DrawerState::Expanded;
    }
    *was_docked = is_docked;
}
