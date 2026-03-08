use async_trait::async_trait;
use fluxer_model::gateway::payload::incoming::ready::Ready;

use crate::events::context::Context;

pub mod context;

#[async_trait]
pub trait EventHandler {
    async fn on_ready(&mut self, ctx: Context, data: Ready);
}
