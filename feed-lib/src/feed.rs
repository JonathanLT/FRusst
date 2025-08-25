// This file defines the Feed struct and its associated methods for managing individual RSS feeds.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub title: String,
    pub url: String,
}

impl Feed {
    pub fn new(title: String, url: String) -> Self {
        println!("Adding RSS feed: {} with title: {}", url, title);
        Feed { title, url }
    }

    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn update_url(&mut self, new_url: String) {
        self.url = new_url;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_creation() {
        let feed = Feed::new("Test Feed".into(), "http://example.com/rss".into());
        assert_eq!(feed.title, "Test Feed");
        assert_eq!(feed.url, "http://example.com/rss");
    }

    #[test]
    fn test_feed_update() {
        let mut feed = Feed::new("Test Feed".into(), "http://example.com/rss".into());
        feed.update_title("Updated Feed".into());
        feed.update_url("http://example.com/updated/rss".into());
        assert_eq!(feed.title, "Updated Feed");
        assert_eq!(feed.url, "http://example.com/updated/rss");
    }
}
