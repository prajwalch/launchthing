use std::rc::Rc;

use gtk::prelude::*;

use crate::app_mode::AppMode;

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    container: gtk::Box,
    app_mode: Rc<AppMode>,
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

        let scroll_window = gtk::ScrolledWindow::new();
        scroll_window.set_min_content_height(500);
        container.append(&scroll_window);

        let app_mode = Rc::new(AppMode::new());
        scroll_window.set_child(Some(app_mode.result_list()));

        Self {
            window,
            container,
            app_mode,
        }
    }

    #[rustfmt::skip]
    pub fn present(&self) {
        self.container.prepend(&self.create_search_box_widget());
        self.window.add_controller(self.create_key_event_handler());
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
        search_box
    }

    fn on_search_query_changed(&self, query: &str) {
        // Clear previous results
        self.app_mode.show_hidden_apps();

        if query.is_empty() {
            return;
        }
        let query = query.to_lowercase();
        self.app_mode.on_search_query_changed(&query);
    }

    fn create_key_event_handler(&self) -> gtk::EventControllerKey {
        let app_mode = Rc::clone(&self.app_mode);
        let key_event_controller = gtk::EventControllerKey::new();

        key_event_controller.connect_key_pressed(move |_, key, _, _| {
            app_mode.on_key_pressed(key);
            // Don't propagate the signal to the default handler/s because it removes the focus
            // from the search box which we don't want.
            gtk::Inhibit(true)
        });
        key_event_controller
    }
}
