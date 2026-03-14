use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, MessageMarker},
};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, Default)]
#[repr(u8)]
pub enum MessageReferenceType {
    #[default]
    Reply = 0,
    Forward = 1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReference {
    /// The ID of the channel containing the referenced message.
    pub channel_id: Id<ChannelMarker>,
    /// The ID of the guild containing the referenced message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// The ID of the referenced message.
    pub message_id: Id<MessageMarker>,
    #[serde(rename = "type")]
    pub r#type: MessageReferenceType,
}
