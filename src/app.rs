use crate::models::sprayer_data::SprayerData;
use crate::models::sprayer_settings::{AppLanguage, SprayerSettings, ThemeMode};
use crate::protocol::{DEFAULT_COMMAND_PORT, DEFAULT_STATUS_PORT};
use crate::screens::monitor::MonitorScreen;
use crate::screens::settings::SettingsScreen;
use crate::services::audio::AudioService;
use crate::services::controller::ControllerService;
use crate::services::storage::SprayerSettingsStorage;
use egui::{Color32, FontData, FontDefinitions, FontFamily, RichText};
use std::time::Instant;
use tokio::sync::broadcast;

#[derive(PartialEq, Clone, Copy)]
enum Screen {
    Monitor,
    Settings,
}

pub struct SalmiacSprayerApp {
    current_screen: Screen,
    sprayer_data: SprayerData,
    sprayer_settings: SprayerSettings,
    controller_service: ControllerService,
    audio_service: AudioService,
    data_rx: broadcast::Receiver<SprayerData>,
    monitor_screen: MonitorScreen,
    settings_screen: SettingsScreen,
    show_nav_warning: bool,
    last_data_received: Option<Instant>,
    last_beep_time: Instant,
}

impl SalmiacSprayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let settings = SprayerSettingsStorage::load_settings().unwrap_or_default();
        let (controller_service, data_rx) = ControllerService::new();
        let audio_service = AudioService::new();

        // Load custom font
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Michroma".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/Michroma-Regular.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "Michroma".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        // Start UDP Receiver
        let srv = controller_service.clone();
        tokio::spawn(async move {
            if let Err(e) = srv.start_udp_receiver(DEFAULT_STATUS_PORT).await {
                log::error!("Failed to start UDP receiver: {}", e);
            }
        });

        // Apply initial settings
        apply_theme(&cc.egui_ctx, settings.theme_mode);
        apply_language(settings.app_language);

        Self {
            current_screen: Screen::Monitor,
            sprayer_data: SprayerData::default(),
            sprayer_settings: settings.clone(),
            controller_service,
            audio_service,
            data_rx,
            monitor_screen: MonitorScreen::new(),
            settings_screen: SettingsScreen::new(settings),
            show_nav_warning: false,
            last_data_received: None,
            last_beep_time: Instant::now(),
        }
    }
}

