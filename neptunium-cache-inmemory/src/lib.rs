use std::{fmt::Debug, sync::Arc};

#[cfg(feature = "user_api")]
use neptunium_model::user::settings::UserSettings;

use bon::Builder;
use mini_moka::sync::Cache as MokaCache;
use neptunium_http::endpoints::users::UserProfileFullResponse;
use neptunium_model::{
    channel::message::Message,
    gateway::payload::incoming::UserPrivateResponse,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
    user::PartialUser,
};
use tokio::sync::OnceCell;

mod structs;
mod traits;
pub use structs::*;
pub use traits::*;

pub type Cached<T> = Arc<tokio::sync::RwLock<T>>;

#[expect(clippy::type_complexity)]
pub struct Cache {
    pub users: MokaCache<Id<UserMarker>, Cached<PartialUser>>,
    pub user_profiles:
        MokaCache<(Id<UserMarker>, Option<Id<GuildMarker>>), Cached<UserProfileFullResponse>>,
    pub channels: MokaCache<Id<ChannelMarker>, Cached<CachedChannel>>,
    pub messages: MokaCache<Id<MessageMarker>, Cached<Message>>,
    pub current_user: OnceCell<Cached<UserPrivateResponse>>,
    #[cfg(feature = "user_api")]
    pub current_user_settings: OnceCell<Cached<UserSettings>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct CacheConfig {
    #[builder(default = 1024)]
    pub users: u64,
    #[builder(default = 256)]
    pub user_profiles: u64,
    #[builder(default = 1024)]
    pub channels: u64,
    #[builder(default = 1024)]
    pub messages: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Cache {
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        Self {
            users: MokaCache::new(config.users),
            user_profiles: MokaCache::new(config.user_profiles),
            channels: MokaCache::new(config.channels),
            messages: MokaCache::new(config.messages),
            current_user: OnceCell::new(),
            #[cfg(feature = "user_api")]
            current_user_settings: OnceCell::new(),
        }
    }
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cache { ... }")
    }
}
