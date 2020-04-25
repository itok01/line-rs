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
#[serde(untagged)]
pub enum Message {
    TextMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        text: String,
        emojis: Option<Emojis>,
    },

    StickerMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "stickerId")]
        sticker_id: String,
    },

    ImageMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        #[serde(rename = "previewImageUrl")]
        preview_image_url: String,
    },
}

impl Message {
    pub fn new_text_message<S: Into<String>>(
        text: S,
        emojis: Option<Emojis>,
        sender: Option<Sender>,
    ) -> Message {
        Message::TextMessage {
            sender: sender,
            message_type: String::from("text"),
            text: text.into(),
            emojis: emojis,
        }
    }

    pub fn new_sticker_message<S: Into<String>>(
        package_id: S,
        sticker_id: S,
        sender: Option<Sender>,
    ) -> Message {
        Message::StickerMessage {
            sender: sender,
            message_type: String::from("sticker"),
            package_id: package_id.into(),
            sticker_id: sticker_id.into(),
        }
    }

    pub fn new_image_message<S: Into<String>>(
        original_content_url: S,
        preview_image_url: S,
        sender: Option<Sender>,
    ) -> Message {
        Message::StickerMessage {
            sender: sender,
            message_type: String::from("sticker"),
            package_id: original_content_url.into(),
            sticker_id: preview_image_url.into(),
        }
    }
}

#[derive(Serialize)]
pub struct Messages {
    pub messages: Vec<Message>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            messages: Vec::new(),
        }
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
