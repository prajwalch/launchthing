use std::rc::Rc;

use gtk::gio;
use gtk::glib::{self, clone};
use gtk::prelude::*;

use crate::search_results::SearchResults;

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    search_results: SearchResults,
    installed_apps: Rc<Vec<gio::AppInfo>>,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_height_request(50);
        window.set_width_request(500);
        window.set_resizable(false);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        container.append(&Self::build_search_box_widget());

        let search_results = SearchResults::new();
        container.append(&search_results);
        window.set_child(Some(&container));

        Self {
            window,
            search_results,
            installed_apps: Rc::new(gio::AppInfo::all()),
        }
    }

    pub fn present(&self) {
        self.window.add_action(&self.create_search_action());
        self.window.present();
    }

    fn build_search_box_widget() -> gtk::SearchEntry {
        let search_box = gtk::SearchEntry::builder().hexpand(true).build();
        search_box.connect_search_changed(move |search_box| {
            let search_query = search_box.text().to_string();
            search_box
                .activate_action("win.search", Some(&search_query.to_variant()))
                .expect("search action should exist");
        });
        search_box
    }

    fn create_search_action(&self) -> gio::SimpleAction {
        let installed_apps = Rc::clone(&self.installed_apps);
        let search_results = &self.search_results;
        let search_action = gio::SimpleAction::new("search", Some(&String::static_variant_type()));

        search_action.connect_activate(clone!(@weak search_results => move |_state, variant| {
            if let Some(variant) = variant {
                // Clear previous results
                search_results.clear();

                let search_query = variant.get::<String>().unwrap_or_default();
                if search_query.is_empty() {
                    return;
                }
                let query_matched_apps = installed_apps.iter().filter_map(|app| {
                    let app_name = app.name();
                    if app_name.matches(&search_query).count() != 0 {
                        Some(app_name)
                    } else {
                        None
                    }
                }).collect::<Vec<glib::GString>>();

                if !query_matched_apps.is_empty() {
                    for app_name in &query_matched_apps {
                        search_results.push(gtk::Text::builder().text(app_name.as_str()).build());
                    }
                }
            }
        }));
        search_action
    }
}
