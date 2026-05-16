use egui::{RichText, Ui, FontId, FontFamily};

pub struct PressureDisplay {
    pub pressure_value: f32,
}

impl PressureDisplay {
    pub fn new(pressure_value: f32) -> Self {
        Self { pressure_value }
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(format!("{:.2}", self.pressure_value))
                    .font(FontId::new(72.0, FontFamily::Monospace))
                    .strong(),
            );
            ui.label(RichText::new("bar").size(18.0));
        });
    }
}
