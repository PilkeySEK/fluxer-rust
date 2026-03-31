use reqwest::Method;

use crate::{
    endpoints::{Endpoint, channel::ScheduledMessageResponse},
    request::Request,
};

#[derive(Copy, Clone, Debug)]
pub struct ListScheduledMessages;

impl Endpoint for ListScheduledMessages {
    type Response = Vec<ScheduledMessageResponse>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/scheduled-messages".to_owned())
            .build()
    }
}
