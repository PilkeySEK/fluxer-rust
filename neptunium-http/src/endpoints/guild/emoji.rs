use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod bulk_create_guild_emojis;
pub mod create_guild_emoji;
pub mod delete_guild_emoji;
pub mod list_guild_emojis;
pub mod update_guild_emoji;

#[derive(Builder, Clone, Debug, Serialize)]
pub struct GuildEmojiCreateData {
    /// The emoji name. 2-32 characters, alphanumeric and underscores only.
    pub name: String,
    /// Base64 encoded image data.
    pub image: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EmojiCreateFailure {
    pub name: String,
    pub error: String,
}
