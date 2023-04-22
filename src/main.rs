mod search_results;

use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Orientation;
use gtk::SearchEntry;
use gtk::Text;

use crate::search_results::SearchResults;

const APP_ID: &str = "org.gtk_rs.la";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_window);
    app.run()
}

fn build_window(app: &Application) {
    ApplicationWindow::builder()
        .application(app)
        .title("la")
        .height_request(50)
        .width_request(500)
        .resizable(false)
        .child(&build_box_of_all_widgets())
        .build()
        .present();
}

/// Builds the container box of all the widgets necessary for an app
fn build_box_of_all_widgets() -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let search_box = SearchEntry::builder().hexpand(true).build();
    container.append(&search_box);

    let search_results = SearchResults::new();
    container.append(&search_results);

    search_box.connect_search_changed(
        clone!(@weak container, @weak search_results => move |term| {
            let term = term.text();
            let matched_terms = get_matched_terms(&term);

            if term.is_empty() || matched_terms.is_empty() {
                search_results.clear_all();
                return;
            }
            for term in matched_terms {
                // TODO: Instead of passing individual text change it to take matched terms directly
                search_results.append_new(Text::builder().text(term).build());
            }
        }),
    );

    container
}

fn get_matched_terms(term: &str) -> Vec<&str> {
    let fake = vec!["Obs", "Clion", "Vim", "Nvim", "Visual Studio Code"];

    if fake.contains(&term) {
        fake
    } else {
        vec![]
    }
}
