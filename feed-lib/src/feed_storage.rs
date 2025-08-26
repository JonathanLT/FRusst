use std::env;
use futures::StreamExt;
use urlencoding::encode;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, error::Error};
use chrono::Utc;
use mongodb::bson::Document;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use log::{info, debug};

// Crate feed-lib
use crate::feed::Feed;
use crate::feed_item::FeedItem;


pub struct MongoDbClient {
    pub client: Arc<Client>,
}

static MONGO_CLIENT: OnceCell<MongoDbClient> = OnceCell::new();

/// Initialise et retourne un client MongoDB partagé
pub async fn get_mongo_client() -> mongodb::error::Result<&'static MongoDbClient> {
    if MONGO_CLIENT.get().is_none() {
        let mongodb_uri: String = env::var("MONGODB_URI").unwrap();
        let mongodb_login: String = env::var("MONGODB_LOGIN").unwrap();
        let mongodb_pwd: String = env::var("MONGODB_PWD").unwrap();
        let mongodb_cluster: String = env::var("MONGODB_CLUSTER").unwrap();

        let url_encoded: String = format!(
            "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}",
            mongodb_login,
            encode(&mongodb_pwd),
            mongodb_uri,
            mongodb_cluster
        );
        let mut client_options: ClientOptions = ClientOptions::parse(&url_encoded).await?;
        let server_api: ServerApi = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;
        MONGO_CLIENT.set(MongoDbClient { client: Arc::new(client) }).ok();
    }
    Ok(MONGO_CLIENT.get().unwrap())
}

/// Add a new feed to the MongoDB
pub async fn add_feed(feed: &Feed) -> mongodb::error::Result<()> {
    let mongo = get_mongo_client().await?;
    let feeds: mongodb::Collection<Document> = mongo
        .client
        .database("frusst")
        .collection("feeds");

    // Create a new feed document
    let new_feed: Document = doc! {
        "title": feed.title.clone(),
        "url": feed.url.clone(),
        "added_at": Utc::now().timestamp_millis()
    };
    // Insert feed if not exists
    let feeds_list: mongodb::Cursor<Document> = feeds.find(doc! {"url": feed.url.clone()}).await?;
    if feeds_list.count().await == 0 {
        let insert_result = feeds.insert_one(new_feed.clone()).await?;
        info!("New feed ID: {}", insert_result.inserted_id);
    } else {
        debug!("Feed with URL '{}' already exists.", feed.url);
    }

    Ok(())
}

/// Get all feeds from MongoDB
pub async fn get_feeds() -> mongodb::error::Result<Vec<Feed>> {
    let mongo = get_mongo_client().await?;
    let feeds_collection: mongodb::Collection<Document> = mongo
        .client
        .database("frusst")
        .collection::<Document>("feeds");
    let mut cursor: mongodb::Cursor<Document> = feeds_collection.find(doc! {}).await?;
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

/// Get a feed in MongoDB
pub async fn get_feed(title: String) -> mongodb::error::Result<Feed> {
    let mongo = get_mongo_client().await?;
    let feeds_collection: mongodb::Collection<Document> = mongo
        .client
        .database("frusst")
        .collection::<Document>("feeds");

    let feed: Document = feeds_collection
        .find_one(doc! { "title": &title })
        .await?
        .ok_or_else(|| Error::custom(format!("No feed named '{}' found", title)))?;
    Ok(Feed {
        title: feed.get_str("title").unwrap_or("").to_string(),
        url: feed.get_str("url").unwrap_or("").to_string(),
    })
}

/// Drop a feed in MongoDB
pub async fn drop_feed(feed: &Feed) -> mongodb::error::Result<()> {
    let mongo = get_mongo_client().await?;
    let feeds_collection: mongodb::Collection<Document> = mongo
        .client
        .database("frusst")
        .collection::<Document>("feeds");
    let delete_result = feeds_collection.delete_many(doc! {"title": feed.title.clone()}).await?;
    info!("Deleted {} document(s)", delete_result.deleted_count);

    Ok(())
}

/// Ping MongoDB to check connection
pub async fn ping() -> mongodb::error::Result<()> {
    let mongo = get_mongo_client().await?;
    mongo
        .client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await?;
    info!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(())
}

/// Add feed item into MongoDB
pub async fn add_feed_item(item: FeedItem) -> mongodb::error::Result<()> {
    let mongo = get_mongo_client().await?;
    let feeds_items: mongodb::Collection<Document> = mongo
        .client
        .database("frusst")
        .collection("feeds_items");

    // Insert feed if not exists
    let new_feed_item: Document = doc! {
        "id": item.id.clone(),
        "title": item.title.clone(),
        "link": item.link.clone(),
        "description": item.description.clone(),
        "pub_date": item.pub_date,
        "source": item.source.clone(),
        "url": item.url.clone(),
        "added_at": Utc::now().timestamp_millis()
    };

    let feed_items_list: mongodb::Cursor<Document> = feeds_items.find(doc! {"id": item.id.clone()}).await?;
    if feed_items_list.count().await == 0 {
        let insert_result  = feeds_items.insert_one(new_feed_item.clone()).await?;
        debug!("New feed ID: {}", insert_result.inserted_id);
    } else {
        debug!("Feed with ID '{}' already exists.", item.id);
    }

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