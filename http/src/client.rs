use bon::bon;
use fluxer_model::id::{Id, marker::ChannelMarker};
use reqwest::{Error, Method, Response};
use zeroize::Zeroizing;

use crate::{
    DEFAULT_API_BASE_URL, DEFAULT_USER_AGENT, VERSION,
    channel::messages::message_create::CreateMessageBody,
};

#[derive(Debug)]
pub struct HttpClient {
    api_base_url: String,
    token: Zeroizing<String>,
    reqwest_client: reqwest::Client,
    user_agent: String,
}

#[bon]
impl HttpClient {
    #[must_use]
    pub fn new(token: String) -> Self {
        Self {
            api_base_url: DEFAULT_API_BASE_URL.to_owned(),
            reqwest_client: reqwest::Client::default(),
            token: Zeroizing::new(token),
            user_agent: format!("{DEFAULT_USER_AGENT}/{VERSION}"),
        }
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    pub fn set_api_base_url(&mut self, url: String) {
        self.api_base_url = url;
    }

    #[builder]
    pub async fn create_message(
        &self,
        channel_id: Id<ChannelMarker>,
        body: &CreateMessageBody,
    ) -> Result<Response, Error> {
        let body = serde_json::to_string(body).unwrap();

        tracing::trace!("Sending create message request: {body:?}");

        let response = self
            .reqwest_client
            .request(
                Method::POST,
                format!("{}/channels/{}/messages", self.api_base_url, channel_id),
            )
            .header("Authorization", format!("Bot {}", *self.token))
            .header("User-Agent", &self.user_agent)
            .body(body)
            .send()
            .await;

        tracing::trace!("Response from create message request: {response:?}");

        response
    }
}
