use reqwest::header;
use reqwest::StatusCode;
use serde::Serialize;
use std::error::Error;

const BROADCAST_API: &str = "https://api.line.me/v2/bot/message/broadcast";

#[derive(Serialize)]
pub struct Sender {
    pub name: Option<String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
}

impl Sender {
    pub fn new(name: Option<String>, icon_url: Option<String>) -> Sender {
        Sender {
            name: name,
            icon_url: icon_url,
        }
    }
}

#[derive(Serialize)]
pub struct Emoji {
    pub index: usize,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}

impl Emoji {
    pub fn new(index: usize, product_id: String, emoji_id: String) -> Emoji {
        Emoji {
            index: index,
            product_id: product_id,
            emoji_id: emoji_id,
        }
    }
}

#[derive(Serialize)]
pub struct Emojis {
    pub emojis: Vec<Emoji>,
}

impl Emojis {
    pub fn new(emojis: Vec<Emoji>) -> Emojis {
        Emojis { emojis: emojis }
    }
}

#[derive(Serialize)]
pub struct TextMessage {
    pub sender: Option<Sender>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: String,
    pub emojis: Option<Emojis>,
}

impl TextMessage {
    pub fn new(sender: Option<Sender>, text: String, emojis: Option<Emojis>) -> TextMessage {
        TextMessage {
            sender: sender,
            message_type: String::from("text"),
            text: text,
            emojis: emojis,
        }
    }
}

#[derive(Serialize)]
pub struct Messages {
    pub messages: Vec<TextMessage>,
}

impl Messages {
    pub fn new(messages: Vec<TextMessage>) -> Messages {
        Messages { messages: messages }
    }
}

pub struct BroadcastResponse {
    pub status: StatusCode,
}

pub async fn broadcast(
    channel_access_token: &str,
    messages: Messages,
) -> Result<BroadcastResponse, Box<dyn Error>> {
    let messages_json = serde_json::to_string(&messages)?;

    let client = reqwest::Client::new();
    let res: reqwest::Response = client
        .post(BROADCAST_API)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(channel_access_token)
        .body(messages_json)
        .send()
        .await?;

    Ok(BroadcastResponse {
        status: res.status(),
    })
}
