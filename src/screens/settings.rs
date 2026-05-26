use crate::models::sprayer_settings::{get_nozzle_types, AppLanguage, SprayerSettings, ThemeMode};
use egui::{Color32, FontFamily, FontId, RichText, Ui};

const NOZZLE_CONSTANT: f32 = 2.3095;

pub struct SettingsScreen {
    pub settings: SprayerSettings,
    original_settings: SprayerSettings,
    // Intermediate strings for TextEdit to avoid parsing issues during typing
    nozzle_spacing_str: String,
    litres_per_ha_str: String,
    min_pressure_str: String,
    max_pressure_str: String,
    nominal_pressure_str: String,
    pressure_alert_threshold_str: String,
    target_ip_str: String,
}

impl SettingsScreen {
    pub fn new(settings: SprayerSettings) -> Self {
        let mut screen = Self {
            nozzle_spacing_str: String::new(),
            litres_per_ha_str: String::new(),
            min_pressure_str: String::new(),
            max_pressure_str: String::new(),
            nominal_pressure_str: String::new(),
            pressure_alert_threshold_str: String::new(),
            target_ip_str: String::new(),
            original_settings: settings.clone(),
            settings,
        };
        screen.sync_strings();
        screen
    }

    pub fn is_dirty(&self) -> bool {
        // Compare with original settings to see if anything actually changed
        // We use a simplified comparison since Nozzle implements PartialEq
        self.settings.nozzle_size != self.original_settings.nozzle_size
            || self.settings.litres_per_ha != self.original_settings.litres_per_ha
            || self.settings.min_pressure != self.original_settings.min_pressure
            || self.settings.max_pressure != self.original_settings.max_pressure
            || self.settings.nominal_pressure != self.original_settings.nominal_pressure
            || self.settings.nozzle_spacing != self.original_settings.nozzle_spacing
            || self.settings.pressure_alert_threshold
                != self.original_settings.pressure_alert_threshold
            || self.settings.target_ip != self.original_settings.target_ip
            || self.settings.theme_mode != self.original_settings.theme_mode
            || self.settings.app_language != self.original_settings.app_language
    }

    pub fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;
        let mut saved_clicked = false;

        ui.vertical(|ui| {
            ui.heading(rust_i18n::t!("Settings"));
            ui.add_space(16.0);

            // Nozzle Size Selector with - and + buttons
            ui.horizontal(|ui| {
                ui.label(RichText::new(rust_i18n::t!("Nozzle Size")).size(16.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let nozzle_types = get_nozzle_types();
                    let current_index = nozzle_types.iter().position(|n| n.number == self.settings.nozzle_size.number);

                    // Plus button (next size)
                    if ui.add_sized([30.0, 30.0], egui::Button::new(RichText::new("+").size(20.0))).clicked() {
                        if let Some(idx) = current_index {
                            if idx + 1 < nozzle_types.len() {
                                self.settings.nozzle_size = nozzle_types[idx + 1].clone();
                                changed = true;
                            }
                        }
                    }

                    ui.add_space(4.0);

                    // ComboBox
                    let selected_text = RichText::new(format!("{} - {}", self.settings.nozzle_size.number, self.settings.nozzle_size.color_name))
                        .font(FontId::new(18.0, FontFamily::Proportional));

                    let combobox_types = nozzle_types.clone();
                    egui::ComboBox::from_id_salt("nozzle_size")
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            for nozzle in combobox_types {
                                let color = nozzle.color_code;
                                ui.horizontal(|ui| {
                                    let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                                    ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(color[0], color[1], color[2]));
                                    let item_text = RichText::new(format!("{} - {}", nozzle.number, nozzle.color_name))
                                        .font(FontId::new(18.0, FontFamily::Proportional));
                                    if ui.selectable_value(&mut self.settings.nozzle_size, nozzle.clone(), item_text).clicked() {
                                        changed = true;
                                    }
                                });
                            }
                        });

                    ui.add_space(4.0);

                    // Color indicator for selected nozzle
                    let current_color = self.settings.nozzle_size.color_code;
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 4.0, Color32::from_rgb(current_color[0], current_color[1], current_color[2]));
                    ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, Color32::GRAY), egui::StrokeKind::Outside);

                    ui.add_space(4.0);

