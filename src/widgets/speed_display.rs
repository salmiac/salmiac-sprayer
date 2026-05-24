use egui::{RichText, Ui, FontId, FontFamily};

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
                        ui.label(
                            RichText::new(format!("{:.1}", self.min_speed))
                                .font(FontId::new(32.0, FontFamily::Monospace))
                                .strong()
                        );
                        ui.label(RichText::new(" km/h").size(14.0));
                    });
                });

                ui.add_space(40.0);

                ui.vertical(|ui| {
                    ui.label("Max");
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{:.1}", self.max_speed))
                                .font(FontId::new(32.0, FontFamily::Monospace))
                                .strong()
                        );
                        ui.label(RichText::new(" km/h").size(14.0));
                    });
                });
            });

            ui.add_space(16.0);

            // Main Speed
            ui.vertical_centered(|ui| {
                ui.label("Speed");
            });
            use egui::text::{LayoutJob, TextFormat};
            let mut job = LayoutJob::default();
            job.append(
                &format!("{:.1}", self.speed_value),
                0.0,
                TextFormat {
                    font_id: FontId::new(96.0, FontFamily::Monospace),
                    color: ui.visuals().text_color(),
                    ..Default::default()
                },
            );
            job.append(
                " km/h",
                0.0,
                TextFormat {
                    font_id: FontId::new(18.0, FontFamily::Proportional),
                    color: ui.visuals().text_color(),
                    ..Default::default()
                },
            );
            ui.vertical_centered(|ui| {
                ui.label(job);
            });
        });
    }
}
