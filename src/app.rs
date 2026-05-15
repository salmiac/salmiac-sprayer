use tokio::sync::broadcast;
use crate::models::sprayer_data::SprayerData;
use crate::models::sprayer_settings::SprayerSettings;
use crate::services::controller::ControllerService;
use crate::services::storage::SprayerSettingsStorage;
use crate::screens::monitor::MonitorScreen;
use crate::screens::settings::SettingsScreen;

#[derive(PartialEq)]
enum Screen {
    Monitor,
    Settings,
}

pub struct SalmiacSprayerApp {
    current_screen: Screen,
    sprayer_data: SprayerData,
    sprayer_settings: SprayerSettings,
    controller_service: ControllerService,
    data_rx: broadcast::Receiver<SprayerData>,
    monitor_screen: MonitorScreen,
    settings_screen: SettingsScreen,
}

impl SalmiacSprayerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = SprayerSettingsStorage::load_settings().unwrap_or_default();
        let (controller_service, data_rx) = ControllerService::new();
        
        // Start UDP Receiver
        let srv = controller_service.clone();
        tokio::spawn(async move {
            let _ = srv.start_udp_receiver(1111).await;
        });

        Self {
            current_screen: Screen::Monitor,
            sprayer_data: SprayerData::default(),
            sprayer_settings: settings.clone(),
            controller_service,
            data_rx,
            monitor_screen: MonitorScreen::new(),
            settings_screen: SettingsScreen::new(settings),
        }
    }
}

impl eframe::App for SalmiacSprayerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        // Receive data from background task
        while let Ok(data) = self.data_rx.try_recv() {
            self.sprayer_data = data;
            ctx.request_repaint(); // Ensure UI updates on new data
        }

        let insets = ctx.input(|i| i.safe_area_insets());
        #[cfg_attr(not(target_os = "android"), allow(unused_mut))]
        let mut top_margin = insets.0.top;
        #[cfg(target_os = "android")]
        if top_margin == 0.0 {
            top_margin = 30.0; // Fallback for status bar
        }

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin {
                    top: (top_margin as i8).saturating_add(8),
                    bottom: 8,
                    left: 8,
                    right: 8,
                })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.current_screen, Screen::Monitor, "Monitor");
                        ui.selectable_value(&mut self.current_screen, Screen::Settings, "Settings");
                    });
                });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.current_screen {
                Screen::Monitor => {
                    let (activated, changed) = self.monitor_screen.ui(ui, &self.sprayer_data, &self.sprayer_settings);
                    if changed {
                        let _ = self.controller_service.send_button_state(
                            "255.255.255.255", 
                            8888, 
                            activated, 
                            self.monitor_screen.constant_pressure_mode
                        );
                    }
                }
                Screen::Settings => {
                    if self.settings_screen.ui(ui) {
                        // Save clicked
                        self.sprayer_settings = self.settings_screen.settings.clone();
                        let _ = SprayerSettingsStorage::save_settings(&self.sprayer_settings);
                        let _ = self.controller_service.send_settings("255.255.255.255", 8888, &self.sprayer_settings);
                    }
                }
            }
        });
    }
}
