use serde::{self, Deserialize, Serialize};

// Add the `bound` attribute here
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SearchEntry<'a> {
    // Tell serde to borrow the data instead of allocating new Strings
    #[serde(borrow)]
    pub title: &'a str,
    #[serde(borrow)]
    pub url: &'a str,
}
