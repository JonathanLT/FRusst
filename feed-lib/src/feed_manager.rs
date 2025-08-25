// This module defines the FeedManager struct and its associated methods for managing multiple RSS feeds.
use crate::feed::Feed;
use crate::feed_storage::{get_feeds, add_feed, drop_feed};

pub struct FeedManager {
    feeds: Vec<Feed>,
}

impl FeedManager {
    pub fn new() -> Self {
        FeedManager { feeds: Vec::new() }
    }

    pub async fn add_feed(&mut self, feed: Feed) {
        add_feed(&feed).await.expect("Failed to add feed");
    }

    pub async fn remove_feed(&mut self, feed: Feed) {
        drop_feed(&feed).await.expect("Failed to drop feed");
    }

    pub async fn get_feeds(&self) -> Vec<Feed> {
        let feeds = get_feeds().await.expect("Failed to get feeds");
        feeds
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feed_manager() {
        let mut manager = FeedManager::new();
        let count_document_before = manager.get_feeds().await.len();
        let feed = Feed::new("Test FeedManager".into(), "http://example.com/feedmanager".into());
        manager.add_feed(feed.clone()).await;
        assert!(manager.get_feeds().await.iter().any(|f| f.url == feed.url && f.title == feed.title));
        manager.remove_feed(feed.clone()).await;
        assert!(manager.get_feeds().await.iter().all(|f| f.url != feed.url));
    }
}
