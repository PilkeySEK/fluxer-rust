use async_trait::async_trait;
use bon::Builder;
use fluxer_model::{
    channel::message::Message,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;

use crate::{
    channel::messages::message_create::CreateMessageBody, endpoints::Endpoint, request::Request,
};

pub mod reactions;

#[derive(Builder, Clone, Debug)]
pub struct CreateMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message: CreateMessageBody,
}

#[async_trait]
impl Endpoint for CreateMessage {
    type Response = Message;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .body(serde_json::to_string(&self.message).unwrap())
            .method(Method::POST)
            .path(format!("/channels/{}/messages", self.channel_id))
            .build()
    }
}
