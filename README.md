# Slack Messenger Rust

A simple Rust application that sends messages to Slack using the Slack Web API.

## Features

- Send messages to a specified Slack channel
- Use environment variables for secure configuration
- Asynchronous operation using Tokio runtime

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager
- A Slack workspace with admin privileges
- A Slack bot token with appropriate permissions

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-username/slack-messenger-rust.git
   cd slack-messenger-rust
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Configuration

1. Create a `.env` file in the project root directory.
2. Add the following environment variables:
   ```
   SLACK_BOT_TOKEN=xoxb-your-bot-token-here
   SLACK_CHANNEL_ID=C0123456789
   ```
   Replace with your actual Slack bot token and channel ID.

## Usage

Run the application:

```
cargo run --release
```

This sends a default "Hello from Rust!" message to the specified Slack channel.

To modify the message, edit the `message` variable in `src/main.rs`.

## Project Structure

```
slack-messenger-rust/
├── src/
│   └── main.rs
├── .env
├── .gitignore
├── Cargo.toml
└── README.md
```

## Dependencies

- `reqwest`: HTTP client for Slack API requests
- `tokio`: Asynchronous runtime
- `serde_json`: JSON serialization/deserialization
- `dotenv`: Loading environment variables

See `Cargo.toml` for versions.

## Contributing

1. Fork the repository
2. Create a new branch: `git checkout -b feature-branch-name`
3. Make your changes and commit them: `git commit -m 'Add some feature'`
4. Push to the original branch: `git push origin feature-branch-name`
5. Create the pull request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.