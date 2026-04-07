//! Demonstration of how to use user-bots (logging in as a user). You need to have the "user_api" feature enabled on fluxer-neptunium.

use std::{env, sync::Arc};

use fluxer_neptunium::{
    cached_payload::{CachedMessageCreate, CachedReady},
    http::client::TokenType,
    model::gateway::payload::incoming::PassiveUpdates,
    prelude::*,
};
use tracing_subscriber::filter::LevelFilter;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_ready(&self, _ctx: Context, data: Arc<CachedReady>) -> Result<(), EventError> {
        let user = data.user.load();
        println!("Logged in as {}#{}", user.username, user.discriminator);
        Ok(())
    }

    async fn on_message_create(
        &self,
        ctx: Context,
        event: Arc<CachedMessageCreate>,
    ) -> Result<(), EventError> {
        let message = event.message.load();
        let author = message.author.load();
        println!(
            "{}#{}: {}",
            author.username, author.discriminator, message.content
        );
        let current_user_id = ctx.get_own_profile().await?.load().id;
        if author.id == current_user_id && message.content == "u?ping" {
            message.reply(&ctx, "Pong!").await?;
        }
        Ok(())
    }

    // PASSIVE_UPDATES is a client-only gateway dispatch event
    async fn on_passive_updates(
        &self,
        _ctx: Context,
        _data: Arc<PassiveUpdates>,
    ) -> Result<(), EventError> {
        // println!("Passive updates received: {:?}", data);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    let mut client = Client::new_with_config(
        env::var("FLUXER_USER_TOKEN").unwrap(),
        ClientConfig::builder().token_type(TokenType::User).build(),
    );

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
