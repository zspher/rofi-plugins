mod search_entry;
pub use search_entry::SearchEntry;

include!(concat!(env!("OUT_DIR"), "/generated_items.rs"));
