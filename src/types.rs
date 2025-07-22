use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub icon: String,
    pub links: HashMap<String, String>,
    pub number: usize,
}