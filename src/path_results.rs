use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gtk::prelude::*;

use crate::search_results::Results;

#[allow(dead_code)]
pub struct PathResults {
    child_paths: Vec<PathBuf>,
}

impl PathResults {
    pub fn new(search_query: &str) -> Self {
        let child_paths: Vec<PathBuf> = fs::read_dir(search_query).map_or(Vec::new(), |entries| {
            entries.map(|entry| entry.unwrap().path()).collect()
        });
        Self { child_paths }
    }
}

impl Results for PathResults {
    fn is_empty(&self) -> bool {
        self.child_paths.is_empty()
    }

    fn create_list_items(&self) -> Vec<gtk::ListBoxRow> {
        self.child_paths
            .iter()
            .map(|child_path| create_list_box_row(child_path))
            .collect()
    }

    fn on_item_selected(&self, _item: &gtk::ListBoxRow) {
        todo!()
    }
}

fn create_list_box_row(child_path: &Path) -> gtk::ListBoxRow {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.append(&create_icon_widget(child_path));
    container.append(&create_label_widget(
        &child_path.file_name().unwrap_or_default().to_string_lossy(),
    ));
    gtk::ListBoxRow::builder().child(&container).build()
}

fn create_icon_widget(_path: &Path) -> gtk::Image {
    // TODO: Use different kind of icon for different kind of file extension
    let icon = gtk::Image::from_icon_name("folder");
    icon.set_margin_top(2);
    icon.set_margin_bottom(2);
    icon.set_margin_start(10);
    icon.set_margin_end(2);
    icon.set_pixel_size(25);
    icon
}

fn create_label_widget(path_name: &str) -> gtk::Label {
    let name = gtk::Label::new(Some(path_name));
    name.set_halign(gtk::Align::Start);
    name.set_css_classes(&["title-5"]);
    name
}