                    // Minus button (previous size)
                    if ui.add_sized([30.0, 30.0], egui::Button::new(RichText::new("-").size(20.0))).clicked() {
                        if let Some(idx) = current_index {
                            if idx > 0 {
                                self.settings.nozzle_size = nozzle_types[idx - 1].clone();
                                changed = true;
                            }
                        }
                    }
                });
            });

            ui.add_space(8.0);

            // Nozzle Spacing
            if let Some(val) = numeric_row(ui, &rust_i18n::t!("Nozzle Spacing (m)"), &mut self.nozzle_spacing_str, self.settings.nozzle_spacing, 0.1, 2.0) {
                self.settings.nozzle_spacing = val;
                changed = true;
            }

            // Litres/ha
            if let Some(val) = numeric_row(ui, &rust_i18n::t!("Litres/ha (10-999)"), &mut self.litres_per_ha_str, self.settings.litres_per_ha, 10.0, 999.0) {
                self.settings.litres_per_ha = val;
                changed = true;
            }

            // Pressure Alert Threshold
            if let Some(val) = numeric_row(ui, &rust_i18n::t!("Pressure Alert (bar)"), &mut self.pressure_alert_threshold_str, self.settings.pressure_alert_threshold, 0.1, 2.0) {
                self.settings.pressure_alert_threshold = val;
                changed = true;
            }

            // Target IP
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(rust_i18n::t!("Target IP")).size(16.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let edit = egui::TextEdit::singleline(&mut self.target_ip_str)
                            .font(FontId::new(16.0, FontFamily::Monospace))
                            .desired_width(250.0);
                        let response = ui.add(edit);
                        if response.changed()
                            && self.target_ip_str.parse::<std::net::Ipv4Addr>().is_ok() {
                                self.settings.target_ip = self.target_ip_str.clone();
                                changed = true;
                            }
                        if response.lost_focus()
                            && self.target_ip_str.parse::<std::net::Ipv4Addr>().is_err() {
                                self.target_ip_str = self.settings.target_ip.clone();
                            }
                    });
                });
                if self.target_ip_str.parse::<std::net::Ipv4Addr>().is_err() && !self.target_ip_str.is_empty() {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(rust_i18n::t!("Invalid IPv4 address")).color(Color32::RED).size(12.0));
                        });
                    });
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Min Pressure
            let min_warning = if let Ok(val) = self.min_pressure_str.parse::<f32>() {
                if val > self.settings.max_pressure { Some(rust_i18n::t!("Min cannot exceed Max").to_string()) } else { None }
            } else { None };

            if let Some(val) = pressure_row(ui, &rust_i18n::t!("Min Pressure"), &mut self.min_pressure_str, self.settings.min_pressure, &self.settings, min_warning) {
                if val <= self.settings.max_pressure {
                    self.settings.min_pressure = val;
                    if self.settings.nominal_pressure < val {
                        self.settings.nominal_pressure = val;
                        self.nominal_pressure_str = format!("{:.1}", val);
                    }
                    changed = true;
                }
            }

            // Max Pressure
            let max_warning = if let Ok(val) = self.max_pressure_str.parse::<f32>() {
                if val < self.settings.min_pressure { Some(rust_i18n::t!("Max cannot be below Min").to_string()) } else { None }
            } else { None };

            if let Some(val) = pressure_row(ui, &rust_i18n::t!("Max Pressure"), &mut self.max_pressure_str, self.settings.max_pressure, &self.settings, max_warning) {
                if val >= self.settings.min_pressure {
                    self.settings.max_pressure = val;
                    if self.settings.nominal_pressure > val {
                        self.settings.nominal_pressure = val;
                        self.nominal_pressure_str = format!("{:.1}", val);
                    }
                    changed = true;
                }
            }

            // Nominal Pressure
            let nom_warning = if let Ok(val) = self.nominal_pressure_str.parse::<f32>() {
                if val < self.settings.min_pressure || val > self.settings.max_pressure {
                    Some(rust_i18n::t!("Must be between Min and Max").to_string())
                } else { None }
            } else { None };

            if let Some(val) = pressure_row(ui, &rust_i18n::t!("Nominal Pressure"), &mut self.nominal_pressure_str, self.settings.nominal_pressure, &self.settings, nom_warning) {
                if val >= self.settings.min_pressure && val <= self.settings.max_pressure {
                    self.settings.nominal_pressure = val;
                    changed = true;
                }
            }

            ui.add_space(24.0);

            ui.horizontal(|ui| {
                let save_btn = egui::Button::new(RichText::new(rust_i18n::t!("Save")).size(18.0));
                if ui.add_sized([100.0, 40.0], save_btn).clicked() {
                    self.original_settings = self.settings.clone();
                    saved_clicked = true;
                }

                ui.add_space(8.0);

                let reset_btn = egui::Button::new(RichText::new(rust_i18n::t!("Reset")).size(18.0));
                if ui.add_sized([100.0, 40.0], reset_btn).clicked() {
                    self.settings = self.original_settings.clone();
                    self.sync_strings();
                    changed = true;
                }

                ui.add_space(8.0);

                let defaults_btn = egui::Button::new(RichText::new(rust_i18n::t!("Defaults")).size(18.0));
                if ui.add_sized([100.0, 40.0], defaults_btn).clicked() {
                    self.settings = SprayerSettings::default();
                    self.sync_strings();
                    changed = true;
                }
            });

            ui.add_space(32.0);
            ui.separator();

            // UI Preferences (Theme & Language)
            let prev_theme = self.settings.theme_mode;
            let prev_lang = self.settings.app_language;

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(rust_i18n::t!("Theme")).size(16.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let selected_text: String = match self.settings.theme_mode {
                        ThemeMode::System => rust_i18n::t!("System").into_owned(),
                        ThemeMode::Light => rust_i18n::t!("Light").into_owned(),
                        ThemeMode::Dark => rust_i18n::t!("Dark").into_owned(),
                    };
                    let selected_text = RichText::new(selected_text).font(FontId::new(16.0, FontFamily::Proportional));
                    egui::ComboBox::from_id_salt("theme_selector")
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            if ui.selectable_value(&mut self.settings.theme_mode, ThemeMode::System, rust_i18n::t!("System")).clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.theme_mode, ThemeMode::Light, rust_i18n::t!("Light")).clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.theme_mode, ThemeMode::Dark, rust_i18n::t!("Dark")).clicked() { changed = true; }
                        });
                });
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(rust_i18n::t!("Language")).size(16.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let selected_text: String = match self.settings.app_language {
                        AppLanguage::System => rust_i18n::t!("System").into_owned(),
                        AppLanguage::English => rust_i18n::t!("English").into_owned(),
                        AppLanguage::Finnish => "Suomi".to_string(),
                        AppLanguage::Swedish => "Svenska".to_string(),
                        AppLanguage::Spanish => "Español".to_string(),
                        AppLanguage::German => "Deutsch".to_string(),
                        AppLanguage::French => "Français".to_string(),
                        AppLanguage::Portuguese => "Português".to_string(),
                        AppLanguage::Italian => "Italiano".to_string(),
                        AppLanguage::Polish => "Polski".to_string(),
                        AppLanguage::Dutch => "Nederlands".to_string(),
                        AppLanguage::Danish => "Dansk".to_string(),
                        AppLanguage::Turkish => "Türkçe".to_string(),
                        AppLanguage::Czech => "Čeština".to_string(),
                        AppLanguage::Hungarian => "Magyar".to_string(),
                        AppLanguage::Estonian => "Eesti".to_string(),
                    };
                    let selected_text = RichText::new(selected_text).font(FontId::new(16.0, FontFamily::Proportional));
                    egui::ComboBox::from_id_salt("language_selector")
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::System, rust_i18n::t!("System")).clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::English, rust_i18n::t!("English")).clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Finnish, "Suomi").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Swedish, "Svenska").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Spanish, "Español").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::German, "Deutsch").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::French, "Français").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Portuguese, "Português").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Italian, "Italiano").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Polish, "Polski").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Dutch, "Nederlands").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Danish, "Dansk").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Turkish, "Türkçe").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Czech, "Čeština").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Hungarian, "Magyar").clicked() { changed = true; }
                            if ui.selectable_value(&mut self.settings.app_language, AppLanguage::Estonian, "Eesti").clicked() { changed = true; }
                        });
                });
            });

            ui.add_space(32.0);
            ui.separator();

            if prev_theme != self.settings.theme_mode {
                crate::app::apply_theme(ui.ctx(), self.settings.theme_mode);
                changed = true;
            }
            if prev_lang != self.settings.app_language {
                crate::app::apply_language(self.settings.app_language);
                changed = true;
            }

            ui.collapsing(rust_i18n::t!("About & Legal"), |ui| {
                ui.small("Salmiac Sprayer v0.1.0");
                ui.small("Copyright © 2026. Licensed under the MIT License.");

                ui.add_space(8.0);
                ui.small("Third-Party Components:");
                ui.small("• Michroma Font: Copyright © 2011 The Michroma Project Authors. Licensed under the SIL Open Font License, Version 1.1.");
                ui.small("• Built with egui and Tokio.");
            });
        });

        if changed {
            self.calculate_speeds();
        }

        saved_clicked
    }

    fn calculate_speeds(&mut self) {
        self.settings.min_speed =
            calculate_speed_for_pressure(&self.settings, self.settings.min_pressure);
        self.settings.max_speed =
            calculate_speed_for_pressure(&self.settings, self.settings.max_pressure);
    }

    fn sync_strings(&mut self) {
        self.nozzle_spacing_str = format!("{:.2}", self.settings.nozzle_spacing);
        self.litres_per_ha_str = format!("{:.0}", self.settings.litres_per_ha);
        self.min_pressure_str = format!("{:.1}", self.settings.min_pressure);
        self.max_pressure_str = format!("{:.1}", self.settings.max_pressure);
        self.nominal_pressure_str = format!("{:.1}", self.settings.nominal_pressure);
        self.pressure_alert_threshold_str =
            format!("{:.1}", self.settings.pressure_alert_threshold);
        self.target_ip_str = self.settings.target_ip.clone();
    }
}

