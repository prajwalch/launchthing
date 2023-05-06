use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

pub trait Results {
    fn is_empty(&self) -> bool;
    /// Creates list items by binding the data
    fn create_list_items(&self) -> Vec<gtk::ListBoxRow>;
    /// Callback for when an item is selected by user
    fn on_item_selected(&self, item: &gtk::ListBoxRow);
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
        self.items.extend(results.create_list_items());

        for item in &self.items {
            self.container.append(item);
        }
        let handler_id = self.container.connect_row_selected(move |container, item| {
            if let Some(item) = item {
                container.unselect_row(item);
                results.on_item_selected(item);
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
