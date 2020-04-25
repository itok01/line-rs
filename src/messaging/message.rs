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
pub struct BaseSize {
    pub width: usize,
    pub height: usize,
}

impl BaseSize {
    pub fn new(width: usize, height: usize) -> BaseSize {
        BaseSize {
            width: width,
            height: height,
        }
    }
}

#[derive(Serialize)]
pub struct Area {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Area {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Area {
        Area {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

#[derive(Serialize)]
pub struct ExternalLink {
    pub link_uri: String,
    pub label: String,
}

impl ExternalLink {
    pub fn new<S: Into<String>>(link_uri: S, label: S) -> ExternalLink {
        ExternalLink {
            link_uri: link_uri.into(),
            label: label.into(),
        }
    }
}

#[derive(Serialize)]
pub struct Video {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
    pub area: Area,
    pub external_link: ExternalLink,
}

impl Video {
    pub fn new<S: Into<String>>(
        original_content_url: S,
        preview_image_url: S,
        area: Area,
        external_link: ExternalLink,
    ) -> Video {
        Video {
            original_content_url: original_content_url.into(),
            preview_image_url: preview_image_url.into(),
            area: area,
            external_link: external_link,
        }
    }
}

#[derive(Serialize)]
pub struct Action {
    sender: Option<Sender>,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "originalContentUrl")]
    original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    preview_image_url: String,
}

impl Action {
    pub fn new<S: Into<String>>(
        original_content_url: S,
        preview_image_url: S,
        sender: Option<Sender>,
    ) -> Action {
        Action {
            sender: sender,
            message_type: String::from("video"),
            original_content_url: original_content_url.into(),
            preview_image_url: preview_image_url.into(),
        }
    }
}

#[derive(Serialize)]
pub struct Actions {
    pub actions: Vec<Action>,
}

impl Actions {
    pub fn new() -> Actions {
        Actions {
            actions: Vec::new(),
        }
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

    VideoMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        #[serde(rename = "previewImageUrl")]
        preview_image_url: String,
    },

    AudioMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        duration: usize,
    },

    LocationMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        title: String,
        address: String,
        latitude: f64,
        longitude: f64,
    },

    ImagemapMessage {
        sender: Option<Sender>,
        #[serde(rename = "type")]
        message_type: String,
        base_url: String,
        alt_text: String,
        base_size: BaseSize,
        video: Option<Video>,
        actions: Actions,
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
        Message::ImageMessage {
            sender: sender,
            message_type: String::from("image"),
            original_content_url: original_content_url.into(),
            preview_image_url: preview_image_url.into(),
        }
    }

    pub fn new_video_message<S: Into<String>>(
        original_content_url: S,
        preview_image_url: S,
        sender: Option<Sender>,
    ) -> Message {
        Message::VideoMessage {
            sender: sender,
            message_type: String::from("video"),
            original_content_url: original_content_url.into(),
            preview_image_url: preview_image_url.into(),
        }
    }

    pub fn new_audio_message<S: Into<String>>(
        original_content_url: S,
        duration: usize,
        sender: Option<Sender>,
    ) -> Message {
        Message::AudioMessage {
            sender: sender,
            message_type: String::from("audio"),
            original_content_url: original_content_url.into(),
            duration: duration,
        }
    }

    pub fn new_location_message<S: Into<String>>(
        title: S,
        address: S,
        latitude: f64,
        longitude: f64,
        sender: Option<Sender>,
    ) -> Message {
        Message::LocationMessage {
            sender: sender,
            message_type: String::from("location"),
            title: title.into(),
            address: address.into(),
            latitude: latitude,
            longitude: longitude,
        }
    }

    pub fn new_imagemap_message<S: Into<String>>(
        base_url: S,
        alt_text: S,
        base_size: BaseSize,
        video: Option<Video>,
        actions: Actions,
        sender: Option<Sender>,
    ) -> Message {
        Message::ImagemapMessage {
            sender,
            message_type: String::from("imagemap"),
            base_url: base_url.into(),
            alt_text: alt_text.into(),
            base_size,
            video,
            actions,
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
