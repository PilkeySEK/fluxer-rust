use bon::Builder;
use neptunium_model::id::{Id, marker::ScheduledMessageMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct CancelScheduledMessage {
    pub scheduled_message_id: Id<ScheduledMessageMarker>,
}

impl Endpoint for CancelScheduledMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/users/@me/scheduled-messages/{}",
                self.scheduled_message_id
            ))
            .build()
    }
}
