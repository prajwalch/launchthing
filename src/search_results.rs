use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SearchResults(ObjectSubclass<imp::SearchResultsContainer>)
        @extends    gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SearchResults {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn push(&self, result: impl IsA<gtk::Widget>) {
        let mut results = self.imp().results.borrow_mut();
        results.push(result.as_ref().to_owned());
        self.append(results.last().unwrap());
    }

    pub fn clear(&self) {
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

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct SearchResultsContainer {
        pub results: RefCell<Vec<gtk::Widget>>,
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
}
