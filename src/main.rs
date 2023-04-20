use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Orientation;
use gtk::SearchEntry;
use gtk::Text;

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

    // TODO: Implement a custom container box to display search results
    let search_results = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    container.append(&search_results);

    search_box.connect_search_changed(
        clone!(@weak container, @weak search_results => move |term| {
            let term = term.text();
            let matched_terms = get_matched_terms(&term);

            if term.is_empty() || matched_terms.is_empty() {
                // TODO: container box of search results should be clear instead of hiding it
                search_results.hide();
                return;
            }
            for term in matched_terms {
                // FIXME: Temporary reference should not be use to create text widget
                //        because it makes impossible to remove them later from container
                search_results.append(&Text::builder().text(term).build());
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
