use feed_lib::feed::Feed;
use feed_lib::feed_manager::FeedManager;
use clap::{Arg, Command};
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = Command::new("FRusst")
        .about("A simple RSS feed manager")
        .subcommand(
            Command::new("add")
                .about("Add a new RSS feed with a custom name")
                .arg(Arg::new("feed url")
                    .help("Feed URL")
                    .required(true)
                    .index(1))
                .arg(Arg::new("feed name")
                    .help("Feed name")
                    .required(true)
                    .index(2)),
        ).subcommand(
            Command::new("remove")
                .about("Remove an existing RSS feed")
                .arg(Arg::new("feed name")
                    .help("Feed name")
                    .required(true)
                    .index(1)),
        ).subcommand(
            Command::new("list")
                .about("List all RSS feeds")
        ).subcommand(
            Command::new("fetch")
                .about("Fetch new items from an existing RSS feed")
                .arg(Arg::new("feeds name")
                    .short('F')
                    .long("feed")
                    .value_delimiter(',')
                    .help("Feed names to fetch")),
        )
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        let url: String = add_matches.get_one::<String>("feed url").unwrap().clone();
        let title: String = add_matches.get_one::<String>("feed name").unwrap().clone();

        let mut feed_manager = FeedManager::new();

        let feed = Feed::new(title, url);
        feed_manager.add_feed(feed).await;
        println!("Feed added successfully!");

    }
    if let Some(remove_matches) = matches.subcommand_matches("remove") {
        let title: String = remove_matches.get_one::<String>("feed name").unwrap().clone();

        let mut feed_manager = FeedManager::new();

        let feed = Feed::new(title, "_".into());
        feed_manager.remove_feed(feed).await;
        println!("Feed removed successfully!");
    }
    if let Some(_) = matches.subcommand_matches("list") {
        let feed_manager = FeedManager::new();

        feed_manager.get_feeds().await.iter().for_each(|f| {
            println!("Feed: {} - URL: {}", f.title, f.url);
        });
    }

    if let Some(fetch_matches) = matches.subcommand_matches("fetch") {
        let titles: Vec<_> = fetch_matches.get_many::<String>("feeds name").unwrap_or_default().collect();
        let feed_manager = FeedManager::new();
        if titles.is_empty() {
            println!("Fetching all feeds");
            feed_manager.fetch_feeds(None).await;
        } else {
            for title in titles {
                println!("Fetching feed: {}", title);
                feed_manager.fetch_feeds(Some(vec![title.to_string()])).await;
            }
        }
    }

}
