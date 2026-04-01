use egui::{Color32, RichText, Ui};
use crate::models::sprayer_data::SprayerData;
use crate::models::sprayer_settings::SprayerSettings;
use crate::widgets::pressure_display::PressureDisplay;
use crate::widgets::speed_display::SpeedDisplay;

pub struct MonitorScreen {
    pub controller_activated: bool,
    pub constant_pressure_mode: bool,
}

impl MonitorScreen {
    pub fn new() -> Self {
        Self {
            controller_activated: false,
            constant_pressure_mode: false,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, data: &SprayerData, settings: &SprayerSettings) -> (bool, bool) {
        let mut state_changed = false;

        ui.vertical_centered(|ui| {
            // Buttons Row
            ui.horizontal(|ui| {
                let btn_text = if self.controller_activated { "Controller ON" } else { "Controller OFF" };
                let btn_color = if self.controller_activated { Color32::from_rgb(76, 175, 80) } else { Color32::from_rgb(33, 150, 243) };
                
                if ui.add(egui::Button::new(RichText::new(btn_text).color(Color32::WHITE).size(20.0)).fill(btn_color)).clicked() {
                    self.controller_activated = !self.controller_activated;
                    state_changed = true;
                }

                let mode_text = if self.constant_pressure_mode { "Constant" } else { "Variable" };
                let mode_color = if !self.constant_pressure_mode { Color32::from_rgb(76, 175, 80) } else { Color32::from_rgb(33, 150, 243) };

                if ui.add(egui::Button::new(RichText::new(mode_text).color(Color32::WHITE).size(20.0)).fill(mode_color)).clicked() {
                    self.constant_pressure_mode = !self.constant_pressure_mode;
                    state_changed = true;
                }
            });

            ui.add_space(20.0);

            // Pressure Displays
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Target Pressure").size(18.0));
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        PressureDisplay::new(data.target_pressure).ui(ui);
                    });
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label(RichText::new("Current Pressure").size(18.0));
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        PressureDisplay::new(data.current_pressure).ui(ui);
                    });
                });
            });

            ui.add_space(20.0);

            // Speed Display
            egui::Frame::group(ui.style()).show(ui, |ui| {
                SpeedDisplay::new(data.speed, settings.min_speed, settings.max_speed).ui(ui);
            });

            ui.add_space(20.0);

            ui.label(format!("Boom Locked: {}", if data.boom_locked { "YES" } else { "NO" }));

            ui.add_space(20.0);

            // Nozzle Display
            ui.horizontal(|ui| {
                let color = settings.nozzle_size.color_code;
                let (rect, _) = ui.allocate_exact_size(egui::vec2(30.0, 30.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 15.0, Color32::from_rgb(color[0], color[1], color[2]));
                ui.painter().circle_stroke(rect.center(), 15.0, egui::Stroke::new(1.0, Color32::BLACK));
                
                ui.add_space(10.0);
                ui.label(RichText::new(format!("{} - {}", settings.nozzle_size.number, settings.nozzle_size.color_name)).size(18.0));
            });
        });

        (self.controller_activated, state_changed)
    }
}
