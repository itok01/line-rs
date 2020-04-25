use reqwest::StatusCode;
use serde::Serialize;
use std::error::Error;

use super::client::post_json;

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
    pub fn new<S: Into<String>>(index: usize, product_id: S, emoji_id: S) -> Emoji {
        Emoji {
            index: index,
            product_id: product_id.into(),
            emoji_id: emoji_id.into(),
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
    pub fn new<S: Into<String>>(
        text: S,
        emojis: Option<Emojis>,
        sender: Option<Sender>,
    ) -> TextMessage {
        TextMessage {
            sender: sender,
            message_type: String::from("text"),
            text: text.into(),
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

    let res = post_json(channel_access_token, BROADCAST_API, &messages_json).await?;

    Ok(BroadcastResponse {
        status: res.status(),
    })
}
