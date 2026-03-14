use std::sync::Arc;

use async_trait::async_trait;
use fluxer_model::gateway::payload::incoming::{
    guild_create::GuildCreate, guild_delete::GuildDelete, message_create::MessageCreate,
    ready::Ready, typing_start::TypingStart,
};

use crate::events::context::Context;

pub mod context;

#[expect(unused)]
#[async_trait]
pub trait EventHandler: Send {
    #[inline]
    async fn on_ready(&self, ctx: Context, data: Arc<Ready>) {}
    #[inline]
    async fn on_message(&self, ctx: Context, data: Arc<MessageCreate>) {}
    #[inline]
    async fn on_guild_create(&self, ctx: Context, data: Arc<GuildCreate>) {}
    #[inline]
    async fn on_guild_delete(&self, ctx: Context, data: Arc<GuildDelete>) {}
    #[inline]
    async fn on_typing_start(&self, ctx: Context, data: Arc<TypingStart>) {}
}
