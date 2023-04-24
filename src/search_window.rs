use gtk::gio;
use gtk::glib::{self, clone};
use gtk::prelude::*;

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    container: gtk::Box,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_height_request(50);
        window.set_width_request(500);
        window.set_resizable(false);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        window.set_child(Some(&container));

        Self { window, container }
    }

    pub fn present(&self) {
        self.container.append(&self.build_search_box_widget());
        self.container
            .append(&self.build_search_results_container());
        self.window.present();
    }

    fn build_search_box_widget(&self) -> gtk::SearchEntry {
        let search_box = gtk::SearchEntry::builder().hexpand(true).build();
        search_box.connect_search_changed(move |search_box| {
            let search_query = search_box.text().to_string();
            search_box
                .activate_action("win.search", Some(&search_query.to_variant()))
                .expect("search action should exist");
        });
        search_box
    }

    fn build_search_results_container(&self) -> crate::search_results::SearchResults {
        let search_results = crate::search_results::SearchResults::new();
        let search_action = gio::SimpleAction::new("search", Some(&String::static_variant_type()));

        search_action.connect_activate(clone!(@weak search_results => move |_state, variant| {
            if let Some(variant) = variant {
                // Clear previous results
                search_results.clear();

                let search_query = variant.get::<String>().unwrap_or_default();
                if search_query.is_empty() {
                    return;
                }
                if let Some(results) = get_matched_terms(&search_query) {
                    for result in results {
                        search_results.push(gtk::Text::builder().text(&result).build());
                    }
                }
            }
        }));
        self.window.add_action(&search_action);
        search_results
    }
}

// TODO: Remove this function
fn get_matched_terms(term: &str) -> Option<Vec<String>> {
    let fake = vec!["Obs", "Clion", "Vim", "Nvim", "Visual Studio Code"];

    let matched_terms = fake
        .iter()
        .filter(|f| f.matches(term).count() != 0)
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    if !matched_terms.is_empty() {
        Some(matched_terms)
    } else {
        None
    }
}
