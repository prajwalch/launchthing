use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;

use crate::app_results::AppResults;
use crate::path_results::PathResults;
use crate::search_results::SearchResults;

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    container: gtk::Box,
    search_results: Rc<RefCell<SearchResults>>,
    installed_apps: Rc<Vec<gio::AppInfo>>,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_height_request(50);
        window.set_width_request(500);
        window.set_resizable(false);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        window.set_child(Some(&container));

        Self {
            window,
            container,
            search_results: Rc::new(RefCell::new(SearchResults::new())),
            installed_apps: Rc::new(get_installed_apps()),
        }
    }

    #[rustfmt::skip]
    pub fn present(&self) {
        self.container.append(&self.create_search_box_widget());
        self.container.append(self.search_results.borrow().container());
        self.window.present();
    }

    fn create_search_box_widget(&self) -> gtk::SearchEntry {
        let search_window = self.clone();
        let search_box = gtk::SearchEntry::builder().hexpand(true).build();

        search_box.connect_search_changed(move |search_box| {
            search_window.on_search_query_changed(search_box.text().as_str());
        });

        #[rustfmt::skip]
        self.window.add_action(&create_change_query_action(&search_box));
        search_box
    }

    fn on_search_query_changed(&self, query: &str) {
        // Clear previous results
        self.search_results.borrow_mut().clear();

        if query.is_empty() {
            return;
        }

        if query.starts_with('~') || query.starts_with('/') {
            let path_results = PathResults::new(query);
            self.search_results.borrow_mut().show(path_results);
            return;
        }
        let app_results = AppResults::new(query, &self.installed_apps);
        self.search_results.borrow_mut().show(app_results);
    }
}

fn get_installed_apps() -> Vec<gio::AppInfo> {
    gio::AppInfo::all()
        .iter()
        .filter(|app| app.icon().is_some() && app.should_show())
        .cloned()
        .collect::<Vec<gio::AppInfo>>()
}

/// NOTE: For now this action is being only used by [PathResults] item
///       not sure whether it will be useful for others in future.
fn create_change_query_action(search_box: &gtk::SearchEntry) -> gio::SimpleAction {
    let change_query_action =
        gio::SimpleAction::new("change-query", Some(&String::static_variant_type()));

    change_query_action.connect_activate(clone!(@weak search_box => move |_, variant| {
        let Some(variant) = variant else {
            return;
        };
        let Some(new_query) = variant.get::<String>() else {
            return;
        };
        let mut current_query = search_box.text().to_string();

        if !current_query.ends_with('/') {
            current_query.push('/');
        }
        current_query.push_str(&new_query);
        // Replace currently showed query with newly created query
        search_box.set_text(&current_query);
        // Move cursor to end
        search_box.set_position(-1);
    }));
    change_query_action
}
