use async_trait::async_trait;
use fluxer_model::channel::message::Message;
use reqwest::StatusCode;

use crate::{client::error::Error, events::context::Context};

use neptunium_http::channel::messages::{
    message_create::CreateMessageBody, message_reference::MessageReference,
};

#[async_trait]
pub trait MessageExt {
    async fn reply(
        &self,
        ctx: &Context,
        data: impl Into<CreateMessageBody> + Send,
    ) -> Result<(), crate::client::error::Error>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply(
        &self,
        ctx: &Context,
        data: impl Into<CreateMessageBody> + Send,
    ) -> Result<(), crate::client::error::Error> {
        // let response = ctx
        //     .messages(self.channel_id)
        //     .create()
        //     .body(data)
        //     .reply_to(self.id)
        //     .await?;
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
                crate::client::error::ClientErrorKind::HttpStatusNot200(response),
            ));
        }
        // TODO change to trace or remove
        tracing::debug!("Response body: {:?}", response.text().await);
        Ok(())

        // let mut data = data.into();
        // data.message_reference = Some(MessageReference {
        //     message_id: self.id,
        //     channel_id: Some(self.channel_id),
        //     guild_id: None,
        //     r#type: MessageReferenceType::Reply,
        // });
        // let response = ctx
        //     .http_client
        //     .messages(self.channel_id)
        //     .create(&data)
        //     .await?;
        // if response.status() != StatusCode::OK {
        //     return Err(Error::new(
        //         crate::client::error::ClientErrorKind::HttpStatusNot200(response),
        //     ));
        // }
        // Ok(())
    }
}
