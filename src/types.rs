use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub tags: Vec<String>,
    pub stars: usize,
    pub forks: usize,
    pub icon: String,
    pub links: HashMap<String, String>,
    pub number: usize,
}