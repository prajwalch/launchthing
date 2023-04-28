mod imp;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct ApplicationRow(ObjectSubclass<imp::ApplicationRow>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for ApplicationRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_info(&self, app_info: &gio::AppInfo) {
        let imp = self.imp();
        imp.name.set_text(&app_info.name());

        if let Some(description) = app_info.description() {
            imp.description.set_text(&description);
        }
        if let Some(icon) = app_info.icon() {
            imp.icon.set_from_gicon(&icon);
        }
    }
}
