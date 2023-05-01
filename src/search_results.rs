use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

pub struct SearchResults {
    scrollable_container: gtk::ScrolledWindow,
    container: gtk::ListBox,
    results_row: Vec<gtk::ListBoxRow>,
    selected_handler_id: RefCell<Option<glib::SignalHandlerId>>,
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
            results_row: Vec::new(),
            selected_handler_id: RefCell::new(None),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show<R, F>(&mut self, results: Vec<R>, results_rows: &[gtk::ListBoxRow], on_selected: F)
    where
        R: 'static,
        F: Fn(&R) + 'static,
    {
        for row in results_rows {
            let result = row.clone();
            self.container.append(&result);
            self.results_row.push(result);
        }
        let signal_handler_id = self.container.connect_row_selected(move |_container, row| {
            let Some(row) = row else {
                return;
            };
            if let Some(result) = results.get(row.index() as usize) {
                on_selected(result);
            }
        });
        self.selected_handler_id.set(Some(signal_handler_id));
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        for result in self.results_row.iter() {
            self.container.remove(result);
        }
        if let Some(selected_handler_id) = self.selected_handler_id.take() {
            self.container.disconnect(selected_handler_id);
        }
        self.results_row.clear();
        self.scrollable_container.hide();
    }
}
