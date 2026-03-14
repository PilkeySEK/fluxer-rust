use async_trait::async_trait;
use fluxer_model::channel::message::Message;
use reqwest::StatusCode;

use crate::{client::error::Error, events::context::Context};

use neptunium_http::channel::messages::{
    message_create::CreateMessageBody, message_reference::MessageReference,
};

pub use neptunium_http::channel::reactions::RequestReactionType as Reaction;

#[async_trait]
pub trait MessageExt {
    async fn reply(
        &self,
        ctx: &Context,
        data: impl Into<CreateMessageBody> + Send,
    ) -> Result<(), crate::client::error::Error>;

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
    ) -> Result<(), crate::client::error::Error> {
        let mut data = data.into();
        data.message_reference = Some(
            MessageReference::builder()
                .channel_id(self.channel_id)
                .message_id(self.id)
                .build(),
        );
        let response = ctx
            .http_client
            .create_message()
            .channel_id(self.channel_id)
            .body(&data)
            .call()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(Error::new(
                crate::client::error::ClientErrorKind::HttpStatusNotOk(response),
            ));
        }
        Ok(())
    }

    async fn add_reaction(
        &self,
        ctx: &Context,
        data: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), crate::client::error::Error> {
        let response = ctx
            .http_client
            .add_reaction()
            .channel_id(self.channel_id)
            .message_id(self.id)
            .emoji(data)
            .call()
            .await?;

        if response.status() != StatusCode::NO_CONTENT {
            return Err(Error::new(
                crate::client::error::ClientErrorKind::HttpStatusNotOk(response),
            ));
        }
        Ok(())
    }
}
