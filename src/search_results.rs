use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SearchResults(ObjectSubclass<SearchResultsContainer>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SearchResults {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn append_new(&self, text: gtk::Text) {
        let mut results = self.imp().results.borrow_mut();
        results.push(text);
        self.append(results.last().unwrap());
    }

    pub fn clear_all(&self) {
        let mut previous_results = self.imp().results.borrow_mut();
        for result in previous_results.iter() {
            self.remove(result);
        }
        previous_results.clear();
    }
}

impl Default for SearchResults {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct SearchResultsContainer {
    pub results: RefCell<Vec<gtk::Text>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SearchResultsContainer {
    const NAME: &'static str = "SearchResultsContainer";
    type Type = SearchResults;
    type ParentType = gtk::Box;
}

impl ObjectImpl for SearchResultsContainer {
    fn constructed(&self) {
        self.obj().set_orientation(gtk::Orientation::Vertical);
    }
}

impl WidgetImpl for SearchResultsContainer {}
impl BoxImpl for SearchResultsContainer {}
