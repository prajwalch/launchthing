use gtk::prelude::*;

pub struct SearchResults {
    scrollable_container: gtk::ScrolledWindow,
    container: gtk::ListBox,
    results: Vec<gtk::ListBoxRow>,
}

impl SearchResults {
    pub fn new() -> Self {
        let scrollable_container = gtk::ScrolledWindow::new();
        scrollable_container.set_min_content_height(200);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let container = gtk::ListBox::new();
        scrollable_container.set_child(Some(&container));

        Self {
            scrollable_container,
            container,
            results: Vec::new(),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show(&mut self, results: &[gtk::ListBoxRow]) {
        for result in results {
            let result = result.clone();
            self.container.append(&result);
            self.results.push(result);
        }
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        for result in self.results.iter() {
            self.container.remove(result);
        }
        self.results.clear();
        self.scrollable_container.hide();
    }
}
