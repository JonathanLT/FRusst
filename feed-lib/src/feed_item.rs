// This file defines the Feed struct and its associated methods for managing individual RSS feeds.

use serde::{Deserialize, Serialize};
use log::{debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: i64,
    pub source: String,
    pub url: String,
    pub added_at: String,
}

impl FeedItem {
    pub fn new(id: String, title: String, link: String, description: Option<String>, pub_date: i64, source: String, url: String, added_at: String) -> Self {
        debug!("Adding RSS feed item: {} with title: {}", id, title);
        FeedItem { id, title, link, description, pub_date, source, url, added_at }
    }

    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
    }
    pub fn update_link(&mut self, new_link: String) {
        self.link = new_link;
    }
    pub fn update_description(&mut self, new_description: Option<String>) {
        self.description = new_description;
    }
    pub fn update_pub_date(&mut self, new_pub_date: i64) {
        self.pub_date = new_pub_date;
    }
    pub fn update_source(&mut self, new_source: String) {
        self.source = new_source;
    }
    pub fn update_url(&mut self, new_url: String) {
        self.url = new_url;
    }
    pub fn update_added_at(&mut self, new_added_at: String) {
        self.added_at = new_added_at;
    }
}
