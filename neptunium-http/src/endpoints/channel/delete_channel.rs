use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct DeleteChannel {
    pub channel_id: Id<ChannelMarker>,
    pub silent: Option<bool>,
}

impl Endpoint for DeleteChannel {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        let path = if let Some(silent) = self.silent
            && silent
        {
            format!("/channels/{}?silent", self.channel_id)
        } else {
            format!("/channels/{}", self.channel_id)
        };

        Request::builder().method(Method::DELETE).path(path).build()
    }
}
