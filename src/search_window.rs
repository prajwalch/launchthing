use std::rc::Rc;

use gtk::gdk;
use gtk::gio;
use gtk::prelude::*;

/// The `ListItem` is a displayable widget used to represent an item in a result list.
///
/// It can contain only one child widget so when multiple widgets need to be displayed on a single
/// item the [gtk::Box] container can be used as a child.
type ListItem = gtk::ListBoxRow;

#[derive(Clone)]
pub struct SearchWindow {
    window: gtk::ApplicationWindow,
    container: gtk::Box,
    result_list: gtk::ListBox,
    list_items: Vec<ListItem>,
    installed_apps: Rc<Vec<gio::AppInfo>>,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_width_request(500);
        window.set_resizable(false);
        window.set_decorated(false);
        window.set_overflow(gtk::Overflow::Hidden);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        window.set_child(Some(&container));

        let scroll_window = gtk::ScrolledWindow::new();
        scroll_window.set_min_content_height(500);
        container.append(&scroll_window);

        let installed_apps = Rc::new(get_installed_apps());
        let (result_list, list_items) = create_result_list_with_items(&installed_apps);
        scroll_window.set_child(Some(&result_list));

        Self {
            window,
            container,
            result_list,
            list_items,
            installed_apps,
        }
    }

    #[rustfmt::skip]
    pub fn present(&self) {
        self.container.prepend(&self.create_search_box_widget());
        self.add_selected_item_handler();
        self.add_key_event_handler();
        self.window.present();
    }

    fn create_search_box_widget(&self) -> gtk::SearchEntry {
        let search_box = gtk::SearchEntry::new();
        search_box.set_search_delay(0);
        search_box.set_height_request(50);
        search_box.set_placeholder_text(Some("Search"));

        let search_window = self.clone();
        search_box.connect_search_changed(move |search_box| {
            search_window.on_search_query_changed(search_box.text().as_str());
        });
        search_box
    }

    fn on_search_query_changed(&self, query: &str) {
        // Clear previous results
        self.clear_results();

        if query.is_empty() {
            return;
        }
        let query = query.to_lowercase();

        // Hide all unmatched apps
        for (index, app) in self.installed_apps.iter().enumerate() {
            if app.name().to_lowercase().contains(&query) {
                continue;
            }

            if let Some(item) = self.list_items.get(index) {
                item.set_visible(false);
            }
        }
    }

    fn clear_results(&self) {
        let hidden_items = self.list_items.iter().filter(|item| !item.is_visible());
        for hidden_item in hidden_items {
            hidden_item.set_visible(true);
        }
    }

    fn add_selected_item_handler(&self) {
        let search_window = self.clone();
        self.result_list.connect_row_activated(move |_, item| {
            search_window.on_item_selected(item);
        });
    }

    fn on_item_selected(&self, item: &ListItem) {
        let Some(selected_app) = self.installed_apps.get(item.index() as usize) else {
            return;
        };

        let app_launch_context = self.result_list.display().app_launch_context();
        if let Err(e) = selected_app.launch(&[], Some(&app_launch_context)) {
            eprintln!("error: Failed to launch {}: {e}", selected_app.name());
        }
        self.window.close();
    }

    fn add_key_event_handler(&self) {
        let search_window = self.clone();
        let key_event_controller = gtk::EventControllerKey::new();

        key_event_controller.connect_key_pressed(move |_, key, _, _| {
            search_window.on_key_pressed(key);
            // Don't propagate the signal to the default handler/s because it removes the focus
            // from the search box which we don't want.
            gtk::Inhibit(true)
        });
        self.window.add_controller(key_event_controller);
    }

    fn on_key_pressed(&self, key: gdk::Key) {
        todo!()
    }
}

fn get_installed_apps() -> Vec<gio::AppInfo> {
    gio::AppInfo::all()
        .iter()
        .filter(|app| app.icon().is_some() && app.should_show())
        .cloned()
        .collect::<Vec<gio::AppInfo>>()
}

fn create_result_list_with_items(installed_apps: &[gio::AppInfo]) -> (gtk::ListBox, Vec<ListItem>) {
    let result_list = gtk::ListBox::new();
    let list_items = create_and_append_list_items(installed_apps, &result_list);
    (result_list, list_items)
}

fn create_and_append_list_items(
    installed_apps: &[gio::AppInfo],
    result_list: &gtk::ListBox,
) -> Vec<ListItem> {
    let list_items = installed_apps
        .iter()
        .map(create_list_item)
        .collect::<Vec<ListItem>>();

    for item in &list_items {
        result_list.append(item);
    }
    result_list.select_row(list_items.first());

    list_items
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
