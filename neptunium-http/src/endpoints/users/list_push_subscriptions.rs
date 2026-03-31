use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListPushSubscriptions;

#[derive(Deserialize, Clone, Debug)]
pub struct ListPushSubscriptionsResponseEntry {
    pub subscription_id: String,
    pub user_agent: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ListPushSubscriptionsResponse {
    pub subscriptions: Vec<ListPushSubscriptionsResponseEntry>,
}

impl Endpoint for ListPushSubscriptions {
    type Response = ListPushSubscriptionsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/push/subscriptions".to_owned())
            .build()
    }
}
