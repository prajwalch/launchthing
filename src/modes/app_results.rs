use gtk::gio;
use gtk::prelude::*;

use super::ListItem;
use super::Mode;

pub struct AppResults {
    matched_apps: Vec<gio::AppInfo>,
}

impl AppResults {
    pub fn new(search_query: &str, installed_apps: &[gio::AppInfo]) -> Self {
        let matched_apps = installed_apps
            .iter()
            .filter(|app| app.name().to_lowercase().contains(search_query))
            .cloned()
            .collect::<Vec<gio::AppInfo>>();

        Self { matched_apps }
    }
}

impl Mode for AppResults {
    fn is_activated(query: &str) -> bool {
        // File naming conventions for linux and windows
        //
        // https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html#guidelines
        // https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions
        !query.starts_with(['~', '/', '\\'])
            && query.contains(|c: char| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    }

    fn contains_data(&self) -> bool {
        !self.matched_apps.is_empty()
    }

    fn create_list_items(&self) -> Vec<ListItem> {
        self.matched_apps
            .iter()
            .map(create_list_item)
            .collect::<Vec<ListItem>>()
    }

    fn on_item_selected(&self, item: &ListItem) {
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

fn create_list_item(app: &gio::AppInfo) -> ListItem {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.set_margin_top(3);
    container.set_margin_start(10);
    container.set_margin_end(3);

    if let Some(icon) = app.icon() {
        container.append(&create_icon_widget(&icon));
    }
    container.append(&create_name_and_description_widget(app));

    let list_item = ListItem::new();
    list_item.set_child(Some(&container));
    list_item
}

fn create_icon_widget(icon: &gio::Icon) -> gtk::Image {
    let icon = gtk::Image::from_gicon(icon);
    icon.set_margin_top(6);
    icon.set_margin_bottom(6);
    icon.set_margin_start(6);
    icon.set_margin_end(6);
    icon.set_pixel_size(40);

    icon
}

fn create_name_and_description_widget(app: &gio::AppInfo) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_margin_top(6);
    container.set_margin_bottom(6);

    let name = gtk::Label::new(Some(&app.name()));
    name.set_halign(gtk::Align::Start);
    container.append(&name);

    if let Some(app_des) = app.description() {
        let description = gtk::Label::new(None);
        description.set_halign(gtk::Align::Start);
        description.set_margin_top(2);
        description.set_opacity(0.5);
        description.set_wrap(true);
        description.set_css_classes(&["body"]);
        description.set_text(&app_des);

        container.append(&description);
    }
    container
}
