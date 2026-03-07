use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::EmojiMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::UserPartial,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PresenceStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "dnd")]
    DoNotDisturb,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomStatus {
    text: String,
    emoji_id: Option<Id<EmojiMarker>>,
    emoji_name: Option<String>,
    /// When the custom status expires.
    expires_at: Option<Timestamp<Iso8601>>,
}

/// Represents a user's presence (online status and activity).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Presence {
    user: UserPartial,
    status: PresenceStatus,
    mobile: bool,
    afk: bool,
    custom_status: Option<CustomStatus>,
}
