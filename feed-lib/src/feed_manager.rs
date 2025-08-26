// This module defines the FeedManager struct and its associated methods for managing multiple RSS feeds.
use crate::feed::Feed;
use crate::feed_item::FeedItem;
use crate::feed_storage::{get_feeds, get_feed, add_feed, drop_feed};
use crate::feed_reader::{read_feed_data};

pub struct FeedManager {
    feeds: Vec<Feed>,
    feeds_items: Vec<FeedItem>,
}

impl FeedManager {
    pub fn new() -> Self {
        FeedManager { feeds: Vec::new(), feeds_items: Vec::new() }
    }

    pub async fn add_feed(&mut self, feed: Feed) {
        add_feed(&feed).await.expect("Failed to add feed");
    }

    pub async fn remove_feed(&mut self, feed: Feed) {
        drop_feed(&feed).await.expect("Failed to drop feed");
    }

    pub async fn get_feeds(&self) -> Vec<Feed> {
        get_feeds().await.expect("Failed to get feeds")
    }

    pub async fn get_feed(&self, title: String) -> Feed {
        get_feed(title).await.expect("Failed to get feed")
    }

    pub async fn fetch_feeds(&self, feeds: Option<Vec<String>>) -> Vec<FeedItem> {
        // Fetch feed items for each feed
        let mut all_items = Vec::new();

        // Check if no specific feeds are provided
        if feeds.is_none() {
            let all_feeds = get_feeds().await;
            for feed in all_feeds.unwrap() {
                println!("Fetching {} feeds...", feed.title);
                read_feed_data(feed).await;
            }
        } else {
            for feed in feeds.unwrap() {
                let feed_data = get_feed(feed).await;
                if feed_data.is_ok() {
                    read_feed_data(feed_data.unwrap()).await;
                }
            }
        }

        all_items
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feed_manager() {
        let mut manager = FeedManager::new();
        let feed = Feed::new("Test FeedManager".into(), "http://example.com/feedmanager".into());
        manager.add_feed(feed.clone()).await;
        assert!(manager.get_feeds().await.iter().any(|f| f.url == feed.url && f.title == feed.title));
        manager.remove_feed(feed.clone()).await;
        assert!(manager.get_feeds().await.iter().all(|f| f.url != feed.url));
    }
}
