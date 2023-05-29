mod app_results;
mod file_browser;

use std::cell::RefCell;

use gtk::gdk;
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
        scrollable_container.set_min_content_height(500);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let result_list = gtk::ListBox::new();
        scrollable_container.set_child(Some(&result_list));

        Self {
            scrollable_container,
            result_list,
            list_items: Vec::new(),
            select_handler_id: RefCell::new(None),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn on_key_pressed(&self, key: gdk::Key) {
        let Some(selected_item_index) = self.result_list.selected_row().map(|item| item.index() as usize) else {
            return;
        };

        match key {
            gdk::Key::Tab | gdk::Key::Down => {
                let last_item_index = self.list_items.len() - 1;
                // If the last item is currently selected, select the first item otherwise select
                // the next item as normal.
                let next_item = if selected_item_index == last_item_index {
                    self.list_items.first()
                } else {
                    self.list_items.get(selected_item_index + 1)
                };
                self.result_list.select_row(next_item);
            }
            gdk::Key::Up => {
                // If the first item is currently selected, select the last item otherwise select
                // the upper item as normal.
                let next_item = if selected_item_index == 0 {
                    self.list_items.last()
                } else {
                    self.list_items.get(selected_item_index - 1)
                };
                self.result_list.select_row(next_item);
            }
            _ => {}
        }
    }

    pub fn run<M: Mode + 'static>(&mut self, mode: M) {
        if !mode.contains_data() {
            return;
        }
        self.list_items.extend(mode.create_list_items());

        for item in &self.list_items {
            self.result_list.append(item);
        }
        self.result_list.select_row(self.list_items.first());

        // TODO: Currently pressing enter key doesn't emits the `row-activated` signal.
        //       Which means items can be selected only by using the touchpad or mouse.
        //
        // Add an item selected signal handler and store its id so that we can remove it when
        // clearing results later.
        let handler_id = self.result_list.connect_row_activated(move |_, item| {
            mode.on_item_selected(item);
        });
        self.select_handler_id.set(Some(handler_id));
        self.scrollable_container.set_visible(true);
    }

    pub fn clear_results(&mut self) {
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
