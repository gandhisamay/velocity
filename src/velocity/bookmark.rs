use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub map: HashMap<String, String>,
}

impl Bookmark {
    pub fn new(name: String, url: String) -> Self {
        let mut bookmark = HashMap::new();
        bookmark.insert(name, url);
        Self { map: bookmark }
    }
}
