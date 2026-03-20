use async_trait::async_trait;
use fluxer_model::{
    channel::message::Message,
    id::{Id, marker::UserMarker},
};

use crate::{client::error::Error, events::context::Context};

use neptunium_http::requests::channel::messages::{
    create_message::{CreateMessage, CreateMessageBody},
    delete_message::DeleteMessage,
    edit_message::{EditMessage, EditMessageUpdates},
    message_reference::MessageReference,
    reactions::{
        AddReaction, DeleteAllReactions, DeleteAllReactionsOfEmoji, DeleteOwnReaction,
        DeleteReaction,
    },
};

pub use neptunium_http::requests::channel::messages::reactions::RequestReactionType as Reaction;

#[async_trait]
pub trait MessageExt {
    async fn reply(
        &self,
        ctx: &Context,
        message_body: impl Into<CreateMessageBody> + Send,
    ) -> Result<Message, Error>;

    async fn add_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error>;

    async fn delete_own_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error>;

    async fn delete_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
        target: Id<UserMarker>,
    ) -> Result<(), Error>;

    async fn delete_all_reactions_of_emoji(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error>;

    async fn delete_all_reactions(&self, ctx: &Context) -> Result<(), Error>;

    /// Delete this message.
    async fn delete(&self, ctx: &Context) -> Result<(), Error>;

    /// Edit this message.
    async fn edit(&self, ctx: &Context, updates: EditMessageUpdates) -> Result<Message, Error>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply(
        &self,
        ctx: &Context,
        message_body: impl Into<CreateMessageBody> + Send,
    ) -> Result<Message, crate::client::error::Error> {
        let mut message_body = message_body.into();
        message_body.message_reference = Some(
            MessageReference::builder()
                .channel_id(self.channel_id)
                .message_id(self.id)
                .build(),
        );
        Ok(ctx
            .http_client
            .execute(
                CreateMessage::builder()
                    .channel_id(self.channel_id)
                    .message(message_body)
                    .build(),
            )
            .await?)
    }

    async fn add_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), crate::client::error::Error> {
        Ok(ctx
            .http_client
            .execute(
                AddReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .build(),
            )
            .await?)
    }

    async fn delete_own_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error> {
        Ok(ctx
            .http_client
            .execute(
                DeleteOwnReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .build(),
            )
            .await?)
    }

    async fn delete_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
        target: Id<UserMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .http_client
            .execute(
                DeleteReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .target(target)
                    .build(),
            )
            .await?)
    }

    async fn delete_all_reactions_of_emoji(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error> {
        Ok(ctx
            .http_client
            .execute(
                DeleteAllReactionsOfEmoji::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .build(),
            )
            .await?)
    }

    async fn delete_all_reactions(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .http_client
            .execute(
                DeleteAllReactions::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .build(),
            )
            .await?)
    }

    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .http_client
            .execute(
                DeleteMessage::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .build(),
            )
            .await?)
    }

    async fn edit(&self, ctx: &Context, updates: EditMessageUpdates) -> Result<Message, Error> {
        Ok(ctx
            .http_client
            .execute(
                EditMessage::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .updates(updates)
                    .build(),
            )
            .await?)
    }
}
