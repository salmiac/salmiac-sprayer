use crate::models::sprayer_data::SprayerData;
use crate::models::sprayer_settings::SprayerSettings;
use crate::widgets::pressure_display::PressureDisplay;
use crate::widgets::speed_display::SpeedDisplay;
use egui::{Color32, RichText, Ui};

pub struct MonitorScreen {
    pub controller_activated: bool,
    pub constant_pressure_mode: bool,
}

impl Default for MonitorScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorScreen {
    pub fn new() -> Self {
        Self {
            controller_activated: false,
            constant_pressure_mode: false,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        data: &SprayerData,
        settings: &SprayerSettings,
        is_connected: bool,
    ) -> (bool, bool, bool) {
        let mut state_changed = false;

        let pressure_discrepancy = (data.target_pressure - data.current_pressure).abs();
        let pressure_warning = self.controller_activated
            && data.target_pressure > 0.1
            && pressure_discrepancy > settings.pressure_alert_threshold;

        ui.vertical_centered(|ui| {
            // Status and Buttons Row
            ui.horizontal(|ui| {
                // Connection Status Indicator
                let (status_text, status_color): (std::borrow::Cow<'_, str>, Color32) =
                    if is_connected {
                        (rust_i18n::t!("CONNECTED"), Color32::from_rgb(76, 175, 80))
                    // Green
                    } else {
                        (
                            rust_i18n::t!("DISCONNECTED"),
                            Color32::from_rgb(244, 67, 54),
                        ) // Red
                    };

                egui::Frame::group(ui.style())
                    .fill(status_color.gamma_multiply(0.1))
                    .stroke(egui::Stroke::new(1.0, status_color))
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(status_text)
                                .color(status_color)
                                .strong()
                                .small(),
                        );
                    });

                ui.add_space(8.0);

                let btn_text = if self.controller_activated {
                    rust_i18n::t!("Controller ON")
                } else {
                    rust_i18n::t!("Controller OFF")
                };
                let btn_color = if self.controller_activated {
                    Color32::from_rgb(76, 175, 80)
                } else {
                    Color32::from_rgb(33, 150, 243)
                };

                if ui
                    .add(
                        egui::Button::new(RichText::new(btn_text).color(Color32::WHITE).size(20.0))
                            .fill(btn_color),
                    )
                    .clicked()
                {
                    self.controller_activated = !self.controller_activated;
                    state_changed = true;
                }

                let mode_text = if self.constant_pressure_mode {
                    rust_i18n::t!("Constant")
                } else {
                    rust_i18n::t!("Variable")
                };
                let mode_color = if !self.constant_pressure_mode {
                    Color32::from_rgb(76, 175, 80)
                } else {
                    Color32::from_rgb(33, 150, 243)
                };

                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(mode_text).color(Color32::WHITE).size(20.0),
                        )
                        .fill(mode_color),
                    )
                    .clicked()
                {
                    self.constant_pressure_mode = !self.constant_pressure_mode;
                    state_changed = true;
                }
            });

            if pressure_warning {
                ui.add_space(8.0);
                egui::Frame::group(ui.style())
                    .fill(Color32::RED.gamma_multiply(0.1))
                    .stroke(egui::Stroke::new(1.0, Color32::RED))
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.label(
                            RichText::new(rust_i18n::t!("⚠ PRESSURE DISCREPANCY DETECTED"))
                                .color(Color32::RED)
                                .strong()
                                .size(20.0),
                        );
                    });
            }

            ui.add_space(20.0);

            // Pressure Displays
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(rust_i18n::t!("Target Pressure")).size(18.0));
                egui::Frame::group(ui.style())
                    .fill(ui.style().visuals.faint_bg_color)
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        PressureDisplay::new(data.target_pressure, false).ui(ui);
                    });

                ui.add_space(16.0);

                ui.label(RichText::new(rust_i18n::t!("Current Pressure")).size(18.0));
                egui::Frame::group(ui.style())
                    .fill(ui.style().visuals.faint_bg_color)
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        PressureDisplay::new(data.current_pressure, pressure_warning).ui(ui);
                    });
            });

            ui.add_space(20.0);

            // Speed Display
            egui::Frame::group(ui.style())
                .fill(ui.style().visuals.faint_bg_color)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.vertical_centered(|ui| {
                        SpeedDisplay::new(data.speed, settings.min_speed, settings.max_speed)
                            .ui(ui);
                    });
                });

            ui.add_space(20.0);

            ui.label(format!(
                "{}: {}",
                rust_i18n::t!("Boom Locked"),
                if data.boom_locked {
                    rust_i18n::t!("YES")
                } else {
                    rust_i18n::t!("NO")
                }
            ));

            ui.add_space(20.0);

            // Nozzle Display
            ui.horizontal(|ui| {
                let color = settings.nozzle_size.color_code;
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(30.0, 30.0), egui::Sense::hover());
                ui.painter().circle_filled(
                    rect.center(),
                    15.0,
                    Color32::from_rgb(color[0], color[1], color[2]),
                );
                ui.painter().circle_stroke(
                    rect.center(),
                    15.0,
                    egui::Stroke::new(1.0, Color32::BLACK),
                );

                ui.add_space(10.0);
                ui.label(
                    RichText::new(format!(
                        "{} - {}",
                        settings.nozzle_size.number, settings.nozzle_size.color_name
                    ))
                    .size(18.0),
                );
            });
        });

        (self.controller_activated, state_changed, pressure_warning)
    }
}
