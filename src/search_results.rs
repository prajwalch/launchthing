use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

/// The `ListItem` is a displayable widget used to represent an item in a results list.
///
/// It can contain only one child widget so when multiple widgets need to be displayed on a single
/// item the [gtk::Box] container can be used as a child.
pub type ListItem = gtk::ListBoxRow;

pub trait Results {
    fn is_empty(&self) -> bool;
    /// Creates list items by binding the data
    fn create_list_items(&self) -> Vec<ListItem>;
    /// Callback for when an item is selected by user
    fn on_item_selected(&self, item: &ListItem);
}

pub struct SearchResults {
    scrollable_container: gtk::ScrolledWindow,
    list_container: gtk::ListBox,
    items: Vec<ListItem>,
    select_handler_id: RefCell<Option<glib::SignalHandlerId>>,
}

impl SearchResults {
    pub fn new() -> Self {
        let scrollable_container = gtk::ScrolledWindow::new();
        scrollable_container.set_min_content_height(200);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let list_container = gtk::ListBox::new();
        scrollable_container.set_child(Some(&list_container));

        Self {
            scrollable_container,
            list_container,
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
            self.list_container.append(item);
        }
        // Add an item selected signal handler and store its id so that we can remove it when
        // clearing results later
        let handler_id = self
            .list_container
            .connect_row_selected(move |list_container, item| {
                if let Some(item) = item {
                    list_container.unselect_row(item);
                    results.on_item_selected(item);
                };
            });
        self.select_handler_id.set(Some(handler_id));
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        self.scrollable_container.hide();

        for item in self.items.iter() {
            self.list_container.remove(item);
        }
        // Remove an item selected signal handler by using previously saved id
        if let Some(handler_id) = self.select_handler_id.take() {
            self.list_container.disconnect(handler_id);
        }
        self.list_container.set_height_request(0);
        self.items.clear();
    }
}
