mod app_mode;
mod search_window;

use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "com.github.prajwalch.launchthing";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.set_accels_for_action("window.close", &["Escape"]);

    app.connect_startup(|_| load_css());
    app.connect_activate(|app| {
        let window = search_window::SearchWindow::new(app);
        window.present();
    });
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style/app.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Display should exist"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
