use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use crate::{client::client_config::GatewayIntents, model::snowflake::Snowflake};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ActivityType {
    Playing = 0,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Timestamps {
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::timestamp::milliseconds::option"
    )]
    pub start: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::timestamp::milliseconds::option"
    )]
    pub end: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Emoji {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum StatusDisplayType {
    Name = 0,
    State,
    Details,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `(current_size, max_size)`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<(i32, i32)>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_cover_image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActivitySecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
}

bitflags! {
    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    pub struct ActivityFlags: u32 {
        const INSTANCE = 1 << 0;
        const JOIN = 1 << 1;
        const SPECTATE = 1 << 2;
        const JOIN_REQUEST = 1 << 3;
        const SYNC = 1 << 4;
        const PLAY = 1 << 5;
        const PARTY_PRIVACY_FRIENDS = 1 << 6;
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
        const EMBEDDED = 1 << 8;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

/// Bot users are only able to set `name`, `state`, `type`, and `url`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ActivityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub created_at: OffsetDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Timestamps>,
    /// Application ID for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<StatusDisplayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ActivityFlags>,
    /// Max. 2 buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePresence {
    /// Time when the client went idle, or `None` if the client is not idle
    #[serde(with = "time::serde::timestamp::milliseconds::option")]
    pub since: Option<OffsetDateTime>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionProperties {
    /// Operating system, e.g. "linux"
    pub os: String,
    /// The library name
    pub browser: String,
    /// The library name
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifyEventData {
    pub token: String,
    pub properties: ConnectionProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    /// Value between 50 and 250, total number of members where the
    /// gateway will stop sending offline members in the guild member list.
    /// The default is 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i32>,
    /// `(shard_id, num_shards)`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<(i32, i32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<UpdatePresence>,
    pub intents: GatewayIntents,
}
