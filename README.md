# FRusst 🧠📡

**FRusst** is a lightweight and fast RSS feed reader built with **Rust**. Stream, manage, and organize your feeds effortlessly while enjoying the power and safety of Rust.

[![Rust](https://img.shields.io/badge/Rust-1.89.0-orange)](https://www.rust-lang.org/)
[![License Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue)](LICENSE-Apache)
[![License MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE-MIT)
[![Issues](https://img.shields.io/github/issues/JonathanLT/FRusst)](https://github.com/JonathanLT/FRusst/issues)

## Features

- 🦀 **Built with Rust**: Fast, safe, and efficient.
- 📡 **RSS Feed Streaming**: Subscribe and get updates in real-time.
- ⚡ **Lightweight**: Minimal dependencies and optimized for speed.
- 🔧 **Easy to Configure**: Simple setup to start reading feeds quickly.
- 🖼️ **Optional GUI** (coming soon) for desktop feed management.

## Installation

Make sure you have Rust installed. Then:

```bash
git clone https://github.com/yourusername/FRusst.git
cd FRusst
mkdir .cargo && touch .cargo/config.toml

cat <<EOF >.cargo/config.toml
[env]
MONGODB_URI = "<mongodb.net cluster>"
MONGODB_LOGIN = "<username>"
MONGODB_PWD = "<password>"
MONGODB_CLUSTER = "<cluster name>"
EOF

cargo build --release
```

The binary will be in `target/release/frusst`.

## Usage

```bash
# Manage some feed
./frusst add https://example.com/rss example       # Add a new RSS feed with a custom name
./frusst list                                      # List all subscribed feeds
./frusst remove example.com                        # Remove a feed

# Update feeds news
./frusst fetch                                     # Fetch the latest articles from all feeds
./frusst fetch -F[--feed] "example"                # Fetch the latest articles from feed(s), seperated by comma
./frusst get                                       # Show the last 10 articles
./frusst get -n 5                                  # Show the last 5 articles
./frusst get -F[--feed] "example"                  # Show the last 10 articles from feed(s), seperated by comma
```

## Roadmap

| Feature                          | Status       |
|---------------------------------|-------------|
| Core CLI feed management          | ✅ Done      |
| Performance optimizations         | ⚡ Planned   |
| GUI for desktop feeds             | 🖼️ Future  |
| Atom feed support                 | 🌐 Future  |
| Publish on crates.io              | 📦 Future  |
| Mobile-friendly interface         | 📱 Future    |

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

Follow Rust's best practices and formatting.

## License

This project is licensed under the Apache 2.0 or MIT licenses. See the [LICENSE-APACHE](LICENSE-Apache) or [LICENSE-MIT](LICENSE-MIT) files for details.

## Tagline

**\"FRusst: Where Rust meets RSS Feed.\"**

---

*Start your Rust-powered RSS journey today!*
