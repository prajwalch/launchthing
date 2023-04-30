use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

use crate::search_results::SearchResults;

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    search_results: Rc<RefCell<SearchResults>>,
    installed_apps: Rc<Vec<gio::AppInfo>>,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_height_request(50);
        window.set_width_request(500);
        window.set_resizable(false);

        let main_container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        main_container.append(&Self::create_search_box_widget());

        let search_results = SearchResults::new();
        main_container.append(search_results.container());
        window.set_child(Some(&main_container));

        let installed_apps = gio::AppInfo::all()
            .iter()
            .filter(|app| app.icon().is_some() && app.should_show())
            .cloned()
            .collect::<Vec<gio::AppInfo>>();

        Self {
            window,
            search_results: Rc::new(RefCell::new(search_results)),
            installed_apps: Rc::new(installed_apps),
        }
    }

    pub fn present(&self) {
        self.window.add_action(&self.create_search_action());
        self.window.present();
    }

    fn create_search_box_widget() -> gtk::SearchEntry {
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
        let search_window = self.clone();
        let search_action = gio::SimpleAction::new("search", Some(&String::static_variant_type()));

        search_action.connect_activate(move |state, variant| {
            search_window.on_search_action_activated(state, variant)
        });
        search_action
    }

    fn on_search_action_activated(
        &self,
        state: &gio::SimpleAction,
        variant: Option<&glib::Variant>,
    ) {
        _ = state;

        let Some(variant) = variant else {
            return;
        };
        // Clear previous results
        self.search_results.borrow_mut().clear();

        let search_query = variant.get::<String>().unwrap_or_default().to_lowercase();
        if search_query.is_empty() {
            return;
        }

        let query_matched_apps = self
            .installed_apps
            .iter()
            .filter(|app| app.name().to_lowercase().matches(&search_query).count() != 0)
            .collect::<Vec<&gio::AppInfo>>();

        if query_matched_apps.is_empty() {
            return;
        }
        let results = query_matched_apps
            .iter()
            .map(|app_info| crate::application_row::create(*app_info))
            .collect::<Vec<gtk::ListBoxRow>>();

        self.search_results.borrow_mut().show(&results);
    }
}
