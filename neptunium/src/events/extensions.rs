use async_trait::async_trait;
use fluxer_model::channel::message::Message;

use crate::events::context::Context;

use neptunium_http::{
    channel::messages::{message_create::CreateMessageBody, message_reference::MessageReference},
    endpoints::messages::{CreateMessage, reactions::AddReaction},
};

pub use neptunium_http::endpoints::messages::reactions::RequestReactionType as Reaction;

#[async_trait]
pub trait MessageExt {
    async fn reply(
        &self,
        ctx: &Context,
        data: impl Into<CreateMessageBody> + Send,
    ) -> Result<Message, crate::client::error::Error>;

    async fn add_reaction(
        &self,
        ctx: &Context,
        data: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), crate::client::error::Error>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply(
        &self,
        ctx: &Context,
        data: impl Into<CreateMessageBody> + Send,
    ) -> Result<Message, crate::client::error::Error> {
        let mut data = data.into();
        data.message_reference = Some(
            MessageReference::builder()
                .channel_id(self.channel_id)
                .message_id(self.id)
                .build(),
        );
        let response = ctx
            .http_client
            .execute(
                CreateMessage::builder()
                    .channel_id(self.channel_id)
                    .message(data)
                    .build(),
            )
            .await?;

        Ok(response)
    }

    async fn add_reaction(
        &self,
        ctx: &Context,
        data: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), crate::client::error::Error> {
        let response = ctx
            .http_client
            .execute(
                AddReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&data.into())
                    .build(),
            )
            .await?;

        Ok(response)
    }
}
