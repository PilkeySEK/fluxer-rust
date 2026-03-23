use bon::Builder;
use neptunium_model::{
    channel::message::{Message, MessageFlags, embed::MessageEmbed},
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{
    endpoints::{
        Endpoint,
        channel::messages::{allowed_mentions::AllowedMentions, attachment::AttachmentRequest},
    },
    request::Request,
};

#[derive(Serialize, Clone, Debug, Builder)]
pub struct EditMessageUpdates {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<MessageEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
}

#[derive(Builder, Clone, Debug)]
pub struct EditMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub updates: EditMessageUpdates,
}

impl Endpoint for EditMessage {
    type Response = Message;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .path(format!(
                "/channels/{}/messages/{}",
                self.channel_id, self.message_id
            ))
            .body(serde_json::to_string(&self.updates).unwrap())
            .build()
    }
}
