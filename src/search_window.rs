use std::rc::Rc;

use gtk::gdk;
use gtk::glib::clone;
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
        let app_mode = Rc::new(AppMode::new());
        window.set_child(Some(&container));

        Self {
            window,
            container,
            app_mode,
        }
    }

    #[rustfmt::skip]
    pub fn present(&self) {
        let search_box = self.create_search_box_widget();
        self.container.append(&search_box);
        self.container.append(&self.create_scroll_window());

        // FIXME: Currently search box doesn't gets auto focus when app loads therefore we are
        //        required to manually focus it.
        search_box.grab_focus();
        self.window.add_controller(self.create_key_event_handler());
        self.window.present();
    }

    fn create_search_box_widget(&self) -> gtk::SearchEntry {
        let app_mode = &self.app_mode;
        let search_box = gtk::SearchEntry::new();
        search_box.set_search_delay(0);
        search_box.set_height_request(50);
        search_box.set_placeholder_text(Some("Search"));

        search_box.connect_activate(
            clone!(@strong app_mode => move |_| app_mode.on_key_pressed(gdk::Key::Return)),
        );

        search_box.connect_search_changed(clone!(@strong app_mode => move |search_box| {
            // Clear previous results
            app_mode.show_hidden_apps();

            let query = search_box.text().to_lowercase();
            if query.is_empty() {
                return;
            }
            app_mode.on_search_query_changed(&query);
        }));
        search_box
    }

    fn create_scroll_window(&self) -> gtk::ScrolledWindow {
        let scroll_window = gtk::ScrolledWindow::new();
        scroll_window.set_min_content_height(500);
        scroll_window.set_child(Some(self.app_mode.list()));

        scroll_window
    }

    fn create_key_event_handler(&self) -> gtk::EventControllerKey {
        // NOTE: We can directly unwrap this since we already know that search box is the first
        //       child of container but it is always good to perform action safely.
        let search_box = self.container.first_child();
        let app_mode = Rc::clone(&self.app_mode);
        let key_event_controller = gtk::EventControllerKey::new();

        key_event_controller.connect_key_pressed(move |_, key, _, _| {
            app_mode.on_key_pressed(key);

            if let Some(search_box) = &search_box {
                search_box.grab_focus();
            }
            // Don't propagate the signal to the default handler/s because it removes the focus
            // from the search box which we don't want.
            gtk::Inhibit(true)
        });
        key_event_controller
    }
}
