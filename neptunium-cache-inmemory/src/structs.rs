use std::{collections::HashMap, sync::Arc};

use neptunium_model::{
    channel::{Channel, ChannelType, PermissionOverwrite, VoiceRegion},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};
use tokio::sync::RwLock;

use crate::Cached;

#[derive(Clone, Debug)]
pub struct CachedChannel {
    /// The bitrate of the voice channel in bits per second
    pub bitrate: Option<i32>,
    /// `None` if this is a DM channel.
    pub guild_id: Option<Id<GuildMarker>>,
    /// The icon hash of the channel (for group DMs)
    pub icon: Option<String>,
    pub id: Id<ChannelMarker>,
    pub last_message_id: Option<Id<MessageMarker>>,
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
    pub name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs)
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    pub nsfw: Option<bool>,
    /// The ID of the owner of the channel (for group DMs)
    pub owner_id: Option<Id<UserMarker>>,
    pub parent_id: Option<Id<ChannelMarker>>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<Cached<PartialUser>>>,
    pub rtc_region: Option<VoiceRegion>,
    pub topic: Option<String>,
    pub r#type: ChannelType,
    pub url: Option<String>,
    pub user_limit: Option<i32>,
}

impl CachedChannel {
    /// Converts this cached channel into a normal `Channel`. This is async because it needs to
    /// access all cached recipients to clone them, which are behind an `RwLock`.
    pub async fn into_channel(self) -> Channel {
        let recipients = if let Some(cached_recipients) = self.recipients {
            let mut recipients = Vec::with_capacity(cached_recipients.len());
            for recipient in cached_recipients {
                recipients.push((*recipient.read().await).clone());
            }
            Some(recipients)
        } else {
            None
        };

        Channel {
            bitrate: self.bitrate,
            guild_id: self.guild_id,
            icon: self.icon,
            id: self.id,
            last_message_id: self.last_message_id,
            last_pin_timestamp: self.last_pin_timestamp,
            name: self.name,
            nicks: self.nicks,
            nsfw: self.nsfw,
            owner_id: self.owner_id,
            parent_id: self.parent_id,
            permission_overwrites: self.permission_overwrites,
            position: self.position,
            rate_limit_per_user: self.rate_limit_per_user,
            recipients,
            rtc_region: self.rtc_region,
            topic: self.topic,
            r#type: self.r#type,
            url: self.url,
            user_limit: self.user_limit,
        }
    }
}

impl From<Channel> for CachedChannel {
    fn from(value: Channel) -> Self {
        let recipients = value.recipients.map(|recipients| {
            recipients
                .into_iter()
                .map(|recipient| Arc::new(RwLock::new(recipient)))
                .collect()
        });
        Self {
            bitrate: value.bitrate,
            guild_id: value.guild_id,
            icon: value.icon,
            id: value.id,
            last_message_id: value.last_message_id,
            last_pin_timestamp: value.last_pin_timestamp,
            name: value.name,
            nicks: value.nicks,
            nsfw: value.nsfw,
            owner_id: value.owner_id,
            parent_id: value.parent_id,
            permission_overwrites: value.permission_overwrites,
            position: value.position,
            rate_limit_per_user: value.rate_limit_per_user,
            recipients,
            rtc_region: value.rtc_region,
            topic: value.topic,
            r#type: value.r#type,
            url: value.url,
            user_limit: value.user_limit,
        }
    }
}
