use std::path::PathBuf;

use gtk::prelude::*;

use crate::search_results::Results;

#[allow(dead_code)]
pub struct PathResults {
    dirs: Vec<PathBuf>,
}

impl PathResults {
    pub fn new(_search_query: &str) -> Self {
        Self { dirs: Vec::new() }
    }
}

impl Results for PathResults {
    fn is_empty(&self) -> bool {
        false
    }

    fn create_list_items(&self) -> Vec<gtk::ListBoxRow> {
        let item = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some("/home/prajwal"));
        let icon = gtk::Image::from_icon_name("folder");

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        container.append(&icon);
        container.append(&label);
        item.set_child(Some(&container));

        Vec::from([item])
    }

    fn on_item_selected(&self, _item: &gtk::ListBoxRow) {
        todo!()
    }
}
