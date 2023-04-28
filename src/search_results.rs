use gtk::prelude::*;

pub struct SearchResults {
    container: gtk::Box,
    childrens: Vec<gtk::Widget>,
}

impl SearchResults {
    pub fn new() -> Self {
        Self {
            container: gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .build(),
            childrens: vec![],
        }
    }

    pub fn container(&self) -> &gtk::Box {
        &self.container
    }

    pub fn push(&mut self, result: impl IsA<gtk::Widget>) {
        self.childrens.push(result.as_ref().to_owned());
        self.container.append(self.childrens.last().unwrap());
    }

    pub fn clear(&mut self) {
        for result in self.childrens.iter() {
            self.container.remove(result);
        }
        self.childrens.clear();
    }
}