fn numeric_row(
    ui: &mut Ui,
    label: &str,
    string_val: &mut String,
    current_val: f32,
    min: f32,
    max: f32,
) -> Option<f32> {
    let mut new_val = None;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(16.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let edit = egui::TextEdit::singleline(string_val)
                .font(FontId::new(24.0, FontFamily::Monospace))
                .desired_width(80.0);

            let response = ui.add(edit);
            if response.changed() {
                if let Ok(val) = string_val.parse::<f32>() {
                    if val >= min && val <= max {
                        new_val = Some(val);
                    }
                }
            }
            if response.lost_focus() {
                if let Ok(val) = string_val.parse::<f32>() {
                    if val < min || val > max {
                        *string_val = current_val.to_string();
                    }
                } else {
                    *string_val = current_val.to_string();
                }
            }
        });
    });
    new_val
}

fn pressure_row(
    ui: &mut Ui,
    label: &str,
    string_val: &mut String,
    current_val: f32,
    settings: &SprayerSettings,
    constraint_warning: Option<String>,
) -> Option<f32> {
    let mut result = None;
    let min_range = 1.0;
    let max_range = 10.0;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).size(16.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Plus button
                if ui
                    .add_sized(
                        [30.0, 30.0],
                        egui::Button::new(RichText::new("+").size(20.0)),
                    )
                    .clicked()
                {
                    let val = (current_val + 0.1).clamp(min_range, max_range);
                    result = Some(val);
                    *string_val = format!("{:.1}", val);
                }

                ui.add_space(4.0);

                let edit = egui::TextEdit::singleline(string_val)
                    .font(FontId::new(24.0, FontFamily::Monospace))
                    .desired_width(80.0);

                let response = ui.add(edit);
                if response.changed() {
                    if let Ok(val) = string_val.parse::<f32>() {
                        if val >= min_range && val <= max_range {
                            result = Some(val);
                        }
                    }
                }
                if response.lost_focus() {
                    if let Ok(val) = string_val.parse::<f32>() {
                        if val < min_range || val > max_range {
                            *string_val = format!("{:.1}", current_val);
                        }
                    } else {
                        *string_val = format!("{:.1}", current_val);
                    }
                }

                ui.add_space(4.0);

                // Minus button
                if ui
                    .add_sized(
                        [30.0, 30.0],
                        egui::Button::new(RichText::new("-").size(20.0)),
                    )
                    .clicked()
                {
                    let val = (current_val - 0.1).clamp(min_range, max_range);
                    result = Some(val);
                    *string_val = format!("{:.1}", val);
                }
            });
        });

        // Show warnings
        let mut warning = constraint_warning;
        if let Ok(val) = string_val.parse::<f32>() {
            if val < min_range || val > max_range {
                warning = Some(format!("Pressure must be {}-{} bar", min_range, max_range));
            }
        } else if !string_val.is_empty() {
            warning = Some(rust_i18n::t!("Invalid number").to_string());
        }

        let speed_val = result.unwrap_or(current_val);
        let speed = calculate_speed_for_pressure(settings, speed_val);

        ui.horizontal(|ui| {
            if let Some(msg) = warning {
                ui.label(RichText::new(msg).color(Color32::RED).size(12.0));
            }
            ui.add_space(ui.available_width() - 80.0); // Align speed with input
            ui.label(
                RichText::new(format!("{:.1} km/h", speed))
                    .color(Color32::GRAY)
                    .size(14.0),
            );
        });
    });
    result
}

fn calculate_speed_for_pressure(settings: &SprayerSettings, pressure: f32) -> f32 {
    let speed_per_sqrt_pressure = NOZZLE_CONSTANT * settings.nozzle_size.size_value * 600.0
        / (settings.litres_per_ha * settings.nozzle_spacing);
    pressure.sqrt() * speed_per_sqrt_pressure
}
