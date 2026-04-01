use egui::{Ui, Color32, RichText};
use crate::models::sprayer_settings::{SprayerSettings, get_nozzle_types};

const NOZZLE_CONSTANT: f32 = 2.3095;

pub struct SettingsScreen {
    pub settings: SprayerSettings,
    // Intermediate strings for TextEdit to avoid parsing issues during typing
    nozzle_spacing_str: String,
    litres_per_ha_str: String,
    min_pressure_str: String,
    max_pressure_str: String,
    nominal_pressure_str: String,
}

impl SettingsScreen {
    pub fn new(settings: SprayerSettings) -> Self {
        Self {
            nozzle_spacing_str: settings.nozzle_spacing.to_string(),
            litres_per_ha_str: settings.litres_per_ha.to_string(),
            min_pressure_str: settings.min_pressure.to_string(),
            max_pressure_str: settings.max_pressure.to_string(),
            nominal_pressure_str: settings.nominal_pressure.to_string(),
            settings,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;
        let mut saved = false;

        ui.vertical(|ui| {
            ui.heading("Settings");
            ui.add_space(16.0);

            // Nozzle Spacing
            ui.horizontal(|ui| {
                ui.label("Nozzle Spacing (meters):");
                if ui.text_edit_singleline(&mut self.nozzle_spacing_str).changed() {
                    if let Ok(val) = self.nozzle_spacing_str.parse::<f32>() {
                        if val >= 0.1 && val <= 2.0 {
                            self.settings.nozzle_spacing = val;
                            changed = true;
                        }
                    }
                }
            });

            // Nozzle Size Dropdown
            ui.horizontal(|ui| {
                ui.label("Nozzle Size:");
                let nozzle_types = get_nozzle_types();
                egui::ComboBox::from_id_salt("nozzle_size")
                    .selected_text(format!("{} - {}", self.settings.nozzle_size.number, self.settings.nozzle_size.color_name))
                    .show_ui(ui, |ui| {
                        for nozzle in nozzle_types {
                            let color = nozzle.color_code;
                            ui.horizontal(|ui| {
                                let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                                ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(color[0], color[1], color[2]));
                                if ui.selectable_value(&mut self.settings.nozzle_size, nozzle.clone(), format!("{} - {}", nozzle.number, nozzle.color_name)).clicked() {
                                    changed = true;
                                }
                            });
                        }
                    });
            });

            // Litres/ha
            ui.horizontal(|ui| {
                ui.label("Litres/ha (10-999):");
                if ui.text_edit_singleline(&mut self.litres_per_ha_str).changed() {
                    if let Ok(val) = self.litres_per_ha_str.parse::<f32>() {
                        if val >= 10.0 && val <= 999.0 {
                            self.settings.litres_per_ha = val;
                            changed = true;
                        }
                    }
                }
            });

            // Pressures with calculated speeds
            if let Some(val) = pressure_input_row(ui, "Min Pressure (1-10)", &mut self.min_pressure_str, self.settings.min_pressure, &self.settings) {
                self.settings.min_pressure = val;
                changed = true;
            }
            if let Some(val) = pressure_input_row(ui, "Max Pressure (1-10)", &mut self.max_pressure_str, self.settings.max_pressure, &self.settings) {
                self.settings.max_pressure = val;
                changed = true;
            }
            if let Some(val) = pressure_input_row(ui, "Nominal Pressure (1-10)", &mut self.nominal_pressure_str, self.settings.nominal_pressure, &self.settings) {
                self.settings.nominal_pressure = val;
                changed = true;
            }

            ui.add_space(24.0);

            if ui.button("Save Settings").clicked() {
                saved = true;
            }

            if ui.button("Reset changes").clicked() {
                self.nozzle_spacing_str = self.settings.nozzle_spacing.to_string();
                self.litres_per_ha_str = self.settings.litres_per_ha.to_string();
                self.min_pressure_str = self.settings.min_pressure.to_string();
                self.max_pressure_str = self.settings.max_pressure.to_string();
                self.nominal_pressure_str = self.settings.nominal_pressure.to_string();
            }
        });

        if changed {
            self.calculate_speeds();
        }

        saved
    }

    fn calculate_speeds(&mut self) {
        self.settings.min_speed = calculate_speed_for_pressure(&self.settings, self.settings.min_pressure);
        self.settings.max_speed = calculate_speed_for_pressure(&self.settings, self.settings.max_pressure);
    }
}

fn pressure_input_row(ui: &mut Ui, label: &str, string_val: &mut String, current_val: f32, settings: &SprayerSettings) -> Option<f32> {
    let mut new_val = None;
    ui.horizontal(|ui| {
        ui.label(label);
        if ui.text_edit_singleline(string_val).changed() {
            if let Ok(val) = string_val.parse::<f32>() {
                if val >= 1.0 && val <= 10.0 {
                    new_val = Some(val);
                }
            }
        }
        let speed_val = new_val.unwrap_or(current_val);
        let speed = calculate_speed_for_pressure(settings, speed_val);
        ui.label(RichText::new(format!("{:.1} km/h", speed)).color(Color32::GRAY));
    });
    new_val
}

fn calculate_speed_for_pressure(settings: &SprayerSettings, pressure: f32) -> f32 {
    let speed_per_sqrt_pressure = NOZZLE_CONSTANT 
        * settings.nozzle_size.size_value 
        * 600.0 
        / (settings.litres_per_ha * settings.nozzle_spacing);
    pressure.sqrt() * speed_per_sqrt_pressure
}
