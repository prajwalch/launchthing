use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

pub trait Results {
    fn rows(&self) -> Vec<gtk::ListBoxRow>;
    fn on_row_selected(&self, index: usize);
    fn is_empty(&self) -> bool;
}

pub struct SearchResults {
    scrollable_container: gtk::ScrolledWindow,
    container: gtk::ListBox,
    results_rows: Vec<gtk::ListBoxRow>,
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
            results_rows: Vec::new(),
            select_handler_id: RefCell::new(None),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show<R: Results + 'static>(&mut self, results: R) {
        self.results_rows.extend(results.rows());

        for row in &self.results_rows {
            self.container.append(row);
        }
        let handler_id = self.container.connect_row_selected(move |container, row| {
            if let Some(row) = row {
                results.on_row_selected(row.index() as usize);
                container
                    .activate_action("window.close", None)
                    .expect("`window.close` action should exist");
            };
        });
        self.select_handler_id.set(Some(handler_id));
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        for row in self.results_rows.iter() {
            self.container.remove(row);
        }
        if let Some(handler_id) = self.select_handler_id.take() {
            self.container.disconnect(handler_id);
        }
        self.results_rows.clear();
        self.scrollable_container.hide();
    }
}
