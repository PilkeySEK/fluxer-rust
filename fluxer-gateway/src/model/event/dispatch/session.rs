use fluxer_api::models::{
    ChannelResponse, CustomStatusResponse, FavoriteMemeResponse, UserGuildSettingsResponse,
    UserPartialResponse, UserPrivateResponse, UserSettingsResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

use crate::model::snowflake::Snowflake;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildReadyResponse {
    id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    unavailable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lazy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    joined_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PresenceResponse {
    user: UserPartialResponse,
    status: String,
    mobile: bool,
    afk: bool,
    custom_status: Option<CustomStatusResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReadStateResponse {
    id: Snowflake,
    mention_count: i32,
    last_message_id: Option<Snowflake>,
    #[serde(default, with = "time::serde::iso8601::option")]
    last_pin_timestamp: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyDispatchData {
    version: i32,
    session_id: String,
    user: UserPrivateResponse,
    guilds: Vec<GuildReadyResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_channels: Option<Vec<ChannelResponse>>,
    /// This is `Vec<Value>` because the docs say it's type is `object[]`
    #[serde(skip_serializing_if = "Option::is_none")]
    relationships: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<UserPartialResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presences: Option<Vec<PresenceResponse>>,
    /// This is `Vec<Value>` because the docs say it's type is `object[]`
    #[serde(skip_serializing_if = "Option::is_none")]
    sessions: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_settings: Option<UserSettingsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_guild_settings: Option<Vec<UserGuildSettingsResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_states: Option<Vec<ReadStateResponse>>,
    /// This is `Value` because the docs say it's type is `object`
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<Value>,
    /// Two-letter country code based on IP geolocation
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
    /// Array of pinned DM channel IDs
    ///
    /// **NOTE:** This was specified as "string[]" in the documentation, but
    /// appears to be a `Snowflake` since channel IDs are `Snowflake`s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned_dms: Option<Vec<Snowflake>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    favorite_memes: Option<Vec<FavoriteMemeResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_session_id_hash: Option<String>,
}
