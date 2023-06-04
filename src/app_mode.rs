use gtk::gdk;
use gtk::gio;
use gtk::glib::clone;
use gtk::prelude::*;

/// The `ListItem` is a displayable widget used to represent an item in a result list.
///
/// It can contain only one child widget so when multiple widgets need to be displayed on a single
/// item the [gtk::Box] container can be used as a child.
type ListItem = gtk::ListBoxRow;

pub struct AppMode {
    apps: Vec<gio::AppInfo>,
    list: gtk::ListBox,
    list_items: Vec<ListItem>,
}

impl AppMode {
    pub fn new() -> Self {
        let apps = gio::AppInfo::all()
            .iter()
            .filter(|app| app.icon().is_some() && app.should_show())
            .cloned()
            .collect::<Vec<gio::AppInfo>>();
        let list = gtk::ListBox::new();
        let list_items = apps.iter().map(create_list_item).collect::<Vec<ListItem>>();

        for item in &list_items {
            list.append(item);
        }
        list.select_row(list_items.first());
        list.connect_row_activated(clone!(@strong apps => move |list, item| {
            Self::on_item_selected(&apps, list, item);
        }));

        Self {
            apps,
            list,
            list_items,
        }
    }

    pub fn list(&self) -> &gtk::ListBox {
        &self.list
    }

    pub fn show_hidden_apps(&self) {
        let hidden_items = self.list_items.iter().filter(|item| !item.is_visible());
        for hidden_item in hidden_items {
            hidden_item.set_visible(true);
        }
        self.list.select_row(self.list_items.first());
    }

    pub fn on_search_query_changed(&self, query: &str) {
        self.list.unselect_all();

        for (index, app) in self.apps.iter().enumerate() {
            let Some(item) = self.list_items.get(index) else {
                continue;
            };

            if !app.name().to_lowercase().contains(query) {
                item.set_visible(false);
                continue;
            }

            // Select the first visible item only
            if self.list.selected_row().is_none() {
                self.list.select_row(Some(item));
            }
        }
    }

    pub fn on_key_pressed(&self, key: gdk::Key) {
        let visible_items = self
            .list_items
            .iter()
            .filter(|item| item.is_visible())
            .collect::<Vec<&ListItem>>();

        let Some((selected_item_index, selected_item)) =
            visible_items.iter().enumerate().find(|(_, item)| item.is_selected()) else
        {
            return;
        };

        let try_select_item = |next_item: Option<&&ListItem>| {
            if let Some(item) = next_item {
                self.list.select_row(Some(*item));
                item.grab_focus();
            }
        };

        match key {
            gdk::Key::Return => {
                selected_item.activate();
            }
            gdk::Key::Tab | gdk::Key::Down => {
                // Round new index to 0 if it's become greater than items length
                let next_item_index = (selected_item_index + 1) % visible_items.len();
                try_select_item(visible_items.get(next_item_index));
            }
            gdk::Key::Up => {
                // NOTE: Figure out how to use modulo operator when subtracting
                if selected_item_index == 0 {
                    try_select_item(visible_items.last())
                } else {
                    try_select_item(visible_items.get(selected_item_index - 1));
                }
            }
            _ => (),
        };
    }

    fn on_item_selected(apps: &[gio::AppInfo], list: &gtk::ListBox, item: &ListItem) {
        let Some(selected_app) = apps.get(item.index() as usize) else {
            return;
        };

        let app_launch_context = list.display().app_launch_context();
        if let Err(e) = selected_app.launch(&[], Some(&app_launch_context)) {
            eprintln!("error: Failed to launch {}: {e}", selected_app.name());
        }
        list.activate_action("window.close", None).unwrap();
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
        let description = gtk::Label::new(Some(&app_des));
        description.set_halign(gtk::Align::Start);
        description.set_margin_top(2);
        description.set_opacity(0.5);
        description.set_wrap(true);
        description.set_css_classes(&["body"]);

        container.append(&description);
    }
    container
}
