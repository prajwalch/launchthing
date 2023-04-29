use gtk::gio;
use gtk::prelude::*;

pub fn create(app_info: &gio::AppInfo) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.append(&create_icon_widget(app_info));
    container.append(&create_name_and_description_widget(app_info));
    container
}

fn create_icon_widget(app_info: &gio::AppInfo) -> gtk::Image {
    let icon = gtk::Image::new();
    icon.set_margin_top(6);
    icon.set_margin_bottom(6);
    icon.set_margin_start(6);
    icon.set_margin_end(6);
    icon.set_pixel_size(48);

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
