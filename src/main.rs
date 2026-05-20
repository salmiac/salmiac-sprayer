#![windows_subsystem = "windows"]
use salmiac_sprayer::desktop_main;

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    if let Err(e) = desktop_main() {
        eprintln!("Error from desktop_main: {}", e);
        std::process::exit(1);
    }
}
