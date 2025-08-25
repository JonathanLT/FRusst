use std::env;
use futures::StreamExt;
use urlencoding::encode;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use mongodb::bson::Document;

// Crate feed-lib
use crate::feed::Feed;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: i64,
    pub source: String,
    pub url: String,
}

/// Add a new feed to the MongoDB
pub async fn add_feed(feed: &Feed) -> mongodb::error::Result<()> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap();
    let mongodb_login = env::var("MONGODB_LOGIN").unwrap();
    let mongodb_pwd = env::var("MONGODB_PWD").unwrap();
    let mongodb_cluster = env::var("MONGODB_CLUSTER").unwrap();

    let url_encoded = format!("mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}", mongodb_login, encode(&mongodb_pwd), mongodb_uri, mongodb_cluster);
    let mut client_options =
    ClientOptions::parse(&url_encoded).await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let new_feed = doc! {
        "title": feed.title.clone(),
        "url": feed.url.clone(),
        "added_at": Utc::now().timestamp_millis()
    };

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    let feeds =client
        .database("frusst")
        .collection("feeds");
    // Insert feed if not exists
    let feeds_list = feeds.find(doc! {"url": feed.url.clone()}).await?;
    if feeds_list.count().await == 0 {
        let insert_result = feeds.insert_one(new_feed.clone()).await?;
        println!("New feed ID: {}", insert_result.inserted_id);
    } else {
        println!("Feed with URL '{}' already exists.", feed.url);
    }

    Ok(())
}

/// Get all feeds from MongoDB
pub async fn get_feeds() -> mongodb::error::Result<Vec<Feed>> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap();
    let mongodb_login = env::var("MONGODB_LOGIN").unwrap();
    let mongodb_pwd = env::var("MONGODB_PWD").unwrap();
    let mongodb_cluster = env::var("MONGODB_CLUSTER").unwrap();

    let url_encoded = format!("mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}", mongodb_login, encode(&mongodb_pwd), mongodb_uri, mongodb_cluster);
    let mut client_options =
    ClientOptions::parse(&url_encoded).await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    let feeds_collection = client.database("frusst").collection::<Document>("feeds");
    let mut cursor = feeds_collection.find(doc! {}).await?;
    let mut feeds: Vec<Feed> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let title: String = document.get_str("title").unwrap_or("").to_string();
                let url: String = document.get_str("url").unwrap_or("").to_string();
                if !title.is_empty() && !url.is_empty() {
                    feeds.push(Feed { title, url });
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(feeds)
}

/// Drop a feed in MongoDB
pub async fn drop_feed(feed: &Feed) -> mongodb::error::Result<()> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap();
    let mongodb_login = env::var("MONGODB_LOGIN").unwrap();
    let mongodb_pwd = env::var("MONGODB_PWD").unwrap();
    let mongodb_cluster = env::var("MONGODB_CLUSTER").unwrap();

    let url_encoded = format!("mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}", mongodb_login, encode(&mongodb_pwd), mongodb_uri, mongodb_cluster);
    let mut client_options =
    ClientOptions::parse(&url_encoded).await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    let feeds_collection = client.database("frusst").collection::<Document>("feeds");
    let delete_result = feeds_collection.delete_many(doc! {"title": feed.title.clone()}).await?;
    println!("Deleted {} document(s)", delete_result.deleted_count);

    Ok(())
}

/// Ping MongoDB to check connection
pub async fn ping() -> mongodb::error::Result<()> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap();
    let mongodb_login = env::var("MONGODB_LOGIN").unwrap();
    let mongodb_pwd = env::var("MONGODB_PWD").unwrap();
    let mongodb_cluster = env::var("MONGODB_CLUSTER").unwrap();

    let url_encoded = format!("mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}", mongodb_login, encode(&mongodb_pwd), mongodb_uri, mongodb_cluster);
    let mut client_options =
    ClientOptions::parse(&url_encoded).await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping() {
        ping().await.expect("Failed to ping");
    }

    #[tokio::test]
    async fn test_add_and_get_feeds() {
        let feed = Feed {
            title: "Test FeedStorage".into(),
            url: "http://example.com/feedstorage".into(),
        };
        add_feed(&feed).await.expect("Failed to add feed");
        let feeds = get_feeds().await.expect("Failed to get feeds");
        assert!(feeds.iter().any(|f| f.url == feed.url && f.title == feed.title));
        drop_feed(&feed).await.expect("Failed to drop feed");
        let feeds_after_drop = get_feeds().await.expect("Failed to get feeds after drop");
        assert!(feeds_after_drop.iter().all(|f| f.url != feed.url));
    }
}