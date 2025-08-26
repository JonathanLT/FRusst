/// This module is responsible for reading and parsing feed data from various sources.

use crate::feed::Feed;
use crate::feed_item::FeedItem;
use crate::feed_storage::add_feed_item;
use tokio::task;
use sha2::{Sha256, Digest};
use chrono::Utc;
use feed_rs::parser;
use reqwest;
use std::io::Cursor;
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::Read;

/// Async helper – fetch a single feed URL and return its body as bytes
async fn fetch_feed_url(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?.error_for_status()?;
    let bytes = resp.bytes().await?;
    Ok(bytes.to_vec())
}

/// Reads the feed data from the specified feed and returns a vector of FeedItem.
pub async fn read_feed_data(feed: Feed) {
    // Get body as bytes
    let body_bytes = fetch_feed_url(&feed.url).await.expect("Failed to fetch feed URL");

    // Build a decoder with auto-detected encoding (UTF-8, ISO-8859-1, etc.)
    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(None) // auto-detection
        .build(Cursor::new(body_bytes));

    // Parse the decoded stream directly
    let feed_parsed = parser::parse(&mut decoder)
        .map_err(|e| {
            eprintln!("Feed parsing error: {e}");
            e
        })
        .ok();

    if let Some(feed_data) = feed_parsed {
        // You can create your FeedItem here
        let items: Vec<FeedItem> = feed_data.entries.iter().map(|entry| {
            let title = entry.title.as_ref().map(|t| t.content.clone()).unwrap_or_else(|| "No title".into());
            let link = entry.links.get(0).map(|l| l.href.clone()).unwrap_or_else(|| "".into());
            let description = entry.summary.as_ref().map(|s| s.content.clone());
            let pub_date = entry.published.map(|d| d.timestamp()).unwrap_or(0);
            let id_hashed = format!("{:x}", Sha256::digest(title.clone()));
            let added_at = Utc::now().to_rfc3339();

            FeedItem::new(
                id_hashed,
                title,
                link,
                description,
                pub_date,
                feed.title.clone(),
                feed.url.clone(),
                added_at,
            )
        }).collect();

        for item in items {
            if let Err(e) = add_feed_item(item).await {
                eprintln!("Failed to add feed item: {e}");
            }
        }
    } else {
        eprintln!("Unable to parse RSS/Atom/JSON feed.");
    }
}

/// To process multiple feeds in parallel:
pub async fn read_multiple_feeds(feeds: Vec<Feed>) {
    let handles: Vec<_> = feeds.into_iter().map(|feed| {
        task::spawn(read_feed_data(feed))
    }).collect();

    for handle in handles {
        handle.await.expect("Feed task panicked");
    }
}
