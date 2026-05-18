use egui::{RichText, Ui, FontId, FontFamily, Color32};

pub struct PressureDisplay {
    pub pressure_value: f32,
    pub is_warning: bool,
}

impl PressureDisplay {
    pub fn new(pressure_value: f32, is_warning: bool) -> Self {
        Self { pressure_value, is_warning }
    }

    pub fn ui(&self, ui: &mut Ui) {
        let available_width = ui.available_width();
        // Science Gothic appears to be wider than the previous font.
        // Using a more conservative ratio (3.8 instead of 2.8) to ensure "00.00" fits.
        let font_size = (available_width / 3.8).clamp(32.0, 128.0);

        let color = if self.is_warning {
            Color32::RED
        } else {
            ui.visuals().text_color()
        };

        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(format!("{:.2}", self.pressure_value))
                    .font(FontId::new(font_size, FontFamily::Monospace))
                    .color(color)
                    .strong(),
            );
            ui.label(RichText::new("bar").size(font_size * 0.25).color(color));
        });
    }
}

