use egui::{RichText, Ui};

pub struct PressureDisplay {
    pub pressure_value: f32,
}

impl PressureDisplay {
    pub fn new(pressure_value: f32) -> Self {
        Self { pressure_value }
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{:.1}", self.pressure_value))
                        .size(72.0)
                        .strong(),
                );
                ui.label(RichText::new(" bar").size(18.0));
            });
        });
    }
}
