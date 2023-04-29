use gtk::prelude::*;

pub struct SearchResults {
    widgets: Vec<gtk::Widget>,
    container: gtk::Box,
    scrollable_container: gtk::ScrolledWindow,
}

impl SearchResults {
    pub fn new() -> Self {
        let scrollable_container = gtk::ScrolledWindow::new();
        scrollable_container.set_min_content_height(200);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        scrollable_container.set_child(Some(&container));

        Self {
            widgets: Vec::new(),
            container,
            scrollable_container,
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show<W: IsA<gtk::Widget>>(&mut self, results: &[W]) {
        for result in results {
            self.widgets.push(result.clone().upcast::<gtk::Widget>());
        }
        for widget in &self.widgets {
            self.container.append(widget);
        }
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        for widget in self.widgets.iter() {
            self.container.remove(widget);
        }
        self.widgets.clear();
        self.scrollable_container.hide();
    }
}