impl eframe::App for SalmiacSprayerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // Receive data from background task
        while let Ok(data) = self.data_rx.try_recv() {
            self.sprayer_data = data;
            self.last_data_received = Some(Instant::now());
            ctx.request_repaint(); // Ensure UI updates on new data
        }

        let is_connected = if let Some(last) = self.last_data_received {
            last.elapsed().as_secs_f32() < 2.0
        } else {
            false
        };

        if !is_connected && self.last_data_received.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(500));
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
                        let settings_dirty = self.settings_screen.is_dirty();

                        // Use a temporary variable for selection
                        let selected_screen = self.current_screen;

                        let mon_resp = ui.add(
                            egui::Button::new(
                                RichText::new(format!("📊 {}", rust_i18n::t!("Monitor")))
                                    .size(20.0),
                            )
                            .selected(selected_screen == Screen::Monitor),
                        );
                        let set_resp = ui.add(
                            egui::Button::new(
                                RichText::new(format!("⚙ {}", rust_i18n::t!("Settings")))
                                    .size(20.0),
                            )
                            .selected(selected_screen == Screen::Settings),
                        );

                        if mon_resp.clicked() {
                            if settings_dirty {
                                self.show_nav_warning = true;
                            } else {
                                self.current_screen = Screen::Monitor;
                                self.show_nav_warning = false;
                            }
                        }

                        if set_resp.clicked() {
                            self.current_screen = Screen::Settings;
                            // self.show_nav_warning = false; // Optional: reset warning when switching to Settings
                        }

                        if self.show_nav_warning && settings_dirty {
                            ui.add_space(16.0);
                            ui.label(
                                RichText::new(format!(
                                    "⚠ {}",
                                    rust_i18n::t!("Save or Reset changes before leaving!")
                                ))
                                .color(Color32::RED)
                                .small(),
                            );
                        } else if !settings_dirty {
                            self.show_nav_warning = false;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                egui::Image::new(egui::include_image!("../assets/logo_64.png"))
                                    .max_width(32.0),
                            );
                        });
                    });
                });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.current_screen {
                Screen::Monitor => {
                    let (activated, changed, warning) = self.monitor_screen.ui(
                        ui,
                        &self.sprayer_data,
                        &self.sprayer_settings,
                        is_connected,
                    );
                    if changed {
                        let _ = self.controller_service.send_button_state(
                            &self.sprayer_settings.target_ip,
                            DEFAULT_COMMAND_PORT,
                            activated,
                            self.monitor_screen.constant_pressure_mode,
                        );
                    }

                    if warning && self.last_beep_time.elapsed().as_secs_f32() > 2.0 {
                        self.audio_service.play_beep();
                        self.last_beep_time = Instant::now();
                    }
                }
                Screen::Settings => {
                    if self.settings_screen.ui(ui) {
                        // Save clicked
                        self.sprayer_settings = self.settings_screen.settings.clone();
                        let _ = SprayerSettingsStorage::save_settings(&self.sprayer_settings);
                        let _ = self.controller_service.send_settings(
                            &self.sprayer_settings.target_ip,
                            DEFAULT_COMMAND_PORT,
                            &self.sprayer_settings,
                        );
                        self.show_nav_warning = false;

                        apply_theme(&ctx, self.sprayer_settings.theme_mode);
                        apply_language(self.sprayer_settings.app_language);
                    }
                }
            }
        });

        // Ensure we keep repainting to poll the network receiver and update timers
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

pub fn apply_theme(ctx: &egui::Context, theme: ThemeMode) {
    match theme {
        ThemeMode::System => {
            // Let egui decide based on OS preference (egui does this by default if we reset)
            // But since we might be switching back from Light/Dark, we can set it to Dark as fallback or just use what it had.
            // A simple approach is just clear our override, but egui Visuals is global.
            // Let's use Dark mode as the default for System if we can't fetch OS easily.
            ctx.set_visuals(egui::Visuals::dark());
        }
        ThemeMode::Light => ctx.set_visuals(egui::Visuals::light()),
        ThemeMode::Dark => ctx.set_visuals(egui::Visuals::dark()),
    }
}

pub fn apply_language(lang: AppLanguage) {
    match lang {
        AppLanguage::System => {
            let loc = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
            let lang_code = loc.split('-').next().unwrap_or("en");
            rust_i18n::set_locale(lang_code);
        }
        AppLanguage::English => rust_i18n::set_locale("en"),
        AppLanguage::Finnish => rust_i18n::set_locale("fi"),
        AppLanguage::Swedish => rust_i18n::set_locale("sv"),
        AppLanguage::Spanish => rust_i18n::set_locale("es"),
        AppLanguage::German => rust_i18n::set_locale("de"),
        AppLanguage::French => rust_i18n::set_locale("fr"),
        AppLanguage::Portuguese => rust_i18n::set_locale("pt"),
        AppLanguage::Italian => rust_i18n::set_locale("it"),
        AppLanguage::Polish => rust_i18n::set_locale("pl"),
        AppLanguage::Dutch => rust_i18n::set_locale("nl"),
        AppLanguage::Danish => rust_i18n::set_locale("da"),
        AppLanguage::Turkish => rust_i18n::set_locale("tr"),
        AppLanguage::Czech => rust_i18n::set_locale("cs"),
        AppLanguage::Hungarian => rust_i18n::set_locale("hu"),
        AppLanguage::Estonian => rust_i18n::set_locale("et"),
    }
}
