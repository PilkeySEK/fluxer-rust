use bon::Builder;
use serde::Serialize;

use fluxer_model::{
    channel::message::{MessageFlags, embed::MessageEmbed, nonce::Nonce},
    id::{Id, marker::StickerMarker},
};

use crate::requests::channel::messages::{
    allowed_mentions::AllowedMentions, attachment::AttachmentRequest,
    message_reference::MessageReference,
};

/// [Source](https://github.com/fluxerapp/fluxer/blob/03813bbe17db008452f0f1be3090a7d2970a5447/packages/schema/src/domains/message/MessageRequestSchemas.tsx#L247)
#[derive(Builder, Serialize, Clone, Debug)]
pub struct CreateMessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[builder(default = vec![])]
    pub embeds: Vec<MessageEmbed>,
    pub attachments: Vec<AttachmentRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    #[builder(default)]
    pub flags: MessageFlags,
    #[builder(default = Nonce::generate())]
    pub nonce: Nonce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_meme_id: Option<String>, // TODO make this be Id<...>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<Id<StickerMarker>>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tts: Option<bool>,
    #[builder(default = false)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub tts: bool,
}

impl<T: Into<String>> From<T> for CreateMessageBody {
    fn from(value: T) -> Self {
        Self {
            content: Some(value.into()),
            embeds: vec![],
            attachments: vec![],
            message_reference: None,
            allowed_mentions: None,
            flags: MessageFlags::empty(),
            nonce: Nonce::generate(),
            favorite_meme_id: None,
            sticker_ids: None,
            tts: false,
        }
    }
}
