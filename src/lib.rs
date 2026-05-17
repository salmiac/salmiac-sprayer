mod models;
pub mod services;
pub mod widgets;
pub mod screens;
pub mod app;

use app::SalmiacSprayerApp;

// Desktop entry point logic
pub fn desktop_main() -> Result<(), eframe::Error> {
    env_logger::init();

    let icon_data = include_bytes!("../assets/logo_64.png");
    let icon = image::load_from_memory(icon_data)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = icon.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 800.0])
            .with_icon(egui::IconData {
                rgba: icon.into_raw(),
                width,
                height,
            }),
        ..Default::default()
    };

    eframe::run_native(
        "Salmiac Sprayer",
        options,
        Box::new(|cc| Ok(Box::new(SalmiacSprayerApp::new(cc)))),
    )
}

// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    // Create a tokio runtime for Android
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let options = NativeOptions {
        android_app: Some(app.clone()),
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    eframe::run_native(
        "Salmiac Sprayer",
        options,
        Box::new(|cc| Ok(Box::new(SalmiacSprayerApp::new(cc)))),
    ).unwrap();
}
