use reqwest::{Client, Error};
use std::{collections::HashMap, time::Duration};

const SLACK_API_URL: &str = "https://slack.com/api/chat.postMessage";

async fn send_message(text: String) -> Result<(), Error> {
    let channel = "";
    let token = "";

    let mut body = HashMap::new();
    body.insert("channel", channel);
    body.insert("text", &text);

    let client = Client::new();
    let _res = client.post(SLACK_API_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&body)
        .timeout(Duration::from_secs(30))
        .send()
        .await?;

    Ok(())
}
