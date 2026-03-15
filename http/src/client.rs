use zeroize::Zeroizing;

use crate::{
    DEFAULT_API_BASE_URL, DEFAULT_USER_AGENT, VERSION,
    endpoints::{Endpoint, ExecuteEndpointRequestError, ResponseBody},
};

#[derive(Debug)]
pub struct HttpClient {
    pub(crate) api_base_url: String,
    pub(crate) token: Zeroizing<String>,
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) user_agent: String,
}

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

    /// Send a request to the specified endpoint, returning the result.
    /// # Errors
    /// Returns an error if the response is not what is expected by the given endpoint or if a network error occurs.
    pub async fn execute<T: Endpoint + Send>(
        &self,
        endpoint: T,
    ) -> Result<T::Response, Box<ExecuteEndpointRequestError>> {
        let request = endpoint.into_request();
        let response = request.execute(self).await?;
        tracing::trace!("API response: {:?}", response);
        // TODO: Check for errors here
        let body = response.bytes().await?.to_vec();
        tracing::trace!("API response body: {:?}", body);
        ResponseBody::deserialize(body)
    }
    /*
        #[builder]
        pub async fn create_message(
            &self,
            channel_id: Id<ChannelMarker>,
            body: &CreateMessageBody,
        ) -> Result<Response, Error> {
            let body = serde_json::to_string(body).unwrap();

            let mut req = Request::from_route(&Route::CreateMessage { channel_id });
            req.body = Some(body.as_bytes().to_vec());

            req.execute(self).await
        }

        #[builder]
        pub async fn add_reaction(
            &self,
            channel_id: Id<ChannelMarker>,
            message_id: Id<MessageMarker>,
            /// For default emojis, this should be the unicode emoji.
            #[builder(into)]
            emoji: RequestReactionType<'_>,
        ) -> Result<Response, Error> {
            let req = Request::from_route(&Route::CreateReaction {
                channel_id,
                emoji: &emoji,
                message_id,
            });
            req.execute(self).await
        }

        #[builder]
        pub async fn remove_own_reaction(
            &self,
            channel_id: Id<ChannelMarker>,
            message_id: Id<MessageMarker>,
            /// For default emojis, this should be the unicode emoji.
            #[builder(into)]
            emoji: RequestReactionType<'_>,
        ) -> Result<Response, Error> {
            let req = Request::from_route(&Route::RemoveOwnReaction {
                channel_id,
                emoji: &emoji,
                message_id,
            });
            req.execute(self).await
        }

        #[builder]
        pub async fn remove_all_reactions(
            &self,
            channel_id: Id<ChannelMarker>,
            message_id: Id<MessageMarker>,
        ) -> Result<Response, Error> {
            let req = Request::from_route(&Route::RemoveAllReactions {
                channel_id,
                message_id,
            });
            req.execute(self).await
        }

        #[builder]
        pub async fn list_reactions(
            &self,
            channel_id: Id<ChannelMarker>,
            message_id: Id<MessageMarker>,
            /// For default emojis, this should be the unicode emoji.
            #[builder(into)]
            emoji: RequestReactionType<'_>,
        ) -> Result<Response, Error> {
            let req = Request::from_route(&Route::ListReactionsForEmoji {
                channel_id,
                message_id,
                emoji: &emoji,
            });
            req.execute(self).await
        }
    */
}
