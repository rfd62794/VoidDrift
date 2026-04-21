use bevy::prelude::*;
use bevy_egui::egui;
use crate::components::*;
use crate::constants::*;

pub fn add_log_entry(station: &mut Station, entry: String) {
    if station.log.back() == Some(&entry) { return; }
    station.log.push_back(entry);
    if station.log.len() > LOG_MAX_LINES {
        station.log.pop_front();
    }
}

pub fn render_queue_card(
    ui: &mut egui::Ui,
    layout: &UiLayout,
    station: &mut Station,
    queue: &mut Option<ProcessingJob>,
    op: ProcessingOperation,
    resource_cost: f32,
    pwr_cost: f32,
    batch_time: f32,
) {
    let (input_name, output_name) = match op {
        ProcessingOperation::MagnetiteRefinery => ("MAGNETITE", "POWER CELLS"),
        ProcessingOperation::CarbonRefinery => ("CARBON", "HULL PLATES"),
        ProcessingOperation::HullForge => ("HULL PLATES", "SHIP HULL"),
        ProcessingOperation::CoreFabricator => ("POWER CELLS", "AI CORE"),
    };

    let max_possible = match op {
        ProcessingOperation::MagnetiteRefinery => (station.magnetite_reserves / resource_cost).min(station.power / pwr_cost),
        ProcessingOperation::CarbonRefinery => (station.carbon_reserves / resource_cost).min(station.power / pwr_cost),
        ProcessingOperation::HullForge => (station.hull_plate_reserves as f32 / resource_cost).min(station.power / pwr_cost),
        ProcessingOperation::CoreFabricator => (station.power_cells as f32 / resource_cost).min(station.power / pwr_cost),
    }.floor() as u32;

    let card_width = layout.content_width;
    let btn_width = (card_width - 8.0) / 3.0;  // 3 add buttons

    ui.set_width(card_width);

    // Header
    ui.label(egui::RichText::new(format!("{} -> {}", input_name, output_name))
        .size(layout.font_size_title)
        .strong());
    ui.label(egui::RichText::new(format!("{:.0} ore · {:.0} sec per batch", resource_cost, batch_time))
        .size(layout.font_size_label)
        .color(egui::Color32::from_gray(140)));

    ui.separator();

    // Progress bar
    if let Some(job) = queue {
        let fraction = job.timer / batch_time;
        ui.add(egui::ProgressBar::new(1.0 - fraction)
            .desired_width(card_width)
            .desired_height(16.0));

        // Status
        if job.clearing {
            ui.label(egui::RichText::new("CLEARING · 1 batch left").color(egui::Color32::YELLOW));
        } else {
            ui.label(egui::RichText::new(format!("PROCESSING · {} batches queued", job.batches)).color(egui::Color32::CYAN));
        }
    } else {
        ui.label(egui::RichText::new("STATUS: IDLE").color(egui::Color32::from_gray(140)));
    }

    // Resource feedback
    if max_possible > 0 {
        ui.label(egui::RichText::new(format!("You can make: {} more", max_possible))
            .color(egui::Color32::GREEN));
    } else {
        ui.label(egui::RichText::new("Insufficient materials/power")
            .color(egui::Color32::LIGHT_RED));
    }

    ui.separator();

    // Add buttons - three equal width
    ui.horizontal(|ui| {
        let btn_size = egui::vec2(btn_width, layout.button_height);
        if ui.add_enabled(max_possible >= 1, egui::Button::new("+1").min_size(btn_size)).clicked() { 
            crate::systems::economy::queue_job(station, queue, op, 1); 
        }
        if ui.add_enabled(max_possible >= 10, egui::Button::new("+10").min_size(btn_size)).clicked() { 
            crate::systems::economy::queue_job(station, queue, op, 10); 
        }
        if ui.add_enabled(max_possible >= 1, egui::Button::new("MAX").min_size(btn_size)).clicked() { 
            crate::systems::economy::queue_job(station, queue, op, max_possible); 
        }
    });

    // Clear button - full width
    let clear_size = egui::vec2(card_width, layout.button_height);
    if let Some(job) = queue {
        if ui.add(egui::Button::new("CLEAR QUEUE").min_size(clear_size)).clicked() {
            job.batches = 1; 
            job.clearing = true;
            add_log_entry(station, format!("> {} QUEUE CLEARED.", input_name));
        }
    }
}
