use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

impl<'a> Bookmark<'a> {
    pub fn new(name: &'a str, url: &'a str) -> Self {
        Self { name, url }
    }
}
