use bon::Builder;
use fluxer_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, requests::Request};

#[derive(Builder)]
pub struct DeleteMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for DeleteMessage {
    type Response = ();

    fn into_request(self) -> crate::requests::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
