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
        // Since pressure displays are vertically stacked (taking full width of the screen),
        // we divide available_width by 7.6 (equivalent to 3.8 in a 2-column layout)
        // and clamp it between 32.0 and 64.0 to prevent size overflow.
        let font_size = (available_width / 7.6).clamp(32.0, 64.0);

        let color = if self.is_warning {
            Color32::RED
        } else {
            ui.visuals().text_color()
        };

        use egui::text::{LayoutJob, TextFormat};
        let mut job = LayoutJob::default();
        job.append(
            &format!("{:.2}", self.pressure_value),
            0.0,
            TextFormat {
                font_id: FontId::new(font_size, FontFamily::Monospace),
                color,
                ..Default::default()
            },
        );
        job.append(
            " bar",
            0.0,
            TextFormat {
                font_id: FontId::new(font_size * 0.35, FontFamily::Proportional),
                color,
                ..Default::default()
            },
        );
        ui.vertical_centered(|ui| {
            ui.label(job);
        });
    }
}

