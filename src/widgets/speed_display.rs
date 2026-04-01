use egui::{RichText, Ui};

pub struct SpeedDisplay {
    pub speed_value: f32,
    pub min_speed: f32,
    pub max_speed: f32,
}

impl SpeedDisplay {
    pub fn new(speed_value: f32, min_speed: f32, max_speed: f32) -> Self {
        Self {
            speed_value,
            min_speed,
            max_speed,
        }
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Min and Max row
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Min");
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{:.1}", self.min_speed)).size(32.0).strong());
                        ui.label(RichText::new(" km/h").size(14.0));
                    });
                });

                ui.add_space(40.0);

                ui.vertical(|ui| {
                    ui.label("Max");
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{:.1}", self.max_speed)).size(32.0).strong());
                        ui.label(RichText::new(" km/h").size(14.0));
                    });
                });
            });

            ui.add_space(16.0);

            // Main Speed
            ui.vertical(|ui| {
                ui.label("Speed");
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{:.1}", self.speed_value)).size(96.0).strong());
                    ui.label(RichText::new(" km/h").size(18.0));
                });
            });
        });
    }
}
