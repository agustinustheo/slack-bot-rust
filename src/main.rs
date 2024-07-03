use dotenv::dotenv;
use reqwest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = reqwest::Client::new();
    let slack_token = env::var("SLACK_BOT_TOKEN").expect("SLACK_BOT_TOKEN not set in .env file");
    let channel = env::var("SLACK_CHANNEL_ID").expect("SLACK_CHANNEL_ID not set in .env file");
    let message = "Hello from Rust!";

    let response = client
        .post("https://slack.com/api/chat.postMessage")
        .header("Authorization", format!("Bearer {}", slack_token))
        .json(&json!({
            "channel": channel,
            "text": message
        }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Message sent successfully");
    } else {
        println!("Failed to send message: {:?}", response.text().await?);
    }

    Ok(())
}
