use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    guild::permissions::Permissions,
    id::{
        Id,
        marker::{
            ChannelMarker, GenericMarker, GuildMarker, MessageMarker, UserMarker, VoiceRegionMarker,
        },
    },
    time::timestamp::{Timestamp, representations::Iso8601},
    user::User,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PermissionOverwrite {
    allow: Permissions,
    deny: Permissions,
    /// Can be either a role ID or a user ID
    id: Id<GenericMarker>,
    /// The type of entity this overwrite applies to. Must be either 0 or 1.
    #[serde(rename = "type")]
    r#type: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    /// The bitrate of the voice channel in bits per second
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<Id<GuildMarker>>,
    /// The icon hash of the channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_pin_timestamp: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    nicks: Option<HashMap<Id<UserMarker>, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    /// The ID of the owner of the channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rtc_region: Option<Id<VoiceRegionMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    // TODO: figure out what this number means (its the type of the channel)
    #[serde(rename = "type")]
    r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<i32>,
}
