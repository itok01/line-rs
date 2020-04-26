use bytes::Bytes;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;

use super::client::{get, post_json};

const SEND_REPLY_MESSAGE_API: &str = "https://api.line.me/v2/bot/message/reply";
const SEND_PUSH_MESSAGE_API: &str = "https://api.line.me/v2/bot/message/push";
const SEND_MULTICAST_MESSAGE_API: &str = "https://api.line.me/v2/bot/message/multicast";
const SEND_NARROWCAST_MESSAGE_API: &str = "https://api.line.me/v2/bot/message/narrowcast";
const GET_NARROWCAST_MESSAGE_STATUS_API: &str =
    "https://api.line.me/v2/bot/message/progress/narrowcast";
const SEND_BROADCAST_MESSAGE_API: &str = "https://api.line.me/v2/bot/message/broadcast";
const GET_ADDITIONAL_MESSAGES_LIMIT_API: &str = "https://api.line.me/v2/bot/message/quota";
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
pub struct SendReplyMessageRequest {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub messages: Messages,
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
}

pub struct SendReplyMessageResponse {
    pub status: StatusCode,
    pub system_message: String,
}

pub async fn send_reply_message(
    channel_access_token: &str,
    request: SendReplyMessageRequest,
) -> Result<SendReplyMessageResponse, Box<dyn Error>> {
    let request_json = serde_json::to_string(&request)?;

    let res = post_json(channel_access_token, SEND_REPLY_MESSAGE_API, &request_json).await?;

    Ok(SendReplyMessageResponse {
        status: res.status(),
        system_message: res.text().await?,
    })
}

#[derive(Serialize)]
pub struct SendPushMessageRequest {
    #[serde(rename = "replyToken")]
    pub to: String,
    pub messages: Messages,
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
}

pub struct SendPushMessageResponse {
    pub status: StatusCode,
    pub system_message: String,
}

pub async fn send_push_message(
    channel_access_token: &str,
    request: SendPushMessageRequest,
) -> Result<SendPushMessageResponse, Box<dyn Error>> {
    let request_json = serde_json::to_string(&request)?;

    let res = post_json(channel_access_token, SEND_PUSH_MESSAGE_API, &request_json).await?;

    Ok(SendPushMessageResponse {
        status: res.status(),
        system_message: res.text().await?,
    })
}

#[derive(Serialize)]
pub struct SendMulticastMessageRequest {
    #[serde(rename = "replyToken")]
    pub to: Vec<String>,
    pub messages: Messages,
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
}

pub struct SendMulticastMessageResponse {
    pub status: StatusCode,
    pub system_message: String,
}

pub async fn send_multicast_message(
    channel_access_token: &str,
    request: SendMulticastMessageRequest,
) -> Result<SendMulticastMessageResponse, Box<dyn Error>> {
    let request_json = serde_json::to_string(&request)?;

    let res = post_json(
        channel_access_token,
        SEND_MULTICAST_MESSAGE_API,
        &request_json,
    )
    .await?;

    Ok(SendMulticastMessageResponse {
        status: res.status(),
        system_message: res.text().await?,
    })
}

#[derive(Serialize)]
pub enum Recipient {
    Audience {
        #[serde(rename = "type")]
        recipient_type: String,
        #[serde(rename = "audienceGroupId")]
        audience_group_id: u64,
    },

    Operator {
        #[serde(rename = "type")]
        recipient_type: String,
        and: Option<Vec<Box<Recipient>>>,
        or: Option<Vec<Box<Recipient>>>,
        not: Option<Box<Recipient>>,
    },
}

#[derive(Serialize)]
pub enum Demographic {
    Gender {
        #[serde(rename = "type")]
        demographic_type: String,
        #[serde(rename = "oneOf")]
        one_of: Vec<String>,
    },

    Age {
        #[serde(rename = "type")]
        demographic_type: String,
        gte: Option<String>,
        lt: Option<String>,
    },

    AppType {
        #[serde(rename = "type")]
        demographic_type: String,
        one_of: Vec<String>,
    },

    Area {
        #[serde(rename = "type")]
        demographic_type: String,
        one_of: Vec<String>,
    },

