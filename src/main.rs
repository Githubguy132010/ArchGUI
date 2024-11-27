mod ui;
mod installer;
mod updater;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use libadwaita as adw;
use updater::Updater;

const APP_ID: &str = "org.archlinux.installer";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "yourusername/arch-installer";

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    
    // Check voor updates
    let updater = Updater::new(VERSION.to_string(), GITHUB_REPO.to_string());
    if let Ok(Some(new_version)) = updater.check_for_updates().await {
        println!("Nieuwe versie {} beschikbaar, updaten...", new_version);
        if let Err(e) = updater.update().await {
            eprintln!("Fout bij updaten: {}", e);
        }
    }
    
    // Initialize libadwaita
    adw::init().unwrap();

    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Arch Linux Installer")
        .default_width(800)
        .default_height(600)
        .build();

    let installer_ui = ui::InstallerUI::new();
    window.set_child(Some(installer_ui.widget()));
    
    window.present();
} 