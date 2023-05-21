mod app_results;
mod file_browser;

pub use app_results::AppResults;
pub use file_browser::FileBrowser;

pub(self) use crate::search_results::ListItem;

pub trait Mode {
    /// Checks if mode is activated from the given query.
    ///
    /// Different modes can have different keywords for activating them.
    /// For eg: A mode named `finder` may activate if query starts with `/`
    fn is_activated(query: &str) -> bool;

    /// Checks if any data is present for creating list items
    fn is_empty(&self) -> bool;

    /// Creates list items by binding the data
    fn create_list_items(&self) -> Vec<ListItem>;

    /// Callback for when an item is selected by user
    fn on_item_selected(&self, item: &ListItem);
}
