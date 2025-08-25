use feed_lib::feed::Feed;
use feed_lib::feed_manager::FeedManager;
use clap::{Arg, Command};

#[tokio::main]
async fn main() {
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
        let mut feed_manager = FeedManager::new();

        feed_manager.get_feeds().await.iter().for_each(|f| {
            println!("Feed: {} - URL: {}", f.title, f.url);
        });
    }
}
