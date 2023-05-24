mod app_results;
mod file_browser;

use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

pub use app_results::AppResults;
pub use file_browser::FileBrowser;

/// The `ListItem` is a displayable widget used to represent an item in a result list.
///
/// It can contain only one child widget so when multiple widgets need to be displayed on a single
/// item the [gtk::Box] container can be used as a child.
type ListItem = gtk::ListBoxRow;

pub trait Mode {
    /// Checks if mode is activated from the given query.
    ///
    /// Different modes can have different keywords for activating them.
    /// For eg: A mode named `finder` may activate if query starts with `/`
    fn is_activated(query: &str) -> bool;

    /// Checks if any data is present for creating list items
    fn contains_data(&self) -> bool;

    /// Creates list items by binding the data
    fn create_list_items(&self) -> Vec<ListItem>;

    /// Callback for when an item is selected by user
    fn on_item_selected(&self, item: &ListItem);
}

pub struct ModeRunner {
    scrollable_container: gtk::ScrolledWindow,
    result_list: gtk::ListBox,
    list_items: Vec<ListItem>,
    select_handler_id: RefCell<Option<glib::SignalHandlerId>>,
}

impl ModeRunner {
    pub fn new() -> Self {
        let scrollable_container = gtk::ScrolledWindow::new();
        scrollable_container.set_min_content_height(200);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let list_container = gtk::ListBox::new();
        scrollable_container.set_child(Some(&list_container));

        Self {
            scrollable_container,
            result_list: list_container,
            list_items: Vec::new(),
            select_handler_id: RefCell::new(None),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show<M: Mode + 'static>(&mut self, mode: M) {
        if !mode.contains_data() {
            return;
        }
        self.list_items.extend(mode.create_list_items());

        for item in &self.list_items {
            self.result_list.append(item);
        }
        // Add an item selected signal handler and store its id so that we can remove it when
        // clearing results later
        let handler_id = self
            .result_list
            .connect_row_selected(move |list_container, item| {
                if let Some(item) = item {
                    list_container.unselect_row(item);
                    mode.on_item_selected(item);
                };
            });
        self.select_handler_id.set(Some(handler_id));
        self.scrollable_container.set_visible(true);
    }

    pub fn clear(&mut self) {
        self.scrollable_container.set_visible(false);

        for item in self.list_items.iter() {
            self.result_list.remove(item);
        }
        // Remove an item selected signal handler by using previously saved id
        if let Some(handler_id) = self.select_handler_id.take() {
            self.result_list.disconnect(handler_id);
        }
        self.result_list.set_height_request(0);
        self.list_items.clear();
    }
}
