use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;

use crate::modes::{AppResults, FileBrowser, Mode, ModeRunner};

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    container: gtk::Box,
    mode_runner: Rc<RefCell<ModeRunner>>,
    installed_apps: Rc<Vec<gio::AppInfo>>,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_width_request(500);
        window.set_resizable(false);
        window.set_decorated(false);
        window.set_overflow(gtk::Overflow::Hidden);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        window.set_child(Some(&container));

        Self {
            window,
            container,
            mode_runner: Rc::new(RefCell::new(ModeRunner::new())),
            installed_apps: Rc::new(get_installed_apps()),
        }
    }

    #[rustfmt::skip]
    pub fn present(&self) {
        let mode_runner = Rc::clone(&self.mode_runner);
        let search_box = self.create_search_box_widget();
        self.container.append(&search_box);
        self.container.append(mode_runner.borrow().container());

        let key_event_controller = gtk::EventControllerKey::new();
        key_event_controller.connect_key_pressed(move |_, key, _, _| {
            mode_runner.borrow().on_key_pressed(key);
            search_box.grab_focus();
            // Don't propagate the signal to the default handler/s because it removes the focus
            // from the search box which we don't want.
            gtk::Inhibit(true)
        });
        self.window.add_controller(key_event_controller);
        self.window.present();
    }

    fn create_search_box_widget(&self) -> gtk::SearchEntry {
        let search_box = gtk::SearchEntry::new();
        search_box.set_search_delay(0);
        search_box.set_height_request(50);
        search_box.set_placeholder_text(Some("Search"));

        let search_window = self.clone();
        search_box.connect_search_changed(move |search_box| {
            search_window.on_search_query_changed(search_box.text().as_str());
        });

        #[rustfmt::skip]
        self.window.add_action(&create_change_query_action(&search_box));
        search_box
    }

    fn on_search_query_changed(&self, query: &str) {
        let mut mode_runner = self.mode_runner.borrow_mut();
        // Clear previous results
        mode_runner.clear_results();

        if query.is_empty() {
            return;
        }
        let query = query.to_lowercase();

        if AppResults::is_activated(&query) {
            mode_runner.run(AppResults::new(&query, &self.installed_apps));
        } else if FileBrowser::is_activated(&query) {
            mode_runner.run(FileBrowser::new(&query));
        }
    }
}

fn get_installed_apps() -> Vec<gio::AppInfo> {
    gio::AppInfo::all()
        .iter()
        .filter(|app| app.icon().is_some() && app.should_show())
        .cloned()
        .collect::<Vec<gio::AppInfo>>()
}

/// NOTE: For now this action is being only used by [FileBrowser] item
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
        // Replace currently showed query with newly created query
        search_box.set_text(&new_query);
        // Move cursor to end
        search_box.set_position(-1);
    }));
    change_query_action
}
