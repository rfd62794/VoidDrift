use bevy::prelude::*;
use bevy_egui::egui;
use bevy::ecs::system::SystemParam;
use crate::components::*;
use crate::config::VisualConfig;
use crate::config::visual::rgb_u8_to_egui;
use crate::systems::visuals::ore_polygon::{self, OrePolygonConfig};
use crate::systems::visuals::ingot_node::{self, IngotNodeConfig};
use crate::systems::visuals::component_nodes::{self, ThrusterConfig, HullConfig, CanisterConfig, AICoreConfig, DroneBayConfig};

pub fn render_production_tree(
    ui: &mut egui::Ui,
    station_query: &mut Query<(Entity, &mut Station, &mut StationQueues), (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    toggles: &mut ProductionToggles,
    visual_cfg: &VisualConfig,
    world_view_rect: &mut WorldViewRect,
    view_state: &mut ViewState,
) {
    let rect = ui.max_rect();
    
    // Update WorldViewRect to zero — hide world view
    world_view_rect.w = 0.0;
    world_view_rect.h = 0.0;
    
    let painter = ui.painter();
    let text_color = egui::Color32::WHITE;
    
    // Grid dimensions
    let col_width = rect.width() / 4.0;
    let row_height = rect.height() / 5.0;
    let node_size = egui::vec2(visual_cfg.production_tree.node_width, visual_cfg.production_tree.node_height);
    let drone_bay_size = egui::vec2(visual_cfg.production_tree.drone_bay_width, visual_cfg.production_tree.drone_bay_height);
    
    // Access station data for active states
    let (station, mut local_toggles) = if let Ok((_, st, _)) = station_query.get_single() {
        (Some(st), toggles.clone())
    } else {
        (None, toggles.clone())
    };
    
    // Compute all node centers first
    let node_center = |col: usize, row: usize| -> egui::Pos2 {
        egui::pos2(
            rect.min.x + col_width * (col as f32 + 0.5),
            rect.min.y + row_height * (row as f32 + 0.5),
        )
    };
    
    // Named centers for arrow routing
    let iron_ore     = node_center(0, 0);
    let tungsten_ore = node_center(1, 0);
    let nickel_ore   = node_center(2, 0);
    let aluminum_ore = node_center(3, 0);
    
    let iron_ingot     = node_center(0, 1);
    let tungsten_ingot = node_center(1, 1);
    let nickel_ingot   = node_center(2, 1);
    let aluminum_ingot = node_center(3, 1);
    
    let hull_plate = node_center(0, 2);
    let thruster   = node_center(1, 2);
    let ai_core    = node_center(2, 2);
    let canister   = node_center(3, 2);
    
    let drone_bay  = node_center(1, 3); // wide, centered
    
    // Arrow drawing helper with three-state colors and ON/OFF label
    let draw_arrow = |from: egui::Pos2, to: egui::Pos2, toggle_on: bool, has_inventory: bool| -> egui::Rect {
        let color = if !toggle_on {
            egui::Color32::from_rgb(180, 40, 40)  // Red — OFF
        } else if has_inventory {
            egui::Color32::from_rgb(40, 180, 40)  // Green — ON + flowing
        } else {
            egui::Color32::from_rgb(80, 80, 80)   // Gray — ON but nothing to flow
        };
        let stroke = egui::Stroke::new(1.5, color);
        
        // Arrow shaft — from bottom of source node to top of target node
        let shaft_from = egui::pos2(from.x, from.y + 20.0); // bottom of node
        let shaft_to = egui::pos2(to.x, to.y - 20.0);       // top of node
        painter.line_segment([shaft_from, shaft_to], stroke);
        
        // Arrowhead — small triangle at shaft_to
        let tip = shaft_to;
        let left = egui::pos2(tip.x - 5.0, tip.y - 8.0);
        let right = egui::pos2(tip.x + 5.0, tip.y - 8.0);
        painter.add(egui::Shape::convex_polygon(
            vec![tip, left, right],
            color,
            egui::Stroke::NONE,
        ));
        
        // ON/OFF label centered on shaft
        let mid = egui::pos2(
            (shaft_from.x + shaft_to.x) / 2.0,
            (shaft_from.y + shaft_to.y) / 2.0,
        );
        let label = if toggle_on { "ON" } else { "OFF" };
        painter.text(mid, egui::Align2::CENTER_CENTER, label, 
            egui::FontId::proportional(9.0), color);
        
        // Return clickable rect for Part 3
        egui::Rect::from_center_size(mid, egui::vec2(50.0, 30.0))
    };
    
    // Ore → Ingot (toggle-based)
    let st = station.as_ref();
    let refine_iron_rect = draw_arrow(iron_ore, iron_ingot, local_toggles.refine_iron, st.map_or(false, |s| s.iron_reserves > 0.0));
    let refine_tungsten_rect = draw_arrow(tungsten_ore, tungsten_ingot, local_toggles.refine_tungsten, st.map_or(false, |s| s.tungsten_reserves > 0.0));
    let refine_nickel_rect = draw_arrow(nickel_ore, nickel_ingot, local_toggles.refine_nickel, st.map_or(false, |s| s.nickel_reserves > 0.0));
    let refine_aluminum_rect = draw_arrow(aluminum_ore, aluminum_ingot, local_toggles.refine_aluminum, st.map_or(false, |s| s.aluminum_reserves > 0.0));
    
    // Ingot → Part (toggle-based)
    let forge_hull_rect = draw_arrow(iron_ingot, hull_plate, local_toggles.forge_hull, st.map_or(false, |s| s.iron_ingots > 0.0));
    let forge_thruster_rect = draw_arrow(tungsten_ingot, thruster, local_toggles.forge_thruster, st.map_or(false, |s| s.tungsten_ingots > 0.0));
    let forge_core_rect = draw_arrow(nickel_ingot, ai_core, local_toggles.forge_core, st.map_or(false, |s| s.nickel_ingots > 0.0));
    let forge_canister_rect = draw_arrow(aluminum_ingot, canister, local_toggles.forge_aluminum_canister, st.map_or(false, |s| s.aluminum_ingots > 0.0));
    
    // Part → Drone Bay (shared build_drones toggle)
    let hull_drone_rect = if let Some(s) = station {
        draw_arrow(hull_plate, drone_bay, local_toggles.build_drones, s.hull_plate_reserves > 0.0)
    } else {
        egui::Rect::NOTHING
    };
    let thruster_drone_rect = if let Some(s) = station {
        draw_arrow(thruster, drone_bay, local_toggles.build_drones, s.thruster_reserves > 0.0)
    } else {
        egui::Rect::NOTHING
    };
    let core_drone_rect = if let Some(s) = station {
        draw_arrow(ai_core, drone_bay, local_toggles.build_drones, s.ai_cores > 0.0)
    } else {
        egui::Rect::NOTHING
    };
    
    // Click detection for arrows
    let response = ui.interact(refine_iron_rect, ui.id().with("toggle_refine_iron"), egui::Sense::click());
    if response.clicked() { local_toggles.refine_iron = !local_toggles.refine_iron; }
    
    let response = ui.interact(refine_tungsten_rect, ui.id().with("toggle_refine_tungsten"), egui::Sense::click());
    if response.clicked() { local_toggles.refine_tungsten = !local_toggles.refine_tungsten; }
    
    let response = ui.interact(refine_nickel_rect, ui.id().with("toggle_refine_nickel"), egui::Sense::click());
    if response.clicked() { local_toggles.refine_nickel = !local_toggles.refine_nickel; }
    
    let response = ui.interact(refine_aluminum_rect, ui.id().with("toggle_refine_aluminum"), egui::Sense::click());
    if response.clicked() { local_toggles.refine_aluminum = !local_toggles.refine_aluminum; }
    
    let response = ui.interact(forge_hull_rect, ui.id().with("toggle_forge_hull"), egui::Sense::click());
    if response.clicked() { local_toggles.forge_hull = !local_toggles.forge_hull; }
    
    let response = ui.interact(forge_thruster_rect, ui.id().with("toggle_forge_thruster"), egui::Sense::click());
    if response.clicked() { local_toggles.forge_thruster = !local_toggles.forge_thruster; }
    
    let response = ui.interact(forge_core_rect, ui.id().with("toggle_forge_core"), egui::Sense::click());
    if response.clicked() { local_toggles.forge_core = !local_toggles.forge_core; }
    
    let response = ui.interact(forge_canister_rect, ui.id().with("toggle_forge_canister"), egui::Sense::click());
    if response.clicked() { local_toggles.forge_aluminum_canister = !local_toggles.forge_aluminum_canister; }
    
    // Shared build_drones toggle — any of the three arrows toggles it
    let response = ui.interact(hull_drone_rect, ui.id().with("toggle_build_drones"), egui::Sense::click());
    if response.clicked() { local_toggles.build_drones = !local_toggles.build_drones; }
    let response = ui.interact(thruster_drone_rect, ui.id().with("toggle_build_drones_thruster"), egui::Sense::click());
    if response.clicked() { local_toggles.build_drones = !local_toggles.build_drones; }
    let response = ui.interact(core_drone_rect, ui.id().with("toggle_build_drones_core"), egui::Sense::click());
    if response.clicked() { local_toggles.build_drones = !local_toggles.build_drones; }
    
    // Canister → future (no arrow for now)
    
    // Title at top (smaller to make room for grid)
    painter.text(
        egui::pos2(rect.center().x, rect.min.y + 30.0),
        egui::Align2::CENTER_CENTER,
        "PRODUCTION PIPELINE",
        egui::FontId::proportional(16.0),
        egui::Color32::from_rgb(0, 200, 200),
    );
    
    // Node rendering helper with active state and inventory display
    let render_node = |col: usize, row: usize, label: &str, inventory: String, is_wide: bool, active: bool, ore_type: Option<OreDeposit>, is_ingot: bool, component_type: Option<&str>| {
        let border_color = if active {
            egui::Color32::from_rgb(0, 200, 200) // Echo cyan
        } else {
            egui::Color32::from_rgb(40, 80, 80) // Dimmed cyan
        };
        let fill_color = if active {
            egui::Color32::from_rgb(4, 16, 20) // Slightly brighter
        } else {
            egui::Color32::from_rgb(4, 8, 16) // Dark
        };
        let border_stroke = egui::Stroke::new(1.5, border_color);
        
        let cell_center = egui::pos2(
            rect.min.x + col_width * (col as f32 + 0.5),
            rect.min.y + row_height * (row as f32 + 0.5),
        );
        let size = if is_wide { drone_bay_size } else { node_size };
        let node_rect = egui::Rect::from_center_size(cell_center, size);
        
        painter.rect_filled(node_rect, 4.0, fill_color);
        painter.rect_stroke(node_rect, 4.0, border_stroke, egui::StrokeKind::Outside);
        
        // Draw procedural visuals for ore/ingot nodes
        if let Some(ore) = ore_type {
            if is_ingot {
                // Draw ingot node (3-rect isometric)
                let ingot_cfg = &visual_cfg.production_tree.ingot_node;
                let base_color = match ore {
                    OreDeposit::Iron => rgb_u8_to_egui(visual_cfg.ore.metal.color_vein),
                    OreDeposit::Tungsten => rgb_u8_to_egui(visual_cfg.ore.h3_gas.color_vein),
                    OreDeposit::Nickel => rgb_u8_to_egui(visual_cfg.ore.void_essence.color_vein),
                    OreDeposit::Aluminum => rgb_u8_to_egui(visual_cfg.ore.metal.color_vein),
                };
                let ingot_config = IngotNodeConfig {
                    width: ingot_cfg.width,
                    height: ingot_cfg.height,
                    depth_offset_x: ingot_cfg.depth_offset_x,
                    depth_offset_y: ingot_cfg.depth_offset_y,
                    color_face_light_factor: ingot_cfg.color_face_light_factor,
                    color_face_dark_factor: ingot_cfg.color_face_dark_factor,
                };
                ingot_node::draw_ingot_node(painter, node_rect.center(), &ingot_config, base_color);
            } else {
                // Draw ore node (procedural polygon)
                let ore_cfg = &visual_cfg.production_tree.ore_node;
                let (body_color, vein_color, band_count, band_width_min, band_width_max, grain_angle_deg) = match ore {
                    OreDeposit::Iron => {
                        let cfg = &visual_cfg.ore.metal;
                        (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                    },
                    OreDeposit::Tungsten => {
                        let cfg = &visual_cfg.ore.h3_gas;
                        (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                    },
                    OreDeposit::Nickel => {
                        let cfg = &visual_cfg.ore.void_essence;
                        (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                    },
                    OreDeposit::Aluminum => {
                        let cfg = &visual_cfg.ore.metal;
                        (rgb_u8_to_egui(cfg.color_body), rgb_u8_to_egui(cfg.color_vein), cfg.band_count, cfg.band_width_min, cfg.band_width_max, cfg.grain_angle_deg)
                    },
                };
                let ore_seed = match ore {
                    OreDeposit::Iron => 1u64,
                    OreDeposit::Tungsten => 2u64,
                    OreDeposit::Nickel => 3u64,
                    OreDeposit::Aluminum => 4u64,
                };
                let ore_config = OrePolygonConfig {
                    radius: ore_cfg.radius,
                    vertex_count: ore_cfg.vertex_count,
                    jaggedness: ore_cfg.jaggedness,
                    color_body: body_color,
                    color_vein: vein_color,
                    band_count,
                    band_width_min,
                    band_width_max,
                    grain_angle_deg,
                    seed: ore_seed,
                };
                ore_polygon::draw_ore_polygon(painter, node_rect.center(), &ore_config, 1.0);
            }
        }
        
        if let Some(component) = component_type {
            let node_center = node_rect.center();
            match component {
                "hull" => {
                    let hull_cfg = &visual_cfg.component.hull;
                    let hull_config = HullConfig {
                        width: hull_cfg.width,
                        rib_count: hull_cfg.rib_count,
                        color_frame: rgb_u8_to_egui(hull_cfg.color_frame),
                        color_outline: rgb_u8_to_egui(hull_cfg.color_outline),
                        stroke_width: hull_cfg.stroke_width,
                    };
                    component_nodes::draw_hull(painter, node_center, &hull_config);
                },
                "thruster" => {
                    let thruster_cfg = &visual_cfg.component.thruster;
                    let thruster_config = ThrusterConfig {
                        width: thruster_cfg.width,
                        color_nozzle: rgb_u8_to_egui(thruster_cfg.color_nozzle),
                        color_body: rgb_u8_to_egui(thruster_cfg.color_body),
                        color_wire: rgb_u8_to_egui(thruster_cfg.color_wire),
                        wire_count: thruster_cfg.wire_count,
                        nozzle_width_ratio: thruster_cfg.nozzle_width_ratio,
                        body_width_ratio: thruster_cfg.body_width_ratio,
                    };
                    component_nodes::draw_thruster(painter, node_center, &thruster_config);
                },
                "ai_core" => {
                    let ai_core_cfg = &visual_cfg.component.ai_core;
                    let ai_core_config = AICoreConfig {
                        radius: ai_core_cfg.radius,
                        fin_count: ai_core_cfg.fin_count,
                        fin_length: ai_core_cfg.fin_length,
                        fin_width: ai_core_cfg.fin_width,
                        color_body: rgb_u8_to_egui(ai_core_cfg.color_body),
                        color_fins: rgb_u8_to_egui(ai_core_cfg.color_fins),
                        color_fan_housing: rgb_u8_to_egui(ai_core_cfg.color_fan_housing),
                        fan_radius_ratio: ai_core_cfg.fan_radius_ratio,
                        fan_blade_count: ai_core_cfg.fan_blade_count,
                    };
                    component_nodes::draw_ai_core(painter, node_center, &ai_core_config);
                },
                "canister" => {
                    let canister_cfg = &visual_cfg.component.canister;
                    let canister_config = CanisterConfig {
                        width: canister_cfg.width,
                        height: canister_cfg.height,
                        lid_height_ratio: canister_cfg.lid_height_ratio,
                        color_body: rgb_u8_to_egui(canister_cfg.color_body),
                        color_lid: rgb_u8_to_egui(canister_cfg.color_lid),
                        color_highlight: rgb_u8_to_egui(canister_cfg.color_highlight),
                        color_handle: rgb_u8_to_egui(canister_cfg.color_handle),
                    };
                    component_nodes::draw_canister(painter, node_center, &canister_config);
                },
                "drone_bay" => {
                    let drone_bay_cfg = &visual_cfg.component.drone_bay;
                    let drone_bay_config = DroneBayConfig {
                        width: drone_bay_cfg.width,
                        height: drone_bay_cfg.height,
                        color_ready: rgb_u8_to_egui(drone_bay_cfg.color_ready),
                        color_empty: rgb_u8_to_egui(drone_bay_cfg.color_empty),
                        nose_height_ratio: drone_bay_cfg.nose_height_ratio,
                        fin_width_ratio: drone_bay_cfg.fin_width_ratio,
                        fin_height_ratio: drone_bay_cfg.fin_height_ratio,
                        porthole_radius: drone_bay_cfg.porthole_radius,
                        porthole_offset_y: drone_bay_cfg.porthole_offset_y,
                        exhaust_radius: drone_bay_cfg.exhaust_radius,
                    };
                    let is_ready = if let Some(st) = station { st.drone_build_progress >= 1.0 } else { false };
                    component_nodes::draw_drone_bay(painter, node_center, &drone_bay_config, is_ready);
                },
                _ => {}
            }
        }
        
        let display = format!("{} ({})", label, inventory);
        painter.text(
            node_rect.center() + egui::vec2(0.0, 35.0),
            egui::Align2::CENTER_CENTER,
            &display,
            egui::FontId::proportional(10.0),
            text_color,
        );
    };
    
    // Render nodes with active states and inventory
    if let Some(st) = station {
        // Row 0: Ore nodes
        render_node(0, 0, "IRON", format!("{:.1}", st.iron_reserves), false, st.iron_reserves > 0.0, Some(OreDeposit::Iron), false, None);
        render_node(1, 0, "TUNGSTEN", format!("{:.1}", st.tungsten_reserves), false, st.tungsten_reserves > 0.0, Some(OreDeposit::Tungsten), false, None);
        render_node(2, 0, "NICKEL", format!("{:.1}", st.nickel_reserves), false, st.nickel_reserves > 0.0, Some(OreDeposit::Nickel), false, None);
        render_node(3, 0, "ALUMINUM", format!("{:.1}", st.aluminum_reserves), false, st.aluminum_reserves > 0.0, Some(OreDeposit::Aluminum), false, None);
        
        // Row 1: Ingot nodes
        render_node(0, 1, "IRON INGOT", format!("{:.1}", st.iron_ingots), false, st.iron_ingots > 0.0, Some(OreDeposit::Iron), true, None);
        render_node(1, 1, "TUNGSTEN INGOT", format!("{:.1}", st.tungsten_ingots), false, st.tungsten_ingots > 0.0, Some(OreDeposit::Tungsten), true, None);
        render_node(2, 1, "NICKEL INGOT", format!("{:.1}", st.nickel_ingots), false, st.nickel_ingots > 0.0, Some(OreDeposit::Nickel), true, None);
        render_node(3, 1, "ALUMINUM INGOT", format!("{:.1}", st.aluminum_ingots), false, st.aluminum_ingots > 0.0, Some(OreDeposit::Aluminum), true, None);
        
        // Row 2: Part nodes
        render_node(0, 2, "HULL PLATE", format!("{:.0}", st.hull_plate_reserves), false, st.hull_plate_reserves > 0.0, None, false, Some("hull"));
        render_node(1, 2, "THRUSTER", format!("{:.0}", st.thruster_reserves), false, st.thruster_reserves > 0.0, None, false, Some("thruster"));
        render_node(2, 2, "AI CORE", format!("{:.0}", st.ai_cores), false, st.ai_cores > 0.0, None, false, Some("ai_core"));
        render_node(3, 2, "CANISTER", format!("{:.0}", st.aluminum_canisters), false, st.aluminum_canisters > 0.0, None, false, Some("canister"));
        
        // Row 3: Convergence (DRONE BAY) — no inventory number
        render_node(1, 3, "DRONE BAY", String::new(), true, 
            st.hull_plate_reserves > 0.0 && 
            st.thruster_reserves > 0.0 && 
            st.ai_cores > 0.0, None, false, Some("drone_bay"));
    } else {
        // Render all nodes as inactive when station not accessible
        render_node(0, 0, "IRON", String::new(), false, false, Some(OreDeposit::Iron), false, None);
        render_node(1, 0, "TUNGSTEN", String::new(), false, false, Some(OreDeposit::Tungsten), false, None);
        render_node(2, 0, "NICKEL", String::new(), false, false, Some(OreDeposit::Nickel), false, None);
        render_node(3, 0, "ALUMINUM", String::new(), false, false, Some(OreDeposit::Aluminum), false, None);
        
        render_node(0, 1, "IRON INGOT", String::new(), false, false, Some(OreDeposit::Iron), true, None);
        render_node(1, 1, "TUNGSTEN INGOT", String::new(), false, false, Some(OreDeposit::Tungsten), true, None);
        render_node(2, 1, "NICKEL INGOT", String::new(), false, false, Some(OreDeposit::Nickel), true, None);
        render_node(3, 1, "ALUMINUM INGOT", String::new(), false, false, Some(OreDeposit::Aluminum), true, None);
        
        render_node(0, 2, "HULL PLATE", String::new(), false, false, None, false, Some("hull"));
        render_node(1, 2, "THRUSTER", String::new(), false, false, None, false, Some("thruster"));
        render_node(2, 2, "AI CORE", String::new(), false, false, None, false, Some("ai_core"));
        render_node(3, 2, "CANISTER", String::new(), false, false, None, false, Some("canister"));
        
        render_node(1, 3, "DRONE BAY", String::new(), true, false, None, false, Some("drone_bay"));
    }
    
    // Write toggles back to resource after any clicks
    *toggles = local_toggles;
    
    // BACK button — bottom center, above DRONE BAY
    let button_rect = egui::Rect::from_center_size(
        rect.center_bottom() - egui::vec2(0.0, 60.0),
        egui::vec2(120.0, 44.0),
    );
    if ui.put(button_rect, egui::Button::new("BACK").min_size(egui::vec2(120.0, 44.0))).clicked() {
        view_state.show_production_tree = false;
        // Restore WorldViewRect on next frame naturally
    }
}
