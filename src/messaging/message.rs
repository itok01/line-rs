use reqwest::StatusCode;
use serde::Serialize;
use std::cell::RefCell;
use std::error::Error;

use super::client::post_json;

const SEND_REPLY_API: &str = "https://api.line.me/v2/bot/message/reply";
const SEND_PUSH_API: &str = "https://api.line.me/v2/bot/message/push";
const SEND_MULTICAST_API: &str = "https://api.line.me/v2/bot/message/multicast";
const SEND_NARROWCAST_API: &str = "https://api.line.me/v2/bot/message/narrowcast";
const GET_NARROWCAST_STATUS_API: &str = "https://api.line.me/v2/bot/message/progress/narrowcast";
const SEND_BROADCAST_API: &str = "https://api.line.me/v2/bot/message/broadcast";
const GET_ADDITIONAL_MESSAGE_LIMIT_API: &str = "https://api.line.me/v2/bot/message/quota";
const GET_THIS_MONTH_MESSAGES_COUNT_API: &str =
    "https://api.line.me/v2/bot/message/quota/consumption";
const GET_REPLY_MESSAGES_COUNT_API: &str = "https://api.line.me/v2/bot/message/delivery/reply";
const GET_PUSH_MESSAGES_COUNT_API: &str = "https://api.line.me/v2/bot/message/delivery/push";
const GET_MULTICAST_MESSAGES_COUNT_API: &str =
    "https://api.line.me/v2/bot/message/delivery/multicast";
const GET_BROADCAST_MESSAGE_COUNT_API: &str =
    "https://api.line.me/v2/bot/message/delivery/broadcast";

macro_rules! get_content_api {
    ($message_id:tt) => {
        format!(
            "https://api-data.line.me/v2/bot/message/{}/content",
            $message_id
        )
    };
}

#[derive(Serialize)]
pub struct Sender {
    pub name: Option<String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
}

impl Sender {
    pub fn new(name: Option<String>, icon_url: Option<String>) -> Sender {
        Sender { name, icon_url }
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
            index,
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
        Emojis { emojis }
    }
}

#[derive(Serialize)]
pub struct BaseSize {
    pub width: usize,
    pub height: usize,
}

impl BaseSize {
    pub fn new(width: usize, height: usize) -> BaseSize {
        BaseSize { width, height }
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
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Serialize)]
pub struct ExternalLink {
    #[serde(rename = "linkUri")]
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
    #[serde(rename = "externalLink")]
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
            area,
            external_link,
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Action {
    URI {
        #[serde(rename = "type")]
        action_type: String,
        label: String,
        #[serde(rename = "linkUri")]
        link_uri: String,
        area: Area,
    },

    Message {
        #[serde(rename = "type")]
        action_type: String,
        label: String,
        text: String,
        area: Area,
    },
}

impl Action {
    pub fn new_uri_action<S: Into<String>>(label: S, link_uri: S, area: Area) -> Action {
        Action::URI {
            action_type: String::from("uri"),
            label: label.into(),
            link_uri: link_uri.into(),
            area,
        }
    }

    pub fn new_message_action<S: Into<String>>(label: S, text: S, area: Area) -> Action {
        Action::Message {
            action_type: String::from("message"),
            label: label.into(),
            text: text.into(),
            area,
        }
    }
}

#[derive(Serialize)]
#[serde(transparent)]
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
        #[serde(rename = "baseUrl")]
        base_url: String,
        #[serde(rename = "altText")]
        alt_text: String,
        #[serde(rename = "baseSize")]
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
            sender,
            message_type: String::from("text"),
            text: text.into(),
            emojis,
        }
    }

    pub fn new_sticker_message<S: Into<String>>(
        package_id: S,
        sticker_id: S,
        sender: Option<Sender>,
    ) -> Message {
        Message::StickerMessage {
            sender,
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
            sender,
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
            sender,
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
            sender,
            message_type: String::from("audio"),
            original_content_url: original_content_url.into(),
            duration,
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
            sender,
            message_type: String::from("location"),
            title: title.into(),
            address: address.into(),
            latitude,
            longitude,
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
#[serde(transparent)]
pub struct Messages {
    pub messages: RefCell<Vec<Message>>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            messages: RefCell::from(Vec::with_capacity(5)),
        }
    }

    pub fn add(&self, message: Message) -> Result<(), &str> {
        if self.messages.borrow().len() < 5 {
            self.messages.borrow_mut().push(message);
            Ok(())
        } else {
            Err("Messages limit is 5")
        }
    }
}

#[derive(Serialize)]
pub struct BroadcastRequest {
    pub messages: Messages,
}

pub struct BroadcastResponse {
    pub status: StatusCode,
    pub system_message: String,
}

pub async fn broadcast(
    channel_access_token: &str,
    request: BroadcastRequest,
) -> Result<BroadcastResponse, Box<dyn Error>> {
    let request_json = serde_json::to_string(&request)?;

    let res = post_json(channel_access_token, SEND_BROADCAST_API, &request_json).await?;

    Ok(BroadcastResponse {
        status: res.status(),
        system_message: res.text().await?,
    })
}
