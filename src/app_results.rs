use gtk::gio;
use gtk::prelude::*;

use crate::search_results::Results;

pub struct AppResults {
    matched_apps: Vec<gio::AppInfo>,
}

impl AppResults {
    pub fn new(search_query: &str, installed_apps: &[gio::AppInfo]) -> Self {
        let matched_apps = installed_apps
            .iter()
            .filter(|app| app.name().to_lowercase().matches(search_query).count() != 0)
            .cloned()
            .collect::<Vec<gio::AppInfo>>();

        Self { matched_apps }
    }

    pub fn is_empty(&self) -> bool {
        self.matched_apps.is_empty()
    }
}

impl Results for AppResults {
    fn rows(&self) -> Vec<gtk::ListBoxRow> {
        self.matched_apps
            .iter()
            .map(create_list_box_row)
            .collect::<Vec<gtk::ListBoxRow>>()
    }

    fn on_row_selected(&self, index: usize) {
        if let Some(app_info) = self.matched_apps.get(index) {
            println!("Selected: {}", app_info.name());
        }
    }
}

fn create_list_box_row(app_info: &gio::AppInfo) -> gtk::ListBoxRow {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.set_margin_top(10);
    container.set_margin_start(10);
    container.set_margin_end(10);
    container.append(&create_icon_widget(app_info));
    container.append(&create_name_and_description_widget(app_info));

    gtk::ListBoxRow::builder().child(&container).build()
}

fn create_icon_widget(app_info: &gio::AppInfo) -> gtk::Image {
    let icon = gtk::Image::new();
    icon.set_margin_top(6);
    icon.set_margin_bottom(6);
    icon.set_margin_start(6);
    icon.set_margin_end(6);
    icon.set_pixel_size(40);

    if let Some(app_icon) = app_info.icon() {
        icon.set_from_gicon(&app_icon);
    }
    icon
}

fn create_name_and_description_widget(app_info: &gio::AppInfo) -> gtk::Box {
    let text_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    text_container.set_margin_top(6);
    text_container.set_margin_bottom(6);

    let name = gtk::Label::new(Some(&app_info.name()));
    name.set_halign(gtk::Align::Start);
    name.set_css_classes(&["title-4"]);
    text_container.append(&name);

    let description = gtk::Label::new(None);
    description.set_halign(gtk::Align::Start);
    description.set_wrap(true);
    description.set_css_classes(&["body"]);

    if let Some(app_des) = app_info.description() {
        description.set_text(&app_des);
    }
    text_container.append(&description);
    text_container
}