    SubscriptionPeriod {
        #[serde(rename = "type")]
        gte: Option<String>,
        lt: Option<String>,
    },

    Operator {
        #[serde(rename = "type")]
        demographic_type: String,
        and: Option<Vec<Box<Demographic>>>,
        or: Option<Vec<Box<Demographic>>>,
        not: Option<Box<Demographic>>,
    },
}

#[derive(Serialize)]
pub struct Filter {
    pub demographic: Demographic,
}

#[derive(Serialize)]
pub struct Limit {
    pub max: usize,
}

#[derive(Serialize)]
pub struct SendNarrowcastMessageRequest {
    #[serde(rename = "replyToken")]
    pub messages: Messages,
    pub recipient: Option<Recipient>,
    pub filter: Option<Filter>,
    pub limit: Option<Limit>,
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
}

pub struct SendNarrowcastMessageResponse {
    pub status: StatusCode,
    pub system_message: String,
}

pub async fn send_narrowcast_message(
    channel_access_token: &str,
    request: SendNarrowcastMessageRequest,
) -> Result<SendNarrowcastMessageResponse, Box<dyn Error>> {
    let request_json = serde_json::to_string(&request)?;

    let res = post_json(
        channel_access_token,
        SEND_NARROWCAST_MESSAGE_API,
        &request_json,
    )
    .await?;

    Ok(SendNarrowcastMessageResponse {
        status: res.status(),
        system_message: res.text().await?,
    })
}

#[derive(Serialize)]
pub struct GetNarrowcastMessageStatusRequest {
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Deserialize)]
pub struct GetNarrowcastMessageStatusResponse {
    #[serde(skip)]
    pub status: StatusCode,
    pub system_message: String,
    pub phase: String,
    pub success_count: Option<usize>,
    pub failure_count: Option<usize>,
    pub target_count: Option<usize>,
    pub failed_description: String,
    pub error_code: u8,
}

pub async fn get_narrowcast_message_status(
    channel_access_token: &str,
    request: GetNarrowcastMessageStatusRequest,
) -> Result<GetNarrowcastMessageStatusResponse, Box<dyn Error>> {
    let res: reqwest::Response = get(
        channel_access_token,
        GET_NARROWCAST_MESSAGE_STATUS_API,
        &request,
    )
    .await?;

    let status = res.status();
    let text = res.text().await?;

    let mut r: GetNarrowcastMessageStatusResponse = serde_json::from_str(&text)?;
    r.status = status;
    r.system_message = text;

    Ok(r)
}

#[derive(Serialize)]
pub struct SendBroadcastMessageRequest {
    pub messages: Messages,
}

pub struct SendBroadcastMessageResponse {
    pub status: StatusCode,
    pub request_id: reqwest::header::HeaderValue,
    pub system_message: String,
}

pub async fn send_broadcast_message(
    channel_access_token: &str,
    request: SendBroadcastMessageRequest,
) -> Result<SendBroadcastMessageResponse, Box<dyn Error>> {
    let request_json = serde_json::to_string(&request)?;

    let res = post_json(
        channel_access_token,
        SEND_BROADCAST_MESSAGE_API,
        &request_json,
    )
    .await?;

    Ok(SendBroadcastMessageResponse {
        status: res.status(),
        request_id: res.headers().get("X-Line-Request-Id").unwrap().clone(),
        system_message: res.text().await?,
    })
}

pub struct GetContentRequest {
    pub message_id: String,
}

pub struct GetContentResponse {
    pub status: StatusCode,
    pub system_message: String,
    pub content: Box<Bytes>,
}

pub async fn get_content(
    channel_access_token: &str,
    request: GetContentRequest,
) -> Result<GetContentResponse, Box<dyn Error>> {
    let res: reqwest::Response = get(
        channel_access_token,
        &get_content_api!((request.message_id)),
        &(),
    )
    .await?;

    let status = res.status();
    let res_bytes: Bytes = res.bytes().await?;
    let text = match String::from_utf8(res_bytes.to_vec()) {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    Ok(GetContentResponse {
        status,
        system_message: text,
        content: Box::from(res_bytes),
    })
}
