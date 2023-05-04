use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

pub trait Results {
    fn is_empty(&self) -> bool;
    fn rows(&self) -> Vec<gtk::ListBoxRow>;
    fn on_row_selected(&self, index: usize);
}

pub struct SearchResults {
    scrollable_container: gtk::ScrolledWindow,
    container: gtk::ListBox,
    items: Vec<gtk::ListBoxRow>,
    select_handler_id: RefCell<Option<glib::SignalHandlerId>>,
}

impl SearchResults {
    pub fn new() -> Self {
        let scrollable_container = gtk::ScrolledWindow::new();
        scrollable_container.set_min_content_height(200);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let container = gtk::ListBox::new();
        scrollable_container.set_child(Some(&container));

        Self {
            scrollable_container,
            container,
            items: Vec::new(),
            select_handler_id: RefCell::new(None),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show<R: Results + 'static>(&mut self, results: R) {
        if results.is_empty() {
            return;
        }
        self.items.extend(results.rows());

        for item in &self.items {
            self.container.append(item);
        }
        let handler_id = self.container.connect_row_selected(move |container, row| {
            if let Some(row) = row {
                results.on_row_selected(row.index() as usize);
                // `window.close`is a built-in action therefore unwrapping is ok
                container.activate_action("window.close", None).unwrap();
            };
        });
        self.select_handler_id.set(Some(handler_id));
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        for item in self.items.iter() {
            self.container.remove(item);
        }
        if let Some(handler_id) = self.select_handler_id.take() {
            self.container.disconnect(handler_id);
        }
        self.items.clear();
        self.scrollable_container.hide();
    }
}
