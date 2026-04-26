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
    station: &mut Station,
    queue: &mut Option<ProcessingJob>,
    op: ProcessingOperation,
    resource_cost: f32,
    batch_time: f32,
) {
    let (input_name, output_name) = match op {
        ProcessingOperation::IronRefinery => ("IRON", "MATERIALS"),
        ProcessingOperation::TungstenRefinery => ("TUNGSTEN", "MATERIALS"),
        ProcessingOperation::NickelRefinery => ("NICKEL", "MATERIALS"),
        ProcessingOperation::HullForge => ("PLATES", "SHIP HULL"),
        ProcessingOperation::CoreFabricator => ("NICKEL", "AI CORE"),
    };

    let max_possible = match op {
        ProcessingOperation::IronRefinery => station.iron_reserves / resource_cost,
        ProcessingOperation::TungstenRefinery => station.tungsten_reserves / resource_cost,
        ProcessingOperation::NickelRefinery => station.nickel_reserves / resource_cost,
        ProcessingOperation::HullForge => station.hull_plate_reserves as f32 / resource_cost,
        ProcessingOperation::CoreFabricator => station.nickel_reserves / resource_cost,
    }.floor() as u32;

    ui.group(|ui| {
        ui.set_width(180.0);
        ui.vertical(|ui| {
            // Header 1: Chain
            ui.label(egui::RichText::new(format!("{} → {}", input_name, output_name)).strong().size(12.0));
            
            // Header 2: Ratio
            ui.label(egui::RichText::new(format!("Ratio: {:.0} Mat", resource_cost))
                .size(9.0)
                .italics()
                .color(egui::Color32::from_gray(160)));

            ui.add_space(4.0);

            if let Some(job) = queue {
                let progress = 1.0 - (job.timer / batch_time);
                ui.add_space(4.0);
                if job.clearing {
                    ui.label(egui::RichText::new("▶ CLEARING...").color(egui::Color32::YELLOW));
                } else {
                    ui.label(egui::RichText::new(format!("▶ PROCESSING... {:.0}s", job.timer)).color(egui::Color32::CYAN));
                }
                ui.add(egui::ProgressBar::new(progress).desired_width(160.0).fill(egui::Color32::CYAN));
                ui.label(format!("Queued: {} / Total: {}", job.batches, job.completed + job.batches));
            } else {
                ui.add_space(4.0);
                ui.label(egui::RichText::new("STATUS: IDLE").color(egui::Color32::GRAY));
                ui.add_space(14.0);
            }

            ui.add_space(8.0);
            
            // FEEDBACK LINE
            if max_possible > 0 {
                ui.label(egui::RichText::new(format!("You can make: {} batches", max_possible)).color(egui::Color32::GREEN).size(10.0));
            } else {
                ui.label(egui::RichText::new("Insufficient materials/power").color(egui::Color32::LIGHT_RED).size(10.0));
            }

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let btn_size = egui::vec2(40.0, 32.0);
                if ui.add_enabled(max_possible >= 1, egui::Button::new("+1").min_size(btn_size)).clicked() { /* crate::systems::economy::queue_job(station, queue, op, 1); */ }
                if ui.add_enabled(max_possible >= 10, egui::Button::new("+10").min_size(btn_size)).clicked() { /* crate::systems::economy::queue_job(station, queue, op, 10); */ }
                if ui.add_enabled(max_possible >= 1, egui::Button::new("MAX").min_size(btn_size)).clicked() { /* crate::systems::economy::queue_job(station, queue, op, max_possible); */ }
            });

            ui.add_space(4.0);
            if let Some(job) = queue {
                if ui.add(egui::Button::new("CLEAR QUEUE").min_size(egui::vec2(160.0, 30.0))).clicked() {
                    job.batches = 1; job.clearing = true;
                    add_log_entry(station, format!("> {} QUEUE CLEARED.", input_name));
                }
            } else {
                ui.add_space(34.0);
            }
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Non-refundable queue").size(8.0).color(egui::Color32::from_gray(100)));
        });
    });
}
