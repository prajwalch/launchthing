use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, SearchEntry};

const APP_ID: &str = "org.gtk_rs.la";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_window);
    app.run()
}

fn build_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("la")
        .height_request(50)
        .width_request(500)
        .resizable(false)
        .child(&build_widgets_box())
        .build();
    window.present();
}

/// Builds the container box with all the widgets necessary for an app
fn build_widgets_box() -> gtk::Box {
    let search_box = SearchEntry::builder().hexpand(true).build();
    let gtk_box = gtk::Box::builder().build();
    gtk_box.append(&search_box);

    gtk_box
}
