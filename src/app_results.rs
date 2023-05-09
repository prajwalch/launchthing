use gtk::gio;
use gtk::prelude::*;

use crate::search_results::Item;
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
}

impl Results for AppResults {
    fn is_empty(&self) -> bool {
        self.matched_apps.is_empty()
    }

    fn create_list_items(&self) -> Vec<Item> {
        self.matched_apps
            .iter()
            .map(create_list_item)
            .collect::<Vec<Item>>()
    }

    fn on_item_selected(&self, item: &Item) {
        let Some(app) = self.matched_apps.get(item.index() as usize) else {
            return;
        };
        if let Err(e) = app.launch(&[], Some(&item.display().app_launch_context())) {
            eprintln!("error: Failed to launch {}: {e}", app.name());
        }
        // `window.close` is a built-in action therefore unwrapping is ok
        item.activate_action("window.close", None).unwrap();
    }
}

fn create_list_item(app: &gio::AppInfo) -> Item {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.set_margin_top(3);
    container.set_margin_start(10);
    container.set_margin_end(3);
    container.append(&create_icon_widget(app));
    container.append(&create_name_and_description_widget(app));

    Item::builder().child(&container).build()
}

fn create_icon_widget(app: &gio::AppInfo) -> gtk::Image {
    let icon = gtk::Image::new();
    icon.set_margin_top(6);
    icon.set_margin_bottom(6);
    icon.set_margin_start(6);
    icon.set_margin_end(6);
    icon.set_pixel_size(40);

    if let Some(app_icon) = app.icon() {
        icon.set_from_gicon(&app_icon);
    }
    icon
}

fn create_name_and_description_widget(app: &gio::AppInfo) -> gtk::Box {
    let text_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    text_container.set_margin_top(6);
    text_container.set_margin_bottom(6);

    let name = gtk::Label::new(Some(&app.name()));
    name.set_halign(gtk::Align::Start);
    text_container.append(&name);

    let description = gtk::Label::new(None);
    description.set_halign(gtk::Align::Start);
    description.set_wrap(true);
    description.set_opacity(0.5);
    description.set_css_classes(&["body"]);

    if let Some(app_des) = app.description() {
        description.set_text(&app_des);
    }
    text_container.append(&description);
    text_container
}
