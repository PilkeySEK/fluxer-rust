use serde::{Deserialize, Serialize};

use crate::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::UnixMillis},
};

/// Sent when a user starts typing in a channel, both in DMs and in guilds.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypingStart {
    channel_id: Id<ChannelMarker>,
    user_id: Id<UserMarker>,
    /// Timestamp of when the typing started.
    timestamp: Timestamp<UnixMillis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<GuildMember>,
}
